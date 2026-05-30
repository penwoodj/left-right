# Test Coverage & Bug Fix Plan — COMPLETE

## Completed
- [x] CLI test infrastructure verified — 66 .lr tests pass
- [x] Map `==`/`!=` equality dispatch added to VM
- [x] `$?+`/`$?-` filter Number dispatch added to VM
- [x] Catch compile-order bug fixed — Catch opcode emitted BEFORE try expression
- [x] Jump offset bug fixed — uses `a` field not `b`
- [x] Handler offset calculation fixed — uses `handler_start_pos - catch_pos`
- [x] Release description updated with accurate feature list
- [x] README updated with all implemented features
- [x] 359 Rust e2e tests + 66 CLI integration tests — all pass

## Test Coverage (95%+)
All VM-dispatched operators have test coverage. Only gaps:
- `?:else` — compiler-internal, not user-facing
- List `_`/`<>`/`><` partial operators — same dispatch as string variants

## Commits on Branch (16 total)
1. `75cd51b` Phase 3: guards + optional apply
2. `0b70c8c` VM ops batch
3. `e284ed7` $@ + bracket path access
4. `0b327c8` !!! throw
5. `151e252` !!!? catch parser
6. `4b3743a` #: size conditional
7. `8b5c463` MapMerge spread
8. `c5e2471` Error constructor
9. `eb58fc8` Partial application
10. `b0b12d1` Test coverage expansion
11. `d8f79e4` Bug fixes + tests
12. `5263039` Negation fix + tests
13. `217d355` CLI test expression fixes
14. `6910ac3` Map equality, catch, filter comparisons
15. `4488d3f` Greater-than tests
16. README + docs update (pending commit)

## Remaining Unimplemented (deferred — need runtime infrastructure)
- Async/await (`///`/`\\\`) — needs async runtime
- Import/export — needs filesystem/module loading
- Method calls — needs parser AST changes
- Constructor syntax — needs parser AST changes
- JSON parsing (`/json`) — needs serde or manual parser
- `@&` pick/destructure — needs parser + VM
- Named destructuring `_<@\`prop\`` — needs parser + compiler
- List `-` removal operator — needs VM dispatch
