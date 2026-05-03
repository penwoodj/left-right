# Pipelines

Left-to-right evaluation, multi-stage data transforms, function composition, and pipeline patterns in Left-Right.

## Single-Stage Pipelines

### Basic Map Pipeline

**Left-Right:**
```left-right
[1, 2, 3, 4, 5]${ _< * 2 }
```

**JavaScript:**
```javascript
[1, 2, 3, 4, 5].map(x => x * 2)
```

**Rust:**
```rust
[1, 2, 3, 4, 5].into_iter().map(|x| x * 2).collect::<Vec<_>>()
```

**Explanation:** Single operation pipeline applies one transformation. Data flows left-to-right.

### Filter Pipeline

**Left-Right:**
```left-right
[1, 2, 3, 4, 5]?{ _< > 2 }
```

**JavaScript:**
```javascript
[1, 2, 3, 4, 5].filter(x => x > 2)
```

**Rust:**
```rust
[1, 2, 3, 4, 5].into_iter().filter(|x| x > 2).collect::<Vec<_>>()
```

**Explanation:** Filter pipeline selects elements matching predicate. Single-stage selection.

### Join Pipeline

**Left-Right:**
```left-right
[`apple`, `banana`, `cherry`]><`, `
```

**JavaScript:**
```javascript
['apple', 'banana', 'cherry'].join(', ')
```

**Rust:**
```rust
["apple", "banana", "cherry"].join(", ")
```

**Explanation:** Join pipeline transforms list to text with separator.

## Multi-Stage Data Transforms

### Filter → Map → Reduce

**Left-Right:**
```left-right
[1, 2, 3, 4, 5, 6]
  ?{ _< % 2 == 0 }
  ${ _< * 2 }
  ${ _< + _< }  // reduce
```

**JavaScript:**
```javascript
[1, 2, 3, 4, 5, 6]
  .filter(x => x % 2 === 0)
  .map(x => x * 2)
  .reduce((sum, x) => sum + x, 0)
```

**Rust:**
```rust
[1, 2, 3, 4, 5, 6]
  .into_iter()
  .filter(|x| x % 2 == 0)
  .map(|x| x * 2)
  .fold(0, |sum, x| sum + x)
```

**Explanation:** Three-stage pipeline filters evens, doubles them, sums result. Each stage receives output of previous.

### Map → Filter → Unique

**Left-Right:**
```left-right
[`alice`, `bob`, `alice`, `charlie`, `bob`]
  ${ _<^_ }
  ?{ _< > 3 }
  ~
```

**JavaScript:**
```javascript
['alice', 'bob', 'alice', 'charlie', 'bob']
  .map(s => s.charAt(0).toUpperCase() + s.slice(1))
  .filter(s => s.length > 3)
  .filter((s, i, arr) => arr.indexOf(s) === i)
```

**Rust:**
```rust
use std::collections::HashSet;

let capitalize = |s: &str| -> String {
  let mut chars: Vec<char> = s.chars().collect();
  chars[0].make_ascii_uppercase();
  chars.into_iter().collect()
};

["alice", "bob", "alice", "charlie", "bob"]
  .into_iter()
  .map(|s| capitalize(s))
  .filter(|s| s.len() > 3)
  .collect::<HashSet<_>>()
  .into_iter()
  .collect::<Vec<_>>()
```

**Explanation:** Capitalize names, filter short names, remove duplicates. Each stage preserves order.

### Complex Data Pipeline

**Left-Right:**
```left-right
[
  {name: `Alice`, age: 30, salary: 50000},
  {name: `Bob`, age: 25, salary: 45000},
  {name: `Charlie`, age: 35, salary: 60000}
]
  ?{ _<@`age` > 28 }
  ${ _<@`salary` * 1.1 }
  ${ _<@`name`^ }
  >< `, `
```

**JavaScript:**
```javascript
[
  {name: 'Alice', age: 30, salary: 50000},
  {name: 'Bob', age: 25, salary: 45000},
  {name: 'Charlie', age: 35, salary: 60000}
]
  .filter(emp => emp.age > 28)
  .map(emp => emp.salary * 1.1)
  .map(emp => emp.name.toUpperCase())
  .join(', ')
```

**Rust:**
```rust
let employees = serde_json::json!([
  {"name": "Alice", "age": 30, "salary": 50000},
  {"name": "Bob", "age": 25, "salary": 45000},
  {"name": "Charlie", "age": 35, "salary": 60000}
]).as_array().unwrap();

employees
  .into_iter()
  .filter(|emp| emp["age"].as_i64().unwrap() > 28)
  .map(|emp| {
    let salary = emp["salary"].as_i64().unwrap() as f64;
    serde_json::json!(salary * 1.1)
  })
  .map(|emp| emp["name"].as_str().unwrap().to_uppercase())
  .collect::<Vec<_>>()
  .iter()
  .map(|s| s.as_str().unwrap())
  .collect::<Vec<_>>()
  .iter()
  .map(|s| s.as_str().unwrap())
  .collect::<Vec<_>>()
  .join(", ")
```

**Explanation:** Realistic ETL pipeline: filter by age, calculate 10% raise, extract names, join to text.

