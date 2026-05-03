# Collections

Working with maps (maps), lists, paths, filtering, mapping, and other collection operations in Left-Right.

## Map Creation and Access

### Simple Map

**Left-Right:**
```left-right
{
  name: `Alice`,
  age: 30,
  city: `NYC`
}
```

**JavaScript:**
```javascript
{
  name: 'Alice',
  age: 30,
  city: 'NYC'
}
```

**Rust:**
```rust
serde_json::json!({
  "name": "Alice",
  "age": 30,
  "city": "NYC"
})
```

**Explanation:** Maps use JSON-like syntax. Keys can be text, values any type.

### Nested Map

**Left-Right:**
```left-right
{
  user: {
    name: `Alice`,
    profile: {
      age: 30,
      preferences: {
        theme: `dark`
      }
    }
  }
}
```

**JavaScript:**
```javascript
{
  user: {
    name: 'Alice',
    profile: {
      age: 30,
      preferences: {
        theme: 'dark'
      }
    }
  }
}
```

**Rust:**
```rust
serde_json::json!({
  "user": {
    "name": "Alice",
    "profile": {
      "age": 30,
      "preferences": {
        "theme": "dark"
      }
    }
  }
})
```

**Explanation:** Maps nest arbitrarily. Deep nesting common for configuration and data structures.

## List Creation and Indexing

### Simple List

**Left-Right:**
```left-right
[1, 2, 3, 4, 5]
```

**JavaScript:**
```javascript
[1, 2, 3, 4, 5]
```

**Rust:**
```rust
vec![1, 2, 3, 4, 5]
```

**Explanation:** Lists hold ordered sequences of values. Can be heterogeneous.

### Mixed Type List

**Left-Right:**
```left-right
[1, `hello`, true, {key: `value`}, [1,2]]
```

**JavaScript:**
```javascript
[1, 'hello', true, {key: 'value'}, [1,2]]
```

**Rust:**
```rust
serde_json::json!([1, "hello", true, {"key": "value"}, [1,2]])
```

**Explanation:** Lists can contain any type. Loosely typed language allows mixing.

### Index Access

**Left-Right:**
```left-right
[10, 20, 30, 40]@2
```

**JavaScript:**
```javascript
[10, 20, 30, 40][2]
```

**Rust:**
```rust
[10, 20, 30, 40][2]
```

**Explanation:** `@` operator accesses list elements by numeric index. Zero-indexed.

### Negative Index

**Left-Right:**
```left-right
[10, 20, 30, 40]@-1
```

**JavaScript:**
```javascript
[10, 20, 30, 40].at(-1)
```

**Rust:**
```rust
[10, 20, 30, 40].last().unwrap()
```

**Explanation:** Negative indices count from end. `-1` is last element.

## Path Access with @ Operator

### Dot Notation Access

**Left-Right:**
```left-right
{ user: { name: `Alice`, age: 30 } }@`user`
```

**JavaScript:**
```javascript
{ user: { name: 'Alice', age: 30 } }['user']
```

**Rust:**
```rust
serde_json::json!({"user": {"name": "Alice", "age": 30}})["user"]
```

**Explanation:** `@` operator retrieves map values by text key. Equivalent to object property access.

### Deep Path Access

**Left-Right:**
```left-right
{
  data: {
    user: {
      profile: {
        age: 30
      }
    }
  }
}@[`data`, `user`, `profile`, `age`]
```

**JavaScript:**
```javascript
{
  data: {
    user: {
      profile: {
        age: 30
      }
    }
  }
}['data']['user']['profile']['age']
```

**Rust:**
```rust
serde_json::json!({
  "data": {
    "user": {
      "profile": {
        "age": 30
      }
    }
  }
})["data"]["user"]["profile"]["age"]
```

**Explanation:** List syntax with `@` navigates deep paths. Each segment is a key.

### List Index in Path

**Left-Right:**
```left-right
{ items: [10, 20, 30] }@[`items`, 1]
```

**JavaScript:**
```javascript
{ items: [10, 20, 30] }['items'][1]
```

**Rust:**
```rust
serde_json::json!({"items": [10, 20, 30]})["items"][1]
```

**Explanation:** Paths can mix map keys and list indices. Navigate mixed structures.

### Optional Path Access

**Left-Right:**
```left-right
{
  user: {
    name: `Alice`
  }
}@[`profile`, `age`] | 30
```

