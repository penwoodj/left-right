# Conditionals

Type checking, conditional filtering, truthy/falsy expressions, equality comparisons, and guard patterns in Left-Right.

## Type Checking (!?)

### Basic Type Check

**Left-Right:**
```left-right
`hello`!?
42!?
true!?
```

**JavaScript:**
```javascript
typeof 'hello' === 'string'
typeof 42 === 'number'
typeof true === 'boolean'
```

**Rust:**
```rust
matches!("hello", &str)
matches!(42, i32)
matches!(true, bool)
```

**Explanation:** The `!?` operator returns type name as text. Used for runtime type inspection. Returns: `text`, `number`, `list`, `map`, `operator`, `undefined`.

### Type Guard in Pipeline

**Left-Right:**
```left-right
[1, `hello`, 42, `world`, true]
  $?{ _< !? = `number` }
```

**JavaScript:**
```javascript
[1, 'hello', 42, 'world', true]
  .filter(x => typeof x === 'number')
```

**Rust:**
```rust
[1, "hello", 42, "world", true]
  .into_iter()
  .filter(|x| matches!(x, i32))
  .collect::<Vec<_>>()
```

**Explanation:** Filter by type in pipeline. Only numbers pass through.

### Complex Type Check

**Left-Right:**
```left-right
value: `test`
result: value!? = `text` & value.length > 3 | value!? = `number` & value > 10 | false
```

**JavaScript:**
```javascript
const value = 'test';
const result = typeof value === 'string' && value.length > 3
  ? true
  : typeof value === 'number' && value > 10
    ? true
    : false;
```

**Rust:**
```rust
let value = "test";
let result = match value {
  s if matches!(s, &str) && s.len() > 3 => true,
  n if matches!(n, i32) && n > 10 => true,
  _ => false
};
```

**Explanation:** Multiple type conditions with boolean logic. `&` for AND, `|` for OR.

### List Type Verification

**Left-Right:**
```left-right
data: [1, 2, 3]
isList: data!? = `list`

isList & `Valid list` | `Not a list`
```

**JavaScript:**
```javascript
const data = [1, 2, 3];
const isList = Array.isArray(data);

isList ? 'Valid list' : 'Not a list'
```

**Rust:**
```rust
let data = vec![1, 2, 3];
let is_list = matches!(data, Vec<_>);

if is_list {
  "Valid list"
} else {
  "Not a list"
}
```

**Explanation:** Verify list type before operations. Type check as guard.

## Conditional Filter ($?)

### Simple Conditional Filter

**Left-Right:**
```left-right
[1, 2, 3, 4, 5]$?{ _< > 3 }
```

**JavaScript:**
```javascript
[1, 2, 3, 4, 5].filter(x => x > 3)
```

**Rust:**
```rust
[1, 2, 3, 4, 5]
  .into_iter()
  .filter(|&x| x > 3)
  .collect::<Vec<_>>()
```

**Explanation:** `$?{ predicate }` filters collection where predicate is true. Basic conditional selection.

### Multi-Condition Filter

**Left-Right:**
```left-right
[1, 2, 3, 4, 5, 6]$?{ _< % 2 == 0 & _< > 2 }
```

**JavaScript:**
```javascript
[1, 2, 3, 4, 5, 6].filter(x => x % 2 === 0 && x > 2)
```

**Rust:**
```rust
[1, 2, 3, 4, 5, 6]
  .into_iter()
  .filter(|&x| x % 2 == 0 && x > 2)
  .collect::<Vec<_>>()
```

**Explanation:** Combine conditions with `&` (AND). Only elements matching all conditions pass.

### OR Condition Filter

**Left-Right:**
```left-right
[1, 2, 3, 4, 5]$?{ _< < 2 | _< > 4 }
```

**JavaScript:**
```javascript
[1, 2, 3, 4, 5].filter(x => x < 2 || x > 4)
```

**Rust:**
```rust
[1, 2, 3, 4, 5]
  .into_iter()
  .filter(|&x| x < 2 || x > 4)
  .collect::<Vec<_>>()
```

**Explanation:** Use `|` (OR) for alternative conditions. Elements matching any condition pass.

### Object Property Filter

**Left-Right:**
```left-right
[
  {name: `Alice`, age: 30},
  {name: `Bob`, age: 25},
  {name: `Charlie`, age: 35}
]$?{ _<@`age` > 28 & _<@`name`^ = `ALICE` }
```

**JavaScript:**
```javascript
[
  {name: 'Alice', age: 30},
  {name: 'Bob', age: 25},
  {name: 'Charlie', age: 35}
].filter(obj =>
  obj.age > 28 && obj.name.toUpperCase() === 'ALICE'
)
```

**Rust:**
```rust
let objs = serde_json::json!([
  {"name": "Alice", "age": 30},
  {"name": "Bob", "age": 25},
  {"name": "Charlie", "age": 35}
]).as_array().unwrap();

objs
  .into_iter()
  .filter(|obj| {
    let age = obj["age"].as_i64().unwrap();
    let name = obj["name"].as_str().unwrap().to_uppercase();
    age > 28 && name == "ALICE"
  })
  .collect::<Vec<_>>()
```

