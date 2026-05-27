# Changelog

## [Unreleased]

### Fixed
- **Operator corrections**: Removed dead `==`, `&&`, `||` from VM dispatch. Equality is `=` only. AND is `&` only. OR is `|` only.
- **`!=` operator**: Confirmed valid as not-equals operator.
- **Context-dependent `|`**: `|` now returns default value when left is falsy, boolean OR otherwise.
- **Compound token recognition**: Lexer now recognizes `!!!`, `!!!?`, `$@`, `+:` as single tokens.

### Removed
- **v0.0.1 release deleted**: Contained incorrect operator descriptions. Will be re-released with correct implementation.