# Interop

Left-Right is its own programming language with its own schematics, designed for transpilation to both JavaScript and Rust. While it has its own language identity, it provides seamless interop with the ecosystems of its target languages.

## Interop Philosophy

Left-Right is **not** a DSL or a subset of JavaScript/Rust. It is a **general-purpose scripting language** with:
- Its own syntax, semantics, and evaluation model
- Independent type system (Operator, Map, List, Text, Number, Undefined)
- Point-free, operator-based paradigm
- Hierarchical-data oriented design

However, Left-Right offers:
- **Full JavaScript library access** when transpiling to JS/Node.js targets
- **Full Rust crate access** when transpiling to Rust targets
- **Native module system** that maps to the target language's import/export system
- **Transparent FFI** for calling host language functions

This dual-target approach means you write code once in Left-Right, then choose the best ecosystem for deployment.

## Using npm Packages from Left-Right

When targeting JavaScript/Node.js, Left-Right can import and use any npm package.

### Importing Lodash FP

**Left-Right:**
```left-right
{
  map: file[`lodash/fp`]@`map`,
  filter: file[`lodash/fp`]@`filter`,
  flow: file[`lodash/fp`]@`flow`,

  processData: { data: _<,
    flow(filter{ _< > 5 }, map{ _< * 2 })(data)
  }
}
```

**JavaScript:**
```javascript
import { map, filter, flow } from `lodash/fp`;

const processData = (data) => {
  return flow(
    filter(x => x > 5),
    map(x => x * 2)
  )(data);
};
```

**Rust:**
```rust
// Rust equivalent would use native iterators or serde_json utilities
fn process_data(data: Vec<i32>) -> Vec<i32> {
  data
    .into_iter()
    .filter(|&x| x > 5)
    .map(|x| x * 2)
    .collect()
}
```

**Explanation:** Import functions from npm packages with `file['package-name']`. When targeting JS, these imports are transpiled to ES6 imports.

### Using Multiple Packages

**Left-Right:**
```left-right
{
  axios: file[`axios`],
  lodash: file[`lodash`],
  moment: file[`moment`],

  fetchData: { url: _<,
    response: axios.get(url),
    data: lodash.get(response, `data`),
    formatted: moment(data).format(`YYYY-MM-DD`)
  }
}
```

**JavaScript:**
```javascript
import axios from `axios`;
import lodash from 'lodash';
import moment from `moment`;

const fetchData = async (url) => {
  const response = await axios.get(url);
  const data = lodash.get(response, 'data');
  const formatted = moment(data).format('YYYY-MM-DD');

  return formatted;
};
```

**Rust:**
```rust
use reqwest;
use chrono;

async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
  let response = reqwest::get(url).send().await?;
  let data: String = response.text()?;
  let formatted = chrono::DateTime::parse_from_rfc3339(&data)
    .unwrap()
    .format("%Y-%m-%d")
    .to_string();

  Ok(formatted)
}
```

**Explanation:** Combine multiple npm packages. Import with `file`, use like native functions. Only works when transpiling to JavaScript.

### Default Imports

**Left-Right:**
```left-right
{
  ...file[`lodash/fp`],
  ...file[`axios`],

  processAndFetch: { data: _<, url: _<,
    processed: flow(map{ _< * 2 }, filter{ _< > 10 })(data),
    result: axios.post(url, processed)
  }
}
```

**JavaScript:**
```javascript
import * as lodashFp from `lodash/fp`;
import axios from `axios`;

const processAndFetch = async (data, url) => {
  const processed = lodashFp.flow(
    lodashFp.map(x => x * 2),
    lodashFp.filter(x => x > 10)
  )(data);

  const result = await axios.post(url, processed);
  return result;
};
```

