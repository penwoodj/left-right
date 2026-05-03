# Functions

Anonymous operators, lambdas, currying, point-free function chains, and function composition in Left-Right.

## Anonymous Operators (the _< _> Pattern)

### Left Section (Left Hungry)

**Left-Right:**
```left-right
increment: { _< + 1 }

[1, 2, 3]${ increment }
```

**JavaScript:**
```javascript
const increment = x => x + 1;

[1, 2, 3].map(increment)
```

**Rust:**
```rust
let increment = |x: i32| x + 1;

[1, 2, 3].into_iter().map(increment).collect::<Vec<_>>()
```

**Explanation:** `{ _< + 1 }` creates left section. `_>` is left argument placeholder. Function adds 1 to whatever is on left.

### Right Section (Right Hungry)

**Left-Right:**
```left-right
decrement: { 1 - _> }

[5, 10, 15]${ decrement }
```

**JavaScript:**
```javascript
const decrement = x => 1 - x;

[5, 10, 15].map(decrement)
```

**Rust:**
```rust
let decrement = |x: i32| 1 - x;

[5, 10, 15].into_iter().map(decrement).collect::<Vec<_>>()
```

**Explanation:** `{ 1 - _> }` creates right section. `_<` is right argument placeholder. Function subtracts argument from 1.

### No Directional (Auto-Curried)

**Left-Right:**
```left-right
double: { _< * 2 }

[1, 2, 3]${ double }
```

**JavaScript:**
```javascript
const double = x => x * 2;

[1, 2, 3].map(double)
```

**Rust:**
```rust
let double = |x: i32| x * 2;

[1, 2, 3].into_iter().map(double).collect::<Vec<_>>()
```

**Explanation:** `{ _< * 2 }` with both directional placeholders creates auto-curried function. Default left-hungry.

### String Transform Function

**Left-Right:**
```left-right
format: { `Value: ` & _< & `, count: ` & _< & `!` }

partial: format `test`
```

**JavaScript:**
```javascript
const format = (value, count) => `Value: ${value}, count: ${count}!`;

const partial = count => format('test', count);
```

**Rust:**
```rust
let format = |value: &str, count: i32| -> String {
  format!("Value: {}, count: {}!", value, count)
};

let partial = |count: i32| format("test", count);
```

**JavaScript:**
```javascript
const greet = name => `Hello, ${name}!`;

['Alice', 'Bob', 'Charlie'].map(greet)
```

**Rust:**
```rust
let greet = |name: &str| format!("Hello, {}!", name);

["alice", "bob", "charlie"]
  .into_iter()
  .map(greet)
  .collect::<Vec<_>>()
```

**Explanation:** Functions can use text interpolation. Template text with `{}` embed arguments.

## getEntityTypes Example (Exact from PenroScript.md)

### Original TypeScript Code

**TypeScript:**
```typescript
// TS
const getEntityTypes = (
  typesToGet: EntityType | EntityType[],
  entities: Entity[]
): Entity[] => {
  const lowerTypesToGet: string[] =
    typeof typesToGet === 'string' ? [toLower(typesToGet)] : map(toLower, typesToGet);

  const entitiesOfTypesToGet: Entity[] = filter((entity: Entity): boolean => {
    const lowerEntityTypes: string[] = map(toLower, entity.types);

    const entityTypesAreInTypesToGet: boolean = some(
      (typeToGet: string): boolean => lowerEntityTypes.includes(typeToGet),
      lowerTypesToGet
    );

    return entityTypesAreInTypesToGet;
  }, entities);

  return entitiesOfTypesToGet;
};

export default getEntityTypes;
```

### Left-Right Implementation

**Left-Right:**
```left-right
{ typesToGet: _<@0, entities: _<@1,
  lowerTypesToGet: {
    typesToGet !?= `text`: [typesToGet],
    typesToGet ${'_}
  },

  entityTypesToGet: entities ?{
    lowerEntityTypes: entities@`types` ${'_},
    entityTypesAreInTypesToGet: lowerTypesToGet ?|{
      typeToGet: _<@0,
      lowerEntityTypes ?>< typeToGet
    },
    entityTypesAreInTypesToGet
  },

  entityTypesToGet
}
```

**JavaScript:**
```javascript
const getEntityTypes = (typesToGet, entities) => {
  const lowerTypesToGet =
    typeof typesToGet === 'string'
      ? [typesToGet.toLowerCase()]
      : typesToGet.map(t => t.toLowerCase());

  const entityTypesToGet = entities.filter(entity => {
    const lowerEntityTypes = entity.types.map(t => t.toLowerCase());

    const entityTypesAreInTypesToGet = lowerTypesToGet.some(
      typeToGet => lowerEntityTypes.includes(typeToGet)
    );

    return entityTypesAreInTypesToGet;
  });

  return entityTypesToGet;
};

export default getEntityTypes;
```

