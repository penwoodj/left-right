# Left-Right Corrections Addendum

**Read this BEFORE the 8 research reports.** The reports are language-agnostic compiler research. This document corrects them for Left-Right's unique design.

---

## 1. String Interpolation Syntax

**Reports say:** `${expr}` or `${name}` JavaScript-style interpolation inside strings.

**Left-Right reality:** Backtick strings use `{expr}` — curly braces ONLY, no dollar sign.

```lr
`Hello {name}, you are {age} years old`
```

Any single-line expression allowed inside `{}`. Result is auto-toString'd at runtime. Nesting allowed — inner interpolated string returning an operator displays as Left-Right code string.

---

## 2. Lexer: ALL Operators Are Identifiers

**Reports say:** Lexer should distinguish operators from identifiers (e.g., `Operator(String)` token kind).

**Left-Right reality:** The lexer does NOT distinguish operators from identifiers. `+`, `@`, `><`, `$@`, `!!!` are ALL `Identifier` tokens. Operator semantics emerge at runtime based on value types, not at lex/parse time.

**Token kinds should be:**
- `Identifier` — any sequence of non-reserved characters (includes all operators)
- `Number` — decimal integer or float
- `String` — backtick-delimited with optional `{expr}` interpolation
- `LBrace`, `RBrace` — `{}`
- `LBracket`, `RBracket` — `[]`
- `LParen`, `RParen` — `()`
- `Colon` — `:`
- `Comma` — `,`
- `Dot` — `.` (reverse-args operator, single-char token)
- `Quote` — `'` (reserved, unused)
- `Backtick` — string delimiter
- `LeftArg` — `_<` (2-char token)
- `RightArg` — `_>` (2-char token)
- `EOF`

**Maximal munch rule:** `!!!?` is ONE identifier (not `!!!` + `?`). `///` is ONE identifier. `\\\` is ONE identifier. `><` is ONE identifier. `$@` is ONE identifier. No whitespace between chars = single identifier.

---

## 3. AST: No Traditional Control Flow Nodes

**Reports say:** AST should include `If`, `Function`, `Loop`, `Break`, `Continue` nodes.

**Left-Right reality:** NONE of these exist. Left-Right has:
- **Maps** for functions, conditionals, and control flow
- **Operators** for iteration (`$`, `$@`, `$?`, `$_`, etc.)
- **No keywords** — no `if`, `else`, `while`, `for`, `return`, `break`, `continue`

**Actual AST node types:**

```
Number(f64)
String(String)                    // backtick string, possibly with interpolation
Boolean(bool)
Undefined
Identifier(String)                // includes ALL operators
List(Vec<Expr>)                   // [a, b, c]
Map(Vec<(Expr, Expr)>)            // {key: value, key: value}
Apply { func: Expr, arg: Expr }   // left-to-right curried application
Grouped(Expr)                     // parenthesized expression
LeftArg                           // _< token
RightArg                          // _> token
```

**Map-as-function:** `{ arg: _<@0, body }` — map with `_<` references inside = unexecuted operator at runtime.

**Map-as-conditional:** `{ _<: trueCase, falseCase }` — expression key `_<` evaluates truthiness, `:` returns if truthy, falls through if not.

**Map-as-loop:** Uses iteration operators like `$`, `$?`, `$_` — these are identifiers with runtime semantics, not AST nodes.

---

## 4. Evaluation: Zero Precedence, Left-to-Right Curried

**Reports say:** Some examples show traditional precedence (`1 + 2 * 3 = 7`).

**Left-Right reality:** `1 + 2 * 3` = `((1 + 2) * 3)` = `9`. Always left-to-right. No exceptions.

**Curried semantics:** `5 + 3` means:
1. Evaluate `5` → Number(5)
2. Encounter `+` → CurriedOperator(+, left=5, waiting for right)
3. Encounter `3` → Apply: 5 + 3 = 8

**`5 +` at end of expression** = partially applied "add 5" operator (closure).

---

## 5. The `.` Dot Operator

**Reports say:** `.` is a property access operator like JavaScript.

**Left-Right reality:** `.` is the **reverse-args operator**. It takes an unexecuted operator on its LEFT and returns a new unexecuted operator with left/right slots SWAPPED.

```lr
`key`@.data
```
This means: `key` string → `@` (curried get) → `.` reverses → data flows in from left as the map.

Everything still evaluates left-to-right. The `.` just swaps which slot the next value fills.

---

## 6. Colon `:` Disambiguation

**Reports say:** `:` is a key-value separator in maps.

**Left-Right reality:** `:` has TWO behaviors based on key type:
- **Alpha-start key** (`name: expr`) → assignment: creates variable AND key-value pair
- **Expression key** (`_<: expr` or `expr: expr`) → early return: if key evaluates truthy, immediately return the value, skip remaining map entries

---

## 7. Numbers: Decimal Only

**Reports say:** Numbers are IEEE 754 doubles with NaN, Infinity, hex, octal, scientific notation.

**Left-Right reality:** Numbers are **decimal only**. No hex, binary, octal, or scientific notation. Floats must start with a digit (`0.5` valid, `.5` invalid). No negative literals — `-` is always a binary operator (`0-5` for negative 5).

---

## 8. Imports: Runtime Variable, Not Keyword

**Reports say:** `import` keyword or static import resolution.

**Left-Right reality:** `imports` is a **runtime variable** (a map), not a keyword. Import syntax:
```lr
imports@[`lodash`, `fp`, `map`]
```
This uses `@` get operator with a LIST of strings for nested path access. `files@[`path`]` for local file imports.

No static import analysis possible. Circular dependencies handled at runtime.

---

## 9. Exports: End-of-File Pattern

**Reports say:** `export` keyword.

**Left-Right reality:** Exports use the pattern `}@&[...]` at end of file:
- `}` = closing brace of the root map
- `@` = get operator
- `&` = pick operator
- `[...]` = list of string keys to export

No `export` keyword. Parser recognizes this as a specific end-of-file pattern.

---

## 10. Error Handling: Identifiers, Not Keywords

**Reports say:** `try`/`catch` keywords or special AST nodes.

**Left-Right reality:**
- `!!! expr` = throw expr as error (`!!!` is an identifier)
- `!!!?` = catch operator (single identifier, NOT `!!!` + `?`)
  - Takes unexecuted operator on left
  - Right side: empty map = swallow error, or operator = catch body
  - Returns wrapped operator

`try` and `catch` are identifiers the runtime recognizes in map context — NOT keywords.

---

## 11. Async/Await: Identifiers

**Reports say:** `async`/`await` keywords.

**Left-Right reality:**
- `///` takes operator on left, returns async operator (single identifier)
- `\\\` takes promise on left, waits for resolution (single identifier)

No keywords. These are identifiers with runtime semantics.

---

## 12. No Type Annotations or Declarations

**Reports say:** Type declarations, type signatures, type annotations.

**Left-Right reality:** No type syntax exists. All 7 types (Operator, Map, List, String, Boolean, Number, Undefined) are inferred at runtime. Compiler cannot rely on explicit type information.

Type-check operators (`?"` isString, `?#` isNumber, `?><` contains) are identifiers used in map expression keys for conditional logic.

