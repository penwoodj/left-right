# Initial Thoughts — Left-Right / PenroScript Language Design

Organized collection of documentation, design decisions, and examples from the left-right programming language project.

## Directory Structure

Content is organized by topic into subdirectories:

- **[`notation-systems/`](./notation-systems/)** - Symbol design rationale, operator symbology analysis, semiotics of notation
- **[`dsl-design/`](./dsl-design/)** - Domain-specific language patterns, DSL design principles
- **[`functional-programming/`](./functional-programming/)** - Code golf, terse syntax, point-free programming patterns
- **[`concatenative-languages/`](./concatenative-languages/)** - Stack-based composition, concatenative patterns
- **[`data-pipeline-etl/`](./data-pipeline-etl/)** - ETL patterns, transformation operations, real-world integration
- **[`template-engines/`](./template-engines/)** - Template literal design, interpolation syntax, string templating
- **[`pipe-operators/`](./pipe-operators/)** - Left-to-right evaluation, pipeline operators, functional composition
- **[`auto-currying/`](./auto-currying/)** - Curried operators, partial application, directional sections
- **[`operator-precedence/`](./operator-precedence/)** - Precedence theory, flat precedence design, grouping rules
- **[`code-transpilation/`](./code-transpilation/)** - LLM-as-compiler architecture, transpiler design, AST manipulation
- **[`language-philosophy/`](./language-philosophy/)** - Design principles, evaluation models, core philosophy
- **[`language-design-comprehensive/`](./language-design-comprehensive/)** - Full specification checklists, detailed design decisions

## Project Background

**Left-Right** (previously named PenroScript, then Penscript) is a programming language design project focused on:

- **Left-to-right evaluation** as fundamental execution model
- **Operator-centric syntax** with first-class operators
- **Functional composition** through point-free style
- **Deterministic execution** with clear semantics
- **JSON-like structure** for data and program expressions
- **Terse DSL** optimized for data transformation and templating

## Key References

| Document | Location | Focus |
|-----------|----------|--------|
| [Index](./language-philosophy/00-index.md) | Master index of all extracted reports |
| [PenroScript examples](./pipe-operators/PenroScript.md) | Syntax examples, operator demonstrations |
| [Designing a Programming Language](./language-design-comprehensive/01-chatgpt-designing-a-programming-language.md) | Initial design, operator table, symbol rationale |
| [LLM Pseudo-Compiler](./code-transpilation/02-chatgpt-llm-pseudo-compiler-model.md) | Transpiler architecture, 5-stage pipeline |
| [Brainstorm Checklist](./language-design-comprehensive/Penscript_LeftRight\ brainstorm.md) | 25-category spec checklist |
| [Syntax Brainstorming](./data-pipeline-etl/Map\ Programming\ Language\ Syntax\ Brainstorming.txt) | File system, functional composition |
| [New Language Creation](./language-philosophy/04-chatgpt-new-language-creation.md) | Earliest exploration — APL/J/K, naming |
| [Lambda Calculus](./functional-programming/05-chatgpt-lambda-calculus.md) | Tromp diagrams, Church encoding, Y-combinator |
| [Operator Theory](./notation-systems/06-chatgpt-operator-theory.md) | Mathematical operator properties |
| [Transpiler Design](./code-transpilation/07-chatgpt-transpiler-design.md) | DSL design, YAML→Rust architecture |
| [Wittgenstein & Language](./semiotics/08-chatgpt-wittgenstein-and-language.md) | Philosophy of language, LLM papers |
| [Functional Programming](./functional-programming/09-chatgpt-functional-programming.md) | lodash/FP point-free style |
| [Early Thoughts (Notes)](./language-philosophy/10-keep-notes-early-thoughts.md) | May 2019 typeless language design |

## Topics Covered

Based on research into programming language design, this collection addresses:

- Notation systems and symbol design
- Esoteric and terse language patterns
- Domain-specific language design
- Code golf and expressiveness trade-offs
- Concatenative language patterns
- Language-oriented programming
- Operator precedence and evaluation order
- Data pipeline and ETL patterns
- Template engine design
- Pipe operators and left-to-right flow
- Auto-currying mechanisms
- Visual programming alternatives
- Cognitive load and readability
- Transpilation architecture
- Deterministic execution models

## Terminology Evolution

1. **Penscript** — earliest name
2. **PenroScript** — intermediate name (used in ChatGPT conversations)
3. **Left-Right** — current name (reflects left-to-right evaluation)

## Usage

This directory serves as a reference for understanding the design decisions and evolution of the left-right language. Each subdirectory contains relevant documentation organized by specific topic area.
