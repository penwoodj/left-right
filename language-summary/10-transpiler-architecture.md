# Transpiler Architecture — LLM-Assisted Compilation

Left-Right transpiler is written in Rust with dual-target compilation to JavaScript and Rust. This document covers the transpiler architecture, LLM-as-compiler model, pipeline stages, and CLI integration.

## Transpiler Overview

### Implementation Language

The Left-Right transpiler is implemented in Rust:

```rust
// Language: Rust
// Performance: Native compiled
// Type Safety: Strong guarantees
// Tooling: Cargo ecosystem
```

### Dual-Target Compilation

Single Left-Right source transpiles to both runtimes:

| Target | Runtime | Use Case |
|--------|----------|-----------|
| JavaScript | Node.js, Browser | Web applications, existing JS ecosystem |
| Rust | Native | High-performance, system programming |

**Architecture Goal:** Write once, run anywhere with optimal performance.

### Three Transpiler Operator Definition Files

The transpiler has 3 operator definition files that define how operators work:

1. **operators-recursive.lr** — Defines all default operators using only recursion and control flow
2. **operators-rust-optimal.lr** — Defines each operator with maximal Rust runtime efficiency
3. **operators-js-optimal.lr** — Defines each operator with maximal JavaScript runtime efficiency

All 3 are overridable at folder/global/intra-script level:

**Override Hierarchy:**
```
intra-script > folder > project > global
```

**File Locations:**
```
~/.left-right/
  ├── operators-recursive.lr        # Default: recursion-based definitions
  ├── operators-rust-optimal.lr     # Rust: optimized implementation
  └── operators-js-optimal.lr      # JS: optimized implementation
```

**Override Mechanism:**
- **Intra-script override:** Create `.lr-operators` file in same directory as script
- **Folder override:** Create `.lr-operators` file in parent directory
- **Project override:** Create `.lr-operators` file at project root
- **Global override:** Override files in `~/.left-right/` directory

**Example Override:**
```penroscript
// Intra-script .lr-operators file
+: {
  left: _<@0,
  right: _<@1,
  // Custom implementation
  left + right + 10  // Always add 10
}
```

## LLM-as-Compiler Concept

### Core Philosophy

> Don't let LLM directly "run" your program. Let it propose rewrites (macro expansion / transpilation / partial evaluation), then you validate + apply deterministically.

### 5-Stage Pipeline

The LLM-as-compiler model uses a 5-stage pipeline:

#### Stage 1: Bind (Variable Resolving)

**Purpose:** LLM outputs JSON bindings for variables

**Process:**
1. LLM analyzes code for variable references
2. Outputs structured bindings JSON
3. Machine validates types and required keys
4. Tracks provenance of each binding

**Output:**
```json
{
  "bindings": {
    "user": { "source": "parameter", "type": "map" },
    "config": { "source": "import", "path": "./config.prsc" }
  },
  "provenance": "model-v4-binding-check-v2"
}
```

#### Stage 2: Expand (Deterministic Substitution)

**Purpose:** System performs variable substitutions

**Process:**
1. Replace variable references with bound values
2. Ensure reproducibility (same input = same output)
3. Cache results (bindings + template hash → output)

**Caching Strategy:**
```rust
fn get_cached_expansion(
    bindings: &Bindings,
    template_hash: &str
) -> Option<String> {
    CACHE.get(&format!("{}+{}", bindings, template_hash))
}
```

**Benefits:**
- **Reproducible** — Deterministic substitution
- **Fast** — Cached expansions reuse results
- **Verifiable** — Output hashable for comparison

#### Stage 3: Rewrite/Optimize (LLM-Guided)

**Purpose:** Transform code using LLM guidance

**Operations:**
- **Lowering:** High-level → low-level constructs
- **Specialization:** Inject domain knowledge
- **Partial Evaluation:** Pre-fill constants
- **Refactoring:** Split into subcalls for clarity

**Example Lowering:**
```javascript
// High-level
data ?{ _< > 5 } ${ _< * 2 }

// Lowered
{
  result: reduce_and(data, >5, *2)
}
```

**Example Partial Evaluation:**
```javascript
// Before partial eval
{ constant: 10, input: _<@0, input + constant }

// After partial eval (if constant known)
{ input: _<@0, input + 10 }
```

#### Stage 4: Execute (Code Generation)

**Purpose:** Generate target code deterministically

