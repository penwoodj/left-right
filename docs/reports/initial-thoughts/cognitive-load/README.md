# Cognitive Load — Readability & Syntax Psychology

Documents covering cognitive load in programming languages, readability research, and syntax psychology.

## Contents

- [`04-chatgpt-new-language-creation.md`](../language-philosophy/04-chatgpt-new-language-creation.md) — Language creation and cognitive considerations

## Key Topics

### Cognitive Load in Syntax Design

#### Readability vs Brevity
From language philosophy:
- Readability often prioritized over brevity in ergonomics
- Balance between terse syntax and learnability
- Mental model matching programmer expectations

#### Left-to-Right Evaluation
- **Cognitive Benefits:**
  - Matches natural reading direction
  - Reduces nesting mental overhead
  - Linear execution model
  - No precedence tables to memorize

- **Implementation Simplicity:**
  - Predictable evaluation order
  - Easier debugging (trace left-to-right)
  - Reduces cognitive dissonance

### Syntax Psychology

#### Symbolic vs Alphanumeric
- **Symbols** — Higher information density, steeper learning curve
- **Words** — Lower density, easier to understand for newcomers
- **Balance** — Trade-off between expressiveness and learnability

#### Familiarity Patterns
- Leverage existing language conventions
- Reduce cognitive friction through similarity
- Avoid surprising behaviors

### Operator Design Considerations

#### Visual Mnemonics
- Symbols chosen for intuitive connection to operations
- Spatial relationships (e.g., `<`/`>` for less/greater)
- Consistent visual patterns across related operations

#### Mental Models
- **Stack Model** — Push/pop mental model for concatenative languages
- **Pipeline Model** — Data flow mental model for LTR evaluation
- **Tree Model** — Nested structure mental model for hierarchical syntax

### Learning Curve

#### Progressive Complexity
- Start simple, build up gradually
- Core concepts first, advanced features later
- Clear feedback mechanisms

#### Code Reading vs Writing
- **Reading** — Prioritize understandability over writability
- **Writing** — Support concise, expressive constructs
- **Balance** — Optimize for both where possible

### Common Pitfalls

#### Excessive Brevity
- Code golf style reduces readability
- Cryptic symbols require extensive memorization
- Hinders collaboration and maintenance

#### Inconsistent Conventions
- Similar operations using different patterns
- Surprising behaviors increase cognitive load
- Learning fragmented across multiple conventions

#### High Cognitive Overhead
- Complex precedence rules
- Implicit conversions
- Hidden state mutations

## Design Principles

1. **Match Mental Models** — Syntax aligns with programmer expectations
2. **Reduce Surprise** — Predictable behavior, clear error messages
3. **Progressive Disclosure** — Introduce concepts gradually
4. **Visual Consistency** — Related concepts use similar visual patterns
5. **Minimize Cognitive Load** — Clear, unambiguous syntax
6. **Support Tools** — Linting, formatting, autocomplete
7. **Document Trade-offs** — Explain design decisions
8. **Iterate Based on Feedback** — Real-world usage drives improvements

## Related Concepts

- **Cognitive Psychology** — How humans process information
- **Human Factors** — Ergonomics in tool design
- **Learnability** — How quickly users become proficient
- **Readability** — Code comprehension by others
- **Code Review** — Collaborative understanding
- **Language Learnability** — Educational design principles
- **Syntax Sugar** — Convenience vs clarity trade-offs
