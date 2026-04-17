# Template Engines — Interpolation & Generation

Documents covering template literal design, string interpolation, and generation patterns.

## Key Topics

### Template Literal Syntax
From [`PenroScript.md`](../pipe-operators/PenroScript.md):

**Interpolation:**
```javascript
// Curly brace interpolation
'Thanks {name}'
```

**Operator Templates:**
```javascript
// Template with directional operators
'Thanks {_<}'
'Thanks {_<} for your help'
```

**String as Operators:**
```javascript
// Strings become operators when they contain placeholders
{ stringTemplate: _<@0 }
```

### Template Operations

From operator table:
- `join` (`><`) — Concatenate with separator
- `split` (`<>`) — Break into parts
- `replace` (`>"<`) — Substitute patterns
- `trim` (`<"`) — Remove whitespace
- `toUpper` (`^`) — Convert to uppercase
- `toLower` (`_"`) — Convert to lowercase
- `capitalize` (`^_`) — Title case

### Design Principles

1. **Unified Syntax** — Interpolation and code use same delimiters (`{ }`)
2. **Operator Lifting** — Strings with placeholders become callable operators
3. **Expression Evaluation** — Template expressions evaluated at runtime
4. **Multi-line Support** — Template strings span lines
5. **Escaping** — Mechanism for literal delimiters

### Template Patterns

#### Conditional Content
```javascript
// Boolean-driven template
{ name: _<@0 }
'Thanks {name! & `Hello`: ``}{name}'
```

#### List Iteration
```javascript
// Transform list to formatted strings
items $>< ', '
```

#### Nested Interpolation
```javascript
// Multi-level substitution
'Processed {processedDate} from {originalDate}'
```

### Advanced Features

#### Custom String Operators
```javascript
// User-defined string transformations
customFormatter: {
  'Value: {_< }^'
}
```

#### Partial Application
```javascript
// Pre-fill template arguments
'Thanks ' -> partialFunction
```

## Related Concepts

- **Template Engines** — Handlebars, Jinja, Mustache patterns
- **String Interpolation** — Variable substitution in strings
- **Code Generation** — Creating code from templates
- **Templating DSLs** — Languages for text generation
- **Curly Brace Templates** — `{variable}` syntax style
