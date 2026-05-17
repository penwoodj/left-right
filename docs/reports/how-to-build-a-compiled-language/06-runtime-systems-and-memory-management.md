# Report 6: Runtime Systems and Memory Management

## Executive Summary

This report explores runtime architecture and memory management strategies for the Left-Right (LR) language. We examine tradeoffs between minimal and heavyweight runtimes, garbage collection approaches, data structure representations, operator dispatch mechanisms, async runtime design, error handling, and standard library organization. The goal is to achieve maximum performance while supporting LR's dynamic typing, universal maps, and async primitives.

**LR Type System Note:** Left-Right has exactly 7 types: Operator, Map, List, String, Boolean, Number, and Undefined. There is no `Null` type — use `Undefined` instead. All operators (including `+`, `@`, `><`, `$@`, `!!!`, `!!!?`, `///`, `\\\`) are identifiers, not special tokens.

## 1. Runtime Architecture

### 1.1 Minimal Runtime vs Heavy Runtime Tradeoffs

**Minimal Runtime Approach**

A minimal runtime provides only essential services needed to execute compiled code. For LR, this includes:

- Type tags and runtime type information (RTTI)
- Memory allocation and garbage collection
- Operator dispatch machinery
- Basic runtime error handling
- FFI boundary (if targeting native)
- Import/export handling (note: `imports` is a runtime variable, not a keyword; `}@&[...]` is the export pattern, not a keyword)

Advantages:
- Faster startup time
- Smaller binary size
- Lower memory footprint
- Easier to embed in other applications
- Predictable performance characteristics
- Fewer dependencies and attack surface

Disadvantages:
- More complexity in generated code
- Less code sharing across compilation units
- Harder to evolve language features without recompiling
- Fewer optimizations available at runtime
- Limited debugging and profiling support

**Heavy Runtime Approach**

A heavy runtime includes extensive support libraries and runtime services:

- Comprehensive standard library
- JIT compilation infrastructure
- Dynamic code loading
- Rich debugging and profiling
- Serialization and deserialization
- Module system
- Platform abstraction layer

Advantages:
- More code sharing reduces binary size for multiple programs
- Runtime enables optimizations impossible at compile time
- Better tooling and debugging support
- Easier to add features without recompiling
- Consistent behavior across all programs

Disadvantages:
- Slower startup time
- Larger memory footprint
- More complexity in runtime itself
- Harder to embed in constrained environments
- Performance overhead from runtime checks

**Recommendation for LR**

LR should adopt a **lean runtime** approach for the native target. The runtime provides essential services but avoids bloat. Standard library functions are statically linked into programs that use them. This balances performance, binary size, and feature evolution.

For the JavaScript target, leverage V8's runtime services directly. No separate LR runtime needed. Compile LR idioms to JavaScript idioms and use JS's built-in capabilities.

### 1.2 Runtime Library Design

**What Belongs in the Runtime**

Core infrastructure that must be present for any LR program to execute:
- Type tag definitions and RTTI tables
- Operator dispatch cache structures
- Exception handling machinery
- Async runtime (event loop, promises)
- FFI glue code for native target
- Primitive type operations (string comparisons, arithmetic)

Note: Memory management (garbage collection or reference counting) is the VM's responsibility. LR targets a bytecode VM, so the runtime focuses on value types, not memory allocation strategies.

**What Belongs in Generated Code**

Business logic and program-specific operations:

- Program literals and constants
- Operators defined in source code
- Control flow structures
- Application-specific data structures
- Calls to runtime library functions

**What Belongs in Standard Library**

Reusable functions and operators built on runtime primitives:

- Math functions
- String manipulation
- I/O operations
- Data structure utilities
- HTTP/networking (for JS target)
- Platform-specific bindings

**Runtime Boundary**

Define clear API boundaries:
- Runtime provides C-compatible ABI for native target
- Generated code calls runtime functions via standardized interface
- No runtime code calls into generated code (callback via registration)

### 1.3 Interpreter Loop vs Compiled Code Execution

For a compiled language, LR should use **direct compiled execution**, not an interpreter loop.

**Why Not an Interpreter Loop**

An interpreter loop reads bytecode or AST nodes and dispatches to handlers. This adds overhead:
- Dispatch cost for each instruction
- No inlining across bytecode boundaries
- Harder for host compiler to optimize
- Branch misprediction on interpreter dispatch

**Why Direct Compilation**

Directly compile LR to target machine code:
- Host compiler performs full optimizations
- No dispatch overhead
- Inlining across function boundaries
- Predictable performance
- Leverage existing compiler technology (Rust's LLVM backend)

**Hybrid Approach: Runtime for Hot Paths**

Use runtime-generated code for hot paths:
- Operator dispatch uses inline caches
- Type checks use polymorphic inline caches
- Async runtime generates optimized continuations
- FFI stubs generated at runtime

This combines direct compilation with runtime specialization.

### 1.4 Runtime Type Information (RTTI) Strategies

LR requires RTTI for dynamic typing. Design options:

**Tagged Pointers**

Store type information in pointer bits:
- Use unused low bits in pointers (alignment assumptions)
- Encode small primitive types directly in value
- Larger values allocated separately with full type tags

Example layout:
- Low bits indicate type
- High bits hold value or pointer

Advantages:
- No separate type field memory overhead
- Fast type checks (bit masking)
- Compact representation for primitives

Disadvantages:
- Complexity in implementation
- Limited type encodings
- Alignment assumptions not portable
- Harder to debug

**Pointer Tagging with Header Objects**

Allocate all heap objects with a header:
- Header contains type tag, GC flags, size
- Value follows header
- Small primitives may use value boxing

Advantages:
- Simple to implement
- Extensible type system
- Works for all object types
- Easy to debug (read header)

Disadvantages:
- Memory overhead per object
- Two memory accesses for type check
- Primitive boxing overhead

**Nan-Boxing**

Use NaN encoding for double values to store types:
- IEEE 754 double has unused bit patterns for NaN
- Encode type information in NaN space
- Works well for languages with doubles

Advantages:
- No memory overhead for type info
- Fast type checks
- Works with JavaScript's number representation

Disadvantages:
- Only for double-sized values
- Complexity in bit manipulation
- Limited to 64-bit values

Note: Not applicable to LR since LR numbers are decimal only and do not include NaN or Infinity.

**Recommendation for LR**

Use **pointer tagging with header objects**:
- All reference types (Map, List, String, Operator) have headers
- Primitives (Boolean, Number, Undefined) use value boxing or immediate encoding
- Type tag in first word, GC flags in second word
- Simplicity favors maintainability and debuggability

Note: LR numbers are decimal only (no hex, binary, octal, scientific notation). Floats must start with a digit (`0.5` valid, `.5` invalid). No negative literals — `-` is always a binary operator (`0-5` for negative 5). No NaN, Infinity, or special IEEE 754 values.

Header layout (8 bytes per header on 64-bit):
```rust
struct Header {
    type_tag: u8,      // Type identifier
    gc_mark: u1,       // GC mark bit
    gc_color: u2,      // GC color (for generational)
    flags: u5,         // Future flags
    size: u32,         // Object size in bytes
    _padding: u16,     // Alignment padding
}
```

### 1.5 FFI (Foreign Function Interface) Design

**Native Target FFI**

LR running natively needs to call Rust/C functions. Design goals:
- Zero or minimal overhead for FFI calls
- Type-safe conversions where possible
- Easy to define foreign functions in LR code
- Support callbacks from foreign code into LR

Implementation:
- Define FFI syntax in LR: `foreign "c" fn printf(fmt: *i8, ...) -> i32`
- Generate Rust `extern "C"` bindings
- Automatic marshaling of LR types to C types:
  - Map, List, String convert to opaque pointers
  - Number converts to f64
  - Boolean converts to bool
  - Undefined converts to unit type
- Provide runtime functions for manual marshaling when needed
- Support registration of LR callbacks for foreign code

**JavaScript Target FFI**

LR transpiled to JavaScript can call any JS function:
- FFI is trivial. Access global JS functions directly
- Operators can reference JS functions: `{ log: console.log }`
- Type coercion handled by JavaScript's dynamic nature
- Performance cost only in actual call overhead

## 2. Memory Management

Note: Since Left-Right targets a bytecode VM, memory management (garbage collection or reference counting) is the VM's responsibility. The LR runtime focuses on value types and operator semantics, not memory allocation strategies. The discussion below is for VM implementors who need to choose a GC approach.

### 2.1 Garbage Collection Approaches

**Tracing Garbage Collection**

Mark-and-sweep or generational collector:
- Traverses object graph from roots
- Marks reachable objects
- Sweeps unmarked objects

Advantages:
- Handles cycles correctly
- No reference counting overhead
- Can compact memory
- Generational reduces pause times

Disadvantages:
- Pause times during GC
- Complex to implement correctly
- May require write barriers
- Hard to predict GC timing

**Reference Counting**

Each object tracks number of references:
- Increment on reference
- Decrement on dereference
- Free when count reaches zero

Advantages:
- Predictable deallocation timing
- No GC pauses
- Simple to implement
- Deterministic memory usage

Disadvantages:
- Cannot handle cycles without cycle detection
- Increment/decrement overhead on every operation
- Poor cache locality due to scattered counters
- Expensive for temporary objects

**Hybrid Approaches**

Combine reference counting with cycle detection:
- Use RC for most operations
- Periodically detect and collect cycles
- Often called "deferred RC" or "RC with cycle collection"

Examples:
- Python: RC with cycle collector
- Swift: ARC (Automatic Reference Counting) with cycle detection
- Nim: ARC with optional cycle collector

### 2.2 Comparison: Boehm GC, Rust's Ownership, Nim's ARC, Pony's ORCA

**Boehm GC**

Conservative mark-and-sweep garbage collector:
- Treats any bit pattern that looks like a pointer as a pointer
- Doesn't require program modifications
- Widely used in C/C++ projects (GNU Guile, Mono)

Advantages:
- Drop-in replacement for malloc/free
- Handles C/C++ code correctly
- Mature and battle-tested
- Portable

Disadvantages:
- Conservative (may keep dead objects alive)
- No generational collection by default
- Slower than precise collectors
- No control over GC timing
- Limited integration with Rust ownership

**Rust's Ownership**

Compile-time memory management without runtime GC:
- Ownership determines lifetimes
- Borrow checking prevents use-after-free
- RAII ensures deterministic cleanup

Advantages:
- Zero runtime overhead
- No GC pauses
- Memory-safe by construction
- Predictable performance
- Excellent for systems programming

Disadvantages:
- Complex ownership rules
- Hard to implement for dynamic languages
- Doesn't handle cycles well
- Requires lifetime annotations
- Steep learning curve for language designers

**Nim's ARC**

Reference counting with optional cycle collector:
- Increment/decrement on every operation
- Optional cycle detection via tracing
- Can disable RC for performance-critical code

Advantages:
- Predictable deallocation
- No long GC pauses
- Optional cycle handling
- Flexible (can disable RC)

Disadvantages:
- Reference counting overhead
- Cycles require special handling
- Not as efficient as tracing for some workloads
- Complex implementation

**Pony's ORCA (Optimistic Reference Counting with Actor concurrency)**

Actor-based GC with reference counting:
- Each actor owns its objects
- Reference counting within actors
- Message passing between actors with ownership transfer
- No global GC

Advantages:
- No pauses
- Excellent for concurrent programs
- Predictable performance
- Scales to many cores

Disadvantages:
- Requires actor model
- Complex to implement
- Not suitable for single-threaded programs
- Message passing overhead

### 2.3 Memory Allocation Strategies

**Bump Allocator**

Simple pointer-increment allocator:
- Single allocation region
- Allocate by moving pointer forward
- Deallocate by resetting pointer (whole region)
- Often used with copying GC

Advantages:
- Extremely fast allocation (pointer increment)
- Excellent cache locality
- No fragmentation
- Simple to implement

Disadvantages:
- Cannot free individual objects
- Requires periodic GC to reset
- Not suitable without copying GC
- Limited total memory

Use case: Nursery generation in generational GC.

**Slab Allocator**

Fixed-size object pools:
- Pre-allocate regions for specific object sizes
- Free lists for each size class
- Fast allocation from appropriate slab

Advantages:
- Fast allocation and deallocation
- No fragmentation within slabs
- Predictable allocation time
- Works well for object pools

Disadvantages:
- Fixed size classes (may waste space)
- Need to track multiple free lists
- May require resizing
- Not for variable-sized objects

Use case: Small objects like Headers, Lists, Strings.

**Free List Allocator**

Track free blocks in a list:
- Maintain sorted list of free blocks
- Allocate by finding first fit
- Coalesce adjacent free blocks

Advantages:
- Handles variable-sized allocations
- Can free individual objects
- Well-understood algorithm

Disadvantages:
- Fragmentation over time
- Slower than bump allocator
- Free list maintenance overhead
- Cache locality issues

Use case: General-purpose allocator for mark-sweep GC.

### 2.4 Stack vs Heap Allocation Decisions

**Stack Allocation**

Allocate on call stack when possible:
- Deterministic lifetime (scope-bound)
- Extremely fast (pointer adjustment)
- No GC overhead
- Excellent cache locality
- Automatically freed on scope exit

When can LR allocate on stack?
- Primitive values (Boolean, Number, Undefined)
- Small, short-lived objects (escape analysis)
- Intermediate values in expressions

Implementation:
- Rust compiler's escape analysis determines stack eligibility
- Compile-time analysis for LR values that don't escape
- Stack-allocated intermediate results in expressions

**Heap Allocation**

Allocate on heap when necessary:
- Reference types that escape current scope
- Large objects
- Values returned from functions
- Objects stored in collections
- Closures capturing environment

All LR reference types (Map, List, String, Operator) are heap-allocated by default, but escape analysis may promote some to stack.

### 2.5 Value Types vs Reference Types in LR

LR's type system:

**Reference Types**
- Operator (closures)
- Map
- List
- String

Allocated on heap. Identity comparison uses pointer equality.

**Primitive Types**
- Boolean
- Number
- Undefined

Value types. Stored directly. No heap allocation for standalone values.

**Value Semantics in LR**

Despite dynamic typing, LR has value-like semantics:
- Numbers, Booleans, Undefined are immutable values
- Maps, Lists, Strings have reference semantics (like JavaScript)
- Identity comparison on reference types compares pointers, not deep equality

**Hybrid: String Interning**

For string-heavy workloads, intern strings:
- Pool of unique string values
- String literals always interned
- Runtime may intern computed strings
- Identity comparison works for interned strings

Tradeoff: Memory overhead for intern table vs comparison speed.

### 2.6 Leveraging Rust's Ownership

LR compiler written in Rust can leverage Rust ownership for memory management:

**Compiler Memory Management**

Use Rust's ownership for compiler data structures:
- AST nodes owned by compilation phase
- Type checker borrows AST
- Code generation consumes typed AST
- Automatic cleanup after each phase

**Runtime Memory Management**

Rust ownership can help implement runtime GC:
- Use `Rc<RefCell<T>>` for reference counting
- Use `Arc<Mutex<T>>` for concurrent access
- Use raw pointers with unsafe for GC-managed objects
- Use `Pin` for fixed-location objects

**FFI and Ownership**

Rust ownership rules simplify FFI:
- No GC interference at FFI boundary
- Deterministic deallocation of FFI resources
- Clear ownership transfer across FFI

**Recommendation**

Use Rust ownership for compiler and FFI code. Implement tracing GC for LR runtime, but use Rust ownership for GC bookkeeping structures. Keep GC-managed memory separate from Rust-managed memory.

## 3. Data Structure Runtime

### 3.1 Map Representation

Maps are universal in LR: functions, conditionals, control flow, and data storage ALL use maps. There is no separate runtime representation for functions vs conditionals — they're all maps with different evaluation strategies.

**Map-as-function:** `{ arg: _<@0, body }` — map with `_<` references inside = unexecuted operator at runtime.

**Map-as-conditional:** `{ _<: trueCase, falseCase }` — expression key `_<` evaluates truthiness, `:` returns if truthy, falls through if not.

**Map-as-loop:** Uses iteration operators like `$`, `$?`, `$_` — these are identifiers with runtime semantics, not separate data structures.

Design options:

**Hash Map**

Standard hash table with chaining:
- O(1) average lookup
- Constant overhead per entry
- Good for sparse maps
- May have collision worst-case

Implementation:
- Use open addressing or separate chaining
- Handle collision resolution
- Support dynamic resizing
- Fast key lookup for operator dispatch

Use case: Most LR maps. Good balance of speed and memory.

**B-Tree**

Ordered tree structure:
- O(log n) lookup
- Cache-friendly (branching factor)
- Good range queries
- No resizing needed

Use case: Maps requiring ordered traversal. Not common in LR.

**Struct-Like (Fixed Layout)**

Compile-time known fields:
- Fields stored in struct
- Direct field access (no hash)
- Extremely fast
- No flexibility

Use case: When map shape known at compile time. Requires static typing or shape inference.

**Representation Choice**

Default: Hash map using open addressing.
- Use FNV or CityHash for hashing
- Linear probing for cache locality
- Robin Hood hashing for reducing variance
- Resize at 75% load factor
- Specialized version for small maps (inline storage)

Layout:
```rust
struct LrMap {
    header: Header,
    capacity: u32,     // Total slots
    size: u32,         // Used slots
    keys: [*LrValue],  // Key slots
    values: [*LrValue], // Value slots
}
```

Inline optimization for small maps:
- If size <= 4, store keys/values inline in struct
- Avoid separate allocation
- Faster access, less memory overhead

### 3.2 List Representation

Lists are dynamic arrays in LR.

Design options:

**Array List (Dynamic Array)**

Contiguous storage with dynamic resizing:
- O(1) append (amortized)
- O(1) random access
- O(n) insert/delete at position
- Cache-friendly

Implementation:
- Allocate backing array with extra capacity
- Resize by 2x when full
- Shrink by 50% when 25% full (optional)
- Store length and capacity

Layout:
```rust
struct LrList {
    header: Header,
    length: u32,
    capacity: u32,
    elements: [*LrValue], // Contiguous array
}
```

**Vector (Persistent Array)**

Immutable functional array:
- O(log n) access and update
- Never modifies in place
- Shares structure

Use case: Functional programming with persistence. Not LR's model.

**Rope (Balanced Tree)**

Tree of chunks for large strings:
- O(log n) operations
- Efficient for concatenation
- Good for text editing

Use case: Extremely large strings or frequent concatenation. Not typical LR use case.

**Representation Choice**

Default: Array list.
- Best performance for most use cases
- Familiar model (like JavaScript arrays)
- Easy to implement efficiently
- Cache-friendly

Optimizations:
- Small list optimization: inline up to 4 elements
- Capacity tracking to avoid overallocation
- Iterator support for `$` operators
- Efficient slice operations

### 3.3 String Representation

Strings are sequences of UTF-8 characters in LR.

Design options:

**UTF-8 Bytes**

Store as byte array with UTF-8 encoding:
- Compact (ASCII is 1 byte)
- Compatible with Rust and Linux
- Variable-length encoding
- O(n) random access to characters

Operations:
- Concatenation: allocate new buffer, copy
- Slicing: byte-level, may split multi-byte char
- Length: byte count vs character count
- Indexing: must find nth codepoint

Layout:
```rust
struct LrString {
    header: Header,
    length_bytes: u32,  // Byte length
    length_chars: u32,  // Codepoint length
    data: [u8],         // UTF-8 bytes
}
```

**UTF-16 (Like JavaScript)**

Store as 16-bit code units:
- Compatible with JavaScript
- Variable-length (surrogate pairs)
- O(n) random access to characters
- 2x larger for ASCII text

Use case: JavaScript target. Not needed for native.

**UTF-32 (Fixed-Width)**

Store as 32-bit code points:
- O(1) random access
- 4x larger for ASCII text
- Simple implementation

Use case: Applications with heavy random access. Rare.

**Representation Choice**

Default: UTF-8 bytes for native target.
- Best memory efficiency
- Industry standard for Rust/Linux
- Compact for ASCII-heavy text
- Fast for forward iteration

For JavaScript target, use JavaScript strings directly. Transpile LR string operations to JavaScript string methods.

Operations implementation:
- `++` (concatenate): allocate new buffer, copy both strings
- Substring: slice byte range, validate UTF-8 boundaries
- Length: track both byte length and codepoint length
- Indexing: scan UTF-8 to find nth codepoint (cache position?)

Optimizations:
- String interning for literals and common strings
- Small string optimization (inline up to 16 bytes)
- Rope representation for concatenation-heavy code
- Copy-on-write for substrings

### 3.4 Operator/Closure Representation

Operators in LR are closures with captured environment.

Representation:
```rust
struct LrOperator {
    header: Header,
    func: fn(*LrContext, *LrValue) -> LrValue, // Function pointer
    env: LrMap,                                 // Captured environment
}
```

Key components:

**Function Pointer**

Native code to execute:
- Compiled from LR operator body
- Takes context and argument
- Returns result value

For JavaScript target, store JavaScript function reference.

**Captured Environment**

Map of captured variables:
- Keys: variable names
- Values: captured values
- Formed at closure creation time

Optimizations:
- Environment chaining for nested closures
- Flat environment when possible
- Free variable analysis to minimize captures
- Closure conversion (lambda lifting)

**Context Pointer**

Runtime context passed to function:
- Current operator being executed
- Local variables map
- Call stack for error handling
- Async task context (if in async)

### 3.5 Reference Type Identity Comparison

Reference types (Map, List, String, Operator) use pointer equality for identity.

Implementation:
```rust
fn is_same(a: LrValue, b: LrValue) -> bool {
    if a.type_tag != b.type_tag {
        return false;
    }
    match a.type_tag {
        TYPE_MAP | TYPE_LIST | TYPE_STRING | TYPE_OPERATOR => {
            a.pointer == b.pointer
        }
        _ => false, // Primitives never have identity
    }
}
```

**Deep Equality (Optional)**

If LR needs deep equality (value semantics), provide separate operator:
- Deep equality for Maps: recursive comparison of key-value pairs
- Deep equality for Lists: element-wise comparison
- Deep equality for Strings: UTF-8 byte comparison
- Deep equality for Operators: pointer equality (cannot compare code)

Cost: O(n) for large structures. Use with caution.

**Identity in Operations**

Many LR operations use identity:
- Map key lookup uses identity
- List membership test uses identity
- Set operations use identity

Deep equality used explicitly when needed.

## 4. Operator Dispatch

### 4.0 Operators Are Identifiers

Important: In Left-Right, ALL operators are identifiers, not special tokens. `+`, `@`, `><`, `$@`, `!!!`, `!!!?`, `///`, `\\\` are all identifiers with runtime semantics. The lexer does not distinguish operators from other identifiers — dispatch happens based on VALUE types at runtime.

