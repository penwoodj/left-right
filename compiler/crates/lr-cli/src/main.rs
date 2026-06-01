use clap::{Parser, Subcommand};
use anyhow::Result;
use colored::Colorize;
use std::cell::RefCell;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::time::Duration;
use std::sync::mpsc::channel;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use lr_vm::ModuleResolver;
use lr_bytecode::Chunk;

struct CliModuleResolver;

impl ModuleResolver for CliModuleResolver {
    fn resolve_and_compile(&mut self, import_path: &str, current_file: &str) -> Result<Chunk, String> {
        let current_dir = Path::new(current_file).parent().unwrap_or(Path::new("."));
        let mut resolved = current_dir.join(import_path);

        if resolved.extension().is_none() {
            resolved.set_extension("lr");
        }

        let resolved_str = resolved.to_string_lossy().to_string();

        let source = std::fs::read_to_string(&resolved)
            .map_err(|e| format!("Cannot read module '{}': {}", resolved_str, e))?;

        lr_compiler::compile_source_with_name(&source, &resolved_str)
            .map_err(|e| format!("Compile error in module '{}': {}", resolved_str, e))
    }
}

#[derive(Parser)]
#[command(name = "lr", version, about = "Left-Right programming language")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        file: Option<String>,
    },
    Compile {
        file: String,
    },
    Check {
        file: String,
    },
    Repl,
    Fmt {
        file: String,
    },
    New {
        name: String,
    },
    Build,
    Test,
    Watch {
        file: String,
    },
    Transpile {
        file: String,
        #[arg(long, default_value = "js")]
        target: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run { file } => cmd_run(file),
        Commands::Compile { file } => cmd_compile(&file),
        Commands::Check { file } => cmd_check(&file),
        Commands::Repl => cmd_repl(),
        Commands::Fmt { file } => cmd_fmt(&file),
        Commands::New { name } => cmd_new(&name),
        Commands::Build => cmd_build(),
        Commands::Test => cmd_test(),
        Commands::Watch { file } => cmd_watch(&file),
        Commands::Transpile { file, target } => cmd_transpile(&file, &target),
    }
}

fn cmd_run(file: Option<String>) -> Result<()> {
    match file {
        Some(path) => {
            let source = std::fs::read_to_string(&path)
                .map_err(|_| anyhow::anyhow!("File not found: {}", path))?;
            run_source(&source, &path)?;
        }
        None => {
            eprintln!("{}", "Usage: lr run <file>".red());
            std::process::exit(1);
        }
    }
    Ok(())
}

