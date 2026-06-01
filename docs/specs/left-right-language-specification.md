# Left-Right Language Specification

## Overview & Philosophy

Left-Right is a point-free, operator-based, array-oriented language with left-to-right evaluation flow. The language is designed for compressed expression data flow readability, emphasizing simplicity and complete correct expression as central principles.

### Core Philosophy

- **Data-first**: Data appears first/on the left in expressions
- **Left-hungry curried operators**: Operators eagerly consume their left operand, return an intermediate operator awaiting the right operand
- **Ticking evaluation**: The "tick" is the semantic moment when an operator's intermediate state executes with the right operand
- **Simple semantics, massive expressiveness**: Minimal syntax rules enable complex transformations through operator composition

## Type System

### Types

Left-Right has seven types:

1. **Operator** - executable functions with curried semantics
2. **Map** - key-value pairs (JavaScript objects)
3. **List** - ordered sequences (JavaScript arrays)
4. **String** - text data (JavaScript strings)
5. **Boolean** - true/false (JavaScript booleans)
6. **Number** - numeric values (JavaScript numbers)
7. **Undefined** - absence of value (JavaScript undefined)

### Type Categories

**Collections** (all are iterable):
- Maps
- Lists
- Strings

**Reference Types**:
- Collections (Maps, Lists, Strings)
- Operators

**Primitives**:
- Booleans
- Numbers
- Undefined

### Type Agnostic Equality

The `=` operator performs type-agnostic equality checks, similar to JavaScript's `==` but with Left-Right semantics.

## Expression Evaluation Model

### Left-Hungry Curried Operators

Left-Right expressions evaluate through the "left-hungry curried" model:

**Fundamental rule**: `"data leftInputOperator"` evaluates like `"((data) leftInputOperator)"`

**Dyadic example**: `"5 + 9"` evaluates like `"(((5)+)9)"`:
1. `((5)+)` - The `+` operator consumes left operand `5`, returns operator awaiting right arg
2. Ticks to next item `9` - The intermediate operator executes with right operand
3. Result: `14`

**Complex example** (from request.lr):
```lr
response@`body` /json
```
1. `response` passes to `@` (dyadic get operator)
2. `response@` returns unexecuted operator awaiting right arg `` `body` ``
3. Right arg `` `body` `` executes, yields response.body value
4. `/json` operator casts to JSON
5. Result: JSON-parsed body

### Ticking and Hunger

We evaluate with this ticking and hunger of operators **before expression**. The "tick" is the semantic moment when an operator's intermediate state executes with the right operand. Operators consume operands before the expression context determines final execution.

Operators "tick" forward through expressions, consuming operands:
- Unary operators consume left operand and execute
- Dyadic operators consume left operand, return intermediate state, then tick to right operand
- Map operators remain unexecuted until placed in expression with required input

Operator "hunger" drives expression evaluation:
- **Unary operators**: consume one operand, execute immediately
- **Dyadic operators**: consume left operand, return intermediate state (partial function), await right operand
- **Map operators**: consume any operand, but remain unexecuted as intermediate state until expression completion

## Lexical Structure

### Tokens

Left-Right tokens include:
- **Identifiers**: alphanumeric sequences (e.g., `logging`, `removePrivateIps`, `entities`)
- **Operators**: symbolic sequences (e.g., `+`, `@`, `$`, `/`, `|`, `&`, `<`, `>`, `^`, `_`, `-`, `%`, `~`, `!`, `=`, `#`)
- **Literals**: Numbers, Booleans, Strings (backtick-enclosed)
- **Punctuation**: `:`, `,`, `.`, `(`, `)`, `[`, `]`, `{`, `}`, `` ` `'`, `/`, `?`
- **Keywords**: `try`, `catch` (as map keys only)

### Whitespace

Whitespace is generally flexible but has semantic significance:
- Spaces separate tokens
- No spaces between operator symbols in compound operators (e.g., `@`, `/json`, `$_`)
- Spaces after operators may affect parsing (especially for `_` prefix patterns)
- Indentation: 2 spaces for readability (not semantically required)

### Comments

Inline comments use triple backtick prefix:
```lr
```@& is kind of like pick in lodash/fp
```

Comments can appear after expressions, separated by whitespace.

## Collection Literals

### Maps

Maps are key-value pairs enclosed in curly braces:
```lr
{
  logging: [`setLogger`, `getLogger`],
  errors: [`parseErrorToReadableJson`]
}
```

**Key patterns**:
- Simple keys: `key: value`
- Expression keys: Keys that are operators or contain operators become expression keys
- Trailing commas: Allowed

### Lists

Lists are ordered sequences enclosed in square brackets:
```lr
[`setLogger`, `getLogger`]
```

Empty list: `[]`

### Strings

Strings are enclosed in backticks:
```lr
`polarity-integration-utils`
```

Empty string: ``

**Template interpolation**: Variables interpolated with `{var}` syntax inside strings:
```lr
`Bearer {token}`
`https://api.{options@`apiRegion`}.app.wiz.io/graphql`
```

### Operator Encapsulation

Curly braces `{}` and backticks `` also serve as operator encapsulation:
- `{}` creates map operators when left/right arg operators (`_<`, `_>`) are present inside, or when the last item in the map is an expression (not a key-value pair), or when one or more of its keys is an expression
- `` creates string operators when any of the interpolated elements inside `{}` contain a left or right argument (`_<`, `_>`)

Both map operators and string operators remain unexecuted unless they are either declared in an expression that has all needed input around them as needed, or stored in a variable and put in an expression where they execute.

## Operators

### Operator Semantics SDK

| Symbol | Meaning | Type |
|--------|---------|------|
| `$` | iterate/loop/map | Prefix for iterable operators |
| `@` | get/navigation | Prefix for get/navigation |
| `?` | toBoolean (or !!) | Prefix for boolean output methods |
| `/` | divide (conversion/casting) | Type conversion/moving forward and generating |
| `` `|` `` | OR | Postfix to sum type/stopping operators |
| `&` | AND | Prefix to product type operators |
| `<` | less than | Multiple operators relating to left direction, separating |
| `>` | greater than | Multiple operators relating to right direction, condensing and grouping |
| `^` | exponent | Uppercase, wrapping |
| `_` | flatten/floor/lowercase | Execute without output; associated with lowercase or flattening |
| `+` | addition/concatenation/spread (type-dependent) | Increasing |
| `-` | subtract/omit/remove (type-dependent) | Decreasing |
| `%` | modulus | Ratios, percentages, sorting |
| `~` | unique | Randomness, entropy |
| `"` | toString | String conversion |
| `#` | size/length | Collection size |
| `=` | type-agnostic equality | Equality check |
| `!` | negate/to boolean/not | Boolean negation |

