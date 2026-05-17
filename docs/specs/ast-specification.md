# Left-Right Language — AST Specification

> **Status**: CONFIRMED — Based on language designer Q&A (Q1-Q16), primary source `manual-language-summary.md`, and SOT files (`wiz/integration.lr`, `wiz/server/request.lr`).

---

## 1. Overview

The Left-Right AST represents the syntactic structure of a `.lr` source file. It is the output of the parser and input to the runtime evaluator or code generator.

### 1.1 Core Design Principles

1. **Everything is an expression** — No statement/expression distinction. Every construct evaluates to a value.
2. **Single root expression** — Every `.lr` file is ONE expression. The AST has a single root node.
3. **Left-to-right curried evaluation** — No operator precedence. All operators are curried (take left arg, return unexecuted operator waiting for right arg). Parser reflects this as a flat left-to-right chain.
4. **No keywords** — Only reserved symbols (`:`, `,`, `.`, `'`, `()`, `[]`, `{}`, `` ` ``, `_<`, `_>`). All operators are identifiers.
5. **Point-free style** — Functions use implicit argument binding via `_<` (left arg) and `_>` (right arg).
6. **Operators ARE identifiers** — The parser treats `+`, `@`, `!!!` etc. as identifier expressions. The runtime determines operator semantics.

### 1.2 Top-Level Structure

```
Program:
  expression: Expression       // The single root expression of the file
  source_path: string          // Path to the .lr file
```

**No separate import/export sections.** Imports and exports are expressions within the root expression, using `imports`/`files` runtime variables and `@`/`@&` operators.

---

## 2. Expression Node (Universal)

ALL AST nodes are expressions. There is no separate statement type.

```
Expression:
  | NumberLiteral
  | StringLiteral
  | BooleanLiteral
  | UndefinedLiteral
  | ListLiteral
  | MapLiteral
  | Identifier                    // includes ALL operators
  | LeftArg                       // _<
  | RightArg                      // _>
  | Application                   // left_expr operator_or_value (curried application)
  | GroupedExpression             // ( expr ) — overrides evaluation order
  | ImportExpression              // imports@`pkg` or files@`path`
  | ExportExpression              // }@&[`name1`, `name2`]
  | ThrowExpression               // expr !!!
  | CatchExpression               // operator !!!? handler
  | AsyncExpression               // operator ///
  | AwaitExpression               // promise \\\
  | StringInterpolation           // {expr} inside backtick string
```

---

## 3. Literal Nodes

### 3.1 NumberLiteral

```
NumberLiteral:
  value: number             // integer or float
  raw: string               // original text: "42", "3.14", "0.5"
```

- Decimal only. No hex, binary, octal, scientific.
- Must start with digit. `.5` is NOT valid — must be `0.5`.
- No negative literals. `-5` is Application(NumberLiteral(0), Identifier("-"), NumberLiteral(5)).

### 3.2 StringLiteral

```
StringLiteral:
  parts: StringPart[]       // sequence of text and interpolation parts

StringPart:
  | TextPart                  // plain text segment
  | InterpolationPart         // {expression} inside string

TextPart:
  text: string

InterpolationPart:
  expression: Expression     // the expression inside { }
```

- Backtick-delimited only.
- Multi-line supported (tab whitespace on continuation lines stripped at runtime).
- Only escape: `\`` (backslash escapes backtick).
- Interpolation: `{expr}` — any expression, result auto-toString'd.
- **Interpolation nesting IS allowed**: `` `{`{inner}`}` `` is valid. Inner expression evaluates first, result is toString'd into outer string.

### 3.3 BooleanLiteral

```
BooleanLiteral:
  value: bool                // true | false
  raw: string                // "true" or "false"
```

### 3.4 UndefinedLiteral

```
UndefinedLiteral:
  raw: string                // "undefined"
```

### 3.5 ListLiteral

```
ListLiteral:
  elements: Expression[]     // comma-separated expressions inside [ ]
```

- Delimited by `[` ... `]`.
- Elements separated by commas ONLY (not whitespace).
- Empty list: `[]` → ListLiteral with empty elements array.
- Unlimited nesting.

### 3.6 MapLiteral

```
MapLiteral:
  entries: MapEntry[]        // comma-separated entries inside { }

MapEntry:
  key: Expression            // key expression (identifier, string, or any expression)
  value: Expression          // value expression (after :)
  is_assignment: bool        // true if key starts with alpha character → creates variable
  is_expression_key: bool    // true if key is expression (not alpha-starting identifier)
```

- Delimited by `{` ... `}`.
- Entries separated by commas ONLY.
- **Colon (`:`) semantics depend on key type**:
- Key starts with alpha → **assignment**: creates variable AND key-value pair
- Key is `_` → **no-output execution**: evaluate value, discard result, return undefined. Multiple `_:` entries all execute, all silent.
- Key is expression/symbol (`_<`, operators, etc.) → **early return**: if key evaluates truthy, return value immediately
- For boolean control flow on variable keys: use `?` toBoolean operator before `:`, e.g., `{booleanKey?: `yes`, `no`}`
- **Map-as-operator detection** (runtime, not parser):
  - Contains `_<` or `_>` → unexecuted operator
  - Last entry has no `:` at top level → expression ending → operator
  - Expression keys with `:` → control flow → operator

---

## 4. Identifier and Argument Nodes

### 4.1 Identifier

```
Identifier:
  name: string               // the raw text: "+", "@", "hello", "!!!?", "$@"
```

This is the universal node for ALL operators and named values. The runtime resolves:
- `+` → add/concat operator
- `@` → get operator
- `hello` → variable lookup or function reference
- `!!!?` → catch operator
- `///` → async operator

### 4.2 LeftArg

```
LeftArg:
  raw: string                // always "_<"
```

Represents the whole left argument value. Inside an operator body, `_<` is substituted with the actual value passed from the left.

### 4.3 RightArg

```
RightArg:
  raw: string                // always "_>"
```

Represents the explicit right argument value. Used in diatic operator definitions: `{ _< + _> }` = left + right.

### 4.4 Argument Indexing (Parser-level construct)

`_<@0`, `_<@1`, etc. are NOT a single AST node. They parse as:

```
Application:
  left: LeftArg
  operator: Identifier("@")
  right: NumberLiteral(0)
```

i.e., `_<` @ `0` = get index 0 from the left argument (which must be a list).

---

## 5. Application Node (Core Evaluation)

### 5.1 Application

The **fundamental** AST node. Left-to-right curried evaluation means the entire program is a chain of applications.

```
Application:
  left: Expression           // the left operand (data or unexecuted operator)
  right: Expression          // the right operand (data or unexecuted operator)
```

**Evaluation rules**:
1. Evaluate `left`:
   - If `left` is data → `left` is the argument to `right`
   - If `left` is an unexecuted operator → substitute `_<` with `right`, evaluate if all slots filled
2. Evaluate `right`:
   - If `right` is data → it's the argument
   - If `right` is an unexecuted operator → it waits for its right arg
3. Result flows left as the new `left` for the next application in the chain

### 5.2 How Currying Shapes the AST

```
Source:   5 + 3 * 2
Tokens:   NumberLiteral(5), Identifier(+), NumberLiteral(3), Identifier(*), NumberLiteral(2)

AST (left-to-right curried):
Application(
  left: Application(
    left: Application(
      left: NumberLiteral(5),
      right: Identifier("+")       // 5 → + → unexecuted (+ 5) waiting for right
    ),
    right: NumberLiteral(3)        // (+ 5) gets 3 → resolves to 8
  ),
  right: Application(
    left: Identifier("*"),         // * waiting for right
    right: NumberLiteral(2)        // * gets 2 → unexecuted (* 2)
  )
)
// Evaluation: 5 → + → 3 → * → 2 = ((5+)3) then (8*)(2) = 16
```

### 5.3 Chained Property Access

```
Source:   entity@value@name
Tokens:   Identifier(entity), Identifier(@), StringLiteral(value), Identifier(@), StringLiteral(name)

AST:
Application(
  left: Application(
    left: Application(
      left: Application(
        left: Identifier("entity"),
        right: Identifier("@")         // entity → @ → unexecuted get
      ),
      right: StringLiteral("value")    // @ gets "value" → entity["value"]
    ),
    right: Identifier("@")             // result → @ → unexecuted get
  ),
  right: StringLiteral("name")         // @ gets "name" → result["name"]
)
```

### 5.4 Reverse-Args (Dot Operator)

```
Source:   `key`@.data
Tokens:   StringLiteral(key), Identifier(@), Identifier(.), Identifier(data)

AST:
Application(
  left: Application(
    left: Application(
      left: StringLiteral("key"),
      right: Identifier("@")            // "key" → @ → unexecuted get (key as left arg)
    ),
    right: Identifier(".")              // . takes unexecuted @ and swaps left/right slots
  ),
  right: Identifier("data")            // now data is left arg, key is right arg
)
```

### 5.5 GroupedExpression

```
GroupedExpression:
  expression: Expression     // the expression inside ( )
```

Parentheses force the inner expression to evaluate first, overriding left-to-right order.

```
Source:   {a:1, b:2} (Logger@`trace`)
AST:
Application(
  left: MapLiteral({a:1, b:2}),
  right: GroupedExpression(
    expression: Application(
      left: Application(
        left: Identifier("Logger"),
        right: Identifier("@")
      ),
      right: StringLiteral("trace")
    )
  )
)
// Parentheses make Logger@trace evaluate FIRST, then result applies to the map
```

---

## 6. Special Expression Nodes

### 6.1 ThrowExpression

```
ThrowExpression:
  value: Expression           // the expression to throw
  // Source: value !!!
```

The `!!!` operator throws the left value as an error.

### 6.2 CatchExpression

```
CatchExpression:
  operator: Expression        // the unexecuted operator to wrap with catch
  handler: Expression         // catch handler: {} (swallow) or {body} (operator)
  // Source: operator !!!? handler
```

`!!!?` wraps an operator with error catching. Empty map `{}` swallows errors. Operator body receives error as `_<`.

### 6.3 AsyncExpression

```
AsyncExpression:
  operator: Expression        // the operator to make async
  // Source: operator ///
```

`///` takes an operator and returns an async version.

### 6.4 AwaitExpression

```
AwaitExpression:
  promise: Expression         // the promise to await
  // Source: promise \\\
```

`\\\` (3 backslashes) awaits a promise resolution.

---

## 7. Import and Export Nodes

### 7.1 ImportExpression

Imports are NOT special syntax — they are regular expressions using runtime variables.

```
Source:   name: imports@`lodash/fp/map`
AST:
MapEntry(
  key: Identifier("name"),            // alpha-starting → assignment
  value: Application(
    left: Application(
      left: Identifier("imports"),    // runtime variable (map)
      right: Identifier("@")          // get operator
    ),
    right: StringLiteral("lodash/fp/map")  // module path as string
  ),
  is_assignment: true
)
```

**Import with pick**:
```
Source:   +: imports@`pkg`@&[`name1`, `name2`]
AST:
MapEntry(
  key: Identifier("+"),               // spread key
  value: Application(
    left: Application(
      left: Application(
        left: Application(
          left: Identifier("imports"),
          right: Identifier("@")
        ),
        right: StringLiteral("pkg")
      ),
      right: Identifier("@&")         // pick operator
    ),
    right: ListLiteral([StringLiteral("name1"), StringLiteral("name2")])
  )
)
```

**File imports**:
```
Source:   +: files@`./path`@&[`name`]
```

Uses `files` runtime variable instead of `imports`.

### 7.2 ExportExpression

```
Source:   }@&[`export1`, `export2`]
AST:
Application(
  left: Application(
    left: CloseBrace,               // } closing brace of containing map
    right: Identifier("@&")         // pick operator
  ),
  right: ListLiteral([StringLiteral("export1"), StringLiteral("export2")])
)
```

**Note**: The `}` is the closing brace of the map that contains the exports. The `@&` pick operator selects which names to export from that map.

---

## 8. Function Definition Pattern

Functions are defined as map entries with operator bodies:

```
Source:   funcName: { arg1: _<@0, bodyExpr }
AST:
MapEntry(
  key: Identifier("funcName"),
  value: MapLiteral(
    entries: [
      MapEntry(
        key: Identifier("arg1"),
        value: Application(
          left: LeftArg,
          operator: Identifier("@"),
          right: NumberLiteral(0)
        ),
        is_assignment: true
      ),
      // body expressions as additional entries or trailing expression
    ]
  ),
  is_assignment: true
)
```

**Async function**:
```
Source:   funcName: { ...body } ///
AST:
MapEntry(
  key: Identifier("funcName"),
  value: AsyncExpression(
    operator: MapLiteral(...)
  )
)
```

**Function invocation**:
```
Source:   data funcName
AST:
Application(
  left: Identifier("data"),
  right: Identifier("funcName")
)
// data flows as left arg into funcName, substituting all _< references
```

**Function invocation with list args**:
```
Source:   [arg1, arg2] funcName
AST:
Application(
  left: ListLiteral([arg1, arg2]),
  right: Identifier("funcName")
)
// _<@0 = arg1, _<@1 = arg2
```

**Await function call**:
```
Source:   [args] funcName \\\
AST:
Application(
  left: Application(
    left: ListLiteral([args]),
    right: Identifier("funcName")
  ),
  right: Identifier("\\\\\\")
)
```

---

## 9. Control Flow Patterns (All Map-Based)

### 9.1 Conditional (Ternary-like)

```
Source:   { _<: trueCase, falseCase }
AST:
MapLiteral(
  entries: [
    MapEntry(key: LeftArg, value: Identifier("trueCase"), is_expression_key: true),
    MapEntry(key: Identifier("falseCase"), is_expression_key: true)  // no : = expression
  ]
)
```

Runtime behavior: if left arg is truthy, `:` returns `trueCase` (early return). Otherwise, falls through to `falseCase`.

### 9.2 Conditional with Negation

```
Source:   { _<@`prop`!: trueValue, falseValue }
```

`!` negates the boolean result of the get operation.

### 9.3 Size-based Conditional

```
Source:   collection #: { body }
AST:
Application(
  left: Application(
    left: Identifier("collection"),
    right: Identifier("#")       // size operator
  ),
  right: MapLiteral(...)
)
// If collection size > 0 (truthy), evaluate body
```

### 9.4 Side-effect Conditional

```
Source:   job ? job.cancel()
AST:
Application(
  left: Application(
    left: Identifier("job"),
    right: Identifier("?")       // toBoolean
  ),
  right: Application(
    left: Identifier("job"),
    right: Application(
      left: Identifier("cancel"),
      right: ListLiteral([])
    )
  )
)
```

### 9.5 Error Handling

```
Source:   someOperation !!!? { error handling }
AST:
CatchExpression(
  operator: Identifier("someOperation"),
  handler: MapLiteral(...)
)
```

```
Source:   `error message` !!!
AST:
ThrowExpression(
  value: StringLiteral("error message")
)
```

---

## 10. Operator Body Types

### 10.1 Map Operator (with left/right args)

```
Source:   { _< + _> }
AST:
MapLiteral(
  entries: [
    MapEntry(
      key: LeftArg,                // contains _<
      value: Application(
        left: LeftArg,
        right: Application(
          left: Identifier("+"),
          right: RightArg           // contains _>
        )
      )
    )
  ]
)
```

Runtime: detects `_<` and `_>` → creates diatic operator (takes left + right args).

### 10.2 Map Operator (expression-ending)

```
Source:   { arg: _<@0, bodyExpression }
AST:
MapLiteral(
  entries: [
    MapEntry(key: Identifier("arg"), value: Application(_<, @, 0), is_assignment: true),
    // last entry has no : → expression ending → operator
  ]
)
```

### 10.3 String Operator (interpolation with args)

```
Source:   `{_<} is the value`
AST:
StringLiteral(
  parts: [
    InterpolationPart(LeftArg),
    TextPart(" is the value")
  ]
)
```

Runtime: detects `_<` or `_>` in interpolation → creates operator.

---

## 11. Evaluation Model (for Parser/AST Understanding)

### 11.1 Left-to-Right Curried Chain

The entire source file is ONE left-to-right chain:

```
token1 token2 token3 token4 token5 ...

