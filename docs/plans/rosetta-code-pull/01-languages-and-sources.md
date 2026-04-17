# Languages and Sources

## Target Languages: URL Patterns and Fetching Strategy

This document details how to fetch solutions from Rosetta Code for each of the 26 target languages.

## URL Patterns

### Base Format

Rosetta Code tasks follow this URL pattern:

```
https://rosettacode.org/wiki/{Task_Name}#{Language_Name}
```

Where:
- `Task_Name`: URL-encoded task name (e.g., `Hello_world`, `Factorial`, `Fibonacci_sequence`)
- `Language_Name`: Exactly as it appears in Rosetta Code (case-sensitive)

### Language-Specific Anchors

Some languages use different anchor formats:

| Language | Anchor Format | Example URL |
|----------|---------------|--------------|
| Python | `#Python` | `https://rosettacode.org/wiki/Hello_world#Python` |
| Rust | `#Rust` | `https://rosettacode.org/wiki/Factorial#Rust` |
| JavaScript | `#JavaScript` | `https://rosettacode.org/wiki/Fibonacci_sequence#JavaScript` |
| Haskell | `#Haskell` | `https://rosettacode.org/wiki/Prime_numbers#Haskell` |
| OCaml | `#OCaml` | `https://rosettacode.org/wiki/Sorting_algorithms#OCaml` |
| Elixir | `#Elixir` | `https://rosettacode.org/wiki/Matrix_operations#Elixir` |
| Erlang | `#Erlang` | `https://rosettacode.org/wiki/Concurrent_computing#Erlang` |
| Clojure | `#Clojure` | `https://rosettacode.org/wiki/Hash_table#Clojure` |
| Ruby | `#Ruby` | `https://rosettacode.org/wiki/String_operations#Ruby` |
| R | `#R` | `https://rosettacode.org/wiki/Statistics#R` |
| Kotlin | `#Kotlin` | `https://rosettacode.org/wiki/Parsing#Kotlin` |
| Scala | `#Scala` | `https://rosettacode.org/wiki/Regular_expressions#Scala` |
| Groovy | `#Groovy` | `https://rosettacode.org/wiki/Database#Groovy` |
| CoffeeScript | `#CoffeeScript` | `https://rosettacode.org/wiki/Functional_composition#CoffeeScript` |
| BQN | `#BQN` | `https://rosettacode.org/wiki/Matrix#BQN` |
| J | `#J` | `https://rosettacode.org/wiki/Prime_numbers#J` |
| Arturo | `#Arturo` | `https://rosettacode.org/wiki/Arrays#Arturo` |
| Wren | `#Wren` | `https://rosettacode.org/wiki/Loops#Wren` |
| Mercury | `#Mercury` | `https://rosettacode.org/wiki/Logic#Mercury` |
| GAP | `#GAP` | `https://rosettacode.org/wiki/Permutations#GAP` |
| newLisp | `#newLISP` | `https://rosettacode.org/wiki/Lists#newLISP` |
| REBOL | `#REBOL` | `https://rosettacode.org/wiki/Parsing#REBOL` |
| OI | `#OI` | `https://rosettacode.org/wiki/Symbolic_computation#OI` |
| Ursala | `#Ursala` | `https://rosettacode.org/wiki/Tree_traversal#Ursala` |
| Slate | `#Slate` | `https://rosettacode.org/wiki/Objects#Slate` |
| Raven | `#Raven` | `https://rosettacode.org/wiki/Stack#Raven` |
| PowerShell | `#PowerShell` | `https://rosettacode.org/wiki/File_operations#PowerShell` |
| Lingo | `#Lingo` | `https://rosettacode.org/wiki/Sound#Lingo` |
| Elm | `#Elm` | `https://rosettacode.org/wiki/FRP#Elm` |

## Scraping Strategy

### HTML Structure Analysis

Rosetta Code uses MediaWiki. Each language section is structured as:

```html
<h2><span id="Language_Name">Language_Name</span></h2>
<pre class="highlighted_code">
  <code class="language-name">
    ... code here ...
  </code>
</pre>
```

Or for multiple solutions:

```html
<h3>Version 1</h3>
<pre class="highlighted_code">
  <code class="language-name">
    ... solution 1 ...
  </code>
</pre>
<h3>Version 2</h3>
<pre class="highlighted_code">
  <code class="language-name">
    ... solution 2 ...
  </code>
</pre>
```

### Extraction Algorithm

1. Fetch page HTML
2. Find the language-specific anchor (e.g., `#Python`)
3. Extract all `<pre>` blocks following that anchor until next language section
4. Within each `<pre>`, extract `<code>` content
5. Store code with version identifier (if multiple versions exist)

### Fallback: MediaWiki API

If HTML parsing fails (e.g., dynamic content, special formatting), use MediaWiki API:

```
https://rosettacode.org/api.php?action=parse&page={Task_Name}&prop=text&format=json
```

Then parse the raw wikitext to extract code blocks between:

```mediawiki
== Language_Name ==
<source lang="language">
... code ...
</source>
```

## Rate Limiting Strategy

### Respectful Access

Rosetta Code is a volunteer-run wiki. We'll be respectful:

| Constraint | Value | Reason |
|------------|--------|--------|
| Requests per second | Max 1 | Avoid server overload |
| Pause interval | Every 50 requests, pause 10s | Burst protection |
| User-Agent | `Left-Right-Language-Bot/1.0 (+https://github.com/user/left-right)` | Transparency |
| Retry delay | Exponential backoff starting at 1s | Handle failures gracefully |
| Timeout per request | 30 seconds | Don't hang |
| Concurrent requests | 1 (sequential) | Simpler, more respectful |

### Exponential Backoff

On HTTP errors (429, 500, 502, 503, 504):

```rust
let mut delay = Duration::from_secs(1);
loop {
    match fetch_url(&url).await {
        Ok(response) => break response,
        Err(e) if is_rate_limit(e) => {
            sleep(delay).await;
            delay *= 2;
            if delay > MAX_DELAY {
                return Err("Max retries exceeded");
            }
        },
        Err(e) => return Err(e),
    }
}
```

## Caching Strategy

### Cache Key

```
{language_name}/{task_name}.html
```

Example: `cache/python/Fibonacci_sequence.html`

### Cache Validity

- HTML content never changes significantly
- Cache indefinitely
- Invalidate manually if needed (force-refresh flag)

### Cache Structure

```
cache/
├── python/
│   ├── Factorial.html
│   ├── Fibonacci_sequence.html
│   └── ...
├── rust/
│   ├── Factorial.html
│   └── ...
└── metadata.json
```

### Metadata

Track fetch status:

```json
{
  "python/Fibonacci_sequence": {
    "fetched_at": "2024-01-15T10:30:00Z",
    "status": "success",
    "has_code": true,
    "num_solutions": 1
  },
  "rust/Nonexistent_Task": {
    "fetched_at": "2024-01-15T10:31:00Z",
    "status": "not_found",
    "has_code": false,
    "num_solutions": 0
  }
}
```

## Error Handling

### Missing Solutions

Some tasks don't have solutions in all languages. Handle gracefully:

```rust
enum FetchResult {
    Success(Vec<String>),  // Multiple versions
    NotFound,              // No language section
    NoCode,               // Language section exists, no code
    NetworkError(String),    // Fetch failed
    ParseError(String),     // HTML parsing failed
}
```

### Validation

Validate extracted code:

1. Non-empty after stripping whitespace
2. Contains plausible language constructs (not just comments)
3. No HTML tags leaked through
4. Reasonable length (1-5000 lines)

## Task Discovery

### Category Pages

Rosetta Code has category pages listing all tasks:

```
https://rosettacode.org/wiki/Category:{Category_Name}
```

Key categories:

- `Category:Programming_Tasks` (all tasks)
- `Category:Arrays`
- `Category:Strings`
- `Category:Algorithms`

We'll fetch these pages first to get the task list, then fetch each task per language.

### Task List Format

Parse category HTML to extract task names:

```html
<li><a href="/wiki/Factorial" title="Factorial">Factorial</a></li>
<li><a href="/wiki/Fibonacci_sequence" title="Fibonacci sequence">Fibonacci sequence</a></li>
```

Extract `href` attribute values, decode URL encoding.

## Technology Implementation

### Rust Crate Dependencies

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
scraper = "0.17"
select = "0.6"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
```

### Fetch Function Signature

```rust
async fn fetch_task(
    language: &str,
    task_name: &str,
    cache_dir: &Path,
) -> Result<Vec<String>, anyhow::Error> {
    // 1. Check cache
    // 2. Fetch if not cached
    // 3. Parse HTML
    // 4. Extract code blocks
    // 5. Cache HTML
    // 6. Return code blocks
}
```

### Main Loop

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let languages = TARGET_LANGUAGES;
    let tasks = fetch_task_list("Category:Programming_Tasks").await?;

    for language in languages {
        for task in &tasks {
            match fetch_task(language, task, &cache_dir).await {
                Ok(solutions) => {
                    println!("{}: {} - {} solutions", language, task, solutions.len());
                    write_solutions(language, task, solutions)?;
                }
                Err(e) => {
                    eprintln!("Error: {} - {}: {}", language, task, e);
                }
            }
            sleep(Duration::from_secs(1)).await; // Rate limit
        }
        sleep(Duration::from_secs(10)).await; // Pause between languages
    }

    Ok(())
}
```

## Language-Specific Notes

### APL-style languages (BQN, J, Arturo)

- May use non-ASCII characters (ensure UTF-8)
- Code is often very dense (1-3 lines)
- High value for array-oriented comparison with Left-Right

### Lisp-family (Clojure, newLisp, OI, REBOL, Ursala)

- Homoiconic structure
- Lots of parentheses
- Good for validating operator precedence in Left-Right

### Functional languages (Haskell, OCaml, Elixir, Erlang, Elm)

- Pattern matching
- Immutable data
- Good for recursion and functional composition comparison

### Imperative/OO (Python, Rust, Ruby, Kotlin, Scala, Groovy, Java)

- Familiar patterns
- Baseline for expressiveness comparison

### Esoteric/Minimal (Wren, Slate, Raven, Lingo)

- Unique approaches
- May reveal edge cases Left-Right needs to handle

---

**Next**: See `02-problem-categories.md` for task categorization and prioritization.
