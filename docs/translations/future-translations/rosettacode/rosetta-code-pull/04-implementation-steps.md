# Implementation Steps

## Step-by-Step Plan

This document outlines the implementation of the Rosetta Code scraping tool.

## Phase 1: Project Setup

### Step 1.1: Initialize Rust Project

```bash
cd ~/code/left-right
cargo new --name rosetta-scraper --bin rosetta-scraper
cd rosetta-scraper
```

### Step 1.2: Add Dependencies

Edit `Cargo.toml`:

```toml
[package]
name = "rosetta-scraper"
version = "0.1.0"
edition = "2021"

[dependencies]
reqwest = { version = "0.11", features = ["json", "cookies"] }
scraper = "0.17"
select = "0.6"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
thiserror = "1.0"
clap = { version = "4.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
dirs = "5.0"
```

### Step 1.3: Create Directory Structure

```bash
mkdir -p cache/
mkdir -p reports/rosetta-code/language-examples/
```

## Phase 2: Configuration

### Step 2.1: Define Constants

Create `src/config.rs`:

```rust
pub const TARGET_LANGUAGES: &[&str] = &[
    "Python", "Rust", "JavaScript", "Haskell", "OCaml", "Elixir", "Erlang",
    "Clojure", "Ruby", "R", "Kotlin", "Scala", "Groovy", "CoffeeScript",
    "BQN", "J", "Arturo", "Wren", "Mercury", "GAP", "newLISP",
    "REBOL", "OI", "Ursala", "Slate", "Raven", "PowerShell",
    "Lingo", "Elm",
];

pub const BASE_URL: &str = "https://rosettacode.org/wiki";

pub const RATE_LIMIT_MS: u64 = 1000; // 1 second
pub const PAUSE_EVERY: usize = 50;
pub const PAUSE_DURATION_MS: u64 = 10_000; // 10 seconds
pub const REQUEST_TIMEOUT_SECS: u64 = 30;

pub const CACHE_DIR: &str = "cache";
pub const OUTPUT_DIR: &str = "reports/rosetta-code/language-examples";
```

### Step 2.2: CLI Arguments

Create `src/cli.rs`:

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rosetta-scraper")]
#[command(about = "Scrape Rosetta Code examples for Left-Right validation")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Fetch all examples
    Fetch {
        /// Language to fetch (default: all)
        #[arg(short, long)]
        language: Option<String>,

        /// Category to fetch (default: all)
        #[arg(short, long)]
        category: Option<String>,

        /// Force refresh, ignore cache
        #[arg(short, long)]
        force: bool,

        /// Dry run, show what would be fetched
        #[arg(short, long)]
        dry_run: bool,
    },

    /// List available tasks
    List {
        /// Category to list tasks for
        category: Option<String>,
    },

    /// Validate cache integrity
    Validate,
}
```

## Phase 3: Task Discovery

### Step 3.1: Fetch Task List

Create `src/discovery.rs`:

```rust
use anyhow::{Context, Result};
use reqwest::Client;
use scraper::{Html, Selector};

pub async fn fetch_task_list(category: &str) -> Result<Vec<String>> {
    let url = format!("{}/Category:{}", BASE_URL, category);
    let html = fetch_html(&url).await?;

    let selector = Selector::parse("li a[href^='/wiki/']")?;
    let mut tasks = Vec::new();

    for element in html.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            let task_name = href.strip_prefix("/wiki/")
                .context("Invalid task URL format")?;
            // URL decode
            let decoded = urlencoding::decode(task_name, true)
                .context("Failed to decode task name")?;
            tasks.push(decoded.to_string());
        }
    }

    Ok(tasks)
}

async fn fetch_html(url: &str) -> Result<Html> {
    let client = Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "Left-Right-Language-Bot/1.0 (+https://github.com/user/left-right)")
        .timeout(std::time::Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("HTTP error: {}", response.status());
    }

    let html = response.text().await?;
    Ok(Html::parse_document(&html))
}
```

### Step 3.2: Map Tasks to Categories

Create `src/categories.rs`:

```rust
pub const TASK_CATEGORIES: &[(&str, &str)] = &[
    ("Category:Arithmetic_operations", "Arithmetic & Math"),
    ("Category:String_operations", "String Processing"),
    ("Category:Array_operations", "Collections & Data Structures"),
    ("Category:Sorting_algorithms", "Algorithms"),
    ("Category:Concurrency", "Concurrency & Parallelism"),
    ("Category:File_operations", "File I/O"),
    ("Category:Networking", "Networking"),
    ("Category:Date_format", "Date/Time"),
    ("Category:Recursion", "Recursion"),
    ("Category:Pattern_matching", "Pattern Matching"),
    ("Category:Control_structures", "Control Flow"),
    ("Category:Type_inference", "Type System"),
    ("Category:Metaprogramming", "Metaprogramming"),
];

