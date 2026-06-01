# $||| Parallel Map Operator Implementation Plan

## Goal
Implement `$|||` parallel map: `[1,2,3] $||| { _< * 2 }` → `[2, 4, 6]` executed in parallel.

## Design
- **Rust VM**: Multi-threaded via `std::thread::scope`. Each element gets its own thread with its own VM + GC arena.
- **JS transpiler** (future): Compile to `await Promise.all(list.map(async fn))`
- **Serialization**: Values serialized to JSON for cross-thread transfer (GC arena is `!Send`)

## Flow
```
[1, 2, 3] $||| { _< * 2 }
```
1. Parser: `$|||` token → Application(list, closure) with operator `$|||`
2. VM: List + Operator("$|||") → PartialOperator("$|||", list)
3. VM: PartialOperator("$|||") + Closure → parallel map:
   a. Serialize each list element to JSON via Value::to_json()
   b. Clone bytecode (code + constants Vec)
   c. std::thread::scope → spawn per element
   d. Each thread: new VM, json_to_lr_value input, run_closure_body, to_json result
   e. Collect JSON results, deserialize to Values, build result list

## Chunks

### Chunk 1: Value serialization
- Add `Value::to_json(&self) -> serde_json::Value` to value.rs
- Handles: Undefined→Null, Boolean, Number, String, List (recursive), Map (string keys)
- Operator/Closure/Error → return Null (not serializable for parallel)
- Add unit test for roundtrip

### Chunk 2: Lexer `$|||` token
- Add `$|||` as multi-char operator token in lexer
- Follow same pattern as `$?`, `$_`, `$|`, etc.

### Chunk 3: Parser handling
- Parser recognizes `$|||` as operator
- Creates same Application/PartialOperator flow as other `$` variants

### Chunk 4: VM parallel dispatch
- Add `$|||` case in PartialOperator + Closure match (line ~355)
- Use std::thread::scope for parallel execution
- Serialize inputs, spawn threads, collect JSON results, deserialize
- Preserve order (collect in order)

### Chunk 5: Tests — CLI + live + Rust unit
- Test parallel map with numbers
- Test parallel map preserves order
- Test with closures that do real computation

### Chunk 6: AGENTS.md update, commit, push

## Constraints
- GC arena Values are `!Send` — must serialize for threads
- `serde_json` already a dependency of lr-vm
- `json_to_lr_value` already exists in vm.rs (line 1950)
- `run_closure_body` takes `&[Instruction]` and `&[Constant]` — can clone for threads
- Module imports inside parallel closures not supported (new VM has no resolver)

## Future: JS transpiler
When JS transpiler exists, `$|||` compiles to:
```javascript
await Promise.all(list.map(async (x) => fn(x)))
```
The async/await design is noted for future implementation.
