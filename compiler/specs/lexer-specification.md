# Left-Right Language — Lexer Specification

> **Status**: CONFIRMED — Based on language designer Q&A (Q1-Q16), primary source `manual-language-summary.md`, and SOT files (`wiz/integration.lr`, `wiz/server/request.lr`).
> All `[?]` markers from the draft have been resolved.

---

## 1. Overview

The Left-Right lexer tokenizes `.lr` source text into a flat stream of tokens. It is the first phase of the compiler pipeline.

**Key design principles confirmed by language designer:**
- Left-Right has **NO keywords** (except reserved symbols). Everything is identifiers, operators, literals, or delimiters.
- ALL operators are identifiers. The lexer does NOT distinguish operators from identifiers. `+`, `@`, `$@`, `!!!?` are all identifiers.
- There is **NO operator precedence**. Evaluation is strictly left-to-right, curried.
- Every `.lr` file is a **single root expression**.
- Whitespace is purely a token separator — no indentation sensitivity.

The lexer produces **tokens only** — it does not interpret semantics. All semantic disambiguation happens in the parser and runtime.

---

## 2. Character Set

### 2.1 Source Encoding
- UTF-8

### 2.2 Whitespace Characters
- Space (` `), Tab (`\t`), Newline (`\n`), Carriage Return (`\r`)
- Whitespace is significant ONLY as a token separator. No other meaning.
- Newlines are treated as whitespace (not statement terminators).

---

## 3. Token Types

### 3.1 Complete Token Category Enum

```
TokenKind:
  // --- Literals ---
  NumberLiteral          // decimal integer or float: 42, 3.14, 0.5
  StringLiteral          // backtick-delimited: `hello`, `{name}!`
  BooleanLiteral         // true | false
  UndefinedLiteral       // undefined (the null/nil value)

  // --- Identifiers (includes ALL operators) ---
  Identifier             // any non-reserved, non-delimiter character sequence
                         // Examples: +, @, $@, !!!, !!!?, $_, hello, response

  // --- Special Identifiers (2-char, cannot be split) ---
  LeftArg                // _<  (left argument reference)
  RightArg               // _>  (right argument reference)

  // --- Structural Delimiters ---
  OpenBrace              // {
  CloseBrace             // }
  OpenBracket            // [
  CloseBracket           // ]
  OpenParen              // (
  CloseParen             // )

  // --- Reserved Symbols (structural, not identifiers) ---
  Colon                  // :
  Comma                  // ,
  Dot                    // .
  SingleQuote            // ' (reserved, unused)

  // --- String Delimiter ---
  Backtick               // ` (opens/closes string literals)

  // --- Comments ---
  Comment                // ``` prefix to end of line

  // --- Special ---
  EOF                    // End of file
```

### 3.2 Token Structure

```
Token:
  kind: TokenKind
  value: string          // raw text of the token
  span: Span             // byte offset range
  line: u32
  column: u32

Span:
  start: u32
  end: u32
```

---

## 4. Token Recognition Rules

### 4.1 Reserved Symbols (Structural Delimiters)

These characters **always** produce their own token type. They can NEVER appear inside identifiers:

| Symbol | Token | Role |
|--------|-------|------|
| `:` | Colon | Assignment/return operator, map key-value separator |
| `,` | Comma | Separator in maps and lists ONLY |
| `.` | Dot | Reverse-args operator (always standalone) |
| `'` | SingleQuote | Reserved for future use |
| `(` | OpenParen | Grouping / precedence override |
| `)` | CloseParen | Grouping / precedence override |
| `[` | OpenBracket | List literal / encapsulation |
| `]` | CloseBracket | List literal / encapsulation |
| `{` | OpenBrace | Map literal / encapsulation |
| `}` | CloseBrace | Map literal / encapsulation |
| `` ` `` | Backtick | String literal delimiter |
| `_<` | LeftArg | Left argument reference (2-char token) |
| `_>` | RightArg | Right argument reference (2-char token) |

### 4.2 Identifier Recognition

**Rule**: An identifier is any sequence of characters that:
1. Does NOT contain any reserved symbol from Section 4.1
2. Is NOT `_<` or `_>`
3. Is NOT preceded by `` ` `` (that would be a string literal start)
4. Is NOT a number literal (starts with digit)