**Rust:**
```rust
fn get_entity_types(types_to_get: serde_json::Value, entities: serde_json::Value) -> serde_json::Value {
  let lower_types_to_get = match types_to_get {
    serde_json::Value::String(s) => {
      vec![s.to_lowercase()]
    }
    serde_json::Value::Array(arr) => {
      arr.iter()
        .map(|t| t.as_str().unwrap().to_lowercase())
        .collect::<Vec<_>>()
    }
    _ => vec![]
  };

  let entity_types_to_get: Vec<serde_json::Value> = entities
    .as_array()
    .unwrap()
    .into_iter()
    .filter(|entity| {
      let lower_entity_types: Vec<String> = entity["types"]
        .as_array()
        .unwrap()
        .into_iter()
        .map(|t| t.as_str().unwrap().to_lowercase())
        .collect();

      lower_types_to_get
        .iter()
        .any(|type_to_get| lower_entity_types.contains(type_to_get))
    })
    .collect();

  serde_json::Value::Array(entity_types_to_get)
}
```

**Explanation:** Complex filtering function from ServiceNow integration. Demonstrates:
- Type checking: `!?= `text`` (if not type, then...)
- Conditional expression: `condition: trueValue, elseValue`
- List indexing: `_<@0`, `_<@1`
- String transformation: `" '_` (lowercase)
- Path access: `entities@`types``
- List method chaining: `><` (includes)
- Nested conditional blocks: `? { ... }`

Key patterns:
- `typesToGet !?= `text`` — runtime type check with ternary-like syntax
- `typesToGet ${'_}` — lowercase operator (map case)
- `lowerTypesToGet ?| { typeToGet: _<@0, lowerEntityTypes >< typeToGet }` — some() with lambda

## Point-Free Function Chains

### Simple Chain

**Left-Right:**
```left-right
process: { _<^_ } >> { _<~ }

[`hello`, `world`, `test`]${ process }
```

**JavaScript:**
```javascript
const process = x => {
  const capitalized = x.charAt(0).toUpperCase() + x.slice(1);
  return [...new Set([capitalized])];
};

['hello', 'world', 'test'].map(process)
```

**Rust:**
```rust
use std::collections::HashSet;

let capitalize = |s: &str| -> String {
  let mut chars: Vec<char> = s.chars().collect();
  chars[0].make_ascii_uppercase();
  chars.into_iter().collect()
};

let process = |s: &str| {
  let capitalized = capitalize(s);
  let mut set = HashSet::new();
  set.insert(capitalized);
  set.into_iter().collect::<Vec<_>>()
};

["hello", "world", "test"]
  .into_iter()
  .map(process)
  .collect::<Vec<_>>()
```

**Explanation:** Chain point-free functions with `>>`. Each function receives previous output.

### Multi-Stage Chain

**Left-Right:**
```left-right
pipeline: { _<^ } >> { _<>>, `` } >> { _<~ }

[`alice`, `bob`, `charlie`]${ pipeline }
```

**JavaScript:**
```javascript
const pipeline = s => {
  const upper = s.toUpperCase();
  const split = upper.split(', ');
  return [...new Set(split)];
};

['alice', 'bob', 'charlie'].map(pipeline)
```

**Rust:**
```rust
use std::collections::HashSet;

let pipeline = |s: &str| -> Vec<String> {
  let upper = s.to_uppercase();
  let split: Vec<&str> = upper.split(", ").collect();
  let set: HashSet<&str> = split.into_iter().collect();
  set.into_iter().map(|&s| s.to_string()).collect()
};

["alice", "bob", "charlie"]
  .into_iter()
  .map(pipeline)
  .collect::<Vec<_>>()
```

**Explanation:** Three-stage chain: uppercase, split on comma, deduplicate. All point-free.

### Data Processing Chain

**Left-Right:**
```left-right
transform: { _< * 2 } >> { _< + 10 } >> { _< % 3 }

[1, 2, 3, 4, 5]${ transform }
```

**JavaScript:**
```javascript
const transform = x => {
  const doubled = x * 2;
  const added = doubled + 10;
  return added % 3;
};

[1, 2, 3, 4, 5].map(transform)
```

**Rust:**
```rust
let transform = |x: i32| -> i32 {
  let doubled = x * 2;
  let added = doubled + 10;
  added % 3
};

[1, 2, 3, 4, 5]
  .into_iter()
  .map(transform)
  .collect::<Vec<_>>()
```

