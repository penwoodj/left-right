# Import/Export Implementation Plan

## Goal
Implement `files@"path"` local file loading. `imports@"path"` stays stubbed.
Export via `}@&[key1, key2]` picks keys from enclosing map.

## Architecture Decision
VM executes imported module bytecode **within the same GC arena** via nested `run_dispatch()` call.
No serialization/deserialization needed — Values share the arena.

## Components

### 1. ModuleResolver Trait (lr-vm)
```rust
pub trait ModuleResolver {
    fn resolve_path(&self, import_path: &str, current_file: &str) -> Result<String, String>;
    fn compile_source(&self, resolved_path: &str) -> Result<Chunk, String>;
}
```
- `resolve_path`: resolve relative path from current file location
- `compile_source`: read + parse + compile .lr file → Chunk

### 2. VM Changes
- Add `module_resolver: Option<Rc<RefCell<dyn ModuleResolver>>>`
- Add `source_path: String`
- Add `module_cache: HashMap<String, Chunk>` (resolved path → compiled bytecode)
- New constructor: `VM::with_resolver(source_path, resolver)`

### 3. Import Opcode
Inside `run_dispatch` (we already have `mc` arena context):
1. Get path string from register `inst.b()`
2. Determine source type from ImportExpression AST source field
3. For `files@"path"`:
   a. Call resolver.resolve_path(path, source_path)
   b. Check module_cache for compiled Chunk
   c. If miss: call resolver.compile_source(resolved_path), cache it
   d. Create new Frame, call self.run_dispatch(mc, &mut new_frame, &chunk.code, &chunk.constants)
   e. Set result Value in register inst.a()
4. For `imports@"path"`:
   a. Return empty map or error (stub)

### 4. Export Opcode
Current compiler emits:
- LoadConstant for each key (into same register — BUG)
- Export(dest, keys_count, 0)

Redesign: Export picks keys from current value.
- Compiler: load all keys as constants, emit Export with count
- VM: read N constants from preceding LoadConstant instructions,
  pick those keys from the value in dest register, return picked map

### 5. Parser: `}@&[...]` Export Pattern
Currently MISSING. Need to add parser recognition for:
- Close brace `}` followed by `@&` followed by `[key1, key2]`
- Creates ExportExpression AST node

### 6. CLI Integration
- main.rs: Implement ModuleResolver using lr_parser + lr_compiler
- Wire into run_source: pass resolver to VM
- Path resolution: std::fs::canonicalize or relative path join

## Chunks

### Chunk A: ModuleResolver trait + VM plumbing
- Define trait in lr-vm
- Add fields to VM (resolver, source_path, module_cache)
- New constructor
- No behavior change yet — all existing tests still pass

### Chunk B: Import opcode implementation
- Replace Import stub with actual file loading
- files@ → resolve, compile, execute
- imports@ → return empty map
- Guard against circular imports

### Chunk C: Export opcode fix
- Fix compiler emission (keys storage)
- Implement VM Export: pick keys from value
- Verify with existing tests

### Chunk D: Parser `}@&[...]` export pattern
- Add parser recognition
- Wire to ExportExpression AST

### Chunk E: CLI integration
- Implement ModuleResolver for CLI
- Wire into run_source, cmd_test, cmd_run

### Chunk F: Tests
- Rust unit tests for module loading
- CLI tests: import local .lr file
- Live tests: import local .lr file

### Chunk G: AGENTS.md update

## Risk Areas
- GC arena: nested run_dispatch must work correctly
- Circular imports: need visited set or depth limit
- Path resolution: relative vs absolute, cross-platform
- Export compiler bug: current LoadConstant overwrites registers