### Loop Operators

| Operator | Semantics | Example |
|----------|-----------|---------|
| `collection $` | Iterate, left arg = item as it loops | `items $ { _< * 2 }` |
| `list $@` | Get string key from each item, returns list of values | `maps $@ `key`` |
| `list of strings $@` | Get those keys, returns list of maps with only selected pairs | `maps $@ [`key1`, `key2`]` |
| `$?` | Filter — with map expression: keeps items where expression is truthy. With string: filters out maps where that key name is not truthy. With list of strings: same but AND logic for all keys | `items $? { _< > 10 }` |
| `$_` | Flatmap | `lists $_` |
| `$~` | UniqueBy | `items $~` |
| `$>` | GroupBy | `items $>` |
| `$"` | EachToString | `items $"` |
| `$&` | Every/All | `items $& { _< > 0 }` |
| `` `$|` `` | Some/Any | `items $| { _< > 0 }` |
| `` `$?|` `` | Find | `items $?| { _< > 10 }` |
| `$%` | Sort | `items $%` |
| `$?!` | Compact | `items $?!` |
| `$|||` | Parallel map — multi-threaded in VM, Promise.all in JS transpiler | `items $||| { _< * 2 }` |

### String Operators

| Operator | Semantics | Example | Result |
|----------|-----------|---------|--------|
| `" or /"` | toString | `5 /"` | `"5"` |
| `"_` | Lowercase | `"HELLO" "_` | `"hello"` |
| `"^` | Uppercase | `"hello" "^` | `"HELLO"` |
| `"^_` | Capitalize | `"hello" "^_` | `"Hello"` |
| `"~` | Replace | `"hello" "~ ["e","a"]` | `"hallo"` |

Note: `"~` replace is not directly observable in the SOT translation files. The syntax shown is inferred from the operator semantics specification.
| `<>` | Split — as long as not immediately preceded without space by `_` | `"a,b,c" <>,` | `["a","b","c"]` |
| `><` | Join — as long as not immediately preceded without space by `_` | `["a","b"] >><,` | `"a,b"` |

**Spacing rules**: `<>` is split and `><` is join. When `_` immediately precedes without spaces (e.g., `_<>` or `_><`), the split/join does NOT apply — `_` becomes part of a different compound operator (e.g., the left/right argument operators `_<`/`_>` or other `_`-prefixed operators).

### Boolean Operators

| Operator | Semantics | Example |
|----------|-----------|---------|
| `` `|` `` | OR | `` `a | b` `` |
| `&` | AND | `a & b` |
| `=` | Equals | `a = b` |
| `?"` | IsString | `value ?"` |
| `?#` | IsNumber | `value ?#` |
| `?>` | Contains | `"hello" ?>< "el"` |

### Conversion Operators

| Operator | Semantics | Example | Result |
|----------|-----------|---------|--------|
| `/json` | Parse JSON | `"{\"a\":1}" /json` | `{a: 1}` |
| `/"` | ToString | `123 /"` | `"123"` |
| `response@`body` /json` | Get property, then parse | - | - |

### Reserved Symbols (cannot appear in operators)

| Symbol | Meaning |
|--------|---------|
| `:` | Assignment and conditional return |
| `,` | Collection item separator |
| `.` | Reverse arguments operator |
| `'` | Reserved for later |
| `()` | Precedence operators in expressions |
| `[]` | An empty list collection and the list encapsulation operators |
| `{}` | Empty map, operator and map encapsulation |
| `_<` | Left argument operator — no matter the context, always parsed as left argument and evaluates to the value of the item passed in on the left |
| `_>` | Right argument operator — no matter the context, always parsed as right argument and evaluates to the value of the item passed in on the right |
| `` | Empty string, string/operator encapsulation |

### Compound Operators

Operators can be combined to form compound operators:
- `$@` - Get each item's property
- `$?` - Filter
- `$_` - Flatmap
- `/json` - Parse JSON
- `?"` - IsString
- `?!` - Negate boolean (filter where falsey)
- `?:` - Check truthy (toBoolean) and proceed (early return guard)
- `!!!` - Throw error
- `+:` - Spread into context

## Import System

### Imports from Packages

**Namespaced import with object destructuring**:
```lr
+: imports@`polarity-integration-utils`@&{
  logging: [`setLogger`, `getLogger`],
  errors: [`parseErrorToReadableJson`]
}
```
→ JS:
```js
const { logging: { setLogger, getLogger }, errors: { parseErrorToReadableJson } } = require('polarity-integration-utils');
```

**Flat array import (no namespace)**:
```lr
+: imports@`lodash/fp`@&[`map`, `get`, `getOr`, `filter`, `flow`, `negate`, `isEmpty`, `size`]
```
→ JS:
```js
const { map, get, getOr, filter, flow, negate, isEmpty, size } = require('lodash/fp');
```

**Default import (no destructuring)**:
```lr
NodeCache: imports@`node-cache`
```
→ JS:
```js
const NodeCache = require('node-cache');
```

### Imports from Local Files

**Spread import with array destructuring**:
```lr
+: files@`./server/queries`@&[`queryIssues`, `queryVulnerabilities`, `queryAssets`]
```
→ JS:
```js
const { queryIssues, queryVulnerabilities, queryAssets } = require('./server/queries');
```

**Spread import with namespace and array**:
```lr
+: imports@`polarity-integration-utils`@&{
  requests: [`createRequestWithDefaults`]
}
```
→ JS:
```js
const { requests: { createRequestWithDefaults } } = require('polarity-integration-utils');
```