The `?` operator IS part of the Operator SDK — it converts values to boolean (truthy/falsy). It's not a missing operator or special construct.

### 4.1 Runtime Operator Lookup Strategies

Operators in LR are Map keys. Lookup pattern:

**Basic Hash Lookup**

Standard hash table lookup:
- Hash operator name (which is an identifier string)
- Probe in map
- Return value or undefined

Performance: O(1) average, but hash + probe overhead.

Note: Since operators are identifiers, there's no special runtime type dispatch for "operator tokens." All identifier lookup follows the same path — dispatch happens when the IDENTIFIED value is evaluated based on its runtime type.

**Inline Caching**

Cache lookup result at call site:
- First call: normal lookup
- Cache map and target operator
- Subsequent calls: check map shape, use cached result

Implementation:
```rust
struct InlineCache {
    map_ptr: *LrMap,
    operator_name: *LrString,
    target_func: *LrOperator,
}

// At call site
let cache = get_inline_cache(call_site_id);
if map_ptr == cache.map_ptr && name == cache.operator_name {
    // Fast path: use cached target
    return call_operator(cache.target_func, args);
} else {
    // Slow path: normal lookup and update cache
    let target = map_lookup(map, name);
    update_cache(cache, map, name, target);
    return call_operator(target, args);
}
```

