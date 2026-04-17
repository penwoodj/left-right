# Basic Operations

Fundamental operations in Left-Right: arithmetic, text, templates, and type checking.

## Arithmetic

### Number Operations

**Left-Right:**
```left-right
5 + 3 * 2
```

**JavaScript:**
```javascript
5 + 3 * 2
```

**Rust:**
```rust
5 + 3 * 2
```

**Explanation:** Standard arithmetic operations follow left-to-right evaluation. Multiplication and division have equal precedence with addition and subtraction.

### Complex Expression

**Left-Right:**
```left-right
10 - 3 + 5 * 2 / 4
```

**JavaScript:**
```javascript
10 - 3 + 5 * 2 / 4
```

**Rust:**
```rust
10 - 3 + 5 * 2 / 4
```

**Explanation:** Left-to-right evaluation without operator precedence. Group with parentheses when needed: `{10 - {3 + 5} * 2} / 4`.

### Modulo and Power

**Left-Right:**
```left-right
17 % 5
2 ** 3
```

**JavaScript:**
```javascript
17 % 5
2 ** 3
```

**Rust:**
```rust
17 % 5
2i32.pow(3)
```

**Explanation:** Modulo (`%`) and power (`**`) operators for numeric operations.

## Text Literals and Case Transforms

### Text Creation

**Left-Right:**
```left-right
`hello world`
`backticks only - quotes reserved for operators`
```

**JavaScript:**
```javascript
`hello world`
`backticks work too`
```

**Rust:**
```rust
"hello world"
"double quotes work too"
```

**Explanation:** Backticks ONLY for text. Single and double quotes reserved for operator names (e.g., `"` is toLower, `^` is toUpper). No other operator may contain backtick.

### Uppercase

**Left-Right:**
```left-right
`hello world`^
```

**JavaScript:**
```javascript
'hello world'.toUpperCase()
```

**Rust:**
```rust
"hello world".to_uppercase()
```

**Explanation:** The `^` operator converts text to uppercase. Works on entire text.

### Capitalize

**Left-Right:**
```left-right
`hello world`^_
```

**JavaScript:**
```javascript
'hello world'.charAt(0).toUpperCase() + 'hello world'.slice(1)
```

**Rust:**
```rust
let mut s = "hello world".to_string();
s.get_mut(0..1).map(|c| c.make_ascii_uppercase());
s
```

**Explanation:** The `^_` operator capitalizes first character of each word.

### Lowercase

**Left-Right:**
```left-right
`HELLO WORLD`"
```

**JavaScript:**
```javascript
'HELLO WORLD'.toLowerCase()
```

**Rust:**
```rust
"HELLO WORLD".to_lowercase()
```

**Explanation:** The `"` operator converts text to lowercase. Symbol placement is spatial: left operator position determines case direction.

### Chained Case Transforms

**Left-Right:**
```left-right
`HELLO`^_"^
```

**JavaScript:**
```javascript
'HELLO'.toLowerCase().capitalize().toUpperCase()
```

**Rust:**
```rust
"HELLO".to_lowercase().to_title_case().to_uppercase()
```

**Explanation:** Multiple case operators chain left-to-right. First lowercase, then capitalize, then uppercase.

## Truthy/Falsy Logic

### Truthy/Falsy Values

**Left-Right:**
```left-right
true & false | true
```

**JavaScript:**
```javascript
true && false || true
```

**Rust:**
```rust
true && false || true
```

**Explanation:** `&` is logical AND, `|` is logical OR. Left-to-right evaluation. Left-Right uses truthy/falsy values rather than a dedicated Boolean type.

### Logical Expressions

**Left-Right:**
```left-right
5 > 3 & 2 < 4 | 10 = 10
```

**JavaScript:**
```javascript
5 > 3 && 2 < 4 || 10 == 10
```

**Rust:**
```rust
5 > 3 && 2 < 4 || 10 == 10
```

**Explanation:** Comparison operators `>`, `<`, `=` (loose equality, like JS `==`) work with truthy/falsy values. Use `==` (strict type checking, like JS `===`) for ordered comparison with strict types.

### Strict vs Loose Equality

**Left-Right:**
```left-right
0 = `0`     // true (loose, type coercion)
0 == `0`    // false (strict, different types)
```

**JavaScript:**
```javascript
0 == '0'     // true (loose, type coercion)
0 === '0'    // false (strict, different types)
```

**Rust:**
```rust
// Rust doesn't have loose equality; types must match
```

**Explanation:**
- `=` is loose equality (type coercion, unordered)
- `==` is strict equality (no coercion, ordered)

### Not Operator

**Left-Right:**
```left-right
!true
!{5 > 3}
```

**JavaScript:**
```javascript
!true
!(5 > 3)
```

**Rust:**
```rust
!true
!(5 > 3)
```

**Explanation:** The `!` operator negates truthy/falsy values. Can negate entire expressions with grouping.

## Undefined Handling

### Undefined Coalescing

**Left-Right:**
```left-right
undefined | `default`
```

**JavaScript:**
```javascript
undefined ?? 'default'
```

**Rust:**
```rust
None.unwrap_or("default")
```

**Explanation:** The `|` operator provides fallback values when left operand is undefined. Similar to nullish coalescing.

### Type Coercion

**Left-Right:**
```left-right
undefined + 1
undefined + `text`
```

**JavaScript:**
```javascript
undefined ?? 1
undefined ?? 'text'
```

**Rust:**
```rust
None.unwrap_or(1)
None.unwrap_or("text")
```

**Explanation:** JSON `null` converts to `undefined` by default (configurable to `null` text string instead). The language prefers `undefined` over `null`.

### Identity Elements for + Operator