**Single function import (no spread)**:
```lr
assembleLookupResults: files@`./server/assembleLookupResults`
```
→ JS:
```js
const assembleLookupResults = require('./server/assembleLookupResults');
```

### Import Syntax Patterns

- `imports@`package`` - Import from npm package
- `files@`path`` - Import from local file
- `+:` - Spread imports into current scope
- `@&{...}` - Namespaced import with object destructuring
- `@&[...]` - Array destructuring import
- Variable name followed by `:` - Named import (as variable)

## Export System

### Export Declaration

Exports are declared on the closing `}` of the top-level map:

```lr
}@&[`startup`,`validateOptions`,`doLookup`]
```
→ JS:
```js
module.exports = {
  startup,
  validateOptions,
  doLookup
};
```

**Multiple exports**:
```lr
}@&[`requestWithDefaults`,`requestsInParallel`]
```
→ JS:
```js
module.exports = {
  requestWithDefaults,
  requestsInParallel
};
```

The `@&` followed by array of keys exports only those keys from the enclosing map.

## Async/Await

### Async Function Definition

Async functions use `///` to declare async context and `\\\` to await:

```lr
{ options: _<@0, query: _<@1,
  token: [options] getToken \\\,
  { method: `POST`, ... } ///,
  ...
} ///
```
→ JS:
```js
async ({ options, query }) => {
  const token = await getToken(options);

  return {
    method: 'POST',
    ...
  };
}
```

`///` marks the function boundary as async (the operator returned is resolved on a different thread or concurrently). `\\\` awaits the result within the async context.

**Key patterns**:
- `\\\` - Await (like JS `await` / promise await). Resolves the async operator on the left and returns the result.
- `///` - Takes an operator on the left and makes it either resolved on a different thread (multi-threading) or async (concurrency). It still returns an unexecuted operator. Whether multi-threading or concurrency, it operates like a promise.
- `[arg] func \\\` - Await the result of calling function `func` with `arg`

### Async Function Calls

**Simple async call**:
```lr
[options] getToken \\\
```
→ JS:
```js
await getToken(options)
```

**Async call with subsequent operations**:
```lr
requestForAuth \\\ @[`body`,`access_token`]
```
→ JS:
```js
get('body.access_token', await requestForAuth(...))
```

The `\\\` after function name awaits the async call. The result can then be operated on with `@` for path access.

### Promise.all Pattern

**Parallel execution**:
```lr
+: options $|||[
  { _<@`queryIssues`: [cveEntities, _<] queryIssues, [] },
  { _<@`queryVulnerabilities`: [cveEntities, _<] queryVulnerabilities, [] },
  { _<@`queryAssets`: [entitiesWithIds, _<] queryAssets, [] }
]
```
→ JS:
```js
const [issues, vulnerabilities, assets] = await Promise.all([
  options.queryIssues ? queryIssues(cveEntities, options) : [],
  options.queryVulnerabilities ? queryVulnerabilities(cveEntities, options) : [],
  options.queryAssets ? queryAssets(entitiesWithIds, options) : []
]);
```

The `$|||` operator performs parallel map execution (Promise.all in JS transpiler, multi-threaded via `std::thread::scope` in compiled VM).

## Error Handling

### Try/Catch Blocks

Try/catch uses map keys `try:` and `catch:`:

```lr
doLookup: { entities: _<@0, _options: _<@1, callback: _<@2,
  try: {
    ...
    [null, lookupResults] callback
  },
  catch: { error: _<,
    err: error parseErrorToReadableJson,
    _: { error:error, formattedError: err } (Logger@`error`)
    [{ detail: error.message || 'Lookup Failed', err }] callback
  }
}
```
→ JS:
```js
const doLookup = async (entities, _options, cb) => {
  try {
    // ... success logic
    cb(null, lookupResults);
  } catch (error) {
    const err = parseErrorToReadableJson(error);
    Logger.error({ error, formattedError: err }, 'Get Lookup Results Failed');
    cb({ detail: error.message || 'Lookup Failed', err });
  }
};
```

**Key patterns**:
- `try: { ... }` - Try block
- `catch: { error: _<, ... }` - Catch block, error bound to left arg
- `error:` key in catch map - Destructure error

### Throw Error

**Throw with `!!!`**:
```lr
bodyError !!!
```
→ JS:
```js
throw bodyError;
```

**Error creation and throw**:
```lr
bodyErrors #: {
  bodyError: Error[bodyErrors@[0,`message`]],
  bodyError@`status`: response@`status`,
  bodyError !!!
}
```
→ JS:
```js
if (size(bodyErrors)) {
  const bodyError = new Error(get('0.message', bodyErrors));
  bodyError.status = response.status;
  throw bodyError;
}
```

### Error Object Creation

**Error constructor with message**:
```lr
Error[bodyErrors@[0,`message`]]
```
→ JS:
```js
new Error(get('0.message', bodyErrors))
```

The `Error` constructor is called with the result of the bracket expression.

### Error Message Assignment

```lr
error@`message`: `{error@`message`} - ({error@`status`}) | {error@`description`}`
```
→ JS:
```js
error.message = `${error.message} - (${error.status}) | ${error.description}`;
```

Property assignment on error object with template interpolation.

## Control Flow

### Ternaries

**Map operator as ternary (conditional execution)**:
```lr
options { _<@`searchPrivateIps`!: entities, entities removePrivateIps }
```
→ JS:
```js
if (!options.searchPrivateIps) {
  searchableEntities = removePrivateIps(entities);
}
```

**Pattern**: `{ condition: trueValue, falseValue }`

When a map has:
- First key is a left arg operator with `!: ` (negate boolean check)
- Second key is an expression or function call
The map acts as ternary: if condition (with `!`) is true, return second key's value.

**Ternary map with boolean property**:
```lr
{ _<@`queryIssues`: [cveEntities, _<] queryIssues, [] }
```
→ JS:
```js
options.queryIssues ? queryIssues(cveEntities, options) : []
```

**Pattern**: `{ _<@`property`: valueIfTruthy, valueIfFalsy }`

When first key is `_<@`property`` (get property from left arg), map acts as ternary.

**Ternary with explicit branches**:
```lr
{_<: `trueCase`, `falseCase`}
```
→ JS:
```js
booleanExpression ? trueCase : falseCase
```

Standard ternary syntax. The `_<:` prefix reads as "left argument, then evaluate branches."

**Full ternary with toString prefix**:
```lr
"booleanExpression {_<: `trueCase`, `falseCase`}
```

The `"` (toString) prefix before the boolean expression is the canonical ternary form. It converts the boolean expression to a value that the map operator can branch on. Ternaries are just control flow — the map acts as a conditional switch based on the truthiness of the expression to its left.