**Rust:**
```rust
async fn process_and_fetch(data: Vec<i32>, url: &str) -> Result<reqwest::Response, reqwest::Error> {
  let processed = data
    .into_iter()
    .map(|x| x * 2)
    .filter(|&x| x > 10)
    .collect::<Vec<_>>();

  reqwest::Client::new()
    .post(url)
    .json(&processed)
    .send()
    .await
}
```

**Explanation:** Spread import with `...file['package']`. Brings all exports into scope.

## Using Rust Crates from Left-Right

When targeting Rust, Left-Right can import and use any Rust crate.

### Using Serde for JSON

**Left-Right:**
```left-right
{
  serde: file[`serde_json`],
  Data: { id: i32, name: str },

  serialize: { obj: _<,
    serde.to_json(obj)
  },

  deserialize: { json: _<,
    serde.from_json(json, Data)
  }
}
```

**Rust:**
```rust
use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};

#[derive(Serialize, Deserialize)]
struct Data {
  id: i32,
  name: String,
}

fn serialize(obj: Data) -> Result<String, serde_json::Error> {
  to_string(&obj)
}

fn deserialize(json: &str) -> Result<Data, serde_json::Error> {
  from_str(json)
}
```

**JavaScript:**
```javascript
const serialize = (obj) => JSON.stringify(obj);
const deserialize = (json) => JSON.parse(json);
```

**Explanation:** Use Rust crates via `file` syntax. Serde for JSON serialization/deserialization. Only works when transpiling to Rust.

### Using Tokio for Async

**Left-Right:**
```left-right
{
  tokio: file[`tokio`],
  http: file[`reqwest`],

  fetchMultiple: { urls: _<,
    results: tokio.join_all(
      urls${ url => http.get(url) }
    )
  }
}
```

**Rust:**
```rust
use tokio;
use reqwest;

async fn fetch_multiple(urls: Vec<&str>) -> Vec<Result<String, reqwest::Error>> {
  let tasks = urls
    .into_iter()
    .map(|url| async move {
      reqwest::get(url).send().await.map(|r| r.text().unwrap())
    });

  tokio::join_all(tasks).await
}
```

**JavaScript:**
```javascript
const fetchMultiple = async (urls) => {
  const results = await Promise.all(
    urls.map(url => fetch(url).then(r => r.text()))
  );

  return results;
};
```

**Explanation:** Async operations with Tokio runtime. `tokio.join_all` equivalent to `Promise.all`. Only works when transpiling to Rust.

### Using Chrono for Date/Time

**Left-Right:**
```left-right
{
  chrono: file[`chrono`],

  formatDate: { timestamp: _<,
    dt: chrono.from_timestamp(timestamp),
    formatted: dt.format(`%Y-%m-%d %H:%M:%S`)
  }
}
```

**Rust:**
```rust
use chrono::{Utc, DateTime};

fn format_date(timestamp: i64) -> String {
  let dt = Utc.timestamp_opt(timestamp, 0).unwrap();
  dt.format("%Y-%m-%d %H:%M:%S").to_string()
}
```

**JavaScript:**
```javascript
const formatDate = (timestamp) => {
  const dt = new Date(timestamp * 1000);
  return dt.toISOString().slice(0, 19).replace('T', ' ');
};
```

**Explanation:** Chrono for date/time operations. Format timestamps to human-readable text. Only works when transpiling to Rust.

## FFI Patterns

Left-Right provides direct access to host language APIs and standard libraries.

### Calling Native JavaScript Functions

**Left-Right:**
```left-right
{
  console: file['console'],

  logResult: { value: _<,
    console.log(`Result: `, value)
  }
}
```

**JavaScript:**
```javascript
const logResult = (value) => {
  console.log(`Result: `, value);
};
```

**Rust:**
```rust
fn log_result(value: serde_json::Value) {
  println!("Result: {}", value);
}
```

**Explanation:** Direct FFI to host JavaScript environment. `console` object available in JS runtime. Only works when targeting JavaScript.

### Browser API Access