**JavaScript:**
```javascript
{
  user: {
    name: 'Alice'
  }
}['profile']?.['age'] ?? 30
```

**Rust:**
```rust
serde_json::json!({
  "user": {
    "name": "Alice"
  }
})["profile"].and_then(|p| p["age"]).unwrap_or(30)
```

**Explanation:** Path access returns `undefined` for missing keys. Use `|` for default values.

## Filtering ($?)

### Simple Filter

**Left-Right:**
```left-right
[1, 2, 3, 4, 5]?{ _< > 3 }
```

**JavaScript:**
```javascript
[1, 2, 3, 4, 5].filter(x => x > 3)
```

**Rust:**
```rust
[1, 2, 3, 4, 5].into_iter().filter(|x| x > 3).collect::<Vec<_>>()
```

**Explanation:** `?{ predicate }` filters collection. `_<` references current element. Returns elements where predicate is true.

### Complex Filter

**Left-Right:**
```left-right
[1, 2, 3, 4, 5, 6]?{ _< % 2 == 0 & _< > 2 }
```

**JavaScript:**
```javascript
[1, 2, 3, 4, 5, 6].filter(x => x % 2 === 0 && x > 2)
```

**Rust:**
```rust
[1, 2, 3, 4, 5, 6].into_iter()
  .filter(|x| x % 2 == 0 && *x > 2)
  .collect::<Vec<_>>()
```

**Explanation:** Predicates can combine with boolean logic. Use `&` for AND, `|` for OR.

### Object Filter

**Left-Right:**
```left-right
[
  {name: `Alice`, age: 30},
  {name: `Bob`, age: 25},
  {name: `Charlie`, age: 35}
]?{ _<@`age` > 28 }
```

**JavaScript:**
```javascript
[
  {name: 'Alice', age: 30},
  {name: 'Bob', age: 25},
  {name: 'Charlie', age: 35}
].filter(obj => obj.age > 28)
```

**Rust:**
```rust
serde_json::json!([
  {"name": "Alice", "age": 30},
  {"name": "Bob", "age": 25},
  {"name": "Charlie", "age": 35}
]).as_array().unwrap()
  .into_iter()
  .filter(|obj| obj["age"].as_i64().unwrap() > 28)
  .collect::<Vec<_>>()
```

**Explanation:** Filter objects by property values. Access properties with `@` inside predicate.

## Mapping ($)

### Simple Map

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

**Explanation:** `$` operator maps over collection. `_<` references current element. Returns transformed collection.

### String Mapping

**Left-Right:**
```left-right
[`alice`, `bob`, `charlie`]${ _<^_ }
```

**JavaScript:**
```javascript
['alice', 'bob', 'charlie'].map(s => s.charAt(0).toUpperCase() + s.slice(1))
```

**Rust:**
```rust
["alice", "bob", "charlie"].into_iter()
  .map(|s| {
    let mut chars: Vec<char> = s.chars().collect();
    chars[0].make_ascii_uppercase();
    chars.into_iter().collect()
  })
  .collect::<Vec<_>>()
```

**Explanation:** Map applies any operation. String transforms work element-wise.

### Object Mapping

**Left-Right:**
```left-right
[
  {name: `alice`, age: 30},
  {name: `bob`, age: 25}
]${ _<@`name`^ }
```

**JavaScript:**
```javascript
[
  {name: 'alice', age: 30},
  {name: 'bob', age: 25}
].map(obj => obj.name.toUpperCase())
```

**Rust:**
```rust
serde_json::json!([
  {"name": "alice", "age": 30},
  {"name": "bob", "age": 25}
]).as_array().unwrap()
  .into_iter()
  .map(|obj| obj["name"].as_str().unwrap().to_uppercase())
  .collect::<Vec<_>>()
```

**Explanation:** Transform object properties. Access and modify in single operation.

## Some/Any (?|)

### List Some

**Left-Right:**
```left-right
[1, 2, 3, 4, 5]?|{ _< > 3 }
```

**JavaScript:**
```javascript
[1, 2, 3, 4, 5].some(x => x > 3)
```

**Rust:**
```rust
[1, 2, 3, 4, 5].iter().any(|&x| x > 3)
```

**Explanation:** The `?|` operator tests if any element satisfies predicate. Returns truthy/falsy.