Benefits:
- Eliminates hash lookup for repeated calls
- Shape check is just pointer comparison
- Massive speedup for monomorphic call sites

### 4.2 Inline Caching for Dynamic Dispatch

Inline caching is critical for dynamic languages. LR should implement:

**Monomorphic Inline Cache**

Single cached target:
- Fast check: map pointer match
- Direct call to cached function
- Works for 90%+ of call sites in typical programs

**Polymorphic Inline Cache**

Multiple cached targets:
- Cache 2-4 targets
- Check each in sequence
- Fall back to full lookup if no match
- Handles moderately polymorphic sites

**Megamorphic Fallback**

Full lookup for highly polymorphic sites:
- Too many shapes to cache
- Use slow path
- May want to generate specialized code later

**Shape-Based Caching**

Instead of caching map pointer, cache map shape:
- Map shape = set of keys
- Multiple maps with same shape share cache
- Works well for object-oriented patterns

### 4.3 Monomorphic vs Polymorphic Call Sites

**Monomorphic Call Site**

Calls same operator on same map shape:
- 95% of call sites in typical dynamic programs
- Perfect for inline caching
- Optimize heavily
- Example: `obj.method()` where `obj` is always same type

**Polymorphic Call Site**

Calls different operators or different map shapes:
- 5% of call sites
- Need polymorphic inline cache
- Accept some overhead
- Example: `obj.method()` where `obj` varies