fn run_source(source: &str, source_name: &str) -> Result<()> {
    match lr_compiler::compile_source_with_name(source, source_name) {
        Ok(chunk) => {
            let resolver = Rc::new(RefCell::new(CliModuleResolver));
            let mut vm = lr_vm::VM::with_resolver(source_name.to_string(), resolver);
            match vm.execute(&chunk) {
                Ok(result) => println!("{}", result),
                Err(e) => {
                    eprintln!("{}", format!("Runtime error: {}", e).red());
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            if let Some(span) = e.span() {
                let diag = lr_diagnostics::Diagnostic::error(span, e.to_string(), source_name);
                diag.eprint(source);
            } else {
                eprintln!("{}", format!("Error: {}", e).red());
            }
            std::process::exit(1);
        }
    }
    Ok(())
}

fn cmd_compile(file: &str) -> Result<()> {
    let source = std::fs::read_to_string(file)
        .map_err(|_| anyhow::anyhow!("File not found: {}", file))?;

    match lr_compiler::compile_source_with_name(&source, file) {
        Ok(chunk) => {
            println!("{}:", "Bytecode".green());
            println!("{} instructions", chunk.code.len());

            for (offset, instr) in chunk.code.iter().enumerate() {
                println!("  {}: {}", offset, instr);
            }

            if !chunk.constants.is_empty() {
                println!("\n{}:", "Constants".green());
                for (idx, constant) in chunk.constants.iter().enumerate() {
                    match constant {
                        lr_bytecode::Constant::Undefined => {
                            println!("  constants[{}] = Undefined", idx);
                        }
                        lr_bytecode::Constant::Boolean(b) => {
                            println!("  constants[{}] = Boolean({})", idx, b);
                        }
                        lr_bytecode::Constant::Number(n) => {
                            println!("  constants[{}] = Number({})", idx, n);
                        }
                        lr_bytecode::Constant::String(s) => {
                            println!("  constants[{}] = String(\"{}\")", idx, s);
                        }
                    }
                }
            }
        }
        Err(e) => {
            if let Some(span) = e.span() {
                let diag = lr_diagnostics::Diagnostic::error(span, e.to_string(), file);
                diag.eprint(&source);
            } else {
                eprintln!("{}", format!("Error: {}", e).red());
            }
            std::process::exit(1);
        }
    }
    Ok(())
}

fn cmd_check(file: &str) -> Result<()> {
    let source = std::fs::read_to_string(file)
        .map_err(|_| anyhow::anyhow!("File not found: {}", file))?;

    let tokens = match lr_lexer::tokenize(&source) {
        Ok(t) => t,
        Err(errs) => {
            for err in &errs {
                let diag = lr_diagnostics::Diagnostic::error(err.span(), err.to_string(), file);
                diag.eprint(&source);
            }
            std::process::exit(1);
        }
    };

    let _program = match lr_parser::parse(tokens, file.to_string()) {
        Ok(p) => p,
        Err(err) => {
            let diag = lr_diagnostics::Diagnostic::error(err.span(), err.to_string(), file);
            diag.eprint(&source);
            std::process::exit(1);
        }
    };

    println!("{}", "OK".green());
    Ok(())
}

fn cmd_repl() -> Result<()> {
    println!("{}", "Left-Right REPL".green());
    println!("Type :quit or :q to exit, :help for help\n");

    let mut rl = DefaultEditor::new()?;
    rl.bind_sequence(rustyline::KeyEvent::ctrl('c'), rustyline::Cmd::Interrupt);

    loop {
        let prompt = "lr> ".cyan();
        let mut input = String::new();
        let mut depth: i32 = 0;

        loop {
            let cont_prompt = if depth > 0 { "..> ".dimmed().cyan() } else { prompt.clone() };
            let line = match rl.readline(&cont_prompt.to_string()) {
                Ok(l) => l,
                Err(ReadlineError::Interrupted) => {
                    println!("^C");
                    if input.is_empty() {
                        break;
                    }
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    println!("{}", "Goodbye!".green());
                    return Ok(());
                }
                Err(e) => return Err(e.into()),
            };

            let trimmed = line.trim();
            if trimmed.is_empty() && input.is_empty() {
                break;
            }

            if !trimmed.is_empty() {
                if input.is_empty() {
                    input = line.clone();
                } else {
                    input.push('\n');
                    input.push_str(&line);
                }

                for ch in trimmed.chars() {
                    match ch {
                        '{' | '[' | '(' => depth += 1,
                        '}' | ']' | ')' => depth = depth.saturating_sub(1),
                        _ => {}
                    }
                }

                if depth == 0 {
                    break;
                }
            }
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some(cmd) = trimmed.strip_prefix(':') {
            match cmd {
                "quit" | "q" => {
                    println!("{}", "Goodbye!".green());
                    break;
                }
                "help" | "h" => {
                    println!("Commands:");
                    println!("  :quit, :q  - Exit REPL");
                    println!("  :help, :h  - Show this help");
                    println!("  :clear     - Clear screen");
                    println!();
                    println!("Enter Left-Right expressions to evaluate them.");
                }
                "clear" => {
                    print!("\x1b[2J\x1b[1;1H");
                    io::stdout().flush()?;
                }
                _ => {
                    println!("{}", format!("Unknown command: :{}", cmd).red());
                }
            }
        } else {
            rl.add_history_entry(input.clone())?;
            match run_source(trimmed, "<repl>") {
                Ok(()) => {}
                Err(_) => {}
            }
        }
    }

    Ok(())
}

fn cmd_fmt(_file: &str) -> Result<()> {
    println!("{}", "Not yet implemented".yellow());
    Ok(())
}

fn cmd_new(name: &str) -> Result<()> {
    let project_dir = Path::new(name);

    if project_dir.exists() {
        eprintln!("{}", format!("Directory '{}' already exists", name).red());
        std::process::exit(1);
    }

    println!("{}", format!("Creating new Left-Right project: {}", name).green());

    std::fs::create_dir_all(project_dir)?;
    std::fs::create_dir_all(project_dir.join("src"))?;
    std::fs::create_dir_all(project_dir.join("tests"))?;

    let package_content = format!(
        "name: {}\nversion: 0.1.0\nentry: src/main.lr\n",
        name
    );
    std::fs::write(project_dir.join("package.lr"), package_content)?;

    std::fs::write(project_dir.join("src/main.lr"), "42")?;

    std::fs::write(project_dir.join("tests/test.lr"), "")?;

    println!("  {}", format!("{}/package.lr", name).cyan());
    println!("  {}", format!("{}/src/main.lr", name).cyan());
    println!("  {}", format!("{}/tests/test.lr", name).cyan());
    println!();
    println!("{}", "Project created successfully!".green());
    println!("{}", "Run 'cd {} && lr build' to build the project.".replace("{}", name).cyan());

    Ok(())
}

fn cmd_build() -> Result<()> {
    let package_path = Path::new("package.lr");
    if !package_path.exists() {
        eprintln!("{}", "package.lr not found. Run this command in a Left-Right project directory.".red());
        std::process::exit(1);
    }

    let package_content = std::fs::read_to_string(package_path)?;
    let entry_line = package_content
        .lines()
        .find(|line| line.starts_with("entry:"))
        .ok_or_else(|| anyhow::anyhow!("No 'entry' field in package.lr"))?;

    let entry_path = entry_line
        .strip_prefix("entry:")
        .map(|s| s.trim())
        .ok_or_else(|| anyhow::anyhow!("Invalid entry line format"))?;

    let source_path = Path::new(entry_path);
    if !source_path.exists() {
        eprintln!("{}", format!("Entry file '{}' not found", entry_path).red());
        std::process::exit(1);
    }

    println!("{}", format!("Building {}...", entry_path).cyan());

    let source = std::fs::read_to_string(source_path)?;

    match lr_compiler::compile_source_with_name(&source, entry_path) {
        Ok(chunk) => {
            println!("{}", format!("  {} instructions", chunk.code.len()).green());
            println!("{}", format!("  {} constants", chunk.constants.len()).green());
            println!();
            println!("{}", "Build successful!".green());
        }
        Err(e) => {
            if let Some(span) = e.span() {
                let diag = lr_diagnostics::Diagnostic::error(span, e.to_string(), entry_path);
                diag.eprint(&source);
            } else {
                eprintln!("{}", format!("Build error: {}", e).red());
            }
            std::process::exit(1);
        }
    }

    Ok(())
}

fn cmd_test() -> Result<()> {
    let tests_dir = Path::new("tests");
    if !tests_dir.exists() {
        println!("{}", "No tests directory found".yellow());
        return Ok(());
    }

    let test_files: Vec<_> = std::fs::read_dir(tests_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension().is_some_and(|ext| ext == "lr")
        })
        .collect();

    if test_files.is_empty() {
        println!("{}", "No test files found".yellow());
        return Ok(());
    }

    println!("{}", format!("Running {} test(s)...", test_files.len()).cyan());
    println!();

    let mut passed = 0;
    let mut failed = 0;

    for test_file in &test_files {
        let path = test_file.path();
        let _display_name = path.display();
        print!("  {} ... ", path.file_name().unwrap().to_string_lossy().cyan());

        let source = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                println!("{}", "ERROR".red());
                eprintln!("    {}", e);
                failed += 1;
                continue;
            }
        };

        if source.trim().is_empty() {
            println!("{}", "SKIP".yellow());
            continue;
        }

        match lr_compiler::compile_source_with_name(&source, path.display().to_string().as_str()) {
            Ok(chunk) => {
                let resolver = Rc::new(RefCell::new(CliModuleResolver));
                let mut vm = lr_vm::VM::with_resolver(path.display().to_string(), resolver);
                match vm.execute(&chunk) {
                    Ok(result) => {
                        let is_truthy = result != "undefined" && result != "false" && result != "0";
                        if is_truthy {
                            println!("{}", "PASS".green());
                            passed += 1;
                        } else {
                            println!("{}", "FAIL".red());
                            eprintln!("    Result: {}", result);
                            failed += 1;
                        }
                    }
                    Err(e) => {
                        println!("{}", "FAIL".red());
                        eprintln!("    Runtime error: {}", e);
                        failed += 1;
                    }
                }
            }
            Err(e) => {
                println!("{}", "FAIL".red());
                eprintln!("    Compile error: {}", e);
                failed += 1;
            }
        }
    }

    println!();
    println!("{}", "Test Results:".cyan());
    println!("  {} passed, {} failed", passed, failed);

    if failed > 0 {
        std::process::exit(1);
    }

    Ok(())
}

