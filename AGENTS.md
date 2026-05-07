# Left-Right Language Project — Agent Rules

## Protected Source Files (NEVER EDIT)

These 3 files are the user's original language design documents. They must NEVER be modified by any agent, tool, or automated process:

1. `Map Programming Language Syntax Brainstorming.txt` — Primary brainstorm document (1753 lines)
2. `PenroScript.md` — PenroScript code examples with operator notes (89 lines)
3. `Penscript_LeftRight brainstorm.md` — 25-category specification checklist (246 lines)

**Enforcement**: Do not use Edit, Write, or any other tool to modify these files. Only Read is permitted.

## DO NOT EDIT Files (NEVER EDIT)

Any file containing the text `DO NOT EDIT` (case-sensitive) anywhere in its content must NEVER be modified by any agent, tool, or automated process. This applies across the entire workspace, not just to specific directories.

**Enforcement**: Before any edit, grep the target file for `DO NOT EDIT`. If found, abort the edit immediately. Only Read is permitted on such files.

## Project Structure

- `/language-summary/` — Generated report suite (safe to create/edit)
- `/docs/reports/initial-thoughts/` — User's research notes (read-only reference)
- Top-level `.txt` and `.md` files — Protected source documents

## Language Context

- **Name**: Left-Right (evolved from Penscript → PenroScript → Left-Right)
- **CLI command**: `lr`
- **Paradigm**: Point-free, operator-based, array-oriented, loosely typed
- **Targets**: Transpiles to both JavaScript and Rust
- **Transpiler**: Written in Rust
