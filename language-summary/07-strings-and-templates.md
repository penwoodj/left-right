# Text and Templates — Interpolation and Manipulation

Left-Right treats all text as template literals by default, with powerful interpolation and transformation capabilities. This document covers text handling, interpolation syntax, case transformation, and comparison with template engines.

## Backtick-Only Text Syntax

**Critical Design Decision:** Text uses ONLY backticks in Left-Right.

- `` `text` `` is the ONLY text syntax
- No single quotes or double quotes for text
- `"` and `'` are reserved for operator names (e.g., `"` is toLower, `^` is toUpper)
- NO other operator may contain the backtick `` ` `` character

This is a foundational design decision that provides:
- Clear distinction between text and operators
- Visual clarity in code
- No ambiguity about text vs operator
- Matches JavaScript template literal syntax

## Template Literals

### Default Behavior

All text in Left-Right are template literals, supporting interpolation:

```javascript
// Basic interpolation
`Hello, {name}!` // Interpolates 'name' variable

// Multiple interpolations
`User: {user@'name'}, Age: {user@'age'}`

// Expression interpolation
`Result: {10 + 20}` // Evaluates expression
```

### Interpolation Syntax

#### Curly Brace Delimiters

Curly braces `{ }` denote interpolation in text:

```javascript
// Simple variable
`Hello {firstName}`

// Property access
`Email: {user@'email'}`

// Expression
`Sum: {a + b}`
```

#### Path Interpolation

Interpolate nested paths:

```javascript
// Deep path access
`Value: {config@'database', 'host'}`

// Array access
`Item: {list@0}`

// Computed path
`Field: {obj@[dynamicField]}`
```

## Text as Operators

### Operator Lifting

An interpolated text becomes an operator when any `{ }` contains `_<` or `_>`. Otherwise the text evaluates as Text (normal text).

```javascript
// Text with placeholder becomes operator
{ input: _<@0,
  `Thanks {_<} for your help`
}

// Usage
user >> `Thanks {_<} for your help`
// Result: "Thanks Alice for your help"
```

### Template Functions

Define reusable text templates:

```javascript
// Greeting template
greeting: {
  name: _<@0,
  `Hello, {name}!`
}

// Usage
greeting['Alice'] // "Hello, Alice!"
```

### Conditional Templates

Truthy/falsy-driven template content:

```javascript
// Conditional template
{
  name: _<@0,
  active: _>@1,
  name! & `Hello` :: `Goodbye`} {active}
}

// Usage
message['Bob', true] // "Hello Bob"
message['Bob', false] // "Goodbye Bob"
```

## Text Transformation Operators

### Case Operators

#### ^ — Uppercase

Convert text to uppercase:

```javascript
`hello`^ // "HELLO"
`hello world`^ // "HELLO WORLD"

// In pipeline
name@'value' ^
```

#### ^_ — Capitalize

Convert first character to uppercase, rest lowercase:

```javascript
`hello`^_ // "Hello"
`hELLO`^_ // "Hello"
`hello world`^_ // "Hello world"

