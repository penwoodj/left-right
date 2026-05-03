# Modules and Interop — File System and Integration

Left-Right uses a file-based module system with seamless interop to JavaScript and Rust ecosystems. This document covers the module system, file organization, import/export patterns, and cross-language integration.

## File as Module

### Module Definition

Every Left-Right file is a module that exports a single value:

```javascript
// File: utils.lr

{
  add: { a: _<@0, b: _>@1, a + b },
  multiply: { a: _<@0, b: _>@1, a * b }
}
// Exports: { add, multiply }
```

### Single Export Value

Each file implicitly exports the result of its top-level expression:

```javascript
// Export single function
{ input: _<@0, input * 2 }

// Export configuration
{
  apiKey: `secret`,
  endpoint: `https://api.example.com`
}

// Export array
[1, 2, 3, 4, 5]
```

## Language Identity and Interop Model

Left-Right is a general-purpose scripting language with its own schematics, operators, and semantics. When transpiling to JavaScript or Rust targets, it gains full ability to use libraries from those ecosystems, but it remains its own distinct language.

**Key Points:**
- Left-Right has its own type system (Operator, Map, List, Text, Number, Undefined)
- Left-Right has its own operator semantics and evaluation model
- When targeting JavaScript: can import NPM packages and use Node.js/Browser APIs
- When targeting Rust: can import crates and use Rust's ecosystem
- The language is not a DSL — it's a full general-purpose programming language
- Code written in Left-Right follows Left-Right semantics, not host language semantics

## Import Syntax

### File Imports

Import modules using `file` operator:

```javascript
// Import entire module
utils: file[`./utils.lr`]

// Import and spread
...file[`./utils.lr`]
// Spread all exported keys into current scope
```

### Named Imports

Import specific exports:

```javascript
// Import specific keys
{ add, multiply }: file[`./utils.lr`]
// Only 'add' and 'multiply' imported

// Import with rename
{ add: addFn, multiply: multFn }: file[`./utils.lr`]
```

### Relative Path Imports

Use relative paths for project structure:

```javascript
// Same directory
local: file[`./helper.lr`]

// Parent directory
parent: file[`../utils.lr`]

// Nested path
deep: file[`./lib/core/math.lr`]
```

## Module Patterns

### Giant Map-Array of Intent

Conceptual model: Each file is a "giant map-array" that represents program intent:

```javascript
// File: dataProcessor.lr

{
  // Dependencies
  ...{ map, filter, reduce }: file[`lodash/fp`],

  // Configuration
  config: file[`./config.lr`],

  // Data transformations
  processData: {
    input: _<@0,
    input
      >> filter[predicate]
      >> map[transform]
      >> reduce[combiner]
  },

  // Utilities
  helpers: file[`./helpers.lr`],

  // Export main functionality
  return: processData
}
```

### Library as Module

Package related functionality:

```javascript
// File: stringOps.lr

{
  // Text utilities
  capitalize: { str: _<@0, str`^_ },
  slugify: { str: _<@0, str <` _` >`< ` `, `-` },
  truncate: { str: _<@0, len: _>@1, str@[0, len] },

  // Export collection
  exports: {
    capitalize,
    slugify,
    truncate
  }
}
```

### Application as Module

Main entry point orchestrating components:

```javascript
// File: main.lr