fn cmd_watch(file: &str) -> Result<()> {
    let file_path = PathBuf::from(file);
    if !file_path.exists() {
        eprintln!("{}", format!("File '{}' not found", file).red());
        std::process::exit(1);
    }

    println!("{}", format!("Watching {} for changes...", file).cyan());
    println!("{}", "Press Ctrl+C to stop".yellow());
    println!();

    let watch_path = file_path.clone();
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(
        move |res: Result<notify::Event, _>| {
            if let Ok(event) = res
                && event.kind.is_modify()
                && let Some(path) = event.paths.first()
                && path == &watch_path
            {
                let _ = tx.send(()).ok();
            }
        },
        notify::Config::default(),
    )?;

    watcher.watch(&file_path, RecursiveMode::NonRecursive)?;

    loop {
        let source = std::fs::read_to_string(&file_path)?;
        println!("\n{} {} {}",
                 "\x1b[2K\r".clear(),
                 "Running".cyan(),
                 file);

        match lr_compiler::compile_source_with_name(&source, file) {
            Ok(chunk) => {
                let resolver = Rc::new(RefCell::new(CliModuleResolver));
                let mut vm = lr_vm::VM::with_resolver(file.to_string(), resolver);
                match vm.execute(&chunk) {
                    Ok(result) => {
                        println!("  {}", result.green());
                    }
                    Err(e) => {
                        println!("  {}", format!("Error: {}", e).red());
                    }
                }
            }
            Err(e) => {
                println!("  {}", format!("Error: {}", e).red());
            }
        }

        println!("Watching for changes...");

        rx.recv_timeout(Duration::from_secs(1))?;
    }
}

fn cmd_transpile(file: &str, target: &str) -> Result<()> {
    if target != "js" {
        eprintln!("{}", format!("Unsupported target: {}", target).red());
        std::process::exit(1);
    }

    let source = std::fs::read_to_string(file)
        .map_err(|_| anyhow::anyhow!("File not found: {}", file))?;

    match lr_codegen_js::transpile_source_with_name(&source, file) {
        Ok(js) => println!("{}", js),
        Err(e) => {
            eprintln!("{}", format!("Transpile error: {}", e).red());
            std::process::exit(1);
        }
    }
    Ok(())
}