**Left-Right:**
```left-right
{
  window: file['window'],
  document: file['document'],

  updateDOM: { elementId: _<, content: _<,
    element: document.getElementById(elementId),
    element.innerText = content
  }
}
```

**JavaScript:**
```javascript
const updateDOM = (elementId, content) => {
  const element = document.getElementById(elementId);
  element.innerText = content;
};
```

**Rust:**
```rust
// Not applicable - Rust doesn't have browser DOM APIs
// This would target WebAssembly runtime in browser
```

**Explanation:** Access browser DOM APIs. Transpiles to JS for web targets. Only works when targeting JavaScript in browser environments.

### Node.js API Access

**Left-Right:**
```left-right
{
  fs: file['fs'],
  path: file['path'],

  readFile: { filepath: _<,
    resolved: path.resolve(filepath),
    content: fs.readFile(resolved, `utf-8`)
  }
}
```

**JavaScript:**
```javascript
import fs from 'fs/promises';
import path from 'path';

const readFile = async (filepath) => {
  const resolved = path.resolve(filepath);
  const content = await fs.readFile(resolved, 'utf-8');
  return content;
};
```

**Rust:**
```rust
use std::fs;
use std::path::Path;

fn read_file(filepath: &str) -> Result<String, std::io::Error> {
  let path = Path::new(filepath);
  fs::read_to_string(path)
}
```

**Explanation:** Node.js filesystem APIs. Use `fs`, `path` packages for file operations. Only works when targeting JavaScript in Node.js environment.

## Import/Export Between Left-Right Modules

Left-Right has its own module system that maps to the target language's import/export system.

### Named Exports

**Left-Right:**
```left-right
// File: utils.lr
{
  double: { _< * 2 },
  triple: { _< * 3 },
  greet: { name: _<, `Hello, ` & name & `!` }
}
```

**JavaScript:**
```javascript
// File: utils.js
export const double = x => x * 2;
export const triple = x => x * 3;
export const greet = name => `Hello, ${name}!`;
```

**Rust:**
```rust
// File: utils.rs
pub fn double(x: i32) -> i32 {
  x * 2
}

pub fn triple(x: i32) -> i32 {
  x * 3
}

pub fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}
```

**Explanation:** Define named exports. Each top-level key becomes export in the target language.

### Default Export

**Left-Right:**
```left-right
// File: processor.lr
{
  processData: { data: _<,
    data${ _< * 2 }$?{ _< > 10 }
  }
}
```

**JavaScript:**
```javascript
// File: processor.js
export default function processData(data) {
  return data
    .map(x => x * 2)
    .filter(x => x > 10);
}
```

**Rust:**
```rust
// File: processor.rs
pub fn process_data(data: Vec<i32>) -> Vec<i32> {
  data
    .into_iter()
    .map(|x| x * 2)
    .filter(|&x| x > 10)
    .collect()
}
```

**Explanation:** Default export uses special key name. Convention: use main export key.

### Named Imports

**Left-Right:**
```left-right
// File: main.lr
{
  utils: file['./utils'],
  double: utils@`double`,
  greet: utils@`greet`,

  result: [double(5), greet(`Alice`)]
}
```

**JavaScript:**
```javascript
// File: main.js
import { double, greet } from './utils';

const result = [double(5), greet(`Alice`)];
```

**Rust:**
```rust
// File: main.rs
mod utils;
use utils::{double, greet};

fn main() {
  let result = vec![double(5), greet("Alice")];
}
```

**Explanation:** Import specific exports with `file['path']@'name'`. Selective imports.

### Re-Exports

**Left-Right:**
```left-right
// File: api.lr
{
  ...file['./utils'],
  ...file['./http'],

  combined: {
    process: { data: _<,
      data >> utils.double >> http.post
    }
  }
}
```

**JavaScript:**
```javascript
// File: api.js
export * from './utils';
export * from './http';

export const combined = {
  process: (data) => {
    const transformed = utils.double(data);
    return http.post(transformed);
  }
};
```