## Threat Analysis Example (Exact from PenroScript.md)

### Original Lodash FP Code

**JavaScript:**
```javascript
// JS with Lodash FP
({ threats }) => {
  const maliciousThreatsCount = flow(
    filter((threat) => get(['AI Confidence Level'].value, threat) === 'malicious'),
    size
  )(threats);

  const threatClassifications = flow(
    map(flow(get(['Classification'].value), capitalize)),
    uniq,
    join(', '),
    (threatClassifications) =>
      threatClassifications && `Threat Classifications: ${threatClassifications}`
  )(threats);

  return []
    .concat(maliciousThreatsCount)
    .concat(threatClassifications)
}
```

### Left-Right Anonymous Function

**Left-Right:**
```left-right
{
  threats: _<@[0, `threats`],
  maliciousThreatsCount: threats
    ?{ _<@[`AI Confidence Level`, `value`] = `malicious` }
    #,
  threatClassifications: threats
    ${ _<@[`Classification`, `value`] ^_ }
    ~,
    >< `, `,
  {
    threatClassifications: _<,
    `Threat Classifications: {threatClassifications}`
  },
  [] + maliciousThreatsCount + threatClassifications
}
```

**JavaScript:**
```javascript
({ threats }) => {
  const maliciousThreatsCount = threats
    .filter(threat => threat['AI Confidence Level'].value === 'malicious')
    .length;

  const threatClassifications = threats
    .map(threat => threat['Classification'].value)
    .map(classification => classification.charAt(0).toUpperCase() + classification.slice(1))
    .filter((classification, index, arr) => arr.indexOf(classification) === index)
    .join(', ');

  return [
    ...maliciousThreatsCount,
    threatClassifications ? `Threat Classifications: ${threatClassifications}` : undefined
  ];
}
```

**Rust:**
```rust
use serde_json;

fn analyze_threats(threats: serde_json::Value) -> serde_json::Value {
  let threats_array = threats[0]["threats"].as_array().unwrap();

  let malicious_threats_count = threats_array
    .iter()
    .filter(|threat| threat["AI Confidence Level"]["value"].as_str().unwrap() == "malicious")
    .count();

  let threat_classifications: String = threats_array
    .iter()
    .map(|threat| threat["Classification"]["value"].as_str().unwrap().to_string())
    .map(|classification| {
      let mut chars: Vec<char> = classification.chars().collect();
      chars[0].make_ascii_uppercase();
      chars.into_iter().collect()
    })
    .collect::<std::collections::HashSet<_>>()
    .into_iter()
    .collect::<Vec<_>>()
    .join(", ");

  let formatted = if !threat_classifications.is_empty() {
    format!("Threat Classifications: {}", threat_classifications)
  } else {
    String::new()
  };

  json!([
    malicious_threats_count,
    formatted
  ])
}
```

**Explanation:** Complex threat analysis pipeline from ServiceNow integration. Demonstrates:
- Path access with list syntax: `@[`AI Confidence Level`, `value`]`
- Filtering by nested properties
- Counting filtered results: `?{...} #`
- Mapping and capitalizing: `${ _< ^_ }`
- Deduplicating: `~`
- Joining: `>< `, `
- Conditional formatting
- List concatenation: `[] + ... + ...`

## Chaining with Composition (>>, <<)

### Forward Composition

**Left-Right:**
```left-right
double: { _< * 2 }
increment: { _< + 1 }
transform: double >> increment

[1, 2, 3, 4]${ transform }
```

**JavaScript:**
```javascript
const double = x => x * 2;
const increment = x => x + 1;
const transform = x => increment(double(x));

[1, 2, 3, 4].map(transform)
```

**Rust:**
```rust
let double = |x: i32| x * 2;
let increment = |x: i32| x + 1;
let transform = |x: i32| increment(double(x));

[1, 2, 3, 4].into_iter().map(transform).collect::<Vec<_>>()
```

**Explanation:** `>>` composes functions left-to-right. `double >> increment` means apply `double`, then `increment`.

### Backward Composition

**Left-Right:**
```left-right
double: { _< * 2 }
increment: { _< + 1 }
transform: increment << double

[1, 2, 3, 4]${ transform }
```

**JavaScript:**
```javascript
const double = x => x * 2;
const increment = x => x + 1;
const transform = x => double(increment(x));

[1, 2, 3, 4].map(transform)
```

**Rust:**
```rust
let double = |x: i32| x * 2;
let increment = |x: i32| x + 1;
let transform = |x: i32| double(increment(x));

[1, 2, 3, 4].into_iter().map(transform).collect::<Vec<_>>()
```

**Explanation:** `<<` composes functions right-to-left. `increment << double` means apply `increment`, then `double`.

### Multi-Function Composition

**Left-Right:**
```left-right
{
  uppercase: { _<^ },
  capitalize: { _<^_ },
  reverse: { _<@0... },
  pipeline: uppercase >> capitalize >> reverse
}
`hello world` ${ pipeline }
```

**JavaScript:**
```javascript
const uppercase = s => s.toUpperCase();
const capitalize = s => s.charAt(0).toUpperCase() + s.slice(1);
const reverse = s => s.split('').reverse().join('');
const pipeline = s => reverse(capitalize(uppercase(s)));