= (((((token1) token2) token3) token4) token5)
```

Each step:
1. `token1` evaluates → value (data or unexecuted operator)
2. `token1 token2` → if token2 is operator, token1 becomes left arg
3. Result flows left, becomes left arg for token3
4. Continue until all tokens consumed

### 11.2 When Operators Resolve

- **Monatic** (left-only): `5 !` → negate 5. Resolves immediately when left arg is data.
- **Diatic** (left + right): `5 + 3` → `5+` creates unexecuted "add 5", then `3` fills right slot → resolves to 8.
- **Partial application**: `5 +` at end of expression → unexecuted "add 5" operator. No error.

### 11.3 No Precedence

```
5 + 3 * 2 = ((5 + 3) * 2) = 16    // NOT 5 + (3 * 2) = 11
```

Parentheses override: `(5 + 3) * 2` → same as without parens in this case, but `(3 * 2) + 5` → `((3 * 2) evaluated first) + 5` = 11.

---

## 12. AST Node Summary Table

| Node Type | Children | Source Pattern |
|-----------|----------|----------------|
| Program | expression | entire .lr file |
| NumberLiteral | value, raw | `42`, `3.14` |
| StringLiteral | parts[] | `` `text {expr} more` `` |
| BooleanLiteral | value | `true`, `false` |
| UndefinedLiteral | — | `undefined` |
| ListLiteral | elements[] | `[1, 2, 3]` |
| MapLiteral | entries[] | `{ key: value, ... }` |
| MapEntry | key, value, flags | `key: value` |
| Identifier | name | `+`, `@`, `hello`, `!!!?` |
| LeftArg | — | `_<` |
| RightArg | — | `_>` |
| Application | left, right | `a b` (any two adjacent expressions) |
| GroupedExpression | expression | `(expr)` |
| ThrowExpression | value | `expr !!!` |
| CatchExpression | operator, handler | `op !!!? handler` |
| AsyncExpression | operator | `op ///` |
| AwaitExpression | promise | `promise \\\` |

---

## 13. Parser Construction Notes

### 13.1 Parser Strategy

The parser is remarkably simple because:
1. No precedence table
2. No statement/expression distinction
3. Everything is left-to-right application

**Algorithm**:
1. Parse the first token as the initial `left` expression
2. For each subsequent token:
   - If delimiter (`[`, `{`, `(`) → parse the collection/group recursively as `right`
   - If `:` → parse the value expression after it as the map entry value
   - If `,` → close current map/list entry, start next
   - Otherwise → wrap current `left` and new `right` into Application node
3. Return the final Application chain

### 13.2 Map Parsing

Maps are the most complex construct:

```
parseMap():
  expect OpenBrace
  entries = []
  while next token is not CloseBrace:
    key = parseExpression()
    if next token is Colon:
      consume Colon
      value = parseExpression()
      entries.append(MapEntry(key, value, is_assignment=detectAlpha(key)))
    else:
      // Last entry with no colon = expression ending
      entries.append(MapEntry(key, None, is_expression_key=true))
    if next token is Comma:
      consume Comma
  expect CloseBrace
  return MapLiteral(entries)
```

### 13.3 Expression Key Detection

The parser can determine if a map key is "assignment" vs "expression" by checking:
- Key is Identifier starting with alpha character → assignment
- Key is anything else (LeftArg, expression, symbol identifier) → expression key

### 13.4 Context-Sensitive Parsing

The lexer does NOT need context. The parser needs minimal context:
- Inside `{}` → parsing map entries (look for `:` and `,`)
- Inside `[]` → parsing list elements (look for `,`)
- Inside `()` → parsing grouped expression
- Inside `` ` `` → parsing string literal with interpolation
- Otherwise → parsing left-to-right application chain

---

## 14. Grammar (PEG-style)

```
Program       ← Expression EOF

Expression    ← Application

Application   ← Primary (Primary)*
                // Left-to-right fold into nested Application nodes

Primary       ← NumberLiteral
              / StringLiteral
              / BooleanLiteral
              / UndefinedLiteral
              / LeftArg
              / RightArg
              / MapLiteral
              / ListLiteral
              / GroupedExpression
              / Identifier

NumberLiteral ← [0-9]+ ("." [0-9]+)?
StringLiteral ← "`" StringPart* "`"
StringPart    ← Interpolation / EscapedBacktick / TextChar
Interpolation ← "{" Expression "}"
EscapedBacktick ← "\\`"
TextChar      ← [^`{}\\] / "\\" [^`]

BooleanLiteral ← "true" / "false"
UndefinedLiteral ← "undefined"
LeftArg       ← "_<"
RightArg       ← "_>"

MapLiteral    ← "{" (MapEntry ("," MapEntry)* ","?)? "}"
MapEntry      ← Expression ":" Expression
              / Expression           // trailing expression (no colon)

ListLiteral   ← "[" (Expression ("," Expression)* ","?)? "]"
GroupedExpression ← "(" Expression ")"

Identifier    ← IdentifierChar+
IdentifierChar ← [^:\,\.'()\[\]\{\}`_\s]  // anything not reserved, not whitespace
              // BUT: "_<" and "_>" are captured before Identifier rule

Comment       ← "```" [^\n]* "\n"    // only at line start
```

**Note**: The PEG above is simplified. Real implementation needs:
- Maximal munch for `_` before `<` and `>` to detect `_<` and `_>`
- String interpolation brace depth tracking
- Number vs identifier disambiguation (digits start → number)

---

## 15. Open Questions (Remaining)

These questions are lower-priority and do NOT block lexer/AST implementation:

| # | Question | Impact |
|---|---------|-------|
| 1 | Exact tab-stripping rules for multi-line strings (which tabs? all leading tabs?) | StringLiteral value |
| 2 | How does `package.lr` relate to module resolution? | Import resolution (runtime concern) |
| 3 | Is there a maximum expression nesting depth? | Parser stack limits |
| 4 | How are external library constructors typed? | Runtime typing, not parser |
| 5 | Exact semantics of `@{...}` spread on maps (vs `+:`) | Runtime behavior |
| 6 | How does `reduce` work as built-in vs operator? | Runtime library, not parser |
| 7 | Can `_:` no-output entries appear before the last entry in a map? | Map parsing edge case |