**Process:**
1. Use deterministic templates for known AST nodes
2. LLM generates complex transformations with strict format
3. Structured output as "execution units"

**Code Generation Patterns:**
```rust
// Map generation
fn generate_map(ast: &MapNode) -> String {
    match ast {
        MapNode::Simple(map) => {
            let entries: Vec<String> = map.entries.iter()
                .map(|(k, v)| format!("\"{}\": {}", k, generate_value(v)))
                .collect();
            format!("{{{}}}", entries.join(", "))
        }
    }
}
```

#### Stage 5: Verify (Validation)

**Purpose:** Ensure correctness of generated code

**Checks:**
- **Syntax Validation** — Parse output AST
- **Semantic Validation** — Sample input/output tests
- **Length Constraints** — Ensure within limits
- **Policy Checks** — Validate against rules

**Validation Output:**
```json
{
  "validation": {
    "syntax": "pass",
    "semantic": "pass",
    "tests": [
      {"name": "addition", "input": [1, 2], "expected": 3, "actual": 3}
    ]
  },
  "status": "valid"
}
```

## PenroScript Transpiler Architecture

### JavaScript/TypeScript Implementation

#### Pipeline Stages

**1. Parsing & AST Generation**

```rust
// Grammar: PEG or recursive-descent
fn parse_prsc(source: &str) -> Result<AST, ParseError> {
    let parser = Parser::new(source);
    parser.parse_program()
}

// LLM-assisted for ambiguous cases
fn resolve_ambiguous(ast: &AST) -> Result<AST, AmbiguityError> {
    match ast {
        AST::Ambiguous(context) => {
            let resolved = llm_disambiguate(context)?;
            Ok(resolved)
        }
        _ => Ok(ast)
    }
}
```

**2. Semantic Analysis & Disambiguation**

```rust
fn semantic_analysis(ast: &AST) -> Result<AnalyzedAST, SemanticError> {
    // Type checking
    type_checker.check(&ast)?;

    // Identifier resolution
    resolver.resolve_identifiers(&ast)?;

    // LLM resolves underspecified constructs
    let enhanced = llm_enhance_ast(&ast)?;
    Ok(enhanced)
}
```

**3. Code Generation**

```rust
fn generate_javascript(ast: &AnalyzedAST) -> String {
    // Deterministic templates for known nodes
    let mut codegen = CodeGenerator::new();
    codegen.add_template("map", MAP_TEMPLATE);
    codegen.add_template("operator", OPERATOR_TEMPLATE);

    // LLM for complex transformations
    for node in ast.complex_nodes() {
        let js = llm_generate_javascript(node)?;
        codegen.add_code(js);
    }

    codegen.finalize()
}
```

**4. Validation**

```javascript
// Syntactic validation (Babel/Acorn parsing)
const validateJS = (code) => {
  try {
    parse(code, { sourceType: 'module' });
    return { valid: true };
  } catch (error) {
    return { valid: false, error };
  }
};

// Semantic validation (sample tests)
const validateSemantics = (generated, samples) => {
  for (const sample of samples) {
    const output = eval(generated(sample.input));
    if (!deepEquals(output, sample.expected)) {
      return { valid: false, sample };
    }
  }
  return { valid: true };
};
```

### Rust Implementation

#### Advantages

1. **Performance:** Native speed, zero-cost abstractions
2. **Type Safety:** Compile-time error detection
3. **Robust Tooling:** Cargo ecosystem, excellent tooling
4. **Strong Guarantees:** Memory safety, thread safety

#### Pipeline Differences

**LLM Integration:**
```rust
// LangChain-rs for LLM orchestration
use langchain::LLM;

async fn llm_enhance_ast(ast: &AST) -> Result<AST, Error> {
    let llm = LLM::new("gpt-4");
    let prompt = format!("Enhance AST: {:?}", ast);
    let response = llm.call(&prompt).await?;
    parse_enhanced_ast(&response)
}
```

**Concurrency Support:**
```rust
// Parallel transpilation
async fn transpile_parallel(sources: Vec<Path>) -> Vec<Result<String, Error>> {
    let tasks: Vec<_> = sources.into_iter()
        .map(|src| tokio::spawn(async move {
            transpile_file(src).await
        }))
        .collect();

    let results = join_all(tasks).await;
    results.into_iter().map(|r| r.unwrap()).collect()
}
```

## Determinism & Reliability