### Early Return

**Early return with `?:`**:
```lr
cachedToken?: cachedToken
```
→ JS:
```js
if (cachedToken) return cachedToken;
```

**Pattern**: `value?:` - Convert to boolean with `?`, then `:` triggers conditional return if truthy, else continue.

The `?:` operator uses the toBoolean `?` to check truthiness of left value and returns early if true.

**Ternary with early return**:
```lr
responseGetPath!! {_<: response@responseGetPath, response }
```
→ JS:
```js
return responseGetPath ? get(responseGetPath, response) : response;
```

**Pattern**: `condition? {_<: trueValue, falseValue }`

Early return with ternary selection. Uses `?` toBoolean then map operator for branching.

## Map Operators

### Making Non-Operators into Operators

There are 4 ways to make a non-operator into an operator — 3 for Maps and 1 for Strings:

**Map Operators (`{}`) — 3 ways**:

1. **Map has left or right arg operator inside**:
```lr
options { _<@`searchPrivateIps`!: entities, entities removePrivateIps }
```
Map contains `_<` or `_<@` (left arg operator inside), becomes ternary operator.

**2. Map operator ending in expression**:
```lr
{ _<@`queryIssues`: [cveEntities, _<] queryIssues, [] }
```
Map's last item is an expression (not a key-value pair), becomes ternary operator.

**3. Map operator with expression as a key**:
```lr
_<@`searchPrivateIps`!: entities
```
One or more of its keys is itself an expression (e.g., `_>@0` as a key, or a negated boolean check as a key), making the map an operator. When the key is an expression, the operator returns what is to the right or executes a map operator to the right, and the key does not become a variable binding.

**String Operators — 1 way**:
```lr
`Bearer {token}`
```
A string becomes a string operator if any of the interpolated elements inside `{}` contain a left or right argument (`_<`, `_>`).

**Unexecuted unless**:
- In expression with needed input
- Stored in variable and put in expression

### Map Operator Execution Rules

**No-output execution** (underscore key):
```lr
_: { issues:issues, vulnerabilities:vulnerabilities, assets:assets } (Logger@`trace`)
```
→ JS:
```js
Logger.trace({ issues, vulnerabilities, assets });
```

The `_` key executes the expression to its right without producing variable output (side effect only). This applies when `_` appears by itself before `:`.

**`_` has three conditions**:

1. **By itself before `:`** — runs the expression without variable output (side effect only)
2. **Immediately preceding `<` or `>` without spaces** — forms part of the reserved argument operators (`_<` and `_>`)
3. **Immediately preceded (or inside of) another operator symbol without spaces** — forms part of that compound operator (e.g., `$_` flatmap, `$>` groupBy, `"~` replace)

The `_` operator is semantically associated with lowercase and flattening.

**Side-effect with value return**:
```lr
[null, lookupResults] callback
```
→ JS:
```js
cb(null, lookupResults);
```

Calling callback (side effect) without return value.

## String Operators & Template Interpolation

### Template Interpolation

Variables interpolated with `{var}` syntax inside backtick strings:

```lr
`Bearer {token}`
`https://api.{options@`apiRegion`}.app.wiz.io/graphql`
```
→ JS:
```js
`Bearer ${token}`
`https://api.${options.apiRegion}.app.wiz.io/graphql`
```

**Nested interpolation**:
```lr
`{error@`message`} - ({error@`status`}) | {error@`description`}`
```
→ JS:
```js
`${error.message} - (${error.status}) | ${error.description}`
```

String interpolation can contain Left-Right expressions (property access, etc.).

### String as Operator

Strings become operators when:
1. They contain interpolated left or right arg
2. They are used in expressions requiring operator output

```lr
`Bearer {token}`
```
This is a string operator that takes `token` as input and returns formatted string.

## Arguments & Function Definition

### Positional Arguments

**Function definition with named arguments**:
```lr
doLookup: { entities: _<@0, _options: _<@1, callback: _<@2,
```
→ JS:
```js
const doLookup = async (entities, _options, cb) => {
```

**Pattern**: `{ name: _<@N, ... }`
- `_<@0` - First argument
- `_<@1` - Second argument
- `_<@2` - Third argument
- Etc.

**Async function definition**:
```lr
{ options: _<@0, query: _<@1,
  ...
} ///
```
→ JS:
```js
async ({ options, query }) => {
  ...
}
```

**Pattern**: `{ name: _<@N, ... }` followed by `///` marks the function as async.

### Named Arguments (Destructuring)

**Named argument from left arg**:
```lr
responseGetPath? {_<: response@responseGetPath, response }
```
→ JS:
```js
return responseGetPath ? get(responseGetPath, response) : response;
```

`_<@`property`` - Get property from left argument

```lr
_<@`searchPrivateIps`!: entities
```
→ JS:
```js
!options.searchPrivateIps ? entities : ...
```

`_<@`key`` - Access key from left argument

### Right Argument Operator

```lr
_< - `entity`
```
→ JS:
```js
({ entity, ...requestOptions })
```

Omit `entity` from left argument, return rest (destructuring with omit).

## Assignment & Variable Binding

### Variable Assignment with `:`

**Simple assignment**:
```lr
options: {
  +: _options,
  maxConcurrent: 1,
  ...
}
```
→ JS:
```js
const options = {
  ..._options,
  maxConcurrent: 1,
  ...
};
```

**Pattern**: `key: expression` binds result of expression to key.

### Spread Assignment

**Spread with `+:`**:
```lr
+: imports@`polarity-integration-utils`@&{...}
+: _options
```
→ JS:
```js
const { ... } = require('...');
const options = { ..._options, ... };
```

`+:` at map level spreads into current scope (top-level import).
`+: _options` spreads object into parent map.

### Property Assignment on Imported Value

```lr
config@`request`: { cert: ``, key: ``, ... }
```
→ JS:
```js
config.request = {
  cert: '',
  key: '',
  ...
};
```

Access property on imported value (`config`), then assign to it.

## Spread

### Spread Operator `+:`

**Spread object into current context**:
```lr
+: imports@`polarity-integration-utils`@&{...}
```
→ JS:
```js
const { logging: { setLogger, getLogger }, ... } = require('...');
```

**Spread into parent map**:
```lr
options: {
  +: _options,
  maxConcurrent: 1,
  ...
}
```
→ JS:
```js
const options = {
  ..._options,
  maxConcurrent: 1,
  ...
};
```

**Spread into list**:
```lr
[entities, issues, vulnerabilities, assets, options] assembleLookupResults
```
→ JS:
```js
assembleLookupResults(entities, issues, vulnerabilities, assets, options)
```

**Spread to create new map**:
```lr
response + { body: jsonResponseBody }
```
→ JS:
```js
{
  ...response,
  body: jsonResponseBody
}
```

## Key-Value Patterns

### Simple Key-Value

```lr
logging: [`setLogger`, `getLogger`]
```
→ JS:
```js
logging: ['setLogger', 'getLogger']
```

### Nested Key-Value

```lr
logging: [`setLogger`, `getLogger`],
errors: [`parseErrorToReadableJson`]
```
→ JS:
```js
{
  logging: ['setLogger', 'getLogger'],
  errors: ['parseErrorToReadableJson']
}
```

### Expression Key

**Key is operator**:
```lr
_<{ _<@`searchPrivateIps`!: entities, entities removePrivateIps }
```

When key is operator (contains `_<`), map becomes operator.

**Key is nested expression**:
```lr
_: { issues:issues, vulnerabilities:vulnerabilities, assets:assets }
```

Nested maps make parent map a no-output operator.

### Key Referencing

Keys in maps and map operators can be referenced as variables after the `,` at the end of their value expression:

```lr
options { _<@`searchPrivateIps`!: entities, entities removePrivateIps }
```

**Exception**: If the key is itself an expression (not a simple identifier), the operator returns what is to the right or executes a map operator to the right. In that case, the key does not become a variable binding.

## General Semantics

- **Data first**: Data appears first/on the left in expressions
- **Left-hungry curried**: Operators eagerly consume left operand
- **Ticking**: Evaluation proceeds through expression, operators "tick" to next item
- **Unexecuted operators**: Map/string operators remain unexecuted until in expression with needed input
- **Input type based execution behavior**: Operators have invariant representation — the same operator symbol adapts its behavior based on input types (e.g., `+` adds numbers, concatenates strings, spreads maps; `-` subtracts numbers, omits map keys, removes list items; `/` divides numbers, casts types)
- **Simple semantics, massive expressiveness**: The operator SDK combines a small set of symbols with semantic associations to create a fully expressive point-free functional system

# Line-by-Line Translation Reference

## integration.lr (52 lines) → integration.js

### Line 1-3
```lr
{
  +: imports@`polarity-integration-utils`@&{
    logging: [`setLogger`, `getLogger`],
    errors: [`parseErrorToReadableJson`]
  },
```
**Translation**:
```js
const {
  logging: { setLogger, getLogger },
  errors: { parseErrorToReadableJson }
} = require('polarity-integration-utils');
```

**Semantics**: Namespaced import from npm package. `@&{}` destructures nested object. `+:` spreads into scope.

### Line 4-5
```lr
  +: files@`./server/dataTransformations'@&[`removePrivateIps`,`getEntityTypes`,`addIdsToEntities`], ```@& is kind of like pick in lodash/fp
```
**Translation**:
```js
const {
  removePrivateIps,
  getEntityTypes,
  addIdsToEntities,
} = require('./server/dataTransformations');
```

**Semantics**: Local file import with array destructuring. Triple backtick at end is inline comment.

### Line 6
```lr
  +: files@`./server/queries`@&[`queryIssues`, `queryVulnerabilities`, `queryAssets`],
```
**Translation**:
```js
const { queryIssues, queryVulnerabilities, queryAssets } = require('./server/queries');
```

**Semantics**: Another array-destructured local import.

### Line 8
```lr
  assembleLookupResults: files@`./server/assembleLookupResults`,
```
**Translation**:
```js
const assembleLookupResults = require('./server/assembleLookupResults');
```

**Semantics**: Single function import without destructuring (no `+:` prefix).

### Line 9
```lr
  doLookup: { entities: _<@0, _options: _<@1, callback: _<@2,
```
**Translation**:
```js
const doLookup = async (entities, _options, cb) => {
```

**Semantics**: Function definition with positional arguments. `_<@N` binds N-th argument. Function is async (uses callback pattern, parallel execution with `$|||`).

### Line 10
```lr
    try: {
```
**Translation**:
```js
  try {
```

**Semantics**: Try block as map key.

### Line 11-14
```lr
      options: {
        +: _options,
        maxConcurrent: 1,
        minimumMillisecondsRequestWillTake: 350,
        parsedAssetQueryTypes: _options@`assetQueryTypeList` $@ `value`
      },
```
**Translation**:
```js
    const options = {
      ..._options,
      maxConcurrent: 1,
      minimumMillisecondsRequestWillTake: 350,
      parsedAssetQueryTypes: _options.assetQueryTypeList.map(asset => asset.value)
    };
```

**Semantics**: Object creation with spread. `_options@`assetQueryTypeList`` accesses property. `$@` maps list. `value` is literal string used to get property from each item.

### Line 15-16 (implied in JS)
```lr
      Logger: getLogger(),

      searchableEntities: options {
        _<@`searchPrivateIps`!: entities,
        entities removePrivateIps
      },
```
**Translation**:
```js
    Logger.debug({ entities }, 'Entities');

    let searchableEntities = entities;

    if(!options.searchPrivateIps){
      searchableEntities = removePrivateIps(entities);
    }
```

**Semantics**: `Logger` creation (omitted in .lr, inferred). Map operator as ternary: `{ condition!: valueIfTrue, valueIfFalse }`. `!` negates boolean.

### Line 18
```lr
      entitiesWithIds: searchableEntities addIdsToEntities,
```
**Translation**:
```js
    const entitiesWithIds = addIdsToEntities(searchableEntities);
```

**Semantics**: Function call syntax: left operand passed as first argument.

### Line 19
```lr
      cveEntities: [`cve`, entitiesWithIds] getEntityTypes,
```
**Translation**:
```js
    const cveEntities = getEntityTypes('cve', entitiesWithIds);
```

**Semantics**: Array arguments to function: array elements passed as positional arguments.

### Line 21-26
```lr
      +: options $|||[
        { _<@`queryIssues`: [cveEntities, _<] queryIssues, [] },
        { _<@`queryVulnerabilities`: [cveEntities, _<] queryVulnerabilities, [] },
        { _<@`queryAssets`: [entitiesWithIds, _<] queryAssets, [] }
      ],
```
**Translation**:
```js
    const [issues, vulnerabilities, assets] = await Promise.all([
      options.queryIssues ? queryIssues(cveEntities, options) : [],
      options.queryVulnerabilities ? queryVulnerabilities(cveEntities, options) : [],
      options.queryAssets ? queryAssets(entitiesWithIds, options) : []
    ]);
```

**Semantics**: Parallel execution with `$|||` operator (Promise.all). Each ternary map: `{ property: valueIfTruthy, valueIfFalsy }`. `<` at end of list destructures result into variables.

### Line 27
```lr
      _: { issues:issues, vulnerabilities:vulnerabilities, assets:assets } (Logger@`trace`)
```
**Translation**:
```js
    Logger.trace({ issues, vulnerabilities, assets });
```

**Semantics**: Underscore key = no-output operator. Executes Logger@`trace` (Logger.trace) with map argument.

### Line 28
```lr
      lookupResults: [entities, issues, vulnerabilities, assets, options] assembleLookupResults,
```
**Translation**:
```js
    const lookupResults = assembleLookupResults(
      entities,
      issues,
      vulnerabilities,
      assets,
      options
    );
```

**Semantics**: Function call with array arguments.

### Line 29
```lr
      [null, lookupResults] callback
```
**Translation**:
```js
    cb(null, lookupResults);
```

**Semantics**: Callback invocation (side effect). Array passed as positional arguments.

### Line 30
```lr
    },
```
**Translation**:
```js
  } catch (error) {
```

**Semantics**: End of try block (implicit start of catch in JS).

### Line 31-34
```lr
    catch: { error: _<,
      err: error parseErrorToReadableJson,
      _: { error:error, formattedError: err } (Logger@`error`)
      [{ detail: error.message || 'Lookup Failed', err }] callback
    }
```
**Translation**:
```js
    const err = parseErrorToReadableJson(error);

    Logger.error({ error, formattedError: err }, 'Get Lookup Results Failed');
    cb({ detail: error.message || 'Lookup Failed', err });
```

**Semantics**: Catch block. `error: _<` binds error to left arg. Underscore operator for logging. Callback with error object.

### Line 36-38
```lr
  },

  startup: setLogger,
  validateOptions: validateOptions
}@&[`startup`,`validateOptions`,`doLookup`]
```
**Translation**:
```js
};

module.exports = {
  startup: setLogger,
  validateOptions,
  doLookup
};
```

**Semantics**: `startup: setLogger` assigns function reference. `validateOptions: validateOptions` assigns function reference. `}@&[...]` exports listed keys from enclosing map.

## request.lr (131 lines) → request.js

### Line 1-5
```lr
{
  +: imports@`lodash/fp`@&[
    `map`,
    `get`,
    `getOr`,
    `filter`,
    `flow`,
    `negate`,
    `isEmpty`,
    `size`
  ],
```
**Translation**:
```js
const { map, get, getOr, filter, flow, negate, isEmpty, size } = require('lodash/fp');
```

**Semantics**: Flat array import (no namespace) from npm package.

### Line 7
```lr
  +: imports@`async`@&[`parallelLimit`],
```
**Translation**:
```js
const { parallelLimit } = require('async');
```

**Semantics**: Simple array import.

### Line 9-11
```lr
  +: imports@`polarity-integration-utils`@&{
    requests: [`createRequestWithDefaults`]
  },
```
**Translation**:
```js
const {
  requests: { createRequestWithDefaults }
} = require('polarity-integration-utils');
```

**Semantics**: Namespaced import with array destructuring.

### Line 13-18
```lr
  config: files@`../config/config`,
  config@`request`: {
    cert: ``
    key: ``
    passphrase: ``
    ca: ``
    proxy: ``
  },
```
**Translation**:
```js
const config = require('../config/config');

config.request = {
  cert: '',
  key: '',
  passphrase: '',
  ca: '',
  proxy: ''
};
```

**Semantics**: Import config object, then mutate property on it (`config@`request`` = access, assign to it).

### Line 20-22
```lr
  NodeCache: imports@`node-cache`,
  tokenCache: NodeCache [{
    stdTTL: 23 * 60 * 60
  }],
```
**Translation**:
```js
const NodeCache = require('node-cache');
const tokenCache = new NodeCache({
  stdTTL: 23 * 60 * 60
});
```

**Semantics**: Class/constructor import. Instantiation with map argument.

### Line 24-26
```lr
  requestForAuth: {
    config:config,
    roundedSuccessStatusCodes: [200],
    requestOptionsToOmitFromLogsKeyPaths: [`form.client_secret`]
  } createRequestWithDefaults,
```
**Translation**:
```js
const requestForAuth = createRequestWithDefaults({
  config,
  roundedSuccessStatusCodes: [200],
  requestOptionsToOmitFromLogsKeyPaths: ['form.client_secret']
});
```

**Semantics**: Map passed as last argument to function call (curried pattern).

### Line 28-42
```lr
  requestWithDefaults: {
    config:config,
    roundedSuccessStatusCodes: [200],
    useLimiter: true,
    requestOptionsToOmitFromLogsKeyPaths: [`authorization`, `query`],
    preprocessRequestOptions: { options: _<@0, query: _<@1,
      token: [options] getToken \\,

      {
        method: `POST`,
        url: `https://api.{options@`apiRegion`}.app.wiz.io/graphql`,
        headers: {
          accept: `application/json`,
          authorization: `Bearer {token}`,
          content-type: `application/json`
        },
        body: { query } /"
      } ///,
```
**Translation**:
```js
const requestWithDefaults = createRequestWithDefaults({
  config,
  roundedSuccessStatusCodes: [200],
  useLimiter: true,
  requestOptionsToOmitFromLogsKeyPaths: ['authorization', 'query'],
  preprocessRequestOptions: async ({ options, query }) => {
    const token = await getToken(options);

    return {
      method: 'POST',
      url: `https://api.${options.apiRegion}.app.wiz.io/graphql`,
      headers: {
        accept: 'application/json',
        authorization: `Bearer ${token}`,
        'content-type': 'application/json'
      },
      body: JSON.stringify({ query })
    };
  },
```

**Semantics**: `///` starts async context (operator resolved on different thread or concurrently). `\\\` awaits the result of an async call. Template interpolation with `{var}`. `/"` converts to string (JSON.stringify).

### Line 44-48
```lr
    postprocessRequestResponse: { response: _<,
      jsonResponseBody:response@`body` /json
      bodyErrors: jsonResponseBody@[`errors`],
      bodyErrors #: {
```
**Translation**:
```js
  postprocessRequestResponse: async (response) => {
    const bodyErrors = flow(get('body'), JSON.parse, get('errors'))(response);
    if (size(bodyErrors)) {
```

**Semantics**: Async function. `response@`body`` gets property. `/json` parses JSON. `@[`errors`]` bracket access with array literal. `#` gets size, acts as conditional (if > 0).

### Line 49-51
```lr
        bodyError: Error[bodyErrors@[0,`message`]],
        bodyError@`status`: response@`status`,
        bodyError !!!
```
**Translation**:
```js
      const bodyError = new Error(get('0.message', bodyErrors));
      bodyError.status = response.status;

      throw bodyError;
    }
```

**Semantics**: `Error[expression]` constructor call. `@` property assignment. `!!!` throw.

### Line 52
```lr
      response + { body: jsonResponseBody }
```
**Translation**:
```js
    return {
      ...response,
      body: JSON.parse(response.body)
    };
```

**Semantics**: `+` spread/merge maps. Implicit return from async function (last expression).

### Line 54-56
```lr
    },
    postprocessRequestFailure: { error: _<,
      error@`message`: `{error@`message`} - ({error@`status`}) | {error@`description`}`,
      error !!!
```
**Translation**:
```js
  },
  postprocessRequestFailure: (error) => {
    error.message = `${error.message} - (${error.status}) | ${error.description}`;

    throw error;
  }
});
```

**Semantics**: Error handling with template interpolation. `|` is literal in string (not operator).

### Line 57-61
```lr
  token: {
    method: `POST`,
    url: `https://{options@`authTokenDomain`}/oauth/token`,
```
**Translation**:
```js
const token = get(
  'body.access_token',
  await requestForAuth({
    method: 'POST',
    url: `https://${options.authTokenDomain}/oauth/token`,
```

**Semantics**: `\\\ @[`body`,`access_token`]` async call then path access. Template interpolation.

### Line 65-67
```lr
      headers: {
        accept: `application/json`,
        content-type: `application/x-www-form-urlencoded`
      },
```
**Translation**:
```js
    headers: {
      accept: 'application/json',
      'content-type': 'application/x-www-form-urlencoded'
    },
```

**Semantics**: Object with string keys and string values.

### Line 68-71
```lr
      form: {
        grant_type: `client_credentials`,
        audience: `wiz-api`,
        client_id: options@`clientId`,
        client_secret: options@`clientSecret`
      },
```
**Translation**:
```js
    form: {
      grant_type: 'client_credentials',
      audience: 'wiz-api',
      client_id: options.clientId,
      client_secret: options.clientSecret
    },
```

**Semantics**: Nested object with property access (`options@`key``).

### Line 72-74
```lr
      json: true
    } requestForAuth \\\ @[`body`,`access_token`],
```
**Translation**:
```js
    json: true
  })
);

```

**Semantics**: `\\\` makes async call. `@[`body`,`access_token`]` gets nested property path. Implicit return from async function.

### Line 76
```lr
    tokenCache set [tokenCacheKey, token],
```
**Translation**:
```js
tokenCache.set(tokenCacheKey, token);
```

**Semantics**: Method call with array arguments (space-separated key then array).

### Line 78
```lr
    token
```
**Translation**:
```js
return token;
```

**Semantics**: Last expression is implicit return.

### Line 80
```lr
  } ///,
```
**Translation**:
```js
};
```

**Semantics**: `///` marks function boundary as async (operator resolved on different thread or concurrently).

### Line 82-95
```lr
  createRequestsInParallel: { requestWithDefaults: _<,
    { requestOptions: _<@0,
      responseGetPath: _<@1,
      limit: _<@2 | 10,
      onlyReturnPopulatedResults: _<@3 | true,

      unexecutedRequestFunctions: requestOptions ${
        entity: _<@`entity`,
        requestOptions: _< - `entity`,
        {
          response: requestOptions requestWithDefaults \\\,
```
**Translation**:
```js
const createRequestsInParallel =
  (requestWithDefaults) =>
  async (
    requestOptions,
    responseGetPath,
    limit = 10,
    onlyReturnPopulatedResults = true
  ) => {
    const unexecutedRequestFunctions = map(
      ({ entity, ...requestOptions }) =>
        async () => {
          const response = await requestWithDefaults(requestOptions);
```

**Semantics**: Curried function. Default parameters (`| 10`, `| true`). `$` maps list. `_< - `entity`` omits entity from left arg.

### Line 96-97
```lr
          result: responseGetPath? {_<: response@responseGetPath, response },
          entity? {_<: { entity:entity, result:result }, result}
        } ///
```
**Translation**:
```js
          const result = responseGetPath ? get(responseGetPath, response) : response;
          return entity ? { entity, result } : result;
        },
```

**Semantics**: Ternary with early return (`?`). `{_<: trueValue, falseValue}` ternary syntax.

### Line 99-100
```lr
      },

      results: [unexecutedRequestFunctions, limit] parallelLimit \\\,
```
**Translation**:
```js
      requestOptions
    );

    const results = await parallelLimit(unexecutedRequestFunctions, limit);
```

**Semantics**: Array arguments to function. `\\\` async call.

### Line 102-105
```lr
      onlyReturnPopulatedResults? {_<:
        results $?{ _<@`result` | _< ?_ ! },
        results
      }
```
**Translation**:
```js
    return onlyReturnPopulatedResults
      ? filter(
          flow((result) => getOr(result, 'result', result), negate(isEmpty)),
          results
        )
      : results;
```

**Semantics**: Ternary with early return (`?`). `$?` filters list. `{ _<@`result` | _< ?_ ! }` ternary map: `?!` negates boolean (filters where falsey). `|` OR in ternary.

### Line 107
```lr
    } ///
```
**Translation**:
```js
};
```

**Semantics**: Close async function.

### Line 109
```lr
  },

  requestsInParallel: [requestWithDefaults] createRequestsInParallel