**Rust:**
```rust
// File: api.rs
pub mod utils;
pub mod http;

pub struct Combined;

impl Combined {
  pub fn process(data: i32) -> String {
    let transformed = utils::double(data);
    http::post(transformed)
  }
}
```

**Explanation:** Re-export with `...file['path']`. Combine multiple modules in one.

## Mixing Left-Right with JS/Rust

Left-Right can call into existing JS/Rust code, and JS/Rust can call transpiled Left-Right code.

### Left-Right Calling JavaScript

**Left-Right:**
```left-right
// File: app.lr
{
  jsLib: file['./legacy.js'],

  processData: { input: _<,
    preprocessed: jsLib.legacyTransform(input),
    result: preprocessed${ _< * 2 }
  }
}
```

**JavaScript:**
```javascript
// File: legacy.js
export function legacyTransform(input) {
  // Complex legacy logic
  return input.split('').reverse().join('');
}

// File: app.js
import { legacyTransform } from './legacy';

const processData = (input) => {
  const preprocessed = legacyTransform(input);
  return preprocessed.map(x => x * 2);
};
```

**Rust:**
```rust
// File: legacy.rs
pub fn legacy_transform(input: &str) -> String {
  input.chars().rev().collect()
}

// File: app.rs
mod legacy;

fn process_data(input: &str) -> Vec<i32> {
  let preprocessed = legacy::legacy_transform(input);
  preprocessed.chars().map(|c| c.len() as i32).collect()
}
```

**Explanation:** Call existing JavaScript from Left-Right. Interop layer for migration. Only works when targeting JavaScript.

### JavaScript Calling Left-Right

**Left-Right:**
```left-right
// File: algorithms.lr
{
  fibonacci: { n: _<,
    n < 2 & n |
    {
      a: fibonacci(n - 1),
      b: fibonacci(n - 2),
      a + b
    }
  }
}
```

**JavaScript:**
```javascript
// File: usage.js
import { fibonacci } from './algorithms';

// Use in JS application
const result = fibonacci(10);
console.log(result);
```

**Rust:**
```rust
// File: algorithms.rs
pub fn fibonacci(n: i32) -> i32 {
  if n < 2 {
    return n;
  }

  let a = fibonacci(n - 1);
  let b = fibonacci(n - 2);
  a + b
}

// File: main.rs
mod algorithms;

fn main() {
  let result = algorithms::fibonacci(10);
  println!("{}", result);
}
```

**Explanation:** Use Left-Right functions from JavaScript. Transpile to JS, import like any module.

### Mixed Project Structure

**Left-Right:**
```left-right
// File: project.lr
{
  // Left-Right core logic
  core: file['./core.lr'],

  // JavaScript utilities
  jsUtils: file['./utils.js'],

  // Rust performance module
  rustPerf: file['./performance.rs'],

  pipeline: { data: _<,
    validated: core.validate(data),
    optimized: rustPerf.optimize(validated),
    formatted: jsUtils.prettyPrint(optimized)
  }
}
```

**JavaScript:**
```javascript
// File: core.lr (transpiled to core.js)
export const validate = (data) => {
  // Validation logic
  return data.filter(x => x != null);
};

// File: utils.js
export const prettyPrint = (data) => {
  return JSON.stringify(data, null, 2);
};

// File: usage.js
import { validate } from './core.js';
import { prettyPrint } from './utils.js';
import { optimize } from './performance.wasm';

const pipeline = (data) => {
  const validated = validate(data);
  const optimized = optimize(validated);
  const formatted = prettyPrint(optimized);
  return formatted;
};
```

**Rust:**
```rust
// File: performance.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn optimize(data: Vec<i32>) -> Vec<i32> {
  // High-performance Rust implementation
  data.into_iter().map(|x| x * 2).collect()
}

// File: main.rs
mod core;
mod performance;

fn main() {
  let data = vec![1, 2, 3, 4, 5];
  let validated = core::validate(data);
  let optimized = performance::optimize(validated);
  // Use optimized data
}
```