**Explanation:** Arithmetic chain: double, add 10, mod 3. Each stage pure transformation.

## Auto-Currying Behavior

### Partial Application

**Left-Right:**
```left-right
add: { _< + _> }
addFive: add + 5

[1, 2, 3]${ addFive }
```

**JavaScript:**
```javascript
const add = (a, b) => a + b;
const addFive = a => add(a, 5);

[1, 2, 3].map(addFive)
```

**Rust:**
```rust
let add = |a: i32, b: i32| a + b;
let add_five = |a: i32| add(a, 5);

[1, 2, 3].into_iter().map(add_five).collect::<Vec<_>>()
```

**Explanation:** `add + 5` partially applies right argument. Returns function waiting for left argument.

### Left-Curried Default

**Left-Right:**
```left-right
multiply: { _< * _> }
multiplyByThree: multiply * 3

[2, 4, 6]${ multiplyByThree }
```

**JavaScript:**
```javascript
const multiply = (a, b) => a * b;
const multiplyByThree = a => multiply(a, 3);

[2, 4, 6].map(multiplyByThree)
```

**Rust:**
```rust
let multiply = |a: i32, b: i32| a * b;
let multiply_by_three = |a: i32| multiply(a, 3);

[2, 4, 6].into_iter().map(multiply_by_three).collect::<Vec<_>>()
```

**Explanation:** Default left-hungry currying. `multiply * 3` applies right argument to create new function.

### Multi-Argument Partial

**Left-Right:**
```left-right
format: { `Value: ` & _< & `, count: ` & _> & `!` }
partial: format `test`

[1, 2, 3]${ partial }
```

**JavaScript:**
```javascript
const format = (value, count) => `Value: ${value}, count: ${count}!`;
const partial = count => format('test', count);

[1, 2, 3].map(partial)
```

**Rust:**
```rust
let format = |value: &str, count: i32| format!("Value: {}, count: {}!", value, count);
let partial = |count: i32| format("test", count);

[1, 2, 3].into_iter().map(partial).collect::<Vec<_>>()
```

**Explanation:** Partial application from either side. `format 'test' binds left argument.

## Directional Forms (_< vs _>)

### Left Section Only

**Left-Right:**
```left-right
subtractFive: { 5 - _> }

[10, 15, 20]${ subtractFive }
```

**JavaScript:**
```javascript
const subtractFive = x => 5 - x;

[10, 15, 20].map(subtractFive)
```

**Rust:**
```rust
let subtract_five = |x: i32| 5 - x;

[10, 15, 20].into_iter().map(subtract_five).collect::<Vec<_>>()
```

**Explanation:** `{ 5 - _> }` only has right placeholder. Subtracts argument from 5.

### Right Section Only

**Left-Right:**
```left-right
addToTen: { _< + 10 }

[1, 2, 3]${ addToTen }
```

**JavaScript:**
```javascript
const addToTen = x => x + 10;

[1, 2, 3].map(addToTen)
```

**Rust:**
```rust
let add_to_ten = |x: i32| x + 10;

[1, 2, 3].into_iter().map(add_to_ten).collect::<Vec<_>>()
```

**Explanation:** `{ _< + 10 }` only has left placeholder. Adds argument to 10.

### Both Sections

**Left-Right:**
```left-right
power: { _< ** _> }

[2, 3, 4]${ { 2 >> power } }
```

**JavaScript:**
```javascript
const power = (base, exponent) => base ** exponent;

[2, 3, 4].map(x => power(x, 2))
```

**Rust:**
```rust
let power = |base: i32, exponent: i32| base.pow(exponent as u32);

[2, 3, 4].into_iter()
  .map(|x| power(x, 2))
  .collect::<Vec<_>>()
```

**Explanation:** `{ _< ** _> }` has both placeholders. `{ 2 >> power }` applies 2 as base.

## Function Composition (>>, <<)

### Forward Composition (>>)

**Left-Right:**
```left-right
uppercase: { _<^ }
exclaim: { _< & `!` }
loud: uppercase >> exclaim

[`hello`, `world`]${ loud }
```

**JavaScript:**
```javascript
const uppercase = s => s.toUpperCase();
const exclaim = s => s + '!';
const loud = s => exclaim(uppercase(s));

['hello', 'world'].map(loud)
```

**Rust:**
```rust
let uppercase = |s: &str| s.to_uppercase();
let exclaim = |s: &str| format!("{}!", s);
let loud = |s: &str| exclaim(uppercase(s));

["hello", "world"]
  .into_iter()
  .map(loud)
  .collect::<Vec<_>>()
