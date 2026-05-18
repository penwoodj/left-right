use clap::{Parser, Subcommand};
use anyhow::Result;
use colored::Colorize;
use std::io::{self, Write, BufRead};

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
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run { file } => cmd_run(file),
        Commands::Compile { file } => cmd_compile(&file),
        Commands::Check { file } => cmd_check(&file),
        Commands::Repl => cmd_repl(),
        Commands::Fmt { file } => cmd_fmt(&file),
    }
}

fn cmd_run(file: Option<String>) -> Result<()> {
    match file {
        Some(path) => {
            let source = std::fs::read_to_string(&path)
                .map_err(|_| anyhow::anyhow!("File not found: {}", path))?;
            run_source(&source)?;
        }
        None => {
            eprintln!("{}", "Usage: lr run <file>".red());
            std::process::exit(1);
        }
    }
    Ok(())
}

fn run_source(source: &str) -> Result<()> {
    match lr_compiler::compile_source(source) {
        Ok(chunk) => {
            let mut vm = lr_vm::VM::new();
            match vm.execute(&chunk) {
                Ok(result) => println!("{}", result),
                Err(e) => {
                    eprintln!("{}", format!("Runtime error: {}", e).red());
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("{}", format!("Error: {}", e).red());
            std::process::exit(1);
        }
    }
    Ok(())
}

fn cmd_compile(file: &str) -> Result<()> {
    let source = std::fs::read_to_string(file)
        .map_err(|_| anyhow::anyhow!("File not found: {}", file))?;

    match lr_compiler::compile_source(&source) {
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
            eprintln!("{}", format!("Error: {}", e).red());
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
                eprintln!("{}", format!("Lexer error: {}", err).red());
            }
            std::process::exit(1);
        }
    };

    let _program = match lr_parser::parse(tokens, file.to_string()) {
        Ok(p) => p,
        Err(err) => {
            eprintln!("{}", format!("Parse error: {}", err).red());
            std::process::exit(1);
        }
    };

    println!("{}", "OK".green());
    Ok(())
}

fn cmd_repl() -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    println!("{}", "Left-Right REPL".green());
    println!("Type :quit or :q to exit, :help for help\n");

    loop {
        print!("{} ", "lr>".cyan());
        stdout.flush()?;

        let mut line = String::new();
        stdin.lock().read_line(&mut line)?;

        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        match trimmed {
            ":quit" | ":q" => {
                println!("{}", "Goodbye!".green());
                break;
            }
            ":help" | ":h" => {
                println!("Commands:");
                println!("  :quit, :q  - Exit REPL");
                println!("  :help, :h  - Show this help");
                println!();
                println!("Enter Left-Right expressions to evaluate them.");
            }
            _ => {
                match run_source(trimmed) {
                    Ok(()) => {}
                    Err(_) => {
                        // Error already printed by run_source
                    }
                }
            }
        }
    }

    Ok(())
}

fn cmd_fmt(_file: &str) -> Result<()> {
    println!("{}", "Not yet implemented".yellow());
    Ok(())
}