**Left-Right:**
```left-right
// Number identity
5 + undefined     // 5

// Text identity (with coercion)
1 + ``             // `1`

// List identity
[] + 42            // [42]

// Non-identity disappears
`hello` + undefined // `hello`
[1, 2] + undefined   // [1, 2]
```

**JavaScript:**
```javascript
// Similar behavior (no direct equivalent)
const numIdentity = (5 + (undefined || 0)); // 5
const strIdentity = String(1) + '';        // "1"
const arrIdentity = [...[], 42];             // [42]
```

**Rust:**
```rust
// Similar behavior (no direct equivalent)
let num_identity = 5 + Option::<i32>::None.unwrap_or(0);     // 5
let str_identity = format!("{}{}", 1, "");                      // "1"
let arr_identity: Vec<i32> = vec![].into_iter().chain([42].into_iter()).collect(); // [42]
```

**Explanation:** The `+` operator has type-specific identity elements:
- `undefined` is identity for numbers
- `` `` `` (empty text) is identity for text
- `[]` is identity for lists
- Non-identity from different set disappears

### Safe Path Access

**Left-Right:**
```left-right
{ name: `Alice` }@`age` | 30
```

**JavaScript:**
```javascript
{ name: 'Alice' }['age'] ?? 30
```

**Rust:**
```rust
serde_json::json!({"name": "Alice"})["age"].unwrap_or(30)
```

**Explanation:** Path access returns `undefined` for missing keys. Use `|` for default values.

## Simple Variable Binding in Maps

### Map Creation

**Left-Right:**
```left-right
{
  name: `Alice`,
  age: 30,
  active: true
}
```

**JavaScript:**
```javascript
{
  name: 'Alice',
  age: 30,
  active: true
}
```

**Rust:**
```rust
serde_json::json!({
  "name": "Alice",
  "age": 30,
  "active": true
})
```

**Explanation:** Maps use JSON-like syntax with key-value pairs. Keys are evaluated top-to-bottom.

### Computed Values

**Left-Right:**
```left-right
{
  base: 10,
  double: base * 2,
  triple: base * 3
}
```

**JavaScript:**
```javascript
{
  base: 10,
  double: base * 2,
  triple: base * 3
}
```

**Rust:**
```rust
let base = 10;
serde_json::json!({
  "base": base,
  "double": base * 2,
  "triple": base * 3
})
```

**Explanation:** Later keys can reference earlier keys. Values are computed left-to-right.

### Nested Maps

**Left-Right:**
```left-right
{
  user: {
    name: `Alice`,
    profile: {
      age: 30,
      city: `NYC`
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
      city: 'NYC'
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
      "city": "NYC"
    }
  }
})
```

**Explanation:** Maps nest arbitrarily. Use path access `@` to navigate nested structures.

## Template String Interpolation

### Basic Interpolation

**Left-Right:**
```left-right
`Hello {name}`
```

**JavaScript:**
```javascript
`Hello ${name}`
```

**Rust:**
```rust
format!("Hello {}", name)
```

**Explanation:** Backtick strings support interpolation. Use `{variable}` syntax without `&` suffix.

### Multiple Interpolations

**Left-Right:**
```left-right
`Hello {name}, you are {age} years old`
```

**JavaScript:**
```javascript
`Hello ${name}, you are ${age} years old`
```

**Rust:**
```rust
format!("Hello {}, you are {} years old", name, age)
```

**Explanation:** Multiple interpolations in single text. Order matches expression order.

### Expression Interpolation

**Left-Right:**
```left-right
`Result: {5 + 3 * 2}`
```

**JavaScript:**
```javascript
`Result: ${5 + 3 * 2}`
```

**Rust:**
```rust
format!("Result: {}", 5 + 3 * 2)
```

**Explanation:** Any expression can be interpolated, not just variables. Complex expressions work.

### Nested Path Interpolation

**Left-Right:**
```left-right
`User: {user@profile@name}`
```

**JavaScript:**
```javascript
`User: ${user.profile.name}`
```

**Rust:**
```rust
format!("User: {}", user["profile"]["name"])
```

**Explanation:** Path access works inside interpolation. Navigate nested structures with `@`.

### Escaping Curly Braces

**Left-Right:**
```left-right
`Literal \{brace\}`
```

**JavaScript:**
```javascript
`Literal {brace}`
```

**Rust:**
```rust
"Literal {brace}"
```

**Explanation:** Escape curly braces with `\{` and `\}` for literal braces in text.

## Type Checking

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

### Type Guard

**Left-Right:**
```left-right
value!? = `text` & value.length > 0 | value!? = `number` & value > 0 | false
```

**JavaScript:**
```javascript
typeof value === 'string' && value.length > 0 || typeof value === 'number' && value > 0 || false
```

**Rust:**
```rust
match value {
  s if matches!(s, &str) && s.len() > 0 => true,
  n if matches!(n, i32) && n > 0 => true,
  _ => false
}
```

**Explanation:** Type checks combine with boolean logic. Use `=` for unordered comparison.

### List Type Check

**Left-Right:**
```left-right
[1,2,3]!?
```

**JavaScript:**
```javascript
Array.isArray([1,2,3])
```

**Rust:**
```rust
matches!([1,2,3], Vec<_>)
```

**Explanation:** `!?` on lists returns `list`.

### Map Type Check

**Left-Right:**
```left-right
{key: `value`}!?
```

**JavaScript:**
```javascript
typeof {key: 'value'} === 'object' && {key: 'value'} !== null
```

**Rust:**
```rust
matches!({key: "value"}, serde_json::Map<String, serde_json::Value>)
```

**Explanation:** Maps check for object/hashmap type. Distinguishes from other object types.