**Explanation:** Hybrid project using Left-Right, JS, and Rust together. Each language for its strengths.

### WebAssembly Integration

**Left-Right:**
```left-right
{
  wasmModule: file['./heavy_computation.rs'],

  processData: { data: _<,
    // Offload heavy computation to Rust/WASM
    result: wasmModule.heavyCalculation(data)
  }
}
```

**JavaScript:**
```javascript
// File: usage.js
import { heavyCalculation } from './heavy_computation.wasm';

const processData = async (data) => {
  // Offload to WebAssembly
  const result = await heavyCalculation(data);
  return result;
};
```

**Rust:**
```rust
// File: heavy_computation.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn heavy_calculation(data: Vec<i32>) -> Vec<i32> {
  // CPU-intensive operations
  data.into_iter()
    .map(|x| {
      let mut result = x;
      for _ in 0..1000 {
        result = (result * 7 + 3) % 1000;
      }
      result
    })
    .collect()
}

// Build with: wasm-pack build --target web
```

**Explanation:** WebAssembly integration for performance. Left-Right orchestrates, Rust computes, JS runs in browser.

## Target-Specific Transpilation

Left-Right's transpiler generates different output based on the target language.

### JavaScript Target

When transpiling to JavaScript:
- `file['package']` imports become ES6 `import` statements
- Left-Right maps transpile to JavaScript objects
- Left-Right lists transpile to JavaScript arrays
- Left-Right text transpiles to JavaScript strings
- Type system maps loosely (truthy/falsy values)
- Async operations use JavaScript Promises

**Example:**
```bash
lr build app.lr --target js --output app.js
```

### Rust Target

When transpiling to Rust:
- `file['crate']` imports become `use` statements
- Left-Right maps transpile to `serde_json::Value` or typed structs
- Left-Right lists transpile to `Vec<T>`
- Left-Right text transpiles to `String` or `&str`
- Type system maps more strictly (compile-time checks)
- Async operations use `tokio` runtime

**Example:**
```bash
lr build app.lr --target rust --output app.rs
```

## Type Mapping Between Targets

### Text Type

- **Left-Right**: Text type (backtick strings)
- **JavaScript**: JavaScript `String`
- **Rust**: Rust `String` or `&str`

### List Type

- **Left-Right**: List type (ordered collections)
- **JavaScript**: JavaScript `Array`
- **Rust**: Rust `Vec<T>`

### Map Type

- **Left-Right**: Map type (key-value collections)
- **JavaScript**: JavaScript `Object` (or `Map` for structured data)
- **Rust**: Rust `HashMap<K, V>` or `BTreeMap<K, V>`

### Number Type

- **Left-Right**: Number type (all numeric values)
- **JavaScript**: JavaScript `Number` (float64)
- **Rust**: Rust `i32`, `i64`, `f64`, etc. (type inference)

### Undefined Type

- **Left-Right**: Undefined type (missing/null values)
- **JavaScript**: JavaScript `undefined` (JSON `null` converts to undefined by default, configurable)
- **Rust**: Rust `Option<T>` with `None`

### Operator Type

- **Left-Right**: Operator type (functions with `_<`/`_>` placeholders)
- **JavaScript**: JavaScript function (lambda)
- **Rust**: Rust closure or function

## Conclusion

Left-Right's interop model provides:
1. **Language independence**: It's its own language with unique paradigms
2. **Dual target support**: Transpile to either JavaScript or Rust
3. **Full ecosystem access**: Use npm packages or Rust crates depending on target
4. **Seamless integration**: Mix Left-Right with existing JS/Rust code
5. **Type-aware transpilation**: Output respects target language conventions

Choose Left-Right when you want a modern, expressive language with the flexibility to deploy to either the JavaScript or Rust ecosystem.