### Problems & Solutions

| Problem | Solution |
|----------|----------|
| Nondeterminism | Cache bindings + expanded text + model/version |
| Prompt injection via variables | Wrap untrusted text, label as inert input |
| LLM invents bindings | Require provenance, whitelisted sources |
| Evaluation correctness | Automated checks, cross-checking two models |
| Validation bypass | Schema validation, reject non-conforming outputs |

### Artifacts to Save

**Transpilation Cache:**
```
cache/
├── bindings.json              # Variable bindings
├── expanded_prompts/          # After substitution
│   ├── abc123.json
│   └── def456.json
├── ir.json                    # AST representation
├── patches/                   # Rewrite diffs
│   ├── patch-001.diff
│   └── patch-002.diff
└── verification_report.json    # Test results
```

### Reproducibility Strategy

```rust
// Hash-based caching
fn compute_cache_key(
    bindings: &Bindings,
    template_hash: &str,
    model: &str
) -> String {
    format!(
        "{}:{}:{}",
        serde_json::to_string(bindings),
        template_hash,
        model
    )
}

// Deterministic LLM calls
fn llm_call_deterministic(
    prompt: &str,
    cache_key: &str
) -> Result<String, Error> {
    if let Some(cached) = CACHE.get(cache_key) {
        return Ok(cached);
    }

    let response = llm_call_with_seed(prompt, cache_key)?;
    CACHE.insert(cache_key.to_string(), response.clone());
    Ok(response)
}
```

## CLI Integration

### Commands

**Run:**
```bash
# Execute file
lr run path/to/script.lr

# Execute with arguments
lr run script.lr arg1 arg2

# Transpile only (no execute)
lr run --transpile-only script.lr
```

**Build:**
```bash
# Transpile to JavaScript
lr build src/ --target js --output dist/

# Transpile to Rust
lr build src/ --target rust --output dist/

# Both targets
lr build src/ --target both --output dist/
```

**Watch:**
```bash
# Watch directory for changes
lr watch src/ --target js --output dist/

# Watch with auto-execution
lr watch src/ --target both --output dist/ --exec
```

**Validate:**
```bash
# Validate syntax only
lr validate src/**/*.lr

# Type checking
lr check src/ --strict

# Lint rules
lr lint src/ --rules all
```

### File Watching

**Watch Mode Features:**
1. **File Monitoring** — Detect changes in source files
2. **Incremental Re-transpilation** — Only rebuild changed files
3. **Hot Reload** — Restart runtime on change
4. **Dependency Tracking** — Rebuild when imports change

**Watch Implementation:**
```rust
use notify::{Watcher, RecursiveMode, watcher};

async fn watch_directory(path: &Path) -> Result<(), Error> {
    let (tx, rx) = channel();
    let mut watcher = Watcher::new(tx, Duration::from_secs(2))?;

    watcher.watch(path, RecursiveMode::Recursive)?;

    while let Some(event) = rx.recv().await {
        match event.kind {
            EventKind::Create(_) | EventKind::Write(_) => {
                let source = event.path.unwrap();
                transpile_file(source)?;
            }
            _ => ()
        }
    }
}
```

## Design Principles

1. **Three Operator Definition Files** — Recursive, Rust-optimal, JS-optimal operator implementations
2. **Two-Level Language** — Compile-time (LLM-assisted) + runtime
3. **Deterministic Core** — Parsing, AST, validation are deterministic
4. **LLM as Assistant** — LLM in specific, controlled roles
5. **Modular Pipeline** — Each stage is separable and testable
6. **Reproducibility** — Same input → same output, always
7. **Pluggable LLM** — Swap models without changing logic
8. **Guardrails** — Reject invalid LLM outputs
9. **Iterative Refinement** — Generate → test → reflect → fix
10. **Operator Override System** — Override operators at intra-script, folder, project, and global levels

## Related Concepts

- **Transpilation** — Source-to-source translation
- **Compiler Architecture** — Frontend, middle-end, backend
- **AST Manipulation** — Intermediate representation transforms
- **Macro Expansion** — Compile-time code generation
- **Partial Evaluation** — Evaluate what's known at compile-time
- **LLM-Assisted Programming** — AI in compiler workflows
- **Verification** — Ensuring correctness of generated code
- **Rust** — Systems programming language
- **TypeScript** — Typed JavaScript superset
- **Source Maps** — Debug mapping to original source