**Explanation:** Filter objects by multiple properties. Combine property conditions.

## Truthy/Falsy Expressions

### Basic AND

**Left-Right:**
```left-right
true & true
true & false
```

**JavaScript:**
```javascript
true && true
true && false
```

**Rust:**
```rust
true && true
true && false
```

**Explanation:** `&` is logical AND. Returns true only if both operands are true (truthy in Left-Right).

### Basic OR

**Left-Right:**
```left-right
true | false
false | false
```

**JavaScript:**
```javascript
true || false
false || false
```

**Rust:**
```rust
true || false
false || false
```

**Explanation:** `|` is logical OR. Returns true if either operand is true (truthy in Left-Right).

### Complex Logical Expression

**Left-Right:**
```left-right
5 > 3 & 2 < 4 | 10 == 10
```

**JavaScript:**
```javascript
5 > 3 && 2 < 4 || 10 === 10
// true && true || true
// true || true
// true
```

**Rust:**
```rust
5 > 3 && 2 < 4 || 10 == 10
// true
```

**Explanation:** Left-to-right evaluation. No operator precedence confusion. Group explicitly if needed.

### Negated Expression

**Left-Right:**
```left-right
!{5 > 3 & 2 < 4}
!true
```

**JavaScript:**
```javascript
!(5 > 3 && 2 < 4)
!true
```

**Rust:**
```rust
!(5 > 3 && 2 < 4)
!true
```

**Explanation:** `!` negates truthy/falsy values. Apply to entire expression with grouping.

## Equality: = vs ==

### Unordered Equality (=)

**Left-Right:**
```left-right
[1, 2, 3] = [3, 2, 1]
{a: 1, b: 2} = {b: 2, a: 1}
```

**JavaScript:**
```javascript
new Set([1, 2, 3]).size === new Set([3, 2, 1]).size
Object.keys({a: 1, b: 2}).length === Object.keys({b: 2, a: 1}).length
```

**Rust:**
```rust
use std::collections::HashSet;

let set1: HashSet<i32> = [1, 2, 3].into_iter().collect();
let set2: HashSet<i32> = [3, 2, 1].into_iter().collect();
set1.len() == set2.len()

// For maps
let map1 = serde_json::json!({"a": 1, "b": 2}).as_object().unwrap();
let map2 = serde_json::json!({"b": 2, "a": 1}).as_object().unwrap();
map1.len() == map2.len()
```

**Explanation:** The `=` operator checks unordered equality. Lists and maps with same elements are equal regardless of order.

### Ordered Equality (==)

**Left-Right:**
```left-right
[1, 2, 3] == [1, 2, 3]
[1, 2, 3] == [3, 2, 1]
```

**JavaScript:**
```javascript
JSON.stringify([1, 2, 3]) === JSON.stringify([1, 2, 3])
JSON.stringify([1, 2, 3]) === JSON.stringify([3, 2, 1])
// true
// false
```

**Rust:**
```rust
[1, 2, 3] == [1, 2, 3]
[1, 2, 3] == [3, 2, 1]
// true
// false
```

**Explanation:** The `==` operator checks ordered equality. Position and order must match exactly.

### Type-Insensitive Equality

**Left-Right:**
```left-right
5 = `5`  // false, different types
5 == `5`  // false, different types
```

**JavaScript:**
```javascript
5 === '5'  // false
5 == '5'    // true in JS (loose equality)
```

**Rust:**
```rust
5 == "5"  // compile error: type mismatch
```

**Explanation:** Left-Right is loosely typed but type-aware. Both operators require type match.

### Deep Equality

**Left-Right:**
```left-right
{a: {b: 1}} = {a: {b: 1}}
{a: {b: 1}} == {a: {b: 1}}
```

**JavaScript:**
```javascript
JSON.stringify({a: {b: 1}}) === JSON.stringify({a: {b: 1}})
JSON.stringify({a: {b: 1}}) === JSON.stringify({a: {b: 1}})
// Both true for deep equality
```

**Rust:**
```rust
let obj1 = serde_json::json!({"a": {"b": 1}});
let obj2 = serde_json::json!({"a": {"b": 1}});
obj1 == obj2  // true
```

**Explanation:** Both operators perform deep comparison for nested structures.

## Ternary-Like Patterns

### Conditional Selection

**Left-Right:**
```left-right
true & `yes` | `no`
false & `selected` | `not selected`
```

**JavaScript:**
```javascript
true ? 'yes' : 'no'
false ? 'selected' : 'not selected'
```

**Rust:**
```rust
if true { "yes" } else { "no" }
if false { "selected" } else { "not selected" }
```

**Explanation:** `&`/`|` pattern acts as ternary. `condition & truthy | falsy`.

### Value Selection

