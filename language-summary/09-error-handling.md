# Error Handling — Graceful Degradation

Left-Right uses a loosely typed philosophy where errors default to `undefined` rather than throwing exceptions. This document covers the error handling model, type checking operators, and comparison with other error handling approaches.

## Error Philosophy

### Undefined as Default

Errors in Left-Right tend to return `undefined` rather than throwing:

```javascript
// Missing key returns undefined
obj@`nonexistentKey` // Returns: undefined

// Invalid index returns undefined
items@999 // Returns: undefined

// Failed operation returns undefined
result@`errorField` // If 'errorField' missing
```

### No Exception Mechanism

Left-Right does not have explicit try/catch or throw:

```javascript
// No throw statement
// throw new Error('Something went wrong'); // Not valid

// No try/catch blocks
// try { ... } catch (e) { ... } // Not valid
```

### Graceful Degradation

Operations degrade gracefully on error:

```javascript
// Path access with missing intermediate
obj@[`nested`, `missing`, `key`] // Returns: undefined

// Entire expression evaluates
result: obj@[`nested`, `missing`, `key`] + 10
// Result: undefined + 10 = undefined
```

## Type Checking with ? Operator

### ?= — Type Check

The `?=` operator performs type checking and conditional branching:

```javascript
// Type checking
value ?= `number` // True if value is number

// Conditional branching
value ?= `text` ? 'Is text' : 'Not text'
```

### Type Check Predicates

Check for specific types:

```javascript
// Number check
value ?= `number`

// Text check
value ?= `text`

// Map check
value ?= `map`

// List check
value ?= `list`

// Operator check
value ?= `operator`

// Undefined check
value ?= `undefined`
```

**Note:** The `?=` operator returns lowercase type names: `text`, `number`, `list`, `map`, `operator`, `undefined`.

### Conditional Type Handling

Branch based on type:

```javascript
// Handle different types
{
  input: _<@0,
  return: {
    input ?= `text` ? 'Text value' :
    input ?= `number` ? 'Number value' :
    'Unknown type'
  }
}
```

## Undefined Propagation

### Pipeline Propagation

`undefined` propagates through pipelines:

```javascript
// Start with undefined
undefinedValue >> process >> transform
// Result: undefined (process never called)

// Later stage missing data
data ?{ @'missingField' } ${ _< * 2 }
// Result: [] (undefined fails predicate)
```

### Optional Chaining

Gracefully handle potentially undefined paths:

```javascript
// Optional chaining pattern
user
  >> @`profile`
  >> @`settings`
  >> @`theme`
// If any intermediate is undefined, result is undefined
```

### Default Values

Provide fallbacks for undefined:

```javascript
// Default with OR
value || fallback

// Default in map
{
  value: _<@0,
  result: value || 0 // Default to 0
}

// Nested default
obj@[, , ] || 'default'
```

## Comparison with Other Error Handling

### JavaScript Try/Catch

**JavaScript:**
```javascript
try {
  const result = riskyOperation();
  return result;
} catch (error) {
  console.error(error);
  return undefined;
}
```

**Left-Right:**
```javascript
// No try/catch, undefined by default
{
  result: riskyOperation,
  return: result || undefined
}
```

### Rust Result Type

**Rust:**
```rust
fn risky_operation() -> Result<i32, Error> {
    if error_condition {
        Err(Error::new("Something went wrong"))
    } else {
        Ok(42)
    }
}

// Usage
match risky_operation() {
    Ok(value) => value,
    Err(error) => {
        eprintln!("Error: {}", error);
        0
    }
}
```

**Left-Right:**
```javascript
// Simple undefined approach
{
  result: riskyOperation,
  return: result || 0
}
```

### Haskell Maybe/Option

**Haskell:**
```haskell
-- Maybe type
riskyOperation :: Maybe Int

-- Pattern matching
case riskyOperation of
    Just value -> value
    Nothing -> 0
```

**Left-Right:**
```javascript
// Direct undefined check
{
  result: riskyOperation,
  return: result || 0
}
```

### TypeScript Nullish Coalescing

**TypeScript:**
```typescript
// Optional chaining and null coalescing
const result = obj?.nested?.field ?? 'default';
```

**Left-Right:**
```javascript
// Path access and OR
obj@[`nested`, `field`] || `default`
```

**Left-Right:**
```javascript
// Nested default
obj@[`path`, `to`, `value`] || `default`
```

## Error Patterns

### Validation with ?

Validate data before processing:

```javascript
// Validate user input
{
  name: user@`name`,
  age: user@`age`,

  return: {
    name ?= `text` & name.length > 0 &
    age ?= `number` & age > 0 ?
      'Valid input' :
      'Invalid input'
  }
}
```

### Safe Property Access

Guard against undefined properties:

```javascript
// Safe access pattern
{
  user: _<@0,
  profile: user@`profile` || {},
  email: profile@`email` || `unknown@example.com`,
  return: email
}
```