**Megamorphic Call Site**

Many different operators and shapes:
- Rare (<1% of sites)
- Use slow lookup
- Consider JIT specialization

**Feedback-Guided Optimization**

Collect runtime statistics:
- Count polymorphic sites
- Count hot call sites
- Deoptimize when assumptions violated
- Re-optimize with new information

### 4.4 Method Dispatch Optimization Techniques

**Hidden Classes (Shapes)**

Assign each map shape a hidden class:
- Monomorphic call sites compare hidden classes
- Fast shape transitions when adding properties
- Enables inline caching and map access optimization

Note: The `.` operator in Left-Right is NOT property access — it is the reverse-args operator. It takes an unexecuted operator on its LEFT and returns a new unexecuted operator with left/right slots SWAPPED. Example: `key`@.data means: `key` string → `@` (curried get) → `.` reverses → data flows in from left as the map.

**PIC (Polymorphic Inline Cache)**

Cache multiple targets:
- Store 2-4 map pointers and targets
- Linear search in cache
- Fast path for moderate polymorphism

**IC (Inline Cache) with Guards**

Compile guards into generated code:
- Check map pointer or shape
- Direct call to target
- Bail to slow path on guard failure
- Enables inlining of called code

**JIT Compilation**

Hot call sites compiled to native code:
- Eliminate dispatch entirely
- Inline targets
- Specialize for common shapes
- Requires JIT infrastructure