// Capitalize names
threats ${ @['AI Confidence Level', 'value'] "^_}
```

#### " — Lowercase

Convert text to lowercase:

```javascript
`HELLO"^ // "hello"
`HELLO WORLD`^ // "hello world"

// Normalize for comparison
searchTerm^
```

### Spatial Symbology

The spatial arrangement of `^` and `"` creates case transformation operators:

| Operator | Position | Effect |
|----------|----------|---------|
| `^` | Prefix | Uppercase entire text |
| `^_` | Prefix + suffix | Capitalize first character |
| `_"` | Suffix + prefix | Lowercase entire text |

**Rationale:**
- `^` suggests raising/elevating (uppercase)
- `_` suggests lowering (lowercase)
- Combination `^_` suggests title case (raise then lower)

### Join Operator

#### >< — Join

Concatenate collection elements with separator:

```javascript
// Join strings
[`Alice`, `Bob`, `Carol`] >< `, `
// Result: "Alice, Bob, Carol"

// Join numbers
[1, 2, 3] >< ``
// Result: "123"

// Custom separator
[`a`, `b`, `c`] >< ` | `
// Result: "a | b | c"

// Empty separator (concat)
[`a`, `b`, `c`] >< ``
// Result: "abc"
```

### Split Operator

#### <> — Split

Break text into parts:

```javascript
// Split by comma
`a,b,c` <> `,`
// Result: [`a`, `b`, `c`]

// Split by whitespace
`hello world` <> ` `
// Result: [`hello`, `world`]

// Split by multiple characters
`a:b;c` <> `:`
// Result: [`a`, `b;c`]
```

### Trim Operator

#### <" — Trim

Remove whitespace from both ends:

```javascript
// Trim spaces
`  hello  ` <"
// Result: "hello"

// Trim newlines
`\n  hello  \n` <"
// Result: "hello"

// In pipeline
userInput <" ~~ lowercase // Trim and lowercase
```

### Replace Operator

#### >"< — Replace

Substitute patterns in text:

```javascript
// Replace substring
`hello world` >"< `world`, `Left-Right`
// Result: "hello Left-Right"

// Multiple replacements
`aaa` >"< `a`, `b`
// Result: "bbb"

// Pattern replacement
`test@example.com` >"< `@`, ` [at] `
// Result: "test [at] example.com"
```

## Advanced Text Patterns

### Multi-Line Text

Text can span multiple lines:

```javascript
// Multi-line text
`Line 1
Line 2
Line 3`

// Preserves newlines
```

### Template Expressions

Complex expressions in templates:

```javascript
// Arithmetic in template
`Total: {price * quantity}`

// Function call in template
`Processed: {processDate[date]}`

// Conditional in template
`Status: {status ? `Active` : `Inactive`}`
```

### Nested Interpolation

Templates can contain nested interpolations:

```javascript
// Multi-level substitution
`Processed {processedDate} from {originalDate}`

// Deep path interpolation
`Value: {config@'database', 'host', 'port'}`
```

### Interpolation with Directional Operators

When interpolation expressions contain `_<` or `_>` operators, special behavior applies:

#### _< in Interpolation

The `_<` operator in an interpolated expression evaluates to an unexecuted operator that will be applied to the value being interpolated:

```javascript
// Interpolation contains _<
`Result: {_<+ 1}`

// If interpolated with 5, becomes "Result: 6"
// because _< receives 5, then + 1 is applied
```

#### _> in Interpolation

The `_>` operator in an interpolated expression evaluates to an unexecuted operator that expects a right argument:

```javascript
// Interpolation contains _>
`Result: {5 + _>}`

// If interpolated with 3, becomes "Result: 8"
// because _> receives 3, then 5 + 3 is evaluated
```

#### Multiple _< in Same String

When multiple interpolations in one string contain `_<`, they all receive the same left value:

```javascript
// Multiple _< in one template
`Min: {_< min}, Max: {_< max}, Avg: {_< avg}`

// Interpolated with a dataset, all three _< placeholders receive the same dataset
// Result: "Min: 10, Max: 100, Avg: 55"
```

#### Multiple _> in Same String

When multiple interpolations in one string contain `_>`, they all receive the same right value:

```javascript
// Multiple _> in one template
`Sum: {10 + _>}, Diff: {100 - _>}, Product: {5 * _>}`

// Interpolated with 5
// All three _> placeholders receive 5
// Result: "Sum: 15, Diff: 95, Product: 25"
```

#### _< and _> Together in Interpolated Expressions

If an interpolated string has both `_<` and `_>` in expressions, typical operator currying behavior applies:

```javascript
// Both directional operators present
`Value: {_< * 2 + _>}`

// Interpolated with [5, 3]
// _< receives 5, _> receives 3
// Result: "Value: 13" (5 * 2 + 3)
```

This behavior allows for flexible template composition where operators can be partially applied and later completed during interpolation.

### Escaping Literal Braces

To include literal curly braces in text, escape them with backslash:

```javascript
// Escaped braces appear literally
`This has \{literal\} braces`
// Result: "This has {literal} braces"

// Escape only one brace
`Unescaped {value} but escaped \{this\}`
// Result: "Unescaped [interpolated value] but escaped {this}"

// Double escape for backslash itself
`Use \\ to escape`
// Result: "Use \ to escape"
```

**Escape Rules:**
- `\{` → literal `{`
- `\}` → literal `}`
- `\\` → literal `\`
- Unescaped `{` and `}` trigger interpolation

### Map-to-Text Conversion

When a map is interpolated into text or used in text context, it textifies as minified JSON-like text (no spaces, no line breaks):

```javascript
// Map textifies in template
{
  data: { a: 1, b: 2 },
  `Result: {data}`
}
// Evaluates to: `Result: {a:1,b:2}`

// Example from evaluation model
[`param1content`, `param2content`] {
  param1: _<@0,
  param2: _<@1,
  `Result: {param1 + param2}`
}
// Evaluates to: `Result: param1contentparam2content`
```

## Comparison with Template Engines

### JavaScript Template Literals

**JavaScript:**
```javascript
// Backtick syntax
`Hello, ${name}! You are ${age} years old.`
```

**Left-Right:**
```javascript
// Curly brace syntax
`Hello, {name}! You are {age} years old.`

// Same interpolation, different delimiter
```

### Handlebars / Mustache

**Handlebars:**
```handlebars
// Mustache syntax
<p>Hello, {{name}}!</p>
{{#if active}}
  <p>Status: Active</p>
{{/if}}
```

**Left-Right:**
```javascript
// Unified syntax
<p>Hello, {name}!</p>
<p>Status: {active ? `Active` : `Inactive`}</p>
```

### Jinja2

**Jinja2:**
```jinja
// Double braces
<p>Hello, {{ name }}!</p>
{% if active %}
  <p>Status: Active</p>
{% endif %}
```

**Left-Right:**
```javascript
// Same delimiters for all constructs
<p>Hello, {name}!</p>
<p>Status: {active ? `Active` : `Inactive`}</p>
```

## Text Pipeline Patterns

### Sequential Transformations

Apply multiple text operations:

```javascript
// Chain transformations
userInput
  <"                 // Trim
  ^                   // Uppercase
  >"< ` `, `_`       // Replace spaces with underscores
  ^                   // Uppercase again

// Clean and format input
```

### Collection Processing

Transform text in collections:

```javascript
// Capitalize all names
names ${ _< ^_ }

// Normalize emails
emails ${ _< <" ^ }

// Generate slugs
titles ${ _< <" ^ >"< ` `, `-` ^ }
```

### Data Formatting

Format structured data:

```javascript
// Format user data
{
  name: user@'name',
  email: user@'email',
  output: `Name: {name}, Email: {email}`
}
```

## Design Principles

1. **Universal Templates** — All strings support interpolation
2. **Unified Syntax** — Curly braces for all operations
3. **Operator Lifting** — Strings with placeholders become callable
4. **Spatial Symbology** — Position indicates operation direction
5. **Composable** — String operations chain in pipelines
6. **Multi-Line Support** — Natural text formatting
7. **Graceful Escaping** — Literal delimiters mechanism

## Related Concepts

- **Template Literals** — Text interpolation
- **Text Interpolation** — Variable substitution
- **Template Engines** — Handlebars, Jinja, Mustache
- **Code Generation** — Creating code from templates
- **Templating DSLs** — Languages for text generation
- **Curly Brace Templates** — `{variable}` syntax style
- **Text Manipulation** — Transformation operations
- **Case Conversion** — Uppercase, lowercase, capitalize
