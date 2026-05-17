# Rosetta Code Pull Initiative

## Overview

This initiative pulls solutions from Rosetta Code across 26 programming languages to create a corpus of examples for validating Left-Right language design. Rosetta Code is a programming chrestomathy site that presents solutions to the same task in multiple programming languages, making it ideal for comparative language analysis.

## Why This Matters

Left-Right is a point-free, operator-based, array-oriented language that transpiles to both JavaScript and Rust. By studying how other languages solve well-defined problems, we can:

- **Validate language design**: See if Left-Right can express solutions as elegantly as established languages
- **Discover gaps**: Identify missing operators or patterns by analyzing what other languages do well
- **Build reference implementations**: Create a corpus of problems to solve in Left-Right
- **Generate documentation**: Use examples as the basis for "by example" documentation approach suggested by Kieran Brown
- **Benchmark expressiveness**: Compare line counts and conceptual complexity across languages

## Scope

### Target Languages (26)

| Language | Paradigm | Key Characteristics |
|-----------|-----------|-------------------|
| Python | Multi-paradigm | Clean syntax, extensive stdlib |
| Rust | Systems | Memory safety, ownership |
| JavaScript | Multi-paradigm | Ubiquitous, dynamic |
| Haskell | Functional | Pure, type-safe |
| OCaml | Functional | Strong static typing |
| Elixir | Functional, Concurrent | BEAM VM, actor model |
| Erlang | Functional, Concurrent | BEAM VM, telecom heritage |
| Clojure | Functional, Lisp | JVM, immutable data |
| Ruby | Object-oriented | Developer happiness |
| R | Statistical | Data analysis focus |
| Kotlin | Multi-paradigm | JVM, null safety |
| Scala | Functional/Object | JVM, type inference |
| Groovy | Dynamic | JVM, concise |
| CoffeeScript | Functional | Compiles to JS |
| BQN | Array | Array language, APL family |
| J | Array | Array language, concise |
| Arturo | Array | Modern array language |
| Wren | Scripting | Small, embeddable |
| Mercury | Functional, Logic | Strong typing, purity |
| GAP | Mathematical | Computational group theory |
| newLisp | Functional, Lisp | Small footprint |
| REBOL | Data-oriented | Minimal, homoiconic |
| OI | Lisp dialect | Symbolic computation |
| Ursala | Functional | High-level, point-free |
| Slate | Object-oriented | Prototype-based, multiple dispatch |
| Raven | Array | Stack-based |
| PowerShell | Object-oriented | Windows automation |
| Lingo | Scripting | Multimedia (Director) |
| Elm | Functional | FRP, web frontend |

### Problem Categories

Rosetta Code contains 1000+ tasks. We'll focus on representative problems across 13 semantic categories:

1. Arithmetic & Math
2. String Processing
3. Collections & Data Structures
4. Algorithms (sorting, searching)
5. Concurrency & Parallelism
6. File I/O
7. Networking
8. Date/Time
9. Recursion
10. Pattern Matching
11. Control Flow
12. Type System
13. Metaprogramming

Each category has 5-10 representative tasks prioritized for Left-Right validation.

## Approach

### Scraping Strategy

We will NOT use Rosetta Code's API (which is rate-limited and incomplete). Instead, we'll use respectful web scraping:

1. **Base URL pattern**: `https://rosettacode.org/wiki/Task_Name#Language_Name`
2. **Rate limiting**: 1 request per second minimum, with 10-second pauses every 50 requests
3. **Caching**: Store fetched HTML locally to avoid re-fetching
4. **Parsing**: Extract code blocks using HTML structure (pre/code tags with language-specific classes)
5. **Fallback**: Use MediaWiki API if scraping fails for specific tasks

### Technology Stack

- **Language**: Rust (matches Left-Right transpiler implementation)
- **HTTP Client**: `reqwest` for fetching
- **HTML Parsing**: `scraper` or `select` crate
- **Caching**: Local filesystem with content-based deduplication
- **Error Handling**: Retry logic with exponential backoff

### Respect for Rosetta Code

- Add User-Agent header identifying the project
- Respect robots.txt
- Cache aggressively to minimize requests
- Make results available back to Rosetta Code community
- Attribute all examples to Rosetta Code

## File Structure

Output goes to: `~/code/left-right/reports/rosetta-code/language-examples/`

```
reports/rosetta-code/
└── language-examples/
    ├── python/
    │   ├── 01-arithmetic-math.md
    │   ├── 02-string-processing.md
    │   ├── 03-collections-data-structures.md
    │   └── ...
    ├── rust/
    ├── bqn/
    ├── j/
    ├── javascript/
    └── ...
```

Each language folder contains 13 markdown files (one per problem category), each containing 5-10 tasks with:
- Task name and Rosetta Code URL
- Original code
- Notes on language-specific patterns
- Line count for expressiveness comparison

## Project Goals

### Short-term (Phase 1)

1. Pull all examples for 26 target languages across 13 categories
2. Generate aggregate reports per language per category
3. Identify missing Left-Right features by analyzing patterns

### Medium-term (Phase 2)

1. Solve 5-10 high-priority tasks per category in Left-Right
2. Write "by example" documentation based on solutions
3. Create comparative analysis report

### Long-term (Phase 3)

1. Contribute Left-Right solutions to Rosetta Code
2. Build automated test suite from corpus
3. Use corpus for Left-Right compiler validation

## Success Criteria

- All 26 languages have complete example sets for all 13 categories
- Each example is validated (parses correctly, no missing sections)
- At least 50 tasks solved in Left-Right with documentation
- Comparative analysis identifies 10+ Left-Right improvement opportunities

## Related Work

This initiative complements:
- Left-Right spec development (validating design decisions)
- "By example" documentation (Kieran Brown's suggestion)
- Compiler/transpiler development (real-world test cases)

## Open Questions

1. Should we include languages not in the target 26? (e.g., APL, Julia)
2. How often should we sync with Rosetta Code updates?
3. Should we prioritize certain task categories over others?
4. What's the minimum viable subset for Phase 1?

---

**Status**: Planning phase. See linked documents for detailed specifications.
**Next Step**: Implement scraping tool per `04-implementation-steps.md`.