pub fn category_to_filename(category: &str) -> String {
    let normalized = category.to_lowercase().replace(' ', "-");
    // Map to two-digit prefix
    match normalized.as_str() {
        "arithmetic-&-math" => "01-arithmetic-math",
        "string-processing" => "02-string-processing",
        // ... etc
        _ => normalized,
    }
    .to_string()
}
```

## Phase 4: HTML Parsing

### Step 4.1: Extract Code Blocks

Create `src/parser.rs`:

```rust
use scraper::{Html, Selector};

pub struct TaskSolution {
    pub version: Option<String>,
    pub code: String,
    pub language: String,
}

pub fn extract_solutions(html: &Html, language: &str) -> Result<Vec<TaskSolution>> {
    let mut solutions = Vec::new();

    // Find language section
    let lang_selector = Selector::parse(&format!(r#"span[id="{}"]"#, language))?;
    if html.select(&lang_selector).next().is_none() {
        return Ok(vec![]); // No solutions for this language
    }

    // Find all pre tags after language section
    let pre_selector = Selector::parse("pre.highlighted_code")?;
    let code_selector = Selector::parse("code")?;

    let mut current_version = 1;

    for pre in html.select(&pre_selector) {
        if let Some(code_element) = pre.select(&code_selector).next() {
            let code = code_element.inner_html()
                .trim()
                .to_string();

            if !code.is_empty() {
                // Check if there's a version header (h3) before this pre
                // This is simplified - real implementation needs more careful traversal

                solutions.push(TaskSolution {
                    version: if solutions.is_empty() {
                        None
                    } else {
                        Some(format!("Version {}", current_version))
                    },
                    code,
                    language: language.to_string(),
                });
            }
        }
    }

    Ok(solutions)
}
```

### Step 4.2: Validate Extracted Code

```rust
pub fn validate_code(code: &str) -> bool {
    let trimmed = code.trim();

    // Not empty
    if trimmed.is_empty() {
        return false;
    }

    // Not just comments
    let non_comment_lines: Vec<_> = trimmed
        .lines()
        .filter(|line| {
            let stripped = line.trim();
            !stripped.is_empty()
                && !stripped.starts_with("#")
                && !stripped.starts_with("//")
                && !stripped.starts_with("/*")
        })
        .collect();

    if non_comment_lines.is_empty() {
        return false;
    }

    // Reasonable length
    if trimmed.len() > 50_000 {
        return false;
    }

    // No HTML tags leaked
    if trimmed.contains('<') && trimmed.contains('>') {
        return false;
    }

    true
}
```

## Phase 5: Caching

### Step 5.1: Cache Storage

Create `src/cache.rs`:

```rust
use anyhow::Result;
use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct CacheMetadata {
    pub fetched_at: DateTime<Utc>,
    pub status: String,
    pub has_code: bool,
    pub num_solutions: usize,
}

pub struct Cache {
    cache_dir: PathBuf,
}

impl Cache {
    pub fn new(cache_dir: &str) -> Result<Self> {
        let dir = PathBuf::from(cache_dir);
        fs::create_dir_all(&dir)?;
        Ok(Self { cache_dir: dir })
    }

    pub fn get(&self, language: &str, task_name: &str) -> Option<String> {
        let path = self.cache_path(language, task_name)?;
        fs::read_to_string(path).ok()
    }

    pub fn set(&self, language: &str, task_name: &str, html: &str) -> Result<()> {
        let path = self.cache_path(language, task_name)?;
        let dir = path.parent().context("Invalid cache path")?;
        fs::create_dir_all(dir)?;
        fs::write(path, html)?;
        Ok(())
    }

    fn cache_path(&self, language: &str, task_name: &str) -> Option<PathBuf> {
        let safe_name = sanitize_filename(task_name)?;
        Some(self.cache_dir.join(language).join(format!("{}.html", safe_name)))
    }
}

fn sanitize_filename(name: &str) -> Option<String> {
    // Remove/replace unsafe characters
    let safe: String = name
        .chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            c => c,
        })
        .collect();

    if safe.is_empty() {
        None
    } else {
        Some(safe)
    }
}
```

## Phase 6: Rate Limiting

### Step 6.1: Rate Limiter

Create `src/rate_limit.rs`:

```rust
use std::time::{Duration, Instant};
use tokio::time::sleep;