{
  // Import libraries
  ...{ express, cors }: file[`express`],
  config: file[`./config.lr`],

  // Import application modules
  routes: file[`./routes.lr`],
  middleware: file[`./middleware.lr`],

  // Setup server
  app: {
    middleware: cors,
    routes: routes
  },

  // Start server
  return: app
}
```

## Operator Definitions and Overriding

### Transpiler Operator Definition Files

Left-Right supports three operator definition files for different transpilation targets and optimization strategies. All files are overridable at folder/global/intra-script level:

#### operators-recursive.lr

**Purpose:** All default operators implemented using only recursion and control flow.

**Characteristics:**
- Pure Left-Right implementation
- No host language optimizations
- Reference semantics
- Easiest to understand

**Use Case:** Understanding operator semantics, teaching, reference implementation

**Example:**
```javascript
// operators-recursive.lr
{
  // Map implemented with recursion
  $: {
    list: _<@0,
    fn: _>@1,
    ...implement_map_recursively
  }
}
```

#### operators-rust-optimal.lr

**Purpose:** Maximal Rust runtime efficiency.

**Characteristics:**
- Rust-native implementations
- Optimized for performance
- Uses Rust standard library
- Minimal runtime overhead

**Use Case:** Production Rust targets, performance-critical code

**Example:**
```javascript
// operators-rust-optimal.lr
{
  // Map using Rust iterator
  $: {
    list: _<@0,
    fn: _>@1,
    ...rust_iter_map_impl
  }
}
```

#### operators-js-optimal.lr

**Purpose:** Maximal JavaScript runtime efficiency.

**Characteristics:**
- JavaScript-native implementations
- Optimized for V8/Node.js
- Uses JS array methods
- Browser-compatible

**Use Case:** Production JS targets, web applications

**Example:**
```javascript
// operators-js-optimal.lr
{
  // Map using Array.prototype.map
  $: {
    list: _<@0,
    fn: _>@1,
    ...js_array_map_impl
  }
}
```

### Operator Override Levels

Operators can be overridden at multiple levels, with top-down dominance in scope:

#### Override Levels (Highest to Lowest Priority)

1. **Intra-Script Override** — Highest priority, script-local
2. **Folder-Level Override** — All files in folder
3. **Project-Level Override** — Entire project
4. **Global Machine Override** — System-wide default

#### Full Override Example

Override entire operator in current scope:

```javascript
// Override + for this script
{
  +: { a: _<@0, b: _>@1, a + b + 100 },

  result: 5 + 3
  // Result: 108 (5 + 3 + 100)
}
```

#### Partial Override Example

Override specific operator variants:

```javascript
// Override + when concatenating text
{
  +: {
    // When left is text and right is not text
    _<?=`text`|_>?=`text`: `adding text fun`,
    _<+_>
  },

  result: `hello` + 123
  // Result: `adding text fun`
}
```

**Notes:**
- `_<` and `_>` are directional placeholders for operands
- `?` checks type (e.g., `?=`text` means "is text type")
- `|` separates cases (like pattern matching)
- Unspecified cases fall through to original operator

#### Folder-Level Override

Create override file for entire folder:

```bash
# Folder structure
project/
├── lib/
│   ├── .lr-operators        # Override file for lib/ folder
│   └── utils.lr
└── main.lr
```

**.lr-operators content:**
```javascript
// Override operators for all files in lib/
{
  +: { a: _<@0, b: _>@1, a + b * 2 }
}
```

#### Module Scope Integration

Operator overrides work seamlessly with module imports:

```javascript
// File: math.lr
{
  // Override + for this module
  +: { a: _<@0, b: _>@1, a * b },  // * instead of +

  add: { a: _, b: _, a + b }  // Actually multiplies
}

// File: main.lr
{
  math: file[`./math.lr`],
  result: math.add[5, 3]  // Result: 15 (5 * 3)
}
```

#### Top-Down Dominance

In nested scopes, inner overrides take precedence:

```javascript
{
  // Outer scope
  +: { a: _, b: _, a + b * 10 },

  inner: {
    // Inner scope overrides outer
    +: { a: _, b: _, a + b * 100 },

    result: 5 + 3  // Result: 515 (5 + 3 * 100), not 35
  }
}
```

## JavaScript Interop

When transpiling to JavaScript, Left-Right code can import and use NPM packages directly. However, the code follows Left-Right semantics — operators work according to Left-Right rules, type marshalling happens automatically, and the code structure uses Left-Right syntax.

### NPM Package Integration

Import and use JavaScript packages:

```javascript
// Import npm package
{ map, filter, flow, get }: file[`lodash/fp`]

// Use in Left-Right code
items
  >> filter[predicate]
  >> map[transform]
```

### Host Function Calling

Call JavaScript functions directly:

```javascript
// Call host function
{
  fs: file[`fs`],
  path: file[`path`],

  // Read file
  data: fs.readFileSync[path.join[__dirname, `data.json`]],

  // Process data
  processed: processData[data],

  // Write file
  return: fs.writeFileSync[`output.json`, processed]
}
```

### Type Marshalling

Automatic type conversion between Left-Right and JavaScript:

| Left-Right | JavaScript | Conversion |
|------------|-----------|-------------|
| Map | Object | `Object.fromEntries()` |
| List | Array | Direct pass-through |
| Text | String | Direct pass-through |
| Number | Number | Direct pass-through |
| Operator | Function | Direct pass-through |
| Undefined | `undefined` | Direct pass-through |

### JSON Serialization

Serialize to JSON for interoperability:

```javascript
// Serialize to JSON
{
  data: { key: `value` },
  jsonOutput: JSON.stringify[data]
}

// Parse from JSON
{
  jsonString: file[`./data.json`],
  parsedData: JSON.parse[jsonString]
}
```

## Rust Interop

When transpiling to Rust, Left-Right code gains access to Rust's crate ecosystem and native performance. The transpilation generates idiomatic Rust code that preserves Left-Right semantics while leveraging Rust's strengths.

### Crate Integration

Use Rust libraries through FFI:

```javascript
// Import Rust crate
{
  serde: file[`serde`],
  tokio: file[`tokio`],

  // Use Rust types
  data: {
    structure: serde_json::from_str[input]
  }
}
```

### Native Execution

Transpile to Rust for native execution:

```javascript
// Left-Right source
{
  data: [1, 2, 3],
  result: data ${ _< * 2 }
}

// Transpiled to Rust
fn main() {
    let data = vec![1, 2, 3];
    let result: Vec<i32> = data.iter().map(|x| x * 2).collect();
}
```

### Performance Optimization

Rust target provides:
- **Zero-cost abstractions** — No runtime overhead
- **Memory safety** — Compile-time guarantees
- **Parallelism** — Native concurrency support
- **Small binaries** — Efficient deployment

## Transpilation Integration

### Two Target Runtimes

Left-Right transpiles to both JavaScript (Node.js) and Rust (native):

#### JavaScript Target

```bash
# Transpile to JavaScript
lr --target js src/data.lr -o dist/data.js

# Run via Node
node dist/data.js
```

#### Rust Target

```bash
# Transpile to Rust
lr --target rust src/data.lr -o dist/data.rs

# Compile and run
rustc dist/data.rs -o data
./data
```

### Source Maps

Generate source maps for debugging:

```bash
# Transpile with source maps
lr --target js src/data.lr -o dist/data.js --source-maps

# Debug original source
# Maps errors to .lr files
```

## Project Structure

### Typical Organization

```
project/
├── src/
│   ├── main.lr                # Entry point
│   ├── utils.lr               # Utilities
│   ├── data/
│   │   ├── processor.lr      # Data operations
│   │   └── validators.lr    # Validation
│   └── config.lr             # Configuration
├── dist/                      # Transpiled output
├── package.json                # NPM dependencies
├── Cargo.toml                 # Rust dependencies
└── README.md
```

### Module Resolution

```javascript
// File: main.lr

{
  // Local imports
  utils: file[`./utils.lr`],
  processor: file[`./data/processor.lr`],

  // NPM imports
  axios: file[`axios`],

  // Configuration
  config: file[`./config.lr`],

  // Application
  app: {
    processor: processor,
    http: axios,
    settings: config
  }
}
```

## Circular Imports

### Handling Dependencies

Circular import resolution strategy:

```javascript
// File A: depends on B
// a.lr
{
  b: file[`./b.lr`],
  shared: b.sharedValue
}

// File B: depends on A
// b.lr
{
  sharedValue: `shared`,
  a: file[`./a.lr`]
}
```

**Resolution Strategy:**
1. Detect circular dependencies during import resolution
2. Use lazy evaluation for circular references
3. Provide warning for refactor opportunity

## Runtime Execution

### File Execution

Run Left-Right files directly:

```bash
# Execute file
lr path/to/script.lr

# Execute with arguments
lr script.lr arg1 arg2
```

### Watch Mode

Monitor files for changes and re-transpile:

```bash
# Watch directory
lr --watch src/ --target js --output dist/

# Watch single file
lr --watch script.lr
```

## Design Principles

1. **File as Module** — Each file exports single value
2. **Explicit Imports** — `file` operator for dependencies
3. **Seamless Interop** — Direct JS/Rust library access
4. **Type Safety** — Automatic marshalling between systems
5. **Deterministic Resolution** — Predictable import behavior
6. **Dual Targets** — JS and Rust from single source
7. **Watch Mode** — Development workflow support

## Related Concepts

- **Module System** — Code organization units
- **Import/Export** — Dependency management
- **FFI (Foreign Function Interface)** — Cross-language calls
- **Type Marshalling** — Data conversion between languages
- **Transpilation** — Source-to-source translation
- **Source Maps** — Debug mapping
- **NPM** — JavaScript package ecosystem
- **Crates.io** — Rust package ecosystem
- **Dependency Resolution** — Module loading strategy