### 4.5 Type Feedback Collection

**Profiling Infrastructure**

Collect runtime statistics:
- Call site polymorphism counts
- Map shape frequencies
- Hot function detection
- Type distribution of values

**Feedback Mechanisms**

- Inline cache state reveals polymorphism
- Type profiling at operations
- Counter increments for branches
- Sampling to reduce overhead

**Usage in Optimization**

- Specialize hot functions for common types
- Inline monomorphic call sites
- Generate specialized code paths
- Remove unnecessary type checks

**Deoptimization**

When assumptions break:
- Compiled code has guards
- Guard fails, deoptimize
- Fall back to interpreter or recompile
- Update feedback, re-optimize

**Recommendation for LR**

Implement inline caching first. Start with monomorphic, add polymorphic as needed. Defer JIT until performance needs justify complexity. JavaScript target relies on V8's optimization, so no LR-level optimization needed there.

## 5. Async Runtime

### 5.1 Async Primitives in LR

LR provides two async operators (identifiers, not keywords):
- `///` (make async): Takes operator on left, returns async operator
- `\\\` (await): Takes promise on left, waits for resolution

Important: `///` and `\\\` are identifiers with runtime semantics, NOT keywords. They are not reserved — the runtime recognizes them in map context.

Example:
```rust
result: \\\(\\\(fetch_url(url)))
```

### 5.2 Async Runtime Design

**Event Loop Model**

Single-threaded event loop:
- Async tasks scheduled on event loop
- Event loop drives I/O and timers
- Tasks run to completion without yielding
- Only I/O operations yield

Advantages:
- No concurrency issues
- Simple programming model
- Efficient for I/O-bound work
- Matches JavaScript model

**Futures/Promises**

Future represents value that may not be ready yet:
- Created by `///` operator (identifier)
- Can be chained with then/catch
- Resolved or rejected
- Support multiple awaiters

Implementation:
```rust
struct LrFuture {
    header: Header,
    state: FutureState,  // Pending, Resolved, Rejected
    result: Option<LrValue>,
    error: Option<LrValue>,
    awaiters: List<Waiter>,  // Tasks waiting on this future
}

enum FutureState {
    Pending,
    Resolved(LrValue),
    Rejected(LrValue),
}
```

