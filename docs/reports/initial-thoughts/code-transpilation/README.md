# Code Transpilation — LLM-as-Compiler

Documents covering LLM-based transpilation architecture, compiler design patterns, and AST manipulation.

## Contents

- [`02-chatgpt-llm-pseudo-compiler-model.md`](./02-chatgpt-llm-pseudo-compiler-model.md) — Complete 5-stage pipeline architecture

## Key Topics

### LLM as Pseudo-Compiler

**Core Concept:**
> Don't let LLM directly "run" your program. Let it propose rewrites (macro expansion / transpilation / partial evaluation), then you validate + apply deterministically.

#### 5-Stage Pipeline

1. **Bind (Variable Resolving)**
   - LLM outputs JSON bindings
   - Machine-validated (types, required keys)
   - Provenance tracking

2. **Expand (Deterministic Substitution)**
   - System performs replacements
   - Reproducibility
   - Caching (bindings + template hash → output)

3. **Rewrite/Optimize (LLM-Guided)**
   - Lowering: high-level → low-level
   - Specialization: inject domain knowledge
   - Partial evaluation: pre-fill constants
   - Refactoring: split into subcalls

4. **Execute (Code Generation)**
   - Deterministic codegen
   - LLM generates with strict format
   - Structured "execution units"

5. **Verify (Validation)**
   - Tests, schema checks
   - Length constraints
   - Policy checks

### PenroScript Transpiler Architecture

#### JavaScript/TypeScript Implementation

**Pipeline Stages:**

1. **Parsing & AST Generation**
   - Formal grammar (PEG or recursive-descent)
   - LLM-assisted for ambiguous cases
   - Validated AST output

2. **Semantic Analysis & Disambiguation**
   - Type checking
   - Identifier resolution
   - LLM resolves underspecified constructs

3. **Code Generation**
   - Deterministic templates for known nodes
   - LLM for complex transformations
   - Structured prompting (code-only output)

4. **Validation**
   - Syntactic (Babel/Acorn parsing)
   - Semantic (sample input/output tests)
   - Optional LLM-as-judge verification

#### Rust Implementation

**Advantages:**
- Performance and efficiency
- Type safety
- Robust tooling
- Strong compile-time guarantees

**Pipeline Differences:**
- Same stages as JS approach
- `langchain-rs` integration
- Concurrency support
- Tighter LLM integration

### Determinism & Reliability

#### Problems & Fixes

| Problem | Fix |
|----------|------|
| Nondeterminism | Cache bindings + expanded text + model/version |
| Prompt injection via variables | Wrap untrusted text, label as inert input |
| LLM invents bindings | Require provenance, whitelisted sources |
| Evaluation correctness | Automated checks, cross-checking two models |
| Validation bypass | Schema validation, reject non-conforming outputs |

#### Artifacts to Save

- `bindings.json` — Variable bindings
- `expanded_prompt.txt` — After substitution
- `ir.json` — AST representation
- `patches/` — Rewrite diffs
- `verification_report.json` — Test results

### LangChain Integration

**JavaScript (LangChain.js):**
- Chain orchestration for LLM calls
- Guardrails and output parsers
- Structured output facilities
- Agent capabilities

**Rust (langchain-rs):**
- Rust port of LangChain
- Model abstraction
- Pluggable LLM backends
- Async/concurrent support

### Validation Strategies

1. **Syntactic Validation** — Parse output AST
2. **Semantic Validation** — Sample data tests
3. **Unit Testing** — Snapshot tests
4. **Guardrails** — Rule-based validation
5. **LLM Verification** — Optional second-opinion check

## Design Principles

1. **Two-Level Language** — Compile-time (LLM-assisted) + runtime
2. **Deterministic Core** — Parsing, AST, validation are deterministic
3. **LLM as Assistant** — LLM in specific, controlled roles
4. **Modular Pipeline** — Each stage is separable and testable
5. **Reproducibility** — Same input → same output, always
6. **Pluggable LLM** — Swap models without changing logic
7. **Guardrails** — Reject invalid LLM outputs
8. **Iterative Refinement** — Generate → test → reflect → fix

## Related Concepts

- **Transpilation** — Source-to-source translation
- **Compiler Architecture** — Frontend, middle-end, backend
- **AST Manipulation** — Intermediate representation transforms
- **Macro Expansion** — Compile-time code generation
- **Partial Evaluation** — Evaluate what's known at compile-time
- **LLM-Assisted Programming** — AI in compiler workflows
- **Verification** — Ensuring correctness of generated code
