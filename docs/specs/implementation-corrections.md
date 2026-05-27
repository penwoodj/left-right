# Implementation Corrections & Design Rules

Derived from source-of-truth translation files (`docs/translations/javascript/`) and author clarifications.

## Core Rules

### 1. Strict Left-to-Right — No Operator Precedence

All expressions evaluate left-to-right. No operator has higher precedence than another.

```
1 + 2 * 3   →   (1 + 2) * 3   →   9
```

Not `7`. There is no PEMDAS. There is no precedence table. Left-to-right, period.

### 2. No Unary Negation

`-5` is invalid. Negation is diadic: `0 - 5`.

Unary `-` does not exist as an operator. The `-` symbol is only ever diadic (subtraction/omit/remove).

### 3. `_<` Means "Value From the Left"

`_<` is not a generic placeholder — it means "the value that was passed in from the LEFT of this operator."

- `5 { _< + 1 }` → `_<` = 5 → result 6 ✓
- `{ _< + 1 } 5` → nothing is to the LEFT of this closure → `_<` is undefined → error

This applies to ALL closures containing `_<`. The closure must have data to its left for `_<` to bind.

### 4. `_>` Means "Value From the Right"

`_>` means "the value that was passed in from the RIGHT of this operator."

- `3 { _< + _> } 4` → `_<` = 3, `_>` = 4 → result 7 ✓
- `{ _< + _> } 3 4` → nothing to LEFT of closure → error (same as rule 3)

Diadic closures require BOTH left and right context. The operator must be between two values.

### 5. Data-First Evaluation

The fundamental evaluation rule: **data appears first (on the left)**, then the operator.

```
entities removePrivateIps        →   removePrivateIps(entities)
searchableEntities addIdsToEntities  →   addIdsToEntities(searchableEntities)
[`cve`, entitiesWithIds] getEntityTypes  →   getEntityTypes('cve', entitiesWithIds)
```

Source: `lookup-manual-translation.lr` lines 24, 26, 27.

### 6. Program Maps: Data-First Final Expression

In program maps (maps where the last entry is an expression, not a key-value pair), the final expression follows data-first ordering.

```
{ double: { _< * 2 }, 7 double }    →   14   ✓  data first
{ double: { _< * 2 }, double 7 }    →   ERROR     operator first — wrong
```

The data (`7`) comes first, then the operator (`double`).

## The `+` Operator — Polymorphic

`+` is a single operator with type-dependent behavior. There is no `++` operator.

### Number + Number → Addition
```
5 + 3       →   8
23 * 60 * 60   →   82800
```

### String + String → Concatenation
```
options@`clientId` + options@`clientSecret`   →   concatenated string
```

### String + Anything (or Anything + String) → toString + Concatenation
If **either** operand is a string, the other is converted via toString and concatenated.
```
`hello` + 5      →   `hello5`
5 + `hello`      →   `5hello`
`count: ` + 3    →   `count: 3`
```

### List + List → Concatenation
```
[1, 2] + [3, 4]   →   [1, 2, 3, 4]
```

### Number + List (or List + Number) → Flatten In
```
1 + [2, 3]      →   [1, 2, 3]       ← prepends
[1, 2] + 3      →   [1, 2, 3]       ← appends
```

The non-list value is placed directly as an element (flattened in), not wrapped.

### Map + Map → Merge (Object.assign)
```
response + { body: jsonResponseBody }   →   { ...response, body: jsonResponseBody }
```

Source: `async-http-manual-translation.lr` line 67.

### `+:` Spread Into Context
```
+: _options           →   spread _options into current map
+: imports@`pkg`@&[...]  →   spread imported names into scope
```

## The `@` Operator — Get/Navigation

`@` is the get/navigation operator. It accesses properties/keys.

### Basic Property Access
```
options@`searchPrivateIps`        →   options.searchPrivateIps
response@`body`                   →   response.body
```

### Method Access + Call
```
tokenCache@`get`[tokenCacheKey]   →   tokenCache.get(tokenCacheKey)
```

### Bracket Access with Path
```
bodyErrors@[`errors`]            →   get('errors', bodyErrors)
bodyErrors@[0, `message`]        →   get('0.message', bodyErrors)
```

### Spacing
Both `options@`key`` and `options @ `key`` are valid.

### Map Access Syntax
To access a key from a map:
```
{ a: 1 } @ `a`    →   1
```

NOT `{ a: 1 } a`. The `@` operator is required for property access.

## The `_` Operator — Three Conditions

`_` has three distinct behaviors depending on context:

1. **`_: expression`** — No-output execution (side effect only)
   ```
   _: { issues:issues, ... } (Logger@`trace`)
   ```
   Source: `lookup-manual-translation.lr` line 35.

2. **`_<` or `_>`** — Argument operators (no space between `_` and `<`/`>`)
   ```
   _<    →   value from the left
   _>    →   value from the right
   ```
   These are reserved tokens, never parsed as `_` followed by `<`/`>`.

3. **Compound operators** — `_` immediately after another operator symbol (no space)
   ```
   $_    →   flatmap
   $>    →   groupBy
   $_<   →   NOT a compound — this is `_` then `_<` (space matters)
   ```

## The `-` Operator — Subtract/Omit/Remove

Like `+`, `-` is type-dependent:
- Number - Number → subtraction
- Map - String → omit that key
- List - Value → remove that value

Source: Spec line 960: "`-` subtracts numbers, omits map keys, removes list items"

Example from SOT:
```
requestOptions: _< - `entity`    →   omit `entity` key from left arg (destructuring omit)
```
Source: `async-http-manual-translation.lr` line 110.

## Calling Convention Patterns (from SOT)

### Function Call (data-first, single arg)
```
entities removePrivateIps              →   removePrivateIps(entities)
```

### Function Call (multiple args via list)
```
[`cve`, entitiesWithIds] getEntityTypes   →   getEntityTypes('cve', entitiesWithIds)
```

### Method Call
```
tokenCache set [tokenCacheKey, token]   →   tokenCache.set(tokenCacheKey, token)
```

### Constructor
```
NodeCache [{ stdTTL: 23 * 60 * 60 }]   →   new NodeCache({ stdTTL: 23 * 60 * 60 })
```

### Partial Application
```
[requestWithDefaults] createRequestsInParallel   →   createRequestsInParallel(requestWithDefaults)
```

## Things That Do NOT Exist

| Non-feature | Why |
|---|---|
| Unary negation (`-5`) | `-` is diadic only. Use `0 - 5`. |
| `++` operator | `+` handles all concatenation/merging. |
| Operator precedence | Strict left-to-right. All operators same precedence. |
| Prefix closure calls (`{ _< + 1 } 5`) | `_<` requires data on the left. |
| Bare map access (`{ a: 1 } a`) | Must use `@` operator: `{ a: 1 } @ `a`` |

## Source of Truth Files

These files are the authoritative reference. When in doubt, check these:

1. `docs/translations/javascript/lookup-manual-translation.lr` — 47 lines
2. `docs/translations/javascript/async-http-manual-translation.lr` — 128 lines
3. `docs/translations/javascript/lookup.js` — 70 lines (JS equivalent)
4. `docs/translations/javascript/async-http.js` — 138 lines (JS equivalent)

The language specification at `docs/specs/left-right-language-specification.md` is derived from these files. If there is any conflict, the translation files win.