pipeline('hello world')
```

**Rust:**
```rust
let uppercase = |s: &str| s.to_uppercase();
let capitalize = |s: &str| -> String {
  let mut chars: Vec<char> = s.chars().collect();
  chars[0].make_ascii_uppercase();
  chars.into_iter().collect()
};
let reverse = |s: &str| s.chars().rev().collect::<String>();
let pipeline = |s: &str| reverse(capitalize(uppercase(s)));

pipeline("hello world")
```

**Explanation:** Chain multiple composed functions. Each `>>` adds another transformation step.

### Composition in Pipeline

**Left-Right:**
```left-right
{
  square: { _< * _< }
  addOne: { _< + 1 }
  process: square >> addOne
}
[1, 2, 3, 4] ${ process } ?{ _< < 20 }
```

**JavaScript:**
```javascript
const square = x => x * x;
const addOne = x => x + 1;
const process = x => addOne(square(x));

[1, 2, 3, 4]
  .map(process)
  .filter(x => x < 20)
```

**Rust:**
```rust
let square = |x: i32| x * x;
let add_one = |x: i32| x + 1;
let process = |x: i32| add_one(square(x));

[1, 2, 3, 4]
  .into_iter()
  .map(process)
  .filter(|&x| x < 20)
  .collect::<Vec<_>>()
```

**Explanation:** Compose functions, then use in pipeline. Composition reusability meets pipeline clarity.

## How LTR Evaluation Flows Through Pipelines

### Evaluation Order

**Left-Right:**
```left-right
[1, 2, 3]
  ?{ _< > 1 }
  ${ _< * 2 }
  ${ _< + 10 }
```

**Step-by-step:**
```
Step 1: [1, 2, 3]
Step 2: [2, 3]          # After filter > 1
Step 3: [4, 6]          # After map * 2
Step 4: [14, 16]         # After map + 10
```

**JavaScript:**
```javascript
[1, 2, 3]
  .filter(x => x > 1)
  .map(x => x * 2)
  .map(x => x + 10)
  // Result: [14, 16]
```

**Rust:**
```rust
[1, 2, 3]
  .into_iter()
  .filter(|&x| x > 1)
  .map(|x| x * 2)
  .map(|x| x + 10)
  .collect::<Vec<_>>()
  // Result: [14, 16]
```

**Explanation:** Left-to-right evaluation means each operator receives output of previous. No precedence confusion.

### Nested Pipeline Resolution

**Left-Right:**
```left-right
{
  filtered: [1, 2, 3, 4, 5]?{ _< % 2 == 0 },
  result: filtered${ _< * 2 }#  // filtered is now [2, 4]
}
```

**JavaScript:**
```javascript
{
  filtered: [1, 2, 3, 4, 5].filter(x => x % 2 === 0),
  result: filtered.map(x => x * 2).length
}
// filtered = [2, 4], result = 2
```

**Rust:**
```rust
let filtered = [1, 2, 3, 4, 5]
  .into_iter()
  .filter(|&x| x % 2 == 0)
  .collect::<Vec<_>>();
// filtered = [2, 4]

let result = filtered
  .into_iter()
  .map(|x| x * 2)
  .collect::<Vec<_>>()
  .len();
// result = 2
```

**Explanation:** Map keys evaluate top-to-bottom. Later keys reference earlier computed values.

### Pipeline with Error Handling

**Left-Right:**
```left-right
[1, 2, 3, 4, 5]
  ${ _< / 0 | _< }
  ?{ _< ! undefined }
```

**JavaScript:**
```javascript
[1, 2, 3, 4, 5]
  .map(x => {
    try {
      return x / 0;
    } catch {
      return x;
    }
  })
  .filter(x => x !== undefined)
```

**Rust:**
```rust
[1, 2, 3, 4, 5]
  .into_iter()
  .map(|x| x.checked_div(0).unwrap_or(x))
  .filter(|&x| x != 0)  // assuming 0 represents undefined/error case
  .collect::<Vec<_>>()
```

**Explanation:** Error handling in pipeline uses fallback with `|`. Filter out undefined/error values.

### Parallel Pipeline Branches

**Left-Right:**
```left-right
data: [1, 2, 3, 4, 5]
  branch1: data?{ _< > 2 }$#
  branch2: data?{ _< <= 2 }$#
  combined: [branch1, branch2]
```

**JavaScript:**
```javascript
const data = [1, 2, 3, 4, 5];
const branch1 = data.filter(x => x > 2).length;
const branch2 = data.filter(x => x <= 2).length;
const combined = [branch1, branch2];
// [3, 2]
```

**Rust:**
```rust
let data = vec![1, 2, 3, 4, 5];
let branch1 = data.iter().filter(|&&x| x > 2).count();
let branch2 = data.iter().filter(|&&x| x <= 2).count();
let combined = vec![branch1, branch2];
// [3, 2]
```

**Explanation:** Single data source, multiple pipeline branches. Each branch processes independently.