### 5.3 Promise/Future Representation

**State Machine**

Future transitions through states:
- Pending (initial)
- Resolved (success)
- Rejected (failure)

Once resolved/rejected, state is immutable.

**Awaiters**

List of tasks waiting on future:
- When future resolves, resume awaiters
- Support multiple awaiters (shared futures)
- Remove awaiters on cancel

**Chaining**

Support promise chaining:
- `then()` schedules continuation on resolution
- `catch()` handles rejection
- `finally()` runs regardless

### 5.4 Integration with Target Runtime

**JavaScript Target**

Leverage JavaScript Promise directly:
- `///` compiles to `new Promise(...)`
- `\\\` compiles to `await`
- Use async/await in generated JavaScript
- Let V8 handle event loop and scheduling

Zero cost: V8's optimization handles async efficiently.

**Native Target**

Implement custom async runtime:
- Event loop using mio or tokio
- Futures as state machines
- Async I/O via non-blocking syscalls
- Stackful coroutines or state machines

Design options:

1. **async/await with state machines**
   - Compile `\\\` to await points
   - Generate state machines for async functions
   - Use Rust's async/await internally
   - Efficient but complex compilation

2. **Stackful coroutines**
   - Switch between tasks
   - Preserves call stack
   - Easier to implement
   - Higher memory overhead

3. **Green threads**
   - Lightweight threads
   - Run on OS threads
   - M:N scheduling
   - Requires runtime scheduler

**Recommendation**

Use Rust's async/await for native target:
- Compile LR async functions to Rust async functions
- Let Rust compiler generate state machines
- Use tokio or async-std for runtime
- Zero-cost abstractions

### 5.5 Zero-Cost Async When Possible

**Async Elision**

If async operations are immediately available:
- `\\\(value)` where value is not a future
- Compile to synchronous call
- No runtime overhead

**Inlining**

If async function is small and called once:
- Inline async function
- Eliminate future allocation
- Compile to synchronous code

**Compile-Time Analysis**

Detect cases where async can be elided:
- No I/O in async block
- No `\\\` that could yield
- Immediately resolved futures

**Cost Accounting**

Async has costs:
- Future allocation
- State machine code
- Event loop scheduling
- Context switching

Minimize these costs:
- Inline small async functions
- Elide trivial async
- Use stack allocation for futures when possible

## 6. Error Handling Runtime

### 6.1 Exception Implementation Strategies

LR provides:
- `!!! expr` = throw expr as error (`!!!` is an identifier)
- `!!!?` = catch operator (single identifier, NOT `!!!` + `?`)

Important: `!!!` and `!!!?` are identifiers with runtime semantics, NOT keywords. The parser does not treat them specially — they are recognized by the runtime in map context.

Implementation options:

**Exception Objects**

Structured error values:
- Error type (string or enum)
- Error message (string)
- Stack trace (array)
- Optional error data (map)

```rust
struct LrException {
    header: Header,
    error_type: LrString,
    message: LrString,
    stack_trace: LrList,
    data: LrMap,  // Additional error context
}
```

**Throw Mechanism**

`!!!` operator:
- Creates exception object
- Unwinds stack until catch
- Or terminates program if uncaught

Implementation:
- Use Rust's `panic!` with catch
- Or use explicit error propagation
- Or use setjmp/longjmp for C-like unwinding

**Catch Mechanism**

`!!!?` operator:
- Establishes exception handler
- Catches thrown exceptions
- Returns exception or normal result

### 6.2 Result Type vs Exception-Based Error Handling

**Result Type**

Explicit error return values:
- `Result<T, E>` type
- Caller must handle errors
- Compile-time guarantees
- No runtime overhead

Advantages:
- Explicit error handling
- No hidden control flow
- Performance: no unwinding
- Better for API design

Disadvantages:
- Verbose error handling
- Cannot escape call stack
- Requires explicit propagation
- Not LR's model

**Exception-Based**

Throw and catch exceptions:
- `!!!` throws, `!!!?` catches
- Implicit error propagation
- Can escape multiple call frames
- More convenient for error handling

Advantages:
- Convenient for error handling
- Can escape deeply
- Good for unexpected errors
- Matches LR's syntax

Disadvantages:
- Hidden control flow
- Performance cost for unwinding
- May skip cleanup
- Harder to reason about

**Recommendation**

Use exception-based for LR runtime:
- Matches LR's `!!!` and `!!!?` syntax
- Convenient for dynamic languages
- Sufficient for LR's use cases

Use Result type internally in compiler for correctness.

### 6.3 Stack Unwinding Mechanisms

**Table-Based Unwinding**

Use unwind tables (like C++ exceptions):
- Compiler generates unwind tables
- Unwind library walks stack
- Calls destructors during unwind
- Portable but complex

Use case: Interop with C++ code. Not needed for LR.

**Setjmp/Longjmp**

C-style stack unwinding:
- `setjmp` saves stack state
- `longjmp` restores stack
- Skips intermediate frames
- No destructor calls

Use case: Simple unwinding, no cleanup needed. Fast.

**Rust's Panic**

Use Rust's panic mechanism:
- `panic!` aborts or unwinds
- `catch_unwind` catches panics
- Calls Drop during unwind
- Well-tested in Rust

Implementation:
```rust
fn lr_throw(exception: LrException) -> ! {
    panic!(LrPanic { exception })
}