**Left-Right:**
```left-right
score: 85
result: score >= 90 & `A` | score >= 80 & `B` | score >= 70 & `C` | `F`
```

**JavaScript:**
```javascript
const score = 85;
const result =
  score >= 90 ? 'A'
  : score >= 80 ? 'B'
  : score >= 70 ? 'C'
  : 'F';
```

**Rust:**
```rust
let score = 85;
let result = if score >= 90 {
  "A"
} else if score >= 80 {
  "B"
} else if score >= 70 {
  "C"
} else {
  "F"
};
```

**Explanation:** Chain ternary-like expressions. Left-to-right evaluation picks first matching condition.

### Nested Conditional

**Left-Right:**
```left-right
age: 25
status: age < 18 & `minor` | age >= 65 & `senior` | `adult`
```

**JavaScript:**
```javascript
const age = 25;
const status = age < 18 ? 'minor' : age >= 65 ? 'senior' : 'adult';
```

**Rust:**
```rust
let age = 25;
let status = if age < 18 {
  "minor"
} else if age >= 65 {
  "senior"
} else {
  "adult"
};
```

**Explanation:** Nested conditional logic with `&`/`|`. Equivalent to if-else chain.

## Guard Patterns in Pipelines

### Early Exit Guard

**Left-Right:**
```left-right
process: {
  input: _<,
  !input & undefined |
  input!? = `text` & input^ |
  `Error: invalid input`
}

[null, `hello`, 42]${ process }
```

**JavaScript:**
```javascript
const process = input => {
  if (!input) return undefined;
  if (typeof input !== 'text') return 'Error: invalid input';
  return input.toUpperCase();
};

[null, 'hello', 42].map(process)
```

**Rust:**
```rust
fn process(input: Option<serde_json::Value>) -> Option<String> {
  match input {
    None => None,
    Some(val) => {
      match val {
        serde_json::Value::String(s) => Some(s.to_uppercase()),
        _ => Some("Error: invalid input".to_string())
      }
    }
  }
}

[None, Some("hello".into()), Some(42.into())]
  .into_iter()
  .map(process)
  .collect::<Vec<_>>()
```

**Explanation:** Guard pattern checks input type before processing. Early return for invalid inputs.

### Property Guard

**Left-Right:**
```left-right
users: [
  {name: `Alice`, age: 30},
  {name: `Bob`},
  {name: `Charlie`, age: 35}
]
users$?{ _<@`age` | 0 }
```

**JavaScript:**
```javascript
const users = [
  {name: 'Alice', age: 30},
  {name: 'Bob'},
  {name: 'Charlie', age: 35}
];

users.filter(user => user.age || 0)
```

**Rust:**
```rust
let users = serde_json::json!([
  {"name": "Alice", "age": 30},
  {"name": "Bob"},
  {"name": "Charlie", "age": 35}
]).as_array().unwrap();

users
  .into_iter()
  .filter(|user| {
    user["age"].as_i64().unwrap_or(0)
  })
  .collect::<Vec<_>>()
```

**Explanation:** Guard against missing properties. `|` provides default value.

### Validation Guard

**Left-Right:**
```left-right
validate: { email: _<,
  !email & `Email required` |
  email@`@` & !email | `Invalid email` |
  `Valid`
}

[`test@example.com`, ``, `invalid`]${ validate }
```

**JavaScript:**
```javascript
const validate = email => {
  if (!email) return 'Email required';
  if (!email.includes('@')) return 'Invalid email';
  return 'Valid';
};

['test@example.com', '', 'invalid'].map(validate)
```

**Rust:**
```rust
fn validate(email: Option<String>) -> String {
  match email {
    None => "Email required".to_string(),
    Some(e) if !e.contains('@') => "Invalid email".to_string(),
    Some(_) => "Valid".to_string()
  }
}

[Some("test@example.com".to_string()), None, Some("invalid".to_string())]
  .into_iter()
  .map(validate)
  .collect::<Vec<_>>()
```

**Explanation:** Multi-stage validation guard. Each check provides specific error message.

### Range Guard

**Left-Right:**
```left-right
inRange: { num: _<,
  num < 0 & `negative` |
  num > 100 & `too large` |
  num >= 0 & num <= 100 & `valid`
}

[-5, 50, 150]${ inRange }
```

**JavaScript:**
```javascript
const inRange = num => {
  if (num < 0) return 'negative';
  if (num > 100) return 'too large';
  if (num >= 0 && num <= 100) return 'valid';
};

[-5, 50, 150].map(inRange)
```

**Rust:**
```rust
fn in_range(num: i32) -> &'static str {
  if num < 0 {
    "negative"
  } else if num > 100 {
    "too large"
  } else if num >= 0 && num <= 100 {
    "valid"
  } else {
    "unknown"
  }
}

[-5, 50, 150].into_iter().map(in_range).collect::<Vec<_>>()
```

**Explanation:** Range-based guard. Check multiple conditions in order.