### Object List Some

**Left-Right:**
```left-right
[
  {name: `Alice`, active: false},
  {name: `Bob`, active: true}
]?|{ _<@`active` }
```

**JavaScript:**
```javascript
[
  {name: 'Alice', active: false},
  {name: 'Bob', active: true}
].some(obj => obj.active)
```

**Rust:**
```rust
serde_json::json!([
  {"name": "Alice", "active": false},
  {"name": "Bob", "active": true}
]).as_array().unwrap()
  .iter()
  .any(|obj| obj["active"].as_bool().unwrap())
```

**Explanation:** Test object lists for property conditions. Useful for validation.

### Empty List Some

**Left-Right:**
```left-right
[]?|{ _< > 0 }
```

**JavaScript:**
```javascript
[].some(x => x > 0)
```

**Rust:**
```rust
vec::<i32>[].iter().any(|&x| x > 0)
```

**Explanation:** Empty lists always return `false` for `?|` operator.

## All/Every (?|!)

### List Every

**Left-Right:**
```left-right
[1, 2, 3, 4, 5]?|!{ _< > 0 }
```

**JavaScript:**
```javascript
[1, 2, 3, 4, 5].every(x => x > 0)
```

**Rust:**
```rust
[1, 2, 3, 4, 5].iter().all(|&x| x > 0)
```

**Explanation:** The `?|!` operator tests if every element satisfies predicate. Returns truthy/falsy.

### Negative Condition Every

**Left-Right:**
```left-right
[1, 2, 3, 4, 5]?|!{ _< <= 0 }!
```

**JavaScript:**
```javascript
[1, 2, 3, 4, 5].every(x => !(x <= 0))
```

**Rust:**
```rust
[1, 2, 3, 4, 5].iter().all(|&x| !(*x <= 0))
```

**Explanation:** `?|!` does NOT reverse condition. `!` after predicate creates negative check.

### Object List Every

**Left-Right:**
```left-right
[
  {name: `Alice`, active: true},
  {name: `Bob`, active: true}
]?|!{ _<@`active` }
```

**JavaScript:**
```javascript
[
  {name: 'Alice', active: true},
  {name: 'Bob', active: true}
].every(obj => obj.active)
```

**Rust:**
```rust
serde_json::json!([
  {"name": "Alice", "active": true},
  {"name": "Bob", "active": true}
]).as_array().unwrap()
  .iter()
  .all(|obj| obj["active"].as_bool().unwrap())
```

**Explanation:** Test object lists for property conditions. Useful for validation.

### Empty List Every

**Left-Right:**
```left-right
[]?|!{ _< > 0 }
```

**JavaScript:**
```javascript
[].every(x => x > 0)
```

**Rust:**
```rust
vec::<i32>[].iter().all(|&x| x > 0)
```

**Explanation:** Empty lists always return `true` for `?|!` operator (vacuous truth).

### Key Differences

- `?|` — Returns `true` if ANY element satisfies predicate
- `?|!` — Returns `true` if ALL elements satisfy predicate
- `?|!` does NOT reverse condition (use `!` in predicate if needed)
- `?|!` on empty list returns `true`

## Counting (#)

### List Length

**Left-Right:**
```left-right
[1, 2, 3, 4, 5]#
```

**JavaScript:**
```javascript
[1, 2, 3, 4, 5].length
```

**Rust:**
```rust
[1, 2, 3, 4, 5].len()
```

**Explanation:** The `#` operator returns collection length/size.

### Text Length

**Left-Right:**
```left-right
`hello world`#
```

**JavaScript:**
```javascript
'hello world'.length
```

**Rust:**
```rust
"hello world".len()
```

**Explanation:** Count works on text too. Returns character count.

### Map Key Count

**Left-Right:**
```left-right
{a: 1, b: 2, c: 3}#
```

**JavaScript:**
```javascript
Object.keys({a: 1, b: 2, c: 3}).length
```

**Rust:**
```rust
serde_json::json!({"a": 1, "b": 2, "c": 3}).as_object().unwrap().len()
```

**Explanation:** Count returns number of keys in map. Useful for validation.

## Unique (~)

### List Deduplication

**Left-Right:**
```left-right
[1, 2, 2, 3, 3, 3, 4]~
```

**JavaScript:**
```javascript
[...new Set([1, 2, 2, 3, 3, 3, 4])]
```

**Rust:**
```rust
use std::collections::HashSet;
let set: HashSet<_> = [1, 2, 2, 3, 3, 3, 4].into_iter().collect();
set.into_iter().collect::<Vec<_>>()
```

**Explanation:** The `~` operator removes duplicate values. Preserves order of first occurrence.

### Text Deduplication

**Left-Right:**
```left-right
`aabbccdd`~
```

**JavaScript:**
```javascript
[...new Set('aabbccdd'.split(''))].join('')
```

**Rust:**
```rust
use std::collections::HashSet;
let chars: HashSet<char> = "aabbccdd".chars().collect();
chars.into_iter().collect::<String>()
```

**Explanation:** Unique works on text by treating as character sequence.

### Object List Deduplication

**Left-Right:**
```left-right
[
  {id: 1, name: `Alice`},
  {id: 2, name: `Bob`},
  {id: 1, name: `Alice`}
]~
```

**JavaScript:**
```javascript
const unique = (arr) => {
  const seen = new Set();
  return arr.filter(obj => {
    const key = JSON.stringify(obj);
    if (seen.has(key)) return false;
    seen.add(key);
    return true;
  });
};
unique([
  {id: 1, name: 'Alice'},
  {id: 2, name: 'Bob'},
  {id: 1, name: 'Alice'}
])
```

**Rust:**
```rust
use std::collections::HashSet;
let objs = serde_json::json!([
  {"id": 1, "name": "Alice"},
  {"id": 2, "name": "Bob"},
  {"id": 1, "name": "Alice"}
]).as_array().unwrap();
let mut seen = HashSet::new();
objs.into_iter()
  .filter(|obj| {
    let key = obj.to_string();
    if seen.contains(&key) {
      return false;
    }
    seen.insert(key);
    true
  })
  .collect::<Vec<_>>()
```

**Explanation:** Object deduplication uses deep equality. Objects with same values considered duplicates.

## Joining (><)

### List Join with Separator

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

**Explanation:** The `><` operator joins list elements with separator. Separator is right operand.

### Number Join

**Left-Right:**
```left-right
[1, 2, 3, 4]><`-`
```

**JavaScript:**
```javascript
[1, 2, 3, 4].join('-')
```

**Rust:**
```rust
[1, 2, 3, 4].iter()
  .map(|n| n.to_string())
  .collect::<Vec<_>>()
  .join("-")
```

**Explanation:** Numbers convert to text before joining. Type coercion happens automatically.

### Empty List Join

**Left-Right:**
```left-right
[]><`, `
```

**JavaScript:**
```javascript
[].join(', ')
```

**Rust:**
```rust
vec::<i32>[].iter()
  .map(|n| n.to_string())
  .collect::<Vec<_>>()
  .join(", ")
```

**Explanation:** Empty lists join to empty text regardless of separator.

## Concatenation (+)

### List Concatenation

**Left-Right:**
```left-right
[1, 2, 3] + [4, 5, 6]
```

**JavaScript:**
```javascript
[1, 2, 3].concat([4, 5, 6])
```

**Rust:**
```rust
{
  let mut result = [1, 2, 3].to_vec();
  result.extend([4, 5, 6]);
  result
}
```

**Explanation:** The `+` operator concatenates lists. Creates new list with all elements.

### Text Concatenation

**Left-Right:**
```left-right
`Hello, ` + `world!`
```

**JavaScript:**
```javascript
'Hello, ' + 'world!'
```

**Rust:**
```rust
"Hello, ".to_string() + "world!"
```

**Explanation:** Text concatenates with `+`. For template strings use `{}` interpolation instead.

### Map Concatenation

**Left-Right:**
```left-right
{a: 1, b: 2} + {c: 3, d: 4}
```

**JavaScript:**
```javascript
Object.assign({a: 1, b: 2}, {c: 3, d: 4})
```

**Rust:**
```rust
let mut map1 = serde_json::json!({"a": 1, "b": 2}).as_object().unwrap().clone();
let map2 = serde_json::json!({"c": 3, "d": 4}).as_object().unwrap();
for (key, value) in map2 {
  map1.insert(key.clone(), value.clone());
}
serde_json::Value::Object(map1)
```

**Explanation:** Map concatenation merges keys. Right map overwrites left map for duplicate keys.