fn lr_catch(f: fn() -> LrValue) -> Result<LrValue, LrException> {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(value) => Ok(value),
        Err(boxed) => match boxed.downcast::<LrPanic>() {
            Ok(lr_panic) => Err(lr_panic.exception),
            Err(_) => Err(create_exception("Panic", "Non-LR panic")),
        }
    }
}
```

### 6.4 Error Propagation Cost

**Throw Cost**

Creating and throwing exception:
- Allocate exception object
- Unwind stack (walk frames)
- Call cleanup (if using panic)
- Find handler

Cost: O(stack depth). Expensive but rare.

**Catch Cost**

Establishing exception handler:
- Register handler (setjmp or catch_unwind)
- Minimal overhead when no exception thrown

Cost: O(1) for handler registration.

**Zero-Cost When No Errors**

When no errors thrown:
- Handler registration is cheap
- No runtime checks needed
- Code runs at full speed

**Optimization**

Minimize error handling cost:
- Use stack allocation for exceptions when possible
- Avoid exceptions in hot loops
- Use Result type for expected errors in runtime
- Reserve exceptions for truly exceptional cases

**Recommendation**

Use Rust's panic/catch_unwind for LR's `!!!` and `!!!?`. Good balance of functionality and performance. Avoid using exceptions in hot paths of the runtime itself.

## 7. Standard Library Design

### 7.1 What Belongs in Stdlib vs Language Runtime

**Runtime (Required by All Programs)**

Core infrastructure:
- Memory management
- Type system
- Operator dispatch
- Error handling
- Async runtime
- FFI boundary

**Standard Library (Optional but Common)**

Utility functions and operators:
- Math functions
- String operations
- I/O operations
- Data structure utilities
- Platform-specific bindings

**Design Principle**

Minimal runtime. Extensive standard library. Runtime provides primitives. Stdlib builds on primitives.

### 7.2 Iterator Protocol Design for $ Operators

LR's `$` operator iterates over collections. Need iterator protocol.

**Iterator Protocol**

Standard interface:
- `$iterator(collection)` returns iterator
- `has_next(iterator)` returns boolean
- `next(iterator)` returns next value or undefined

Implementation:
```rust
struct LrIterator {
    header: Header,
    collection: LrValue,
    state: LrValue,  // Iterator-specific state
    has_next_fn: fn(LrIterator) -> bool,
    next_fn: fn(LrIterator) -> LrValue,
}
```

**Collection-Specific Iterators**

Each collection type provides iterator:
- List iterator: index-based
- Map iterator: key-value pairs
- String iterator: characters or bytes

**Lazy Evaluation**

Iterators are lazy:
- Don't materialize entire collection
- Compute values on demand
- Enable infinite collections
- Memory efficient for large data

**$ Operator Implementation**

`$` uses iterator protocol:
- Create iterator from collection
- Loop: if has_next, call next
- Collect results if needed
- Pipe to next operator

### 7.3 Built-in Operators as Runtime Functions vs Intrinsics

**Runtime Functions**

Operators implemented in runtime library:
- `+`, `-`, `*`, `/` as runtime functions
- Called like normal functions
- Dispatch through map lookup
- Flexibility: user can override

**Intrinsics**

Operators implemented directly in compiler:
- Known to compiler
- Direct code generation
- No runtime call overhead
- Cannot be overridden

**Hybrid Approach**

Common operators as intrinsics:
- Arithmetic: `+`, `-`, `*`, `/`
- Comparison: `<`, `>`, `=`, `!=` (note: `=` is equality, not `==`)
- Logical: `&&`, `||`, `!`

Important: `=` is the equality operator (identity comparison for reference types), not assignment. Assignment happens via map key `:` with alpha-start keys. `:` has TWO behaviors:
- **Alpha-start key** (`name: expr`) → assignment: creates variable AND key-value pair
- **Expression key** (`_<: expr` or `expr: expr`) → early return: if key evaluates truthy, immediately return the value, skip remaining map entries

Rare operators as runtime functions:
- Custom operators defined in code
- User-overridable operators
- Complex operations

**Benefits**

Intrinsics: Performance, predictability, optimization opportunities.
Runtime functions: Flexibility, consistency, language extensibility.

**Recommendation**

Make core arithmetic and comparison operators intrinsics. Other operators (especially user-defined) are runtime functions. JavaScript target uses JavaScript operators directly for intrinsics.

### 7.4 String Operations Runtime

**Basic Operations**

Runtime provides:
- Concatenation (`++`)
- Substring (slicing)
- Length (bytes and characters)
- Comparison
- Split
- Join
- Trim

Implementation:
- Use Rust's string operations
- Handle UTF-8 correctly
- Validate UTF-8 on construction
- Provide both byte and character operations

**Advanced Operations**

Stdlib provides (not runtime):
- Regex matching
- Case conversion
- Unicode normalization
- Encoding/decoding
- Formatting

**Performance Considerations**

String operations are expensive:
- Concatenation allocates new string
- Slicing may copy or reference
- Regex is especially slow
- Minimize allocations in hot paths

**Optimizations**

- Small string optimization (SSO)
- String interning for literals
- Copy-on-write for substrings
- Lazy evaluation for concatenation

### 7.5 Math Operations Runtime

**Basic Operations**

Intrinsics for:
- `+`, `-`, `*`, `/`
- `%`, `**`, `<<`, `>>`
- Unary `-`, `~`

Runtime functions for:
- `abs`, `min`, `max`, `round`, `floor`, `ceil`
- `sin`, `cos`, `tan`, `sqrt`, `log`

**Number Representation**

LR numbers are decimal only:
- Integer or float (decimal point notation)
- No hex, binary, octal, or scientific notation
- Floats must start with a digit (`0.5` valid, `.5` invalid)
- No negative literals — `-` is always a binary operator (`0-5` for negative 5)
- No NaN, Infinity, or special IEEE 754 values
- Operations fail if operand types are incompatible

**Edge Cases**

Handle special cases:
- Underflow, overflow (dependent on f64 backend)
- Division by zero (error)
- Invalid operations (error)

**Optimizations**

- Use hardware floating point
- Inline simple operations
- Fast path for integer arithmetic when both operands are integers

## 8. Performance-Critical Runtime Decisions

### 8.1 JIT Warmup vs AOT-Compiled Runtime

**AOT-Compiled (Ahead-of-Time)**

Compile to machine code before execution:
- All code compiled upfront
- No runtime compilation overhead
- Predictable performance
- Simpler runtime

Advantages:
- No warmup period
- Stable performance
- Smaller runtime
- Easier to debug

Disadvantages:
- Cannot use runtime feedback
- Less optimization potential
- Fixed code generation

**JIT-Compiled (Just-in-Time)**

Compile code at runtime:
- Compile hot functions
- Use runtime feedback
- Speculative optimization
- Deoptimize on speculation failure

Advantages:
- Better optimization potential
- Runtime type feedback
- Self-optimizing
- Can handle dynamic code

Disadvantages:
- Warmup period
- Larger runtime (JIT compiler)
- Complexity
- Unpredictable performance

**Recommendation for LR**

Use AOT compilation for native target:
- Rust compiler provides excellent optimization
- No JIT complexity in runtime
- Fast startup, consistent performance
- Easier implementation

JavaScript target uses V8's JIT, so LR benefits indirectly.

### 8.2 Startup Time Optimization

**Minimize Initialization**

- Lazy load stdlib components
- Initialize runtime lazily
- Avoid heavy computation at startup

**Reduce Binary Size**

- Static linking for stdlib
- Dead code elimination
- Strip debug symbols in release

**Fast GC Initialization**

- Pre-allocate initial heap
- Avoid first GC if possible
- Use bump allocator for nursery

**Reduce File I/O**

- Use memory-mapped files for assets
- Load modules on demand
- Cache compiled bytecode

### 8.3 Memory Footprint Minimization

**Object Layout**

- Compact headers
- Small object optimization
- Inline small values

**Memory Management**

- Efficient allocator
- Generational GC to reduce retention
- Frequent GC for short-lived objects

**String and Collection Internals**

- Small string optimization
- Compact list representation
- Hash map with low overhead

**Code Size**

- Share runtime code across programs
- Use shared libraries for stdlib
- Avoid code duplication

### 8.4 Cache-Friendly Data Layout

**Data Locality**

- Keep related data together
- Use contiguous arrays
- Avoid pointer chasing

**Alignment**

- Align structures to cache line boundaries
- Pad to avoid false sharing
- Use hot-cold splitting

**Prefetching**

- Prefetch memory before use
- Predict access patterns
- Use hardware prefetch

**Example: Cache-Friendly List**

```rust
struct LrList {
    header: Header,
    length: u32,
    capacity: u32,
    elements: [*LrValue], // Contiguous, cache-friendly
}
```

Avoid linked lists or scattered allocations.

## 9. Implementation Recommendations

### 9.1 Runtime Architecture

**Native Target**

Implement lean runtime in Rust:
- Memory management with generational GC
- Type information with header objects
- Operator dispatch with inline caching
- Async using Rust async/await
- Error handling with panic/catch_unwind
- Minimal FFI boundary

**JavaScript Target**

Leverage V8 runtime:
- No separate LR runtime
- Compile LR idioms to JavaScript
- Use JavaScript objects for maps and lists
- Use JavaScript promises for async
- Use JavaScript errors for exceptions
- Zero overhead for LR-specific features

### 9.2 Memory Management Strategy

**Native Target**

Implement generational mark-sweep GC:
- Nursery generation for young objects
- Old generation for long-lived objects
- Bump allocator for nursery
- Free list allocator for old gen
- Write barriers for generational collection

Why generational:
- Most objects die young (generational hypothesis)
- Reduces GC pause times
- Good balance of throughput and latency

Alternative: Consider Rust ownership for select parts:
- Use `Rc` for reference-counted values
- Use `Arc` for concurrent access
- Mix GC and ownership for best of both worlds

**JavaScript Target**

Rely on V8's GC. No separate memory management needed.

### 9.3 Operator Dispatch Implementation

**Native Target**

Implement inline caching:
- Monomorphic inline cache (primary)
- Polymorphic inline cache (fallback)
- Megamorphic slow path (rare)

Implementation:
- Cache struct at each call site
- Shape check (map pointer or hidden class)
- Direct call to cached function
- Update cache on miss

Benefits:
- Massive speedup for monomorphic sites
- Minimal overhead for cache check
- Simple to implement

**JavaScript Target**

Rely on V8's inline caching. No LR-level optimization needed.

### 9.4 Async Runtime Design

**Native Target**

Use Rust async/await:
- Compile LR `///` to Rust async
- Compile LR `\\\` to Rust await
- Use tokio or async-std for event loop
- Zero-cost futures when possible

Why Rust async:
- Zero-cost abstractions
- Excellent ergonomics
- Well-tested
- Efficient state machine generation

**JavaScript Target**

Compile to async/await:
- `///` → `new Promise(...)`
- `\\\` → `await`
- Let V8 handle optimization
- Native JavaScript promises

### 9.5 Prioritized Implementation

**Phase 1: Core Runtime**
- Memory allocator and basic GC
- Type system and header objects
- Basic data structures (Map, List, String)
- Operator dispatch (without inline caching)
- Error handling with panic/catch_unwind

**Phase 2: Performance**
- Generational GC
- Inline caching for dispatch
- Small object optimizations
- String interning

**Phase 3: Async**
- Rust async/await integration
- Event loop setup
- Promise/future representation

**Phase 4: Optimizations**
- Specialized allocators
- Advanced inline caching
- JIT feedback (if needed)
- Profiling and debugging tools

## Conclusion

LR's runtime should be lean and efficient, balancing simplicity with performance. Use Rust's strengths (ownership, async/await, LLVM backend) for the native target, and leverage V8's optimizations for the JavaScript target. Generational GC, inline caching, and careful data layout provide high performance without excessive complexity. The runtime provides essential infrastructure while the standard library provides reusable functionality. This architecture enables maximum performance for LR's dynamic typing, universal maps, and async primitives.