**This means operators are identifiers:**
- `+` is an identifier
- `@` is an identifier
- `!!!` is an identifier
- `!!!?` is an identifier
- `$@` is an identifier
- `$?` is an identifier
- `><` is an identifier
- `<>` is an identifier
- `///` is an identifier
- `\\\` is an identifier (3 backslashes)
- `^` is an identifier
- `~` is an identifier
- `"^_` is an identifier

**Multi-character identifiers**: Characters with no whitespace between them form a single identifier token. `!!!?` is ONE token. `$@` is ONE token. `$|||` is ONE token.

**Whitespace between characters creates separate tokens**: `! ! !` is THREE identifier tokens, not one `!!!`.

### 4.3 Maximal Munch for Brackets

The lexer applies maximal munch for bracket pairs:
- `[]` → emit `OpenBracket` + `CloseBracket` (empty list literal) — or could be handled as two tokens
- `{}` → emit `OpenBrace` + `CloseBrace` (empty map literal)

Actually: Brackets are individual tokens. `[]` is `OpenBracket` followed immediately by `CloseBracket`. Maximal munch applies at the bracket level — lexer tries to match `[]` and `{}` as paired delimiters first for empty collection literals, but still emits them as two tokens each.

### 4.4 Number Literal Recognition

**Rule**: `[0-9]+(\.[0-9]+)?`

- Numbers are decimal ONLY. No hex, binary, octal, or scientific notation.
- Numbers MUST start with a digit. `.5` is NOT valid — must be `0.5`.
- Numbers stop at the first non-digit character. `5+3` = number `5`, identifier `+`, number `3`.
- `.` is a reserved symbol (dot/reverse-args), NOT a decimal point in all contexts. Lexer recognizes `3.14` as a number because the dot is between digits. But `5.` at end = number `5` + dot token.
- **No negative number literals**. `-5` is identifier `-` followed by number `5`. The minus operator is always binary: `0 - 5` to get negative five.

### 4.5 Boolean and Undefined Literals

| Literal | Token | Value |
|---------|-------|-------|
| `true` | BooleanLiteral | true |
| `false` | BooleanLiteral | false |
| `undefined` | UndefinedLiteral | undefined (the null/nil value) |

These are special identifier-like tokens. The lexer recognizes them by exact match.

### 4.6 Comment Recognition

**Rule**: When `` ``` `` (three backticks) appears at the **start of a line**:
1. Consume the three backtick characters
2. Consume all characters until end of line (`\n`)
3. Emit a `Comment` token (value = comment text without backticks)
4. Resume normal tokenization on next line

**Constraints**:
- Comments are line-start ONLY. Cannot appear between tokens on a line.
- Single-line only. No multi-line comments.
- No nesting.

---

## 5. String Literal Tokenization

### 5.1 String Delimiters

Strings are delimited by **backtick** (`` ` ``). No other string delimiter exists (no single or double quotes).

```
StringLiteral:
  ` StringContent `
```

### 5.2 String Content