pub struct RateLimiter {
    last_request: Instant,
    request_count: usize,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            last_request: Instant::now(),
            request_count: 0,
        }
    }

    pub async fn wait(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_request);

        if elapsed < Duration::from_millis(RATE_LIMIT_MS) {
            sleep(Duration::from_millis(RATE_LIMIT_MS) - elapsed).await;
        }

        self.request_count += 1;

        if self.request_count % PAUSE_EVERY == 0 {
            eprintln!("Pausing for {} seconds...", PAUSE_DURATION_MS / 1000);
            sleep(Duration::from_millis(PAUSE_DURATION_MS)).await;
        }

        self.last_request = Instant::now();
    }
}
```

## Phase 7: Markdown Generation

### Step 7.1: Generate Category File

Create `src/generator.rs`:

```rust
use crate::parser::TaskSolution;
use std::fs;
use std::path::PathBuf;

pub fn generate_category_file(
    language: &str,
    category: &str,
    tasks: &[(String, Vec<TaskSolution>)],
    output_dir: &PathBuf,
) -> Result<()> {
    let filename = format!("{}.md", category_to_filename(category));
    let path = output_dir.join(language).join(&filename);

    let mut content = String::new();

    // Header
    content.push_str(&format!(
        "# {}: {}\n\n",
        language, category
    ));
    content.push_str(&format!("**Language**: {}\n", language));
    content.push_str(&format!("**Category**: {}\n", category));
    content.push_str(&format!("**Source**: Rosetta Code\n"));
    content.push_str(&format!("**Generated**: {}\n", Utc::now().to_rfc3339()));
    content.push_str(&format!("**Tasks**: {}\n", tasks.len()));
    content.push_str("\n---\n\n## Task List\n\n");

    // Tasks
    for (task_name, solutions) in tasks {
        generate_task_entry(&mut content, task_name, solutions);
    }

    // Summary
    generate_summary(&mut content, tasks);

    fs::write(path, content)?;
    Ok(())
}

fn generate_task_entry(content: &mut String, task_name: &str, solutions: &[TaskSolution]) {
    let url = format!("{}/{}#{}", BASE_URL, urlencoding::encode(task_name, true), language);

    content.push_str(&format!("## {}\n\n", task_name));
    content.push_str(&format!("**URL**: {}\n", url));
    content.push_str(&format!("**Solutions**: {}\n\n", solutions.len()));

    for (i, solution) in solutions.iter().enumerate() {
        content.push_str(&format!("### Version {}\n\n", i + 1));

        // Detect language for code fence
        let fence_lang = detect_fence_lang(&solution.language);

        content.push_str(&format!("```{}\n{}\n```\n\n", fence_lang, solution.code));
        content.push_str("**Notes**: TODO: Add notes.\n\n");

        if solutions.len() > 1 {
            content.push_str(&format!("**Lines**: {}\n\n", solution.code.lines().count()));
        }

        content.push_str("---\n\n");
    }
}

fn detect_fence_lang(language: &str) -> String {
    match language {
        "Python" => "python",
        "Rust" => "rust",
        "JavaScript" => "javascript",
        "Haskell" => "haskell",
        "OCaml" => "ocaml",
        "Elixir" => "elixir",
        "Erlang" => "erlang",
        "Clojure" => "clojure",
        "Ruby" => "ruby",
        "R" => "r",
        "Kotlin" => "kotlin",
        "Scala" => "scala",
        "Groovy" => "groovy",
        "CoffeeScript" => "coffeescript",
        "BQN" => "bqn",
        "J" => "j",
        "Arturo" => "arturo",
        "Wren" => "wren",
        "Mercury" => "mercury",
        "GAP" => "gap",
        "newLISP" => "lisp",
        "REBOL" => "rebol",
        "OI" => "oi",
        "Ursala" => "ursala",
        "Slate" => "slate",
        "Raven" => "raven",
        "PowerShell" => "powershell",
        "Lingo" => "lingo",
        "Elm" => "elm",
        _ => "text",
    }
    .to_string()
}
```

## Phase 8: Main Loop

### Step 8.1: Orchestrate Fetching

Create `src/main.rs`:

```rust
mod cli;
mod config;
mod discovery;
mod parser;
mod cache;
mod rate_limit;
mod generator;

use anyhow::Result;
use clap::Parser;
use tokio::time::sleep;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let cache = cache::Cache::new(config::CACHE_DIR)?;

    match cli.command {
        Commands::Fetch {
            language,
            category,
            force,
            dry_run,
        } => {
            fetch_all(language, category, force, dry_run, &cache).await?;
        }
        Commands::List { category } => {
            list_tasks(category)?;
        }
        Commands::Validate => {
            validate_cache(&cache)?;
        }
    }

    Ok(())
}

async fn fetch_all(
    language: Option<String>,
    category: Option<String>,
    force: bool,
    dry_run: bool,
    cache: &cache::Cache,
) -> Result<()> {
    let languages = match language {
        Some(lang) => vec![lang],
        None => config::TARGET_LANGUAGES.to_vec(),
    };

    let categories = match category {
        Some(cat) => vec![cat],
        None => config::TASK_CATEGORIES.iter().map(|(_, name)| name.to_string()).collect(),
    };

    let mut rate_limiter = rate_limit::RateLimiter::new();

    for lang in languages {
        eprintln!("Processing language: {}", lang);

        for (cat_id, cat_name) in config::TASK_CATEGORIES.iter() {
            if !categories.contains(&cat_name.to_string()) {
                continue;
            }

            eprintln!("  Category: {}", cat_name);

            // Fetch task list for category
            let tasks = discovery::fetch_task_list(cat_id).await?;

            for task_name in &tasks {
                if dry_run {
                    println!("Would fetch: {} - {}", lang, task_name);
                    continue;
                }

                // Check cache
                if !force {
                    if let Some(cached) = cache.get(lang, task_name) {
                        eprintln!("    Using cache: {}", task_name);
                        continue;
                    }
                }

                // Fetch
                rate_limiter.wait().await;

                match fetch_and_parse(lang, task_name, cache).await {
                    Ok(solutions) => {
                        eprintln!("    Fetched: {} - {} solutions", task_name, solutions.len());
                    }
                    Err(e) => {
                        eprintln!("    Error: {} - {}", task_name, e);
                    }
                }
            }
        }

        // Pause between languages
        sleep(tokio::time::Duration::from_secs(10)).await;
    }

    Ok(())
}
```

## Phase 9: Testing

### Step 9.1: Unit Tests

Create `tests/parser_test.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use scraper::Html;

    #[test]
    fn test_extract_solutions() {
        let html = r#"
        <h2><span id="Python">Python</span></h2>
        <pre class="highlighted_code">
          <code class="language-python">def hello():</code>
        </pre>
        <h3>Version 2</h3>
        <pre class="highlighted_code">
          <code class="language-python">print("hello")</code>
        </pre>
        "#;

        let parsed = Html::parse_document(html);
        let solutions = extract_solutions(&parsed, "Python").unwrap();

        assert_eq!(solutions.len(), 2);
        assert_eq!(solutions[0].version, None);
        assert_eq!(solutions[1].version, Some("Version 1".to_string()));
    }

    #[test]
    fn test_validate_code() {
        assert!(validate_code("def foo():\n    return 1"));
        assert!(!validate_code(""));
        assert!(!validate_code("# comment only"));
        assert!(!validate_code("<html>leaked</html>"));
    }
}
```

### Step 9.2: Integration Tests

Test with a small subset first:

```bash
cargo run -- fetch --language Python --category "Arithmetic operations" --dry-run
cargo run -- fetch --language Rust --category "Arithmetic operations"
cargo run -- validate
```

## Phase 10: Deployment

### Step 10.1: Build Binary

```bash
cargo build --release
```

Binary: `target/release/rosetta-scraper`

### Step 10.2: Run Full Scraping

```bash
# All languages, all categories
cargo run -- release -- fetch

# Just one language
cargo run -- release -- fetch --language Python

# Force refresh
cargo run -- release -- fetch --force

# Dry run to verify
cargo run -- release -- fetch --dry-run | head -20
```

### Step 10.3: Validate Output

```bash
cargo run -- validate

# Check file counts
find reports/rosetta-code/language-examples -name "*.md" | wc -l

# Spot check a file
cat reports/rosetta-code/language-examples/python/01-arithmetic-math.md | head -30
```

---

**Next**: See `05-left-right-solutions-plan.md` for how to use these examples to solve problems in Left-Right.