```
**Translation**:
```js
};

const requestsInParallel = createRequestsInParallel(requestWithDefaults);
```

**Semantics**: Partial application (wrap in array for first arg, then call function).

### Line 111
```lr
}@&[`requestWithDefaults`,`requestsInParallel`]
```
**Translation**:
```js
module.exports = {
  requestWithDefaults,
  requestsInParallel
};
```

**Semantics**: Export declaration.

## Additional Observations from SOT Files

### Key Referencing Patterns

**Keys as variables after comma**:
```lr
options { _<@`searchPrivateIps`!: entities, entities removePrivateIps }
```

The key `searchPrivateIps` is accessed via `_<@` then used in ternary expression.

### Trailing Commas

Trailing commas are allowed and common:
```lr
{
  key: value,
  anotherKey: anotherValue,
}
```

### Newline Rules

Expressions can span multiple lines. Map entries can be on separate lines.

### Property Access Patterns

- `_options@`assetQueryTypeList`` - Get property from variable
- `response@`body`` - Get nested property
- `options@`request`` - Get then assign

### Method Call Patterns

- `[tokenCacheKey, token] tokenCache set` - Method call with array args
- `[options] getToken \\\` - Async call

### Constructor Patterns

- `Error[expression]` - Constructor with expression argument
- `NodeCache [{ stdTTL: 23 * 60 * 60 }]` - Constructor with map argument

### Default Parameter Pattern

`limit: _<@2 | 10` → `limit = 10` (default if undefined)

### Partial Application Pattern

`[requestWithDefaults] createRequestsInParallel` → `createRequestsInParallel(requestWithDefaults)`

### Destructuring Patterns

- `_<@`entity`` - Destructure named property
- `_< - `entity`` - Omit key from map
- `{ entity: _<@`entity`, ... }` - Destructure with rest

### Negation Patterns

- `!` as prefix - Boolean NOT
- `!:` in ternary - If NOT (negated condition)
- `? !` - Negate boolean result
- `?!` - Negate boolean (compound operator)

### Underscore Execution

`_: expression (Logger@`method`)` - Execute expression without output, then call Logger method

### Async Patterns

- `///` - Takes an operator on the left and makes it async/concurrent (resolved on different thread or like a promise)
- `\\\` - Await (resolves the async operator on the left and returns the result)
- `func \\\` - Await call to function
- `[arg] func \\\` - Await call with argument

### Array Bracket Access

`@[`errors`]` - Bracket access with array literal (get 'errors' property)

`@`body` /json` - Get property then parse JSON

### Ternary Syntax Patterns

1. **Map as ternary**: `{ condition: valueIfTruthy, valueIfFalsy }`
2. **Negated ternary**: `{ condition!: valueIfTrue, valueIfFalse }`
3. **Explicit ternary**: `{_<: trueValue, falseValue}`
4. **Early return ternary**: `condition? {_<: trueValue, falseValue }`

### Operator Composition

`response@`body` /json` → Get property then convert to JSON

Operators chain left-to-right, each consuming output of previous.

### Loop Operator Patterns

- `list $@ key` → Get property from each item
- `list $? condition` → Filter list
- `_options@`assetQueryTypeList` $@ value` → Map list, get property from each

### Spread Patterns

- `+: _options` → Spread into parent map
- `response + { body: ... }` → Merge/extend map
- `[arg1, arg2] func` → Spread array as positional args

### Size as Conditional

`bodyErrors #: { ... }` → If size(bodyErrors) > 0, execute block

The `#` operator returns size, and when used as key in map, acts as conditional (truthy if size > 0).

### Exception Throwing

`bodyError !!!` → `throw bodyError`

Triple exclamation throws the value.

### Template String Interpolation

`Bearer {token}` → `Bearer ${token}`

Variables interpolated with `{var}` inside backtick strings. Can contain Left-Right expressions.

### Boolean Conversion

`cachedToken?: cachedToken` → `if (cachedToken) return cachedToken`

The `?` toBoolean operator checks truthiness and `:` triggers conditional return if truthy.

### Callback Pattern

`[null, lookupResults] callback` → `callback(null, lookupResults)`

Array passed as positional arguments to function (side effect, no return).

### Object Creation Pattern

```lr
key: {
  +: spreadValue,
  key1: value1,
  key2: value2
}
```

Creates new object with spread and additional properties.

### Path Access Pattern

`@[`body`,`access_token`]` → Get nested property path (get('body.access_token'))

Comma-separated path in brackets accesses nested properties.

---

This specification is derived solely from the provided SOT (.lr) files and their corresponding JS translations. Every construct, operator, and pattern documented here is directly observable in the source material.
