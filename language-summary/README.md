<div align="center">
  <img src="../logo.svg" alt="Left-Right Logo" width="256" />
</div>

# Left-Right Language — Documentation Suite

**Left-Right was previously known as PenRoScript**

## Project Overview

**Left-Right** is a general-purpose, point-free, operator-based, hierarchical-data oriented programming language. The language transpiles to both JavaScript and Rust, providing a familiar execution environment with a novel syntax built on the idea of code as data taken literally.

**Core Characteristics:**
- Point-free, operator-based syntax
- Hierarchical-data oriented (inspired by array-oriented languages)
- Loosely typed with runtime inference
- JSON-like structure for data and programs
- Left-to-right evaluation as fundamental execution model
- General-purpose scripting language (not a DSL)

**CLI Command:** `lr`

**Design Goals:**
- Readability over brevity
- Low ceremony over strictness
- Deterministic execution with clear semantics
- Clean transpilation targets (JS/TS and Rust)
- Terse syntax optimized for data transformation and general-purpose scripting

---

## Documentation Index

### Core Reports

| File | Description |
|------|-------------|
| [00-language-overview.md](./00-language-overview.md) | Complete language overview including name evolution, core paradigm, syntax, transpilation targets, key differentiators, design goals, and Kieran Brown's feedback |
| [01-design-philosophy.md](./01-design-philosophy.md) | Deep philosophy document covering LTR evaluation, operators as first-class values, terse syntax for data transformation, determinism, JSON-like readability, point-free style rationale, no explicit control flow, spatial/compounding symbology, type-dependent operator behavior, left-hungry auto-currying, inspiration sources, semiotics connection, cognitive load analysis, and aesthetic dimension |
| [02-type-system.md](./02-type-system.md) | Complete type system documentation covering primitive types, collection types, operators as first-class type, loosely typed nature, type-dependent operator behavior, error handling, comparison with APL/J type systems, and transpilation target interaction |
| [03-operator-reference.md](./03-operator-reference.md) | Comprehensive operator reference — all operators with symbol, behavior per type, examples, spatial compounding, currying |
| [04-evaluation-model.md](./04-evaluation-model.md) | LTR evaluation, no precedence, flat evaluation, left-hungry currying, directional forms |
| [05-functions-and-scope.md](./05-functions-and-scope.md) | Operator syntax, _< _> parameters, point-free chains, auto-currying, composition |
| [06-collections-and-paths.md](./06-collections-and-paths.md) | Maps, arrays, @ path access, collection operations, ETL patterns |
| [07-strings-and-templates.md](./07-strings-and-templates.md) | Template literals, string interpolation, case operators, spatial symbology |
| [08-modules-and-interop.md](./08-modules-and-interop.md) | File/package system, JS/Rust library interop, import/export, FFI |
| [09-error-handling.md](./09-error-handling.md) | Undefined defaults, !? type checking, error propagation through pipelines |
| [10-transpiler-architecture.md](./10-transpiler-architecture.md) | Rust transpiler, JS/Rust targets, compilation pipeline, deterministic design |
| [11-open-questions.md](./11-open-questions.md) | Unresolved design decisions, TODOs from 25-category spec, edge cases |
| [12-file-extension-options.md](./12-file-extension-options.md) | .lr availability analysis, alternative extensions, recommendations |
| [13-operator-overloading-hierarchy.md](./13-operator-overloading-hierarchy.md) | Operator override hierarchy: intra-script, folder, project, global levels |

### CLI User Flows

| File | Description |
|------|-------------|
| [cli-user-flows/README.md](./cli-user-flows/README.md) | CLI overview, installation, first-time experience |
| [cli-user-flows/tui-shell.md](./cli-user-flows/tui-shell.md) | Interactive TUI shell design with Ink, keyboard shortcuts, semantic editor |
| [cli-user-flows/run-transpile.md](./cli-user-flows/run-transpile.md) | `lr "path"` execution flow, Rust/Node targets, error reporting |
| [cli-user-flows/watch-mode.md](./cli-user-flows/watch-mode.md) | `lr --watch` file watching, debouncing, re-transpilation cycle |
| [cli-user-flows/configuration.md](./cli-user-flows/configuration.md) | Global/project config, environment variables, semantic customization |