Inside a backtick string, the following are recognized:
- **Plain text** — any character except `` ` `` and `{`
- **Interpolation** — `{expression}` where expression follows full expression grammar
- **Escape** — `\`` (backslash escapes the backtick character)

### 5.3 Escape Sequences

| Sequence | Meaning |
|----------|---------|
| `\`` | Literal backtick character (escaped to prevent string termination) |
| `\` | Literal backslash (when not followed by backtick) |

**Note**: Only backslash-escaping the backtick is confirmed. No `\n`, `\t`, `\\` etc. are confirmed as escape sequences. Multi-line strings are supported natively (tab whitespace on newlines is stripped at runtime).

### 5.4 Multi-line Strings

Strings CAN span multiple lines. Tab characters at the start of continuation lines are stripped by the runtime (not the lexer). The lexer preserves all content between backticks verbatim.

### 5.5 String Interpolation

`{expression}` inside a string triggers interpolation:
- The `{` and `}` are NOT separate tokens — they are part of the string literal
- The lexer must track brace depth to find the matching `}`
- The expression inside `{}` follows the full expression grammar
- The result is auto-converted to string at runtime

**Nesting confirmed**: Interpolation CAN be nested. `` `{`{inner}`}` `` is valid. Inner interpolation returns a value (or operator), outer string runs toString on it. Brace depth tracking handles nesting.

### 5.6 String Interpolation Lexer State

```
When inside a string (` encountered):
  - Normal text → accumulate as string content
  - { encountered → enter interpolation mode
    - Track brace depth
    - Tokenize expression using normal rules
    - } at depth 0 → exit interpolation mode
  - \` encountered → consume as escaped backtick
  - ` (unescaped) encountered → end of string
```

---

## 6. Token Separation Rules

### 6.1 What Breaks Tokens

Tokens are separated by:
1. **Whitespace** (space, tab, newline)
2. **Reserved symbols** (`:`, `,`, `.`, `'`, `(`, `)`, `[`, `]`, `{`, `}`, `` ` ``)
3. **Special 2-char tokens** (`_<` and `_>` break on these sequences)
4. **Number boundary** — digits stop at first non-digit

### 6.2 Examples

| Source Text | Token Sequence |
|-------------|----------------|
| `response@value` | Identifier(`response`), Identifier(`@`), StringLiteral(`value`) |
| `5+3` | NumberLiteral(`5`), Identifier(`+`), NumberLiteral(`3`) |
| `_<@0` | LeftArg(`_<`), Identifier(`@`), NumberLiteral(`0`) |
| `_: expr` | Identifier(`_`), Colon(`:`), ... (three separate tokens) |
| `+:` | Identifier(`+`), Colon(`:`) — two separate tokens |
| `!!!?` | Identifier(`!!!?`) — one token, no whitespace |
| `!!! ?` | Identifier(`!!!`), Identifier(`?`) — two tokens, whitespace breaks them |
| `entity@`key`` | Identifier(`entity`), Identifier(`@`), StringLiteral(`key`) |
| `[]` | OpenBracket, CloseBracket |
| `{}` | OpenBrace, CloseBrace |
| ``` ```comment``` | Comment(`comment`) |
| `` `hello {name}!` `` | StringLiteral with interpolation |
| `true` | BooleanLiteral(`true`) |
| `undefined` | UndefinedLiteral(`undefined`) |

### 6.3 What Does NOT Break Tokens

- Adjacent identifier characters with no whitespace, no reserved symbols, and not `_<`/`_>` form a single identifier token.
- Numbers continue through digits and one optional `.` between digits.

---

## 7. Lexer State Machine

### 7.1 States

```
LexerState:
  Normal                  // Default tokenization
  InStringLiteral         // Between backticks, accumulating content
  InStringInterpolation   // Inside { } within a string
  InComment               // After ```, consuming until newline
```

### 7.2 State Transitions

```
Normal:
  '`' at line start × 3 → InComment (if triple backtick)
  '`' otherwise → InStringLiteral
  digit → accumulate number
  whitespace → skip
  reserved symbol → emit corresponding token
  ' _< ' → emit LeftArg
  ' _> ' → emit RightArg
  other → accumulate identifier characters until break

InStringLiteral:
  '`' (unescaped) → emit StringLiteral, return to Normal
  '\' followed by '`' → accumulate escaped backtick
  '{' → enter InStringInterpolation
  other → accumulate character

InStringInterpolation:
  '{' → increment depth
  '}' at depth 0 → return to InStringLiteral
  '}' at depth > 0 → decrement depth
  other → tokenize using Normal rules

InComment:
  newline → emit Comment, return to Normal
  other → accumulate comment text
```

---

## 8. Complete Operator Identifier Catalog

This section lists all known operators as identifiers (NOT separate token types). The lexer emits ALL of these as `Identifier` tokens. Semantic meaning is determined by the runtime.

### 8.1 Core Operator SDK (from `manual-language-summary.md`)

| Operator | Category | Semantics |
|----------|----------|-----------|
| `$` | Loop | Iterate/map |
| `$@` | Loop | Get each |
| `$?` | Loop | Filter |
| `$_` | Loop | FlatMap |
| `$~` | Loop | UniqueBy |
| `$>` | Loop | GroupBy |
| `$"` | Loop | EachToString |
| `$&` | Loop | Every/All |
| `$|` | Loop | Some/Any |
| `$?|` | Loop | Find |
| `$%` | Loop | Sort |
| `$?!` | Loop | Compact |
| `@` | Access | Get (map key, list index, nested) |
| `@&` | Access | Pick (select keys from map) |
| `@-` | Access | Omit (remove keys from map) |
| `@|` | Access | GetOr (try key, fallback) |
| `?` | Boolean | ToBoolean |
| `?!"` | Boolean | IsString |
| `?#` | Boolean | IsNumber |
| `?><` | Boolean | Contains |
| `#` | Collection | Size/Length |
| `/` | Math | Divide/cast |
| `|` | Logic | OR |
| `&` | Logic | AND |
| `=` | Logic | Equality |
| `^` | Transform | Uppercase/wrap |
| `_` | Transform | Flatten/floor/run-without-output |
| `+` | Collection | Add/concat/spread |
| `-` | Collection | Subtract/omit/remove |
| `%` | Math | Modulus/sort |
| `~` | Collection | Unique/random |
| `"` | String | ToString |
| `"_` | String | Lowercase |
| `"^` | String | Uppercase |
| `"^_` | String | Capitalize |
| `"~` | String | Replace |
| `<>` | String | Split |
| `><` | String | Join |
| `!` | Logic | Negate |
| `!!!` | Error | Throw |
| `!!!?` | Error | Catch |
| `///` | Async | Make async |
| `\\\` | Async | Await (resolve promise) |
| `/"` | Cast | ToString operator variant |
| `/json` | Cast | JSON parse |
| `.` | Meta | Reverse args |

### 8.2 Extensibility

Operators are extensible through map syntax:
```
{ `+`: { _< ?": `hello`, _< + _> } }
```

This means the operator catalog is NOT closed. The lexer cannot know all operators at tokenization time. It simply emits identifiers.

---

## 9. package.lr File

A special file with project configuration (like `package.json`). Structure:

```
{
  name: `package-name`,
  version: `1.0.0`,
  description: `Package description`,
  scripts: {
    run: `./entry-file`     // ./ and .lr implied
  },
  requiredLibraries: [...],
  optionalLibraries: [...]
}
```

The lexer tokenizes this the same as any `.lr` file — it's just a map literal.

---

## 10. Summary of Confirmed Facts

| # | Fact | Source |
|---|------|--------|
| 1 | No keywords. All operators = identifiers. | Q1-Q2 |
| 2 | Reserved symbols: `:`, `,`, `.`, `'`, `(`, `)`, `[`, `]`, `{`, `}`, `` ` `` | Q1, manual-language-summary |
| 3 | `_<` and `_>` are 2-char tokens | Q15 |
| 4 | Numbers: decimal only, no negative literals, must start with digit | Q1, Q13, Q16 |
| 5 | Booleans: `true`, `false` | Q1 |
| 6 | Null: `undefined` | Q1 |
| 7 | Strings: backtick-only, multi-line, `\`` escape | Q3, Q13 |
| 8 | String interpolation nesting: YES, depth tracked via braces | Q16 |
| 9 | Comments: line-start `` ``` `` prefix to EOL | Q3, Q13 |
| 10 | Whitespace: pure separator, no significance | Q3 |
| 11 | Every .lr file = single root expression | Q3 |
| 12 | Left-to-right curried evaluation, NO precedence | Q2 |
| 13 | `response@value` = 3 tokens | Q13 |
| 14 | `_:` = 2 tokens (not composite) | Q15 |
| 15 | `+:` = 2 tokens (not composite) | Q15 |
| 16 | `!!!` and `!!!?` = single identifiers each | Q15 |
| 17 | `{}` = always map at lexer level | Q15 |
| 18 | `.` = always standalone token | Q15 |
| 19 | `[]` = open+close bracket tokens | Q15 |
| 20 | Comma = maps and lists ONLY | Q13 |
| 21 | Maximal munch: `[]`, `{}` as bracket pairs first | Q15 |
| 22 | `'` reserved, unused | Q13 |
| 23 | Float must start with digit: `.5` invalid, `0.5` valid | Q16 |