```

**Explanation:** `>>` composes left-to-right. Apply `uppercase`, then `exclaim`.

### Backward Composition (<<)

**Left-Right:**
```left-right
uppercase: { _<^ }
exclaim: { _< & `!` }
loud: exclaim << uppercase

[`hello`, `world`]${ loud }
```

**JavaScript:**
```javascript
const uppercase = s => s.toUpperCase();
const exclaim = s => s + '!';
const loud = s => uppercase(exclaim(s));

['hello', 'world'].map(loud)
```

**Rust:**
```rust
let uppercase = |s: &str| s.to_uppercase();
let exclaim = |s: &str| format!("{}!", s);
let loud = |s: &str| uppercase(exclaim(s));

["hello", "world"]
  .into_iter()
  .map(loud)
  .collect::<Vec<_>>()
```

**Explanation:** `<<` composes right-to-left. Apply `exclaim`, then `uppercase`.

### Three-Function Composition

**Left-Right:**
```left-right
trim: { _<@0... }
capitalize: { _<^_ }
exclaim: { _< & `!` }
process: trim >> capitalize >> exclaim

`  hello  ` ${ process }
```

**JavaScript:**
```javascript
const trim = s => s.trim();
const capitalize = s => s.charAt(0).toUpperCase() + s.slice(1);
const exclaim = s => s + '!';
const process = s => exclaim(capitalize(trim(s)));

process('  hello  ')
```

**Rust:**
```rust
let trim = |s: &str| s.trim();
let capitalize = |s: &str| {
  let mut chars: Vec<char> = s.chars().collect();
  chars[0].make_ascii_uppercase();
  chars.into_iter().collect()
};
let exclaim = |s: &str| format!("{}!", s);
let process = |s: &str| exclaim(capitalize(trim(s)));

process("  hello  ")
```

**Explanation:** Chain three functions with `>>`. Each function receives previous output.

## Higher-Order Functions

### Map Function

**Left-Right:**
```left-right
applyTwice: { _< >> _< >> _< }

increment: { _< + 1 }

5 ${ applyTwice } ${ increment }
```

**JavaScript:**
```javascript
const applyTwice = f => x => f(f(x));
const increment = x => x + 1;

applyTwice(increment)(5)
// increment(increment(5)) = 7
```

**Rust:**
```rust
let apply_twice = |f: fn(i32) -> i32| -> fn(i32) -> i32 {
  move |x: i32| f(f(x))
};

let increment = |x: i32| x + 1;

apply_twice(increment)(5)
// 7
```

**Explanation:** Function that takes function, returns new function. `applyTwice` applies function twice.

### Filter Function Generator

**Left-Right:**
```left-right
greaterThan: { threshold: _>,
  { _< > threshold }
}

aboveFive: greaterThan 5
aboveTen: greaterThan 10

[1, 3, 5, 7, 9, 11]
  ?{ aboveFive }
  ?{ aboveTen }
```

**JavaScript:**
```javascript
const greaterThan = threshold => x => x > threshold;
const aboveFive = greaterThan(5);
const aboveTen = greaterThan(10);

[1, 3, 5, 7, 9, 11]
  .filter(aboveFive)
  .filter(aboveTen)
// [11]
```

**Rust:**
```rust
let greater_than = |threshold: i32| -> fn(i32) -> bool {
  move |x: i32| x > threshold
};

let above_five = greater_than(5);
let above_ten = greater_than(10);

[1, 3, 5, 7, 9, 11]
  .into_iter()
  .filter(|&x| above_five(x))
  .filter(|&x| above_ten(x))
  .collect::<Vec<_>>()
// [11]
```

**Explanation:** Higher-order function returns predicate function. Customize filters dynamically.

### Combiner Function

**Left-Right:**
```left-right
combine: { f: _<, g: _>,
  { _< >> f >> g }
}

double: { _< * 2 }
increment: { _< + 1 }
doubleAndIncrement: combine double increment

[1, 2, 3]${ doubleAndIncrement }
```

**JavaScript:**
```javascript
const combine = (f, g) => x => g(f(x));
const double = x => x * 2;
const increment = x => x + 1;
const doubleAndIncrement = combine(double, increment);

[1, 2, 3].map(doubleAndIncrement)
```

**Rust:**
```rust
let combine = |f: fn(i32) -> i32, g: fn(i32) -> i32| -> fn(i32) -> i32 {
  move |x: i32| g(f(x))
};

let double = |x: i32| x * 2;
let increment = |x: i32| x + 1;
let double_and_increment = combine(double, increment);

[1, 2, 3].into_iter().map(double_and_increment).collect::<Vec<_>>()
```

**Explanation:** Function combinator creates composed function. Reusable composition pattern.