### Example IO (By Example)

| File | Description |
|------|-------------|
| [example-io/README.md](./example-io/README.md) | Example index organized by difficulty |
| [example-io/basic-operations.md](./example-io/basic-operations.md) | Arithmetic, strings, booleans, templates, type checking |
| [example-io/collections.md](./example-io/collections.md) | Maps, arrays, path access, filtering, mapping, joining |
| [example-io/pipelines.md](./example-io/pipelines.md) | Multi-stage transforms, composition, LTR evaluation flow |
| [example-io/functions.md](./example-io/functions.md) | Lambdas, currying, point-free chains, getEntityTypes example |
| [example-io/conditionals.md](./example-io/conditionals.md) | Type checks, predicates, equality, guard patterns |
| [example-io/real-world.md](./example-io/real-world.md) | ServiceNow patterns, ETL, threat analysis example |
| [example-io/interop.md](./example-io/interop.md) | JS/Rust interop, FFI, npm packages, Rust crates |

---

## Additional Resources

### Source Documents

| Document | Location | Description |
|-----------|----------|-------------|
| Map Programming Language Syntax Brainstorming.txt | [`../Map Programming Language Syntax Brainstorming.txt`](../Map%20Programming%20Language%20Syntax%20Brainstorming.txt) | Primary brainstorm document (1753 lines) with function composition, operator design, JSON-like structures, and extensive code examples |
| PenroScript.md | [`../PenroScript.md`](../PenroScript.md) | PenroScript code examples with operator notes (89 lines) showing TypeScript and PenroScript comparisons |
| Penscript_LeftRight brainstorm.md | [`../Penscript_LeftRight%20brainstorm.md`](../Penscript_LeftRight%20brainstorm.md) | 25-category specification checklist (246 lines) covering language philosophy, evaluation model, types, operators, and more |

### Research Notes

| Document | Location | Description |
|-----------|----------|-------------|
| Initial Thoughts | [`../docs/reports/initial-thoughts/README.md`](../docs/reports/initial-thoughts/README.md) | Master index of initial research documentation |
| Language Philosophy | [`../docs/reports/initial-thoughts/language-philosophy/README.md`](../docs/reports/initial-thoughts/language-philosophy/README.md) | Design principles, evaluation model, core philosophy, type system, interop philosophy, error handling, and semantics guarantees |
| Comprehensive Design | [`../docs/reports/initial-thoughts/language-design-comprehensive/README.md`](../docs/reports/initial-thoughts/language-design-comprehensive/README.md) | Full specification checklists, detailed design decisions, 25-category spec checklist, operator table, and code examples |

### Protected Source Files

The following source documents are the user's original language design and must **NEVER** be modified by any agent or automated process:

1. `Map Programming Language Syntax Brainstorming.txt` — Primary brainstorm document (1753 lines)
2. `PenroScript.md` — PenroScript code examples with operator notes (89 lines)
3. `Penscript_LeftRight brainstorm.md` — 25-category specification checklist (246 lines)

**Enforcement:** Only read operations are permitted on these files. No editing, writing, or modification of any kind.

---

## Project Status

**Current Status:** Design phase

The language is in active development with comprehensive design documentation completed. Key features and syntax have been specified, but implementation is ongoing.

**Transpiler:** Written in Rust

The transpiler converts Left-Right source code to both JavaScript and Rust targets, enabling cross-platform execution.

---

## Quick Reference

**Language Family:** Point-free, operator-based, hierarchical-data oriented

**Inspiration Sources:** APL, J, K, BQN, Haskell, Clojure, lodash/FP (inspired by, not array-oriented)

**Core Data Types:** Text, Number, List, Map, Operator, Undefined

**Operator Paradigm:** Left-to-right evaluation with type-dependent behavior