### Fallback Values

Provide sensible defaults:

```javascript
// Configuration with defaults
{
  config: _<@0,
  timeout: config@`timeout` || 5000,
  retries: config@`retries` || 3,
  return: { timeout, retries }
}
```

### Pipeline Error Handling

Handle errors in pipeline stages:

```javascript
// Filter out undefined
data
  $_{ @`value` }           // Extract nested values
  ?{ _< || 0 }           // Filter out undefined
  ${ _< * 2 }              // Transform valid values
```

## Type-Dependent Error Behavior

### Numeric Operations

```javascript
// Undefined in arithmetic
undefined + 10     // Result: undefined
undefined * 2        // Result: undefined
undefined / 5        // Result: undefined
```

### Text Operations

```javascript
// Undefined in text operations
undefined ^            // Result: undefined
undefined >< `, '      // Result: undefined
undefined >"< `a`, `b` // Result: undefined
```

### Collection Operations

```javascript
// Undefined in collections
undefined #           // Result: 0
undefined ~           // Result: []
undefined ?{ _< }  // Result: []
undefined ${ _< * 2 } // Result: []
```

### Identity Elements for + Operator

The `+` operator has type-specific identity elements that affect error handling:

**Identity Elements:**
- `undefined` is identity for numbers: `1 + undefined` → `1`
- `` ` `` (empty text) is identity for text: `` `` + `1` → `` `1` `` (type coercion)
- `[]` is identity for lists: `[] + 0` → `[0]`
- Non-identity from different set: value disappears
- Text concat when either side is text
- List concat/append when either side is list

**Error Handling with Identity:**
```javascript
// Number identity
5 + undefined  // Result: 5 (undefined disappears)

// Text identity (with coercion)
1 + ``         // Result: `1` (empty text identity + type coercion)

// List identity
[] + 42        // Result: [42] (empty list identity)

// Non-identity disappears
`hello` + undefined  // Result: `hello` (undefined disappears in text context)
[1, 2] + undefined    // Result: [1, 2] (undefined disappears in list context)
```

**Benefits:**
- Graceful degradation when undefined appears
- Predictable behavior across types
- No exceptions thrown
- Values from different sets simply disappear

**Note on JSON Null Handling:**
When parsing JSON, null values are converted to `undefined` by default. This behavior can be configured globally to convert null to the text string `null` instead.

## Design Rationale

### Benefits of Undefined-First

1. **No Control Flow Disruption** — Errors don't interrupt execution
2. **Predictable Behavior** — Always returns a value
3. **Explicit Handling** — Must check for undefined explicitly
4. **Simpler Code** — No try/catch boilerplate
5. **Functional Style** — Error handling as data transformation

### Trade-offs

| Aspect | Exception-Based | Undefined-First |
|---------|----------------|-------------------|
| Error Detection | Automatic (try/catch) | Explicit checks required |
| Code Clarity | Explicit error paths | May hide errors |
| Performance | Exception overhead | No overhead |
| Debugging | Stack traces available | Harder to trace |
| Composability | Interrupts flow | Continuous flow |

## Advanced Patterns

### Error Boundaries

Define error handling at boundaries:

```javascript
// Error boundary function
{
  operation: _<@0,
  fallback: _>@1 || undefined,

  result: operation,
  return: result || fallback
}
```

### Validation Combinators

Chain validation checks:

```javascript
// Multiple validations
{
  input: _<@0,

  return: {
    input ?= `text` ?
      input.length > 0 ?
        input.length < 100 ?
          'Valid' :
          'Too long' :
        'Empty text' :
      'Not text'
  }
}
```

### Error Aggregation

Collect errors without failing:

```javascript
// Aggregate validation errors
{
  data: _<@0,

  errors: {
    nameMissing: data ?= `text` || data@`name` = undefined,
    ageInvalid: data@`age` ?= `number` || data@`age` < 0,
    emailInvalid: data@`email` ?= `text` || !data@`email`.includes(`@`)
  },

  return: {
    valid: !errors@0 & !errors@1 & !errors@2,
    errors
  }
}
```

## Design Principles

1. **Undefined Default** — Errors return undefined
2. **No Exceptions** — No throw/catch mechanism
3. **Explicit Checking** — Use `?` for type validation
4. **Graceful Degradation** — Operations continue on error
5. **Pipeline-Safe** — Undefined propagates through stages
6. **Loosely Typed** — Type errors become undefined

## Related Concepts

- **Error Handling** — Managing exceptional conditions
- **Graceful Degradation** — Continue operation on error
- **Type Checking** — Runtime type verification
- **Optional Chaining** — Safe property access
- **Nullish Coalescing** — Fallback for null/undefined
- **JSON Null Handling** — Null to Undefined conversion
- **Result Type** — Rust's explicit error handling
- **Option Type** — Haskell's Maybe monad
- **Monadic Error Handling** — Functional error propagation
- **Exception Safety** — Guarantee behavior on error