---

## 13. No Assignment Operator `=`

**Reports say:** Variable assignment with `=`.

**Left-Right reality:** `=` is the **equality operator** (identifier), NOT assignment. Assignment happens via map key `:` with alpha-start keys. Spread/merge uses `+:` (two tokens: identifier `+` + delimiter `:`).

---

## 14. Silent Execution and Spread

**Reports miss:** These important patterns:

- `_: expr` = execute silently, discard result, return undefined (two tokens: `_` + `:`)
- `+: expr` = spread/merge: add all key-values from right to left map (two tokens: `+` + `:`)
- `_: expr` and `+: expr` are NOT composite tokens — always two separate tokens

---

## 15. JIT vs AOT

**Reports say:** JIT compilation strategies relevant.

**Left-Right reality:** LR is an **AOT transpiler** targeting JavaScript and Rust. JIT is irrelevant. Primary compilation pipeline:
```
Source → Tokens → AST → Typed AST → IR → Optimized IR → Target Code
```

No runtime compilation needed. The transpiler outputs target language source code.

---

## 16. Comments

**Reports miss:** Comments use triple-backtick ` ``` ` prefix to end of line. Single-line only. Lexer must distinguish triple-backtick at line start (comment) from single backtick (string delimiter).

---

## 17. Every File = Single Root Expression

**Reports miss:** Every `.lr` file is exactly ONE expression. No separate import/body/export sections. The entire file is parsed as a single curried left-to-right chain. `package.lr` defines entry points via `scripts` map key.

---

## Quick Reference: Operator Token Corrections

| What reports say | Left-Right reality |
|---|---|
| `Operator(String)` token kind | All operators are `Identifier` tokens |
| `${expr}` interpolation | `{expr}` interpolation (no dollar sign) |
| `If` AST node | Maps with expression keys |
| `Function` AST node | Maps with `_<` references |
| `Loop`/`Break` AST nodes | Iteration operators (identifiers) |
| `import` keyword | `imports` runtime variable + `@` operator |
| `export` keyword | `}@&[...]` pattern |
| `try`/`catch` keywords | `!!!`/`!!!?` identifiers |
| `async`/`await` keywords | `///`/`\\\` identifiers |
| `=` assignment | `=` is equality, `:` is assignment |
| IEEE 754 numbers | Decimal only, no hex/scientific |
| `.` property access | `.` reverse-args operator |
| `_<` two tokens | `_<` single 2-char token |
| `_>` two tokens | `_>` single 2-char token |
| `!!!?` two tokens | `!!!?` single identifier |
| `_: ` composite | `_: ` two separate tokens |
| `+:` composite | `+:` two separate tokens |
| `?` operator | `?` IS a toBoolean operator (Operator SDK) |
