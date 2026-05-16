# Report 4: Intermediate Representations and Optimization

## Overview

This report covers the design and implementation of intermediate representations (IRs) and optimization strategies for the Left-Right compiler. The IR sits between the parser and the code generator, providing a structured representation amenable to analysis and transformation. Proper IR design and optimization passes are critical for achieving maximum performance in a compiled language.

## 1. Intermediate Representation Design

### AST vs IR: Why Lower from AST

The Abstract Syntax Tree (AST) produced by parsing captures the syntactic structure of source code. While useful for semantic analysis and type checking, the AST has limitations for optimization:

**AST Limitations:**
- Preserves syntactic detail that is irrelevant for optimization
- Complex nested structures make pattern matching difficult
- No guarantee about execution order or side effects
- Implicit scopes and variable lifetimes
- Redundant parentheses and grouping constructs

**IR Advantages:**
- Canonicalized form reduces semantic equivalence complexity
- Explicit control flow (blocks, branches, loops)
- Lowered operators eliminate syntactic sugar
- Temporary values are explicit, enabling better register allocation
- Uniform representation simplifies optimization passes

**Lowering Process:**
1. AST nodes are converted to IR instructions
2. Temporary variables are introduced for intermediate values
3. Complex expressions are flattened to sequences of simple operations
4. Control flow is made explicit (basic blocks, conditional jumps)
5. Side effects and dependencies are made clear

For Left-Right, this means converting operator chains like `1 $plus 2 $times 3` into explicit temporaries with clear evaluation order, eliminating left-to-right currying ambiguity in the IR.

### SSA (Static Single Assignment) Form

SSA is a property of IR where every variable is assigned exactly once. This simplifies dataflow analysis and enables powerful optimizations.

**Key Concepts:**
- Each variable has a single definition point
- Phi functions merge values from different control flow paths
- Variables become immutable after definition
- Def-use chains are explicit and unambiguous

**Example Transformation:**
```
x = 1
if condition:
    x = 2
print(x)
```

Becomes:
```
x1 = 1
if condition:
    x2 = 2
else:
    x2 = x1
x3 = phi(x1, x2)
print(x3)
```

**SSA Benefits for Left-Right:**
- Curried partial applications can be represented clearly
- Operator chain dependencies are explicit
- Dataflow analysis for type inference is straightforward
- Dead code elimination is trivial (unused definitions)
- Register allocation algorithms work directly

**SSA Implementation Considerations:**
- Insertion of phi functions requires dominance frontier computation
- Converting out of SSA is needed before code generation
- Memory operations and mutable state require special handling
- Loop carried values need phi nodes at loop headers

### CPS (Continuation-Passing Style) for Curried Languages

CPS makes control flow and continuation explicit. Each function receives an extra argument (the continuation) representing "what to do next".

**CPS Structure:**
```javascript
// Direct style
function add(x, y) {
    return x + y
}

// CPS
function add_cps(x, y, k) {
    k(x + y)
}
```

**For Left-Right, CPS is Natural:**
- Currying is explicit: `f x y` becomes `f x (k1 -> y (k2 -> k1 result))`
- Partial application is represented as partial continuation construction
- Control flow (via maps with expression keys) becomes continuations
- Error handling and exceptions are explicit continuation calls

**CPS IR Benefits:**
- Control flow becomes data flow (no stack, no return addresses)
- Tail call optimization is trivial (jump to continuation)
- Coroutines and async transformations are straightforward
- First-class continuations enable advanced language features

**CPS Challenges:**
- Heap allocation for continuations (performance overhead)
- Requires aggressive optimization to eliminate redundant allocations
- Harder to debug (call stack is replaced by continuation chain)
- Code size explosion due to explicit continuations

**Hybrid Approach:**
Left-Right could use CPS only for complex control flow, using SSA for hot computation paths. This balances expressiveness with performance.

### Three-Address Code, Bytecode, and Graph IRs

**Three-Address Code:**
- Each instruction has at most three operands (result, op1, op2)
- Simple linear representation
- Easy to parse and generate
- Memory heavy (many temporaries)
- Example: `t1 = a + b; t2 = t1 * c`

**Bytecode:**
- Compact, instruction-level encoding
- Virtual machine based execution
- Platform independent
- Easy to serialize and distribute
- Example: `[push a, push b, add, push c, mul, store t2]`

**Graph IRs:**
- Represent code as a directed graph
- Nodes are operations, edges are data/control dependencies
- Enable advanced optimizations at graph level
- Example: LLVM IR, SSA graph, dataflow graph

**Left-Right Considerations:**
- Three-address code: good for intermediate representation, easy to optimize
- Bytecode: useful for interpreter mode or distribution format
- Graph IR: essential for advanced optimizations (loop vectorization, fusion)
- Hybrid: start with three-address, convert to graph for optimization passes

### Sea of Nodes IR (Used by V8/HotSpot)

Sea of Nodes is a graph IR where instructions are nodes and edges represent dependencies. No explicit basic blocks or temporaries.

**Characteristics:**
- Nodes are operations (add, load, call, etc.)
- Edges are data dependencies (value flow) or control dependencies
- Nodes can be reordered freely as long as dependencies are preserved
- No explicit program order (dependencies determine execution)
- Enables global scheduling and optimization

**Example:**
```
// Source
if x > 0:
    y = x + 1
else:
    y = x - 1
z = y * 2

// Sea of Nodes
[Z] <-- [*] <-- [Y] <-- [+1] <-- [X]
           |        ^        ^------[Const 1]
           |        |        |
           +--------|-------|------[>0]
                    |       |
                    +-------[-1] <-- [Const -1]
```

**Benefits for Left-Right:**
- Operator chains become dependency graphs
- No need for explicit temporaries (edges represent flow)
- Loop invariant code motion is trivial
- Type specialization can be represented as node variants
- Operator fusion is natural (merge dependent nodes)

**Implementation:**
- Each Left-Right operator becomes a node type
- Curry and partial application become projection nodes
- Map iteration becomes loop nodes with data dependencies
- Type guards and checks become guard nodes

### MLIR (Multi-Level IR) Approach

MLIR (Multi-Level Intermediate Representation) provides a framework for building IRs with multiple abstraction levels.

**MLIR Structure:**
- Dialects: collections of operations at a specific abstraction level
- Example dialects: tensor, linalg, LLVM, GPU, vector
- Operations can be lowered between dialects
- Type system and attributes are extensible
- Supports custom analyses and transformations

**Benefits:**
- Gradual lowering: high-level ops to low-level ops
- Target-specific optimizations at appropriate levels
- Reusable dialects (e.g., vector dialect for SIMD)
- Extensible with custom dialects for language features

**MLIR for Left-Right:**
- High-level dialect: Left-Right operators ($map, $filter, $reduce)
- Array dialect: array operations, broadcasting, shape inference
- Loop dialect: explicit loops and iteration
- Vector dialect: SIMD operations for optimization
- LLVM dialect: target code generation

**Example Lowering:**
```
// Left-Right Dialect
%result = lr.map %array, %fn

// Array Dialect
%result = array.map %array, %fn

// Loop Dialect
%result = loop.for %i in 0..len {
    %elem = array.extract %array[%i]
    %mapped = call %fn(%elem)
    array.insert %result[%i], %mapped
}

// Vector Dialect (optimized)
%result = vector.map %array, %fn
```

### Choosing IR Level for Our Language

Left-Right requires a multi-level IR approach:

**Level 1: High-Level IR (HIR)**
- Direct mapping from AST
- Preserves Left-Right semantics
- Currying and partial application explicit
- Type inference and checking passes
- Semantic validation

**Level 2: Mid-Level IR (MIR)**
- SSA form for dataflow optimization
- Explicit temporaries and basic blocks
- General purpose optimizations (DCE, CSE, inlining)
- Control flow simplification
- Loop canonicalization

**Level 3: Low-Level IR (LIR)**
- Target-specific or bytecode representation
- Explicit register allocation
- Machine code generation
- Backend optimizations

**Justification:**
- HIR maintains Left-Right semantics for analysis
- MIR enables SSA-based optimizations without implementation details
- LIR bridges to specific targets (JS or Rust/MIR)
- Gradual lowering allows optimization at appropriate levels

## 2. IR for Operator-Based Languages

### Important: Identifiers in Left-Right

**Key clarification:** In Left-Right, `$` is NOT a special prefix or sigil. `$plus`, `$times`, `$map`, `add`, `mul` are ALL regular identifiers. The `$` character is simply part of the identifier name, used by convention for operators. The lexer treats all these the same way.

**All operators are identifiers:**
- `$plus` is an identifier (name contains `$`)
- `$times` is an identifier
- `$map` is an identifier
- `add` is an identifier
- `mul` is an identifier
- `!!!`, `!!!?`, `///`, `\\\` are identifiers (multi-character operators)

No special token types exist for operators. Operator semantics emerge at runtime based on value types, not at lex/parse time.

### How APL/J Represent Operations in IR

APL and J use array-oriented, operator-based syntax. Their IR design handles arrays uniformly and optimizes array operations.

**Key Principles:**
- Rank polymorphism: operations work on arrays of any rank
- Implicit iteration: operators automatically iterate over arrays
- Temporary arrays are created lazily
- Fusion of array operations to eliminate temporaries

**APL IR Characteristics:**
- Array shapes are tracked throughout
- Operations are annotated with rank information
- Broadcast operations are explicit
- Index spaces are abstract (no loop nests)

**Example Representation:**
```
// APL source
A + B × C

// IR
%t1 = mul(%B, %C)  // elementwise multiply, rank broadcast
%result = add(%A, %t1)
```

**For Left-Right:**
- Similar rank polymorphism for operators
- Track array shapes and dimensions
- Lazy evaluation of intermediate arrays
- Fusion of operator chains to avoid temporary allocation

### Curried Function IR Representation

Currying transforms multi-argument functions into chains of single-argument functions. IR must capture partial applications efficiently.

**Representation Strategies:**

1. **Lambda Lifting:**
   - Convert closures to top-level functions with environment parameters
   - Partial application creates function pointer + environment tuple
   - Example: `add(1)` becomes `lambda x: add(1, x)`

2. **Direct Currying Encoding:**
   - Represent partial application as (function, applied_args) tuple
   - Call sites check for partial application and dispatch
   - Closure allocation only when needed

3. **Specialized Representation:**
   - Encode common arities as variants
   - Partial application caches specialized functions
   - Avoid closure allocation for known patterns

**Left-Right Currying in IR:**
```
// Source
add = $plus  // partial application of $plus
result = add 5  // complete the partial application

// IR
%plus_func = get_operator($plus)
%partial = make_partial(%plus_func, arity=2, applied=[<none>])
%add = bind_arg(%partial, 0, <none>)  // Left argument not applied yet
%result = call_curried(%add, 5)  // Apply right argument
```

**Optimization:**
- Detect fully-saturated calls and convert to direct calls
- Inline small curried functions
- Specialize for known argument patterns
- Eliminate partial application for known fixed arities

### Map-as-Function Lowering to IR

Maps in Left-Right serve as functions, conditionals, and control flow. IR must handle these multiple roles.

### Left-Right-Specific IR Patterns

**Silent Execution (`_: expr`):**
The `_` identifier with `:` delimiter executes an expression silently, discarding the result.

```
// Source
_: expensive_computation()
_: side_effect_function()

// IR
%tmp1 = call(expensive_computation)
%tmp2 = call(side_effect_function)
// Results discarded, only side effects matter
```

**Spread/Merge (`+: expr`):**
The `+` identifier with `:` delimiter spreads/merges key-values from the right map into the left map.

```
// Source
{a: 1, b: 2} +: {c: 3, d: 4}

// IR
%left_map = {a: 1, b: 2}
%right_map = {c: 3, d: 4}
%result = map_merge(%left_map, %right_map)
```

**Error Handling (`!!!`, `!!!?`):**
These are identifiers, not special IR constructs. `!!!` throws, `!!!?` catches.

```
// Source
!!! error_value
!!!? operator_to_protect { _: recover }

// IR
%err = throw(error_value)
%result = try_catch(operator_to_protect, recover_func)
```

**Async/Await (`///`, `\\\`):**
These are identifiers for async operations, not special IR constructs.

```
// Source
/// async_operation
\\\ await_promise

// IR
%async_op = make_async(async_operation)
%result = await(await_promise)
```

**Import/Export:**
`imports` is a runtime variable (map), exports use the `}@&[...]` pattern.

```
// Source
imports@[`lodash`, `fp`, `map`]
}@&[`add`, `mul`, `div`]

// IR
%imports_var = load_runtime_var("imports")
%module = list_lookup(%imports_var, ["lodash", "fp", "map"])
%exports = filter_keys(current_scope, ["add", "mul", "div"])
set_export_pattern(%exports)
```

**Note:** These are NOT special IR nodes. They are represented using standard IR operations (call, map operations, etc.).

### Map-as-Function Lowering to IR (continued)

Maps in Left-Right serve as functions, conditionals, and control flow. IR must handle these multiple roles.

**Representation Options:**

1. **Dictionary Lookup:**
   - Maps become hash table lookups
   - Function call is map lookup + invocation
   - Simple but loses optimization opportunities

2. **Switch Table:**
   - Compile-time known maps become switch statements
   - Fast dispatch for static maps
   - Requires map keys to be known statically

3. **Hidden Classes:**
   - Track map shapes dynamically
   - Generate specialized code for common shapes
   - Enables inline caching

**Map-as-Function IR:**
```
// Source
%func = {"name": "add", "arity": 2}
%result = %func.call(1, 2)

// IR
%func_map = load_map(%func)
%add_func = map_lookup(%func_map, "call")
%result = call(%add_func, %func, 1, 2)
```

**Map-as-Conditional IR:**
```
// Source
%cond_map = {true: 1, false: 0}
%result = %cond_map[%condition]

// IR
%cond_map = load_map(%cond_map)
%key = to_bool(%condition)
%result = map_lookup(%cond_map, %key)
```

**Optimization:**
- Inline map lookups for static maps
- Convert map-based conditionals to branches
- Specialize map call sites based on shape
- Cache map lookups inline

### Partial Application IR Encoding

Partial application is core to Left-Right. IR must efficiently represent and optimize partially applied operators.

**Encoding Strategies:**

1. **Tuple Representation:**
   ```
   %partial = (%func, %arg1, %arg2, ...)
   %result = apply_partial(%partial, %remaining_args)
   ```

2. **Closure Representation:**
   ```
   %closure = {func: %func, env: {arg1: %arg1, arg2: %arg2}}
   %result = call_closure(%closure, %remaining_args)
   ```

3. **Bitmask Representation:**
   ```
   %partial = {
       func: %func,
       args: [%arg1, %arg2, %arg3, %arg4],
       mask: 0b1010  // Bits 1 and 3 applied
   }
   %result = call_partial(%partial, %remaining_args)
   ```

**Left-Right Partial Application:**
```
// Source
$plus 1  // Apply left argument
$times 2  // Apply right argument

// IR (bitmask encoding)
%plus_op = get_operator($plus)
%partial_plus = {
    func: %plus_op,
    args: [<none>, 1],
    mask: 0b10  // Right argument applied
}

%times_op = get_operator($times)
%partial_times = {
    func: %times_op,
    args: [2, <none>],
    mask: 0b01  // Left argument applied
}
```

**Optimization:**
- Detect and eliminate redundant partial applications
- Combine chained partial applications
- Inline calls to fully-saturated partial applications
- Specialize for common arity patterns

### Operator Fusion Opportunities

Operator fusion combines multiple operations into a single composite operation, reducing overhead and enabling better optimization.

**Fusion Candidates:**
- Element-wise array operations: `map(add, map(mul, arr))` → `map(x => add(mul(x)), arr)`
- Filter-map chains: `filter(pred, map(fn, arr))` → fused iteration
- Reduce-map patterns: `reduce(add, map(fn, arr))` → map-reduce fusion
- Composition: `(f . g)(x)` → fused function

**IR-Level Fusion:**
```
// Before fusion
%t1 = map(%arr, %fn1)
%result = map(%t1, %fn2)

// After fusion
%result = map_fused(%arr, %fn1, %fn2)
// or
%result = map(%arr, %composed_fn)  // where %composed_fn = fn2 . fn1
```

**Fusion Implementation:**
- Pattern matching on IR graph
- Detect compatible operator chains
- Replace chain with fused operation
- Update type information and shape inference

**Fusion Benefits for Left-Right:**
- Eliminate temporary array allocations
- Enable vectorization across fused operations
- Reduce function call overhead
- Improve cache locality

**Fusion Challenges:**
- Side effects in operators prevent fusion
- Complex control flow breaks fusion patterns
- Dynamic typing requires type guards
- Debugging becomes harder after fusion

## 3. Optimization Passes

### Constant Folding and Propagation

Constant folding evaluates constant expressions at compile time. Constant propagation replaces variables with their known constant values.

**Constant Folding:**
```
// Before (Left-Right: zero precedence, left-to-right)
// Source: 1 + 2 * 3  evaluates as ((1 + 2) * 3)
%t1 = 1 + 2
%result = %t1 * 3

// After
%result = 9
```

**Constant Propagation:**
```
// Before
%x = 42
%y = %x + 1

// After
%x = 42
%y = 43
```

**Implementation:**
- Worklist algorithm: iterate through instructions
- Fold constant expressions to constants
- Propagate constants to uses
- Repeat until fixed point
- Remove dead constant definitions

**For Left-Right:**
- Fold operator chains with constant arguments
- Propagate constant map entries
- Eliminate constant conditionals
- Specialize operators based on constant types

**Example:**
```
// Source
1 $plus 2 $times 3

// IR (before)
%t1 = 1 + 2
%result = %t1 * 3

// IR (after constant folding)
%result = 9
```

### Dead Code Elimination (DCE)

Dead code elimination removes code that does not affect program output.

**Dead Code Types:**
1. **Unused definitions:** Variables with no uses
2. **Unreachable code:** Code that cannot be executed
3. **Dead stores:** Stores to memory that are never read
4. **Unused functions:** Functions that are never called

**DCE Algorithm:**
- Mark all instructions as live or dead
- Start from outputs (return values, side effects)
- Propagate liveness backward
- Remove dead instructions
- Repeat until fixed point

**For Left-Right:**
- Remove unused operator applications
- Eliminate unused map entries
- Prune dead branches in conditionals
- Remove unused partial applications

**Example:**
```
// Before
%unused = 1 + 2
%result = 3 + 4

// After
%result = 7
```

### Inlining Strategies for Operator Chains

Inlining replaces function calls with the function body. For operator-based languages, this includes inlining small operators and curried functions.

**Inlining Benefits:**
- Eliminates function call overhead
- Enables constant propagation across call boundaries
- Exposes further optimization opportunities
- Reduces indirection in operator chains

**Inlining Challenges:**
- Code size explosion
- Increased compilation time
- May inhibit other optimizations (e.g., function specialization)
- Harder to debug (loss of call stack)

**Inlining Heuristics:**
1. **Function size:** Inline small functions (e.g., < 10 instructions)
2. **Call frequency:** Inline hot call sites
3. **Argument count:** Inline functions with few arguments
4. **Recursion:** Never inline recursive functions (or inline once)

**For Left-Right:**
- Inline small operators (arithmetic, comparison)
- Inline curried functions that are fully saturated
- Inline map-based functions when shape is known
- Inline composition of operators

**Example:**
```
// Source
add = $plus
result = add 5 5

// Before inlining
%add = get_operator($plus)
%result = call(%add, 5, 5)

// After inlining
%result = 5 + 5
```

**Advanced Inlining:**
- Speculative inlining based on type feedback
- Inline caching for polymorphic call sites
- Partial inlining for functions with cold paths
- On-stack replacement for already-optimized functions

### Loop Optimization for Iteration Operators

Left-Right provides iteration operators like `$map`, `$filter`, `$reduce`. These are equivalent to loops and benefit from loop optimizations.

**Loop Optimization Techniques:**

1. **Loop Invariant Code Motion:**
   Move computations that don't change across iterations outside the loop.

   ```
   // Before
   for i in array:
       x = a + b  // a and b are constant
       result[i] = array[i] * x

   // After
   x = a + b
   for i in array:
       result[i] = array[i] * x
   ```

2. **Loop Unrolling:**
   Replicate loop body to reduce loop overhead.

   ```
   // Before
   for i in 0..10:
       result[i] = array[i] * 2

   // After
   for i in 0..10 step 2:
       result[i] = array[i] * 2
       result[i+1] = array[i+1] * 2
   ```

3. **Loop Fusion:**
   Combine adjacent loops that iterate over the same range.

   ```
   // Before
   for i in 0..10:
       t[i] = array[i] * 2
   for i in 0..10:
       result[i] = t[i] + 1

   // After
   for i in 0..10:
       result[i] = array[i] * 2 + 1
   ```

**For Left-Right Iteration Operators:**

**$map Optimization:**
```
// Source
result = arr $map $times 2

// IR (before optimization)
%temp = array_new(len(arr))
for %i in 0..len(arr):
    %elem = array_get(arr, %i)
    %mapped = %elem * 2
    array_set(%temp, %i, %mapped)
%result = %temp

// IR (after vectorization)
%result = vector_mul(arr, 2)  // SIMD vectorization
```

**$filter Optimization:**
```
// Source
result = arr $filter $gt 5

// IR (before optimization)
%temp = array_new()
for %i in 0..len(arr):
    %elem = array_get(arr, %i)
    if %elem > 5:
        array_push(%temp, %elem)
%result = %temp

// IR (after filter-fusion with map)
// If followed by map, fuse into single iteration
```

**$reduce Optimization:**
```
// Source
result = arr $reduce $add 0

// IR (before optimization)
%acc = 0
for %i in 0..len(arr):
    %elem = array_get(arr, %i)
    %acc = %acc + %elem
%result = %acc

// IR (after associative reduction optimization)
%result = parallel_reduce(arr, $add, 0)  // Parallelizable
```

### Common Subexpression Elimination (CSE)

CSE identifies and eliminates redundant computations. If the same expression is computed multiple times, compute it once and reuse the result.

**CSE Algorithm:**
- Compute hash of each instruction
- Maintain hash table of expressions
- For each instruction, check if expression exists
- If exists, replace with previous result
- If not, add to hash table

**Example:**
```
// Before
%a = x + y
%b = x + y
%c = a * b

// After
%a = x + y
%b = %a  // Reuse previous computation
%c = %a * %a
```

**For Left-Right:**
- Eliminate redundant operator applications
- Share map lookups for same keys
- Reuse partial applications
- Detect common array indexing patterns

**Example:**
```
// Source
a = arr $get 0
b = arr $get 0

// IR (before)
%a = array_get(%arr, 0)
%b = array_get(%arr, 0)

// IR (after)
%a = array_get(%arr, 0)
%b = %a
```

### Specialization for Known Types (Monomorphization)

Dynamic typing requires type checks at runtime. Monomorphization generates specialized versions for known types.

**Type Specialization:**
- Track types of values at runtime (type feedback)
- When type is known, generate specialized code
- Eliminate type checks for specialized paths
- Improve performance for hot code paths

**Example:**
```
// Generic code
function add(x, y):
    if typeof(x) == "number" and typeof(y) == "number":
        return x + y
    else:
        return concat(x, y)

// Specialized for numbers
function add_number_number(x, y):
    return x + y  // No type checks

// Specialized for strings
function add_string_string(x, y):
    return concat(x, y)  // No type checks
```

**For Left-Right:**
- Specialize operators based on operand types
- Generate type-specific map lookup code
- Specialize iteration for array shapes
- Monomorphize polymorphic functions

**Implementation:**
- Collect type information via static analysis (AOT transpilation)
- Generate specialized versions for common type combinations
- Use inline caches for runtime type checks in generated code
- Fallback to generic code for unexpected types

**Example:**
```
// Source
result = 1 $plus 2

// Generic IR
%x = 1
%y = 2
if is_number(%x) and is_number(%y):
    %result = %x + %y
else:
    %result = concat(%x, %y)

// Specialized IR (for numbers)
%x = 1
%y = 2
%result = %x + %y  // Direct addition, no checks
```

### Tail Call Optimization for Curried Chains

Tail call optimization (TCO) replaces tail calls with jumps, preventing stack growth. For curried languages, TCO is critical for deep currying chains.

**Tail Call Definition:**
- A function call is a tail call if the result is immediately returned
- The caller's stack frame can be reused

**TCO Implementation:**
- Detect tail call positions
- Replace call with jump
- Adjust stack frame or reuse caller's frame
- Handle continuation passing correctly

**For Left-Right Currying:**
```
// Source
result = 1 $plus 2 $times 3

// Equivalent curried chain
result = $times($plus(1, 2), 3)

// Without TCO (stack grows)
call $plus(1, 2)  // Stack frame 1
  call $times(result, 3)  // Stack frame 2
  return result

// With TCO (stack frame reused)
call $plus(1, 2)  // Stack frame 1
  jump $times(result, 3)  // Reuse frame 1
  return result
```

**TCO for Left-Right Operators:**
- Operator chains become tail call sequences
- Partial applications are tail-recursive by nature
- Enables constant stack space for currying
- Supports unlimited currying depth

**Challenges:**
- Debugging: lost call stack information
- Exception handling: stack unwinding changes
- Profiling: call counts may be inaccurate
- Tail call detection in presence of side effects

## 4. Array-Oriented Optimization

### Map-Fusion for Chained Iteration Operators

Map fusion combines multiple iteration operations into a single iteration, eliminating intermediate arrays.

**Motivation:**
```
// Naive execution (creates intermediate arrays)
arr = [1, 2, 3, 4, 5]
t1 = arr.map(x => x * 2)  // [2, 4, 6, 8, 10]
t2 = t1.filter(x => x > 5)  // [6, 8, 10]
result = t2.map(x => x + 1)  // [7, 9, 11]
```

**Fused Execution (single iteration):**
```
arr = [1, 2, 3, 4, 5]
result = []
for x in arr:
    t1 = x * 2
    if t1 > 5:
        t2 = t1 + 1
        result.push(t2)
// result = [7, 9, 11]
```

**IR-Level Fusion:**
```
// Before fusion
%t1 = map(%arr, %mul2)
%t2 = filter(%t1, %gt5)
%result = map(%t2, %add1)

// After fusion
%result = map_filter_fused(%arr, %mul2, %gt5, %add1)
```

**Fusion Algorithm:**
1. Identify chains of iteration operators
2. Check for compatible element types
3. Ensure no side effects in operations
4. Generate fused iteration
5. Update type information

**For Left-Right:**
```
// Source
result = arr $map ($times 2) $filter ($gt 5) $map ($plus 1)

// IR (before fusion)
%t1 = map(%arr, %mul2)
%t2 = filter(%t1, %gt5)
%result = map(%t2, %add1)

// IR (after fusion)
%result = map_filter_map_fused(%arr, %mul2, %gt5, %add1)
```

**Benefits:**
- Eliminates temporary array allocations
- Improves cache locality
- Enables better register allocation
- Reduces memory bandwidth

### Vectorization (SIMD) Opportunities

SIMD (Single Instruction, Multiple Data) executes the same operation on multiple data elements simultaneously.

**SIMD Hardware:**
- SSE, AVX, AVX-512 on x86
- NEON on ARM
- Vector instructions on GPUs

**Vectorization Targets:**
- Element-wise array operations: `map(x => x * 2, arr)`
- Arithmetic on arrays: `arr1 + arr2`
- Comparisons and filters: `filter(x => x > 0, arr)`
- Reductions: `reduce(add, arr)`

**Vectorization Example:**
```
// Scalar code
for i in 0..len:
    result[i] = array[i] * 2

// Vectorized code (AVX-512, 512 bits = 8 doubles)
for i in 0..len step 8:
    vec = load_avx512(&array[i])  // Load 8 elements
    vec = vec * 2  // Multiply all 8 at once
    store_avx512(&result[i], vec)  // Store 8 elements
```

**For Left-Right:**
```
// Source
result = arr $map ($times 2)

// IR (scalar)
%result = array_map_scalar(%arr, %mul2)

// IR (vectorized)
%result = array_map_vectorized(%arr, %mul2, width=8)
```

**Vectorization Challenges:**
- Alignment: data must be properly aligned for SIMD
- Remainder: handle array sizes not divisible by vector width
- Dependency: vectorized operations must be independent
- Type constraints: SIMD requires uniform types

**Vectorization Implementation:**
1. Detect vectorizable loops
2. Check alignment and size constraints
3. Generate vectorized code for main loop
4. Generate scalar fallback for remainder
5. Insert runtime checks for vector width support

### Lazy vs Eager Evaluation of Operator Chains

Lazy evaluation postpones computation until the result is needed. Eager evaluation computes immediately.

**Lazy Evaluation:**
- Computations are deferred
- Short-circuits unused branches
- Can reduce total work
- Requires thunk allocation (performance overhead)

**Eager Evaluation:**
- Computations happen immediately
- Simpler control flow
- No thunk overhead
- May compute unused results

**For Left-Right:**
- Operator chains can be evaluated lazily
- Only final result triggers computation
- Enables infinite sequences and streams
- But introduces overhead for thunk allocation

**Lazy IR:**
```
// Source
result = arr $map ($times 2) $filter ($gt 5)

// Lazy IR (create thunks)
%thunk1 = make_thunk(map, %arr, %mul2)
%thunk2 = make_thunk(filter, %thunk1, %gt5)
%result = %thunk2

// When %result is accessed:
%actual_result = force_thunk(%thunk2)
  %t1 = force_thunk(%thunk1)
    %actual_t1 = map(%arr, %mul2)
  %actual_result = filter(%t1, %gt5)
```

**Eager IR:**
```
// Source
result = arr $map ($times 2) $filter ($gt 5)

// Eager IR (compute immediately)
%t1 = map(%arr, %mul2)
%result = filter(%t1, %gt5)
```

**Hybrid Approach:**
- Use lazy evaluation for potentially unused chains
- Use eager evaluation for hot paths
- Convert lazy to eager after type analysis
- Specialize for common patterns

### Deforestation / Stream Fusion

Deforestation eliminates intermediate data structures, similar to map fusion but more general.

**Concept:**
- Recognize producer-consumer patterns
- Eliminate intermediate data structures
- Pass values directly from producer to consumer

**Example:**
```
// Before deforestation
tree = build_tree(data)  // Build tree structure
result = process_tree(tree)  // Process tree

// After deforestation (tree eliminated)
result = process_data_directly(data)  // Process data as it's built
```

**Stream Fusion:**
- Represent intermediate structures as streams
- Fuse producer and consumer into single pass
- Generalizes to many data structures

**For Left-Right:**
```
// Source
result = arr $map ($times 2) $map ($plus 1)

// Before fusion
%t1 = map(%arr, %mul2)  // Intermediate array
%result = map(%t1, %add1)

// After stream fusion
%result = map_stream_fused(%arr, %mul2, %add1)
// Generates code that processes elements one at a time
```

**Implementation:**
1. Identify streamable operations (map, filter, etc.)
2. Convert to stream representation
3. Fuse adjacent stream operations
4. Generate fused loop
5. Eliminate intermediate allocations

### Batch Processing Optimization

Batch processing groups multiple operations to process them together efficiently.

**Motivation:**
- Reduce per-operation overhead
- Improve cache locality
- Enable vectorization
- Reduce function call overhead

**Example:**
```
// Naive (process one at a time)
for elem in array:
    process(elem)

// Batched (process multiple at once)
for batch in chunks(array, 64):
    process_batch(batch)
```

**For Left-Right:**
```
// Source
result = arr $map ($times 2)

// Naive IR
for %i in 0..len(%arr):
    %elem = array_get(%arr, %i)
    %mapped = %elem * 2
    array_set(%result, %i, %mapped)

// Batched IR
for %batch_start in 0..len(%arr) step 64:
    %batch_end = min(%batch_start + 64, len(%arr))
    %batch = array_slice(%arr, %batch_start, %batch_end)
    %processed = vector_mul(%batch, 2)
    array_copy(%result, %batch_start, %processed)
```

**Batch Size Selection:**
- Cache line size: 64 bytes typical
- Vector width: 8 doubles (AVX-512)
- Loop unrolling factor: 4-8 iterations
- Trade-off: larger batches = fewer calls, but more memory

## 5. IR for Dynamic Languages

### Inline Caches

Inline caches cache the result of dynamic property lookups inline in generated code.

**Problem:**
Dynamic property lookups (e.g., map key lookup) are expensive:
- Hash table lookup
- Type check
- Bounds check
- Memory access

**Inline Cache Solution:**
- Cache lookup result inline in generated code
- Fast path: check if cache hits, use cached result
- Slow path: if cache misses, fallback to full lookup, update cache

**Example:**
```
// Source
result = obj.key

// Generated code without inline cache
%result = map_lookup(%obj, "key")

// Generated code with inline cache (after first execution)
if %obj == %cached_obj and %obj.shape == %cached_shape:
    %result = %cached_offset  // Direct offset access
else:
    %result = map_lookup(%obj, "key")
    update_cache(%obj, "key", %result)
```

**For Left-Right:**
- Cache map lookups for frequent keys
- Cache operator dispatch for frequent operators
- Cache type checks for common types
- Enable fast paths for hot code

**Inline Cache Types:**
1. **Monomorphic:** single type cached (fastest)
2. **Polymorphic:** few types cached (2-4 types)
3. **Megamorphic:** many types (fallback to generic)

**Implementation:**
- Generate stub code with inline cache check
- On cache miss, regenerate stub with updated cache
- Use on-stack replacement to hot-patch code
- Limit cache size to avoid code bloat

### Hidden Classes / Shapes for Maps

Hidden classes (also called shapes or maps) track the structure of objects at runtime.

**Motivation:**
- Dynamic maps can have arbitrary keys
- Efficient access requires knowing offset
- Hidden classes provide structure information

**Hidden Class Design:**
- Each map has a hidden class describing its structure
- Hidden class tracks keys and their offsets
- Adding new key creates new hidden class
- Transitions between hidden classes form a tree

**Example:**
```
// Map with key "a"
obj1 = {a: 1}
// Hidden class H1: {a: offset 0}

// Add key "b"
obj2 = {a: 1, b: 2}
// Hidden class H2: {a: offset 0, b: offset 1}
// H2 transitions from H1

// Another map with same keys
obj3 = {a: 3, b: 4}
// Hidden class H2 (reused)
```

**For Left-Right:**
- Maps are used for functions, conditionals, control flow
- Hidden classes enable fast map access
- Optimizes map-based conditionals
- Enables inline caching for map lookups

**IR with Hidden Classes:**
```
// Source
obj = {name: "add", arity: 2}
result = obj.name

// IR with hidden class
%obj_shape = get_shape(%obj)
if %obj_shape == %cached_shape:
    %result = load_field(%obj, %cached_offset)  // Fast path
else:
    %result = map_lookup(%obj, "name")  // Slow path
```

**Optimization:**
- Inline cache with shape check
- Direct field access when shape is known
- Specialize code for common shapes
- Pre-allocate hidden classes for frequent patterns

### Type Feedback and Speculative Optimization

Type feedback collects type information at runtime to guide optimization.

**Type Feedback Collection:**
- Monitor types of variables and operations
- Record frequency of type combinations
- Identify hot code paths with stable types

**Speculative Optimization:**
- Generate optimized code assuming types are stable
- Insert guards to validate assumptions
- Deoptimize if assumptions fail

**Example:**
```
// Source
function add(x, y):
    return x + y

// Type feedback: x and y are always numbers in hot path

// Speculatively optimized
function add_speculative(x, y):
    if is_number(x) and is_number(y):  // Guard
        return x + y  // Optimized path (no type checks)
    else:
        return add_generic(x, y)  // Fallback
```

**For Left-Right:**
- Collect type feedback for operator operands
- Specialize operators for common type combinations
- Speculate on map shapes for map-based functions
- Generate type-specific iteration code

**Type Feedback Implementation:**
1. Profile execution (via test runs or PGO - Profile-Guided Optimization)
2. Collect type statistics for hot spots
3. Generate specialized code based on feedback
4. Insert guards to validate assumptions (optional, for generated code)
5. Fallback to generic code if assumptions fail

**Deoptimization:**
- Restore state before speculative optimization
- Continue with generic code
- Update type feedback based on failure
- May trigger re-optimization with different assumptions

### Deoptimization when Speculation Fails

Deoptimization undoes speculative optimizations when assumptions are violated.

**Deoptimization Triggers:**
- Type guard fails (unexpected type)
- Shape guard fails (unexpected map structure)
- Guard condition fails (unexpected value)
- Overflow/underflow (unexpected arithmetic behavior)

**Deoptimization Process:**
1. Detect guard failure
2. Capture current execution state
3. Translate optimized state to unoptimized state
4. Continue execution in generic code
5. Update type feedback (optional)
6. May trigger re-optimization

**Example:**
```
// Speculatively optimized (assumes x and y are numbers)
function add(x, y):
    if is_number(x) and is_number(y):  // Guard
        return x + y  // Optimized path
    else:
        return add_generic(x, y)  // Fallback

// Execution:
add(1, 2)  // Guard passes, uses optimized path
add("a", "b")  // Guard fails, deoptimizes to fallback
```

**Deoptimization Challenges:**
- State translation: map optimized state to unoptimized state
- Frame reconstruction: rebuild stack frames
- Register allocation: move values from registers to memory
- Performance: deoptimization is expensive, should be rare

**For Left-Reason:**
- Deoptimize on type guard failures for operators
- Deoptimize on shape guard failures for maps
- Deoptimize on unexpected control flow
- Update type feedback after deoptimization

### AOT Transpilation and Bytecode VM Compilation

Left-Right is an AOT (Ahead-Of-Time) transpiler that compiles to JavaScript and Rust. The transpiler can also generate bytecode for a virtual machine.

**AOT Transpilation Approach:**
1. **Source Analysis:** Parse and analyze Left-Right source code
2. **Type Inference:** Infer types dynamically, no static type annotations
3. **IR Optimization:** Apply optimization passes at compile time
4. **Target Code Generation:** Generate JavaScript or Rust code
5. **Bytecode Generation (optional):** Generate bytecode for VM execution

**Bytecode VM Compilation:**
1. **IR to Bytecode:** Lower MIR to bytecode instructions
2. **Bytecode Layout:** Compact instruction encoding
3. **Constant Pool:** Shared constants and strings
4. **Function Table:** Entry points for map-based functions
5. **Export Table:** Exports via `}@&[...]` pattern

**For Left-Right AOT:**
1. **Parse:** Read `.lr` source files
2. **Type Inference:** Dynamic type inference with no annotations
3. **HIR:** Preserve map-based control flow and operator semantics
4. **MIR:** SSA form with explicit temporaries
5. **Optimization:** Apply passes (DCE, CSE, inlining, fusion, vectorization)
6. **Code Generation:**
   - JavaScript: Generate JS code from IR
   - Rust: Generate Rust code from IR
   - Bytecode: Generate bytecode instructions for VM
7. **Build:** Compile generated code to final artifacts

## 6. Optimization Pipeline Design

### Pass Ordering Dependencies

Optimization passes must be ordered correctly to enable further optimizations.

**Common Dependencies:**
- **Constant propagation before DCE:** Dead code may become dead after constants are propagated
- **Inlining before CSE:** Inlining exposes more subexpressions
- **Loop canonicalization before loop optimizations:** Normalize loops before optimizing
- **SSA construction before dataflow analysis:** Dataflow requires SSA

**Typical Pass Order:**
1. **Construction and Canonicalization:**
   - Build IR
   - Convert to SSA
   - Canonicalize loops
   - Simplify control flow

2. **High-Level Optimizations:**
   - Inlining
   - Constant folding and propagation
   - Dead code elimination
   - Common subexpression elimination

3. **Loop Optimizations:**
   - Loop invariant code motion
   - Loop unrolling
   - Loop fusion
   - Induction variable simplification

4. **Specialization:**
   - Type specialization
   - Function specialization
   - Operator chain optimization

5. **Low-Level Optimizations:**
   - Register allocation
   - Instruction scheduling
   - Peephole optimization

6. **Code Generation:**
   - Convert out of SSA
   - Generate target code
   - Final assembly

**For Left-Right:**
1. Parse to HIR (High-Level IR)
2. Type inference and checking
3. Lower to MIR (Mid-Level IR, SSA)
4. Inlining of small operators
5. Constant folding and propagation
6. Operator chain optimization
7. Loop optimization for iteration operators
8. Array-oriented optimization (fusion, vectorization)
9. Type specialization (monomorphization)
10. Dead code elimination
11. Lower to LIR (Low-Level IR)
12. Register allocation
13. Code generation (JS or Rust)

### Fixed-Point Iteration

Many optimizations reach a fixed point where additional passes don't change the IR.

**Fixed-Point Algorithm:**
```
changed = true
while changed:
    changed = false
    for pass in passes:
        if pass.run(ir):
            changed = true
```

**Fixed-Point Justification:**
- Constant propagation may expose new constants
- DCE may expose new CSE opportunities
- Inlining may expose new optimization opportunities
- Loop optimizations may enable further loop optimizations

**Fixed-Point Termination:**
- IR has finite state
- Each pass reduces IR complexity or leaves it unchanged
- Eventually no pass changes the IR
- Loop terminates

**For Left-Right:**
- Fixed-point iteration for constant folding and propagation
- Iterate until no new constants are found
- Iterate until no more dead code is found
- Iterate until no more CSE opportunities

**Optimization:**
- Limit iterations to avoid pathological cases
- Use worklist to track changed instructions
- Only reprocess affected regions

### Analysis Frameworks (Dataflow, Abstract Interpretation)

Analysis frameworks provide infrastructure for implementing optimizations.

**Dataflow Analysis:**
- Analyzes how data flows through the program
- Computes properties (liveness, reaching definitions, available expressions)
- Used for many optimizations (DCE, constant propagation, CSE)

**Dataflow Analysis Components:**
1. **Transfer function:** How properties change at each instruction
2. **Meet operation:** How properties merge at control flow join points
3. **Lattice:** Property values (with partial order)
4. **Direction:** Forward or backward analysis

**Example: Reaching Definitions Analysis**
```
// Determine which definitions reach each point

d1: x = 1
d2: if condition:
d3:     x = 2
d4: print(x)

// Reaching definitions at d4:
// If condition is true: {d3}
// If condition is false: {d1}
// At merge: {d1, d3}
```

**Abstract Interpretation:**
- Interprets program with abstract values instead of concrete values
- Computes over-approximation of program behavior
- Used for type inference, range analysis, null pointer analysis

**Abstract Domain:**
- Abstract values represent sets of concrete values
- Example: "top" (any value), "int" (any integer), "5" (constant 5)
- Operations on abstract values approximate concrete operations

**For Left-Right:**
- **Liveness analysis:** Identify unused values (for DCE)
- **Reaching definitions:** Identify constant propagation opportunities
- **Available expressions:** Identify CSE opportunities
- **Type inference:** Compute types for values (for specialization)
- **Shape inference:** Compute map shapes (for hidden classes)

**Analysis Framework Benefits:**
- Reusable infrastructure for multiple analyses
- Correct-by-construction (guaranteed to converge)
- Easy to add new analyses
- Supports analysis composition

### Cost Models for Optimization Decisions

Cost models estimate the benefit and cost of optimizations to make optimization decisions.

**Cost Model Components:**
- **Benefit:** Estimated speedup from optimization
- **Cost:** Compilation time and code size increase
- **Trade-off:** Balance benefit vs cost

**Cost Model Examples:**

1. **Inlining Cost Model:**
   ```
   benefit = call_frequency * function_call_overhead
   cost = function_size * code_size_weight
   if benefit > cost: inline
   ```

2. **Loop Unrolling Cost Model:**
   ```
   benefit = loop_iterations * loop_overhead / unroll_factor
   cost = unroll_factor * instruction_count
   if benefit > cost: unroll
   ```

3. **Vectorization Cost Model:**
   ```
   benefit = element_count * scalar_cost - element_count / vector_width * vector_cost
   cost = vectorization_overhead
   if benefit > cost: vectorize
   ```

**For Left-Right:**
- **Operator Fusion:** Estimate temporary allocation reduction vs code size
- **Vectorization:** Estimate speedup vs overhead for remainder handling
- **Type Specialization:** Estimate speedup vs code size for specialized versions
- **Inlining:** Estimate call overhead reduction vs code bloat

**Cost Model Inputs:**
- Profile data (call frequencies, loop iterations)
- Hardware characteristics (cache size, vector width)
- Compilation constraints (code size limit, compilation time limit)
- Target requirements (runtime performance vs startup time)

**Cost Model Challenges:**
- Accurate profiling requires execution
- Heuristics may be inaccurate
- Trade-offs depend on workload
- May require tuning for different scenarios

## 7. Production Compiler IRs

### LLVM IR Design and Usage

LLVM IR is a low-level, SSA-based IR used by many compilers (Clang, Rust, Swift, etc.).

**LLVM IR Characteristics:**
- SSA form
- Typed (strong, static typing)
- Low-level but target-independent
- Infinite virtual registers
- Explicit control flow (basic blocks, phi nodes)

**LLVM IR Example:**
```llvm
define i32 @add(i32 %a, i32 %b) {
entry:
  %result = add i32 %a, %b
  ret i32 %result
}
```

**LLVM IR Advantages:**
- Well-tested optimization passes
- Target-independent code generation
- Mature infrastructure
- Extensive documentation

**LLVM IR for Left-Right:**
- Can use LLVM IR as backend LIR
- Leverage LLVM optimization passes
- Generate efficient machine code
- Supports multiple target architectures

**Lowering Left-Right to LLVM IR:**
```
// Left-Right source
result = arr $map ($times 2)

// LLVM IR (simplified)
define [n x i32] @map_mul([n x i32] %arr, i32 %factor) {
entry:
  %result = alloca [n x i32]
  br label %loop

loop:
  %i = phi i32 [0, %entry], [%next, %loop]
  %cond = icmp slt i32 %i, %n
  br i1 %cond, label %body, label %exit

body:
  %elem = extractelement [n x i32] %arr, i32 %i
  %mapped = mul i32 %elem, %factor
  insertelement [n x i32] %result, i32 %mapped, i32 %i
  %next = add i32 %i, 1
  br label %loop

exit:
  ret [n x i32] %result
}
```

### Cranelift IR (Used by Wasmtime)

Cranelift is a code generator for WebAssembly, used by Wasmtime and other projects.

**Cranelift IR Characteristics:**
- SSA form
- Low-level, register-oriented
- Focus on WebAssembly and JIT compilation
- Fast compilation (designed for JIT)
- Extensible architecture

**Cranelift IR Advantages:**
- Fast compilation (good for JIT)
- Designed for dynamic languages
- Modular architecture
- No GPL license (Apache 2.0)

**Cranelift IR for Left-Right:**
- Alternative to LLVM for backend
- Faster compilation (good for JIT)
- WebAssembly support
- Designed for dynamic language features

**Cranelift IR Example:**
```
// Cranelift IR (simplified)
function map_mul(arr: [n x i32], factor: i32) -> [n x i32] {
    loop i in 0..n {
        elem = arr[i]
        mapped = elem * factor
        result[i] = mapped
    }
    return result
}
```

### V8 TurboFan IR

TurboFan is the optimizing JIT compiler for V8 (JavaScript engine in Chrome/Node.js).

**TurboFan IR Characteristics:**
- Sea of Nodes IR
- Graph-based representation
- Advanced optimizations
- Dynamic language support (JavaScript)
- Integration with type feedback

**TurboFan IR Advantages:**
- Optimized for JavaScript (dynamic language)
- Sea of Nodes enables advanced optimizations
- Built-in inline caching
- Deoptimization support
- Tiered compilation (Ignition -> TurboFan)

**TurboFan IR for Left-Right:**
- Similar challenges as JavaScript (dynamic typing)
- Sea of Nodes good for operator chains
- Inline caching for map lookups
- Type feedback and speculative optimization

**TurboFan IR Example:**
```
// Sea of Nodes IR (simplified)
[Result] <-- [Store] <-- [Array]
           |           ^
           +---[*]-----+---[Load]---[Factor]
               |
           +---[Load]---[Index]
           |
        [Loop]
```

### Rust MIR (Mid-level IR)

MIR (Mid-level Intermediate Representation) is Rust's IR, used after type checking and before code generation.

**MIR Characteristics:**
- SSA-like (but not full SSA)
- Mid-level (close to Rust semantics)
- Explicit control flow
- Supports generic monomorphization
- Optimized for Rust's features

**MIR Advantages:**
- Tailored to Rust's language features
- Supports generic specialization
- Good for borrow checking
- Efficient for Rust's ownership model

**MIR for Left-Right:**
- Left-Right transpiler to Rust can leverage MIR
- Rust's optimization passes apply
- Good for generating efficient Rust code
- Supports Rust's strong typing (though Left-Right is dynamic)

**MIR Example:**
```rust
// MIR (simplified)
fn map_mul(arr: Vec<i32>, factor: i32) -> Vec<i32> {
    let mut result = Vec::with_capacity(arr.len());
    for elem in arr {
        let mapped = elem * factor;
        result.push(mapped);
    }
    result
}
```

### Swift SIL

SIL (Swift Intermediate Language) is Swift's IR, used for optimization and analysis.

**SIL Characteristics:**
- SSA form
- High-level (preserves Swift semantics)
- Explicit control flow
- Supports Swift's features (ARC, generics, closures)
- Multi-level (raw SIL -> canonical SIL -> guaranteed SIL)

**SIL Advantages:**
- Tailored to Swift's language features
- Good for ARC optimization
- Supports generics and specialization
- Comprehensive analysis framework

**SIL for Left-Right:**
- Similar dynamic typing challenges
- Reference counting considerations
- Closure and currying support
- Good for analysis and optimization

**SIL Example:**
```
// SIL (simplified)
sil @map_mul : <T> (@arr: [T], @factor: T) -> [T] {
bb0(%0 : $[T], %1 : $T):
  %2 = integer_literal $Builtin.Word, 0
  %3 = function_ref @allocate_array
  %4 = apply %3(%0, %2) : $@convention(thin) ([T], Builtin.Word) -> [T]
  %5 = begin_borrow %0 : $[T]
  br bb1

bb1:
  // Loop iteration
  // ...
}
```

## 8. Implementation Recommendations

### Recommended IR Levels for Our Compiler

Based on Left-Right's characteristics (point-free, operator-based, array-oriented, dynamic typing), we recommend a 3-level IR design:

**Level 1: High-Level IR (HIR)**
- **Purpose:** Preserve Left-Right semantics and enable semantic analysis
- **Representation:** AST-like, operator-based
- **Features:**
  - Currying and partial application explicit
  - Operators as first-class values
  - Maps as functions, conditionals, control flow
  - Array iteration operators ($map, $filter, $reduce)
  - Type inference and checking passes
- **Target:** Type checking, semantic validation, high-level analysis

**Level 2: Mid-Level IR (MIR)**
- **Purpose:** Enable SSA-based optimizations and lowering
- **Representation:** SSA form, explicit temporaries, basic blocks
- **Features:**
  - Static Single Assignment form
  - Explicit control flow (blocks, branches, loops)
  - Phi nodes for value merging
  - Operator chains flattened to temporaries
  - Map operations explicit (lookups, updates)
- **Target:** General optimizations (DCE, CSE, inlining, constant propagation)

**Level 3: Low-Level IR (LIR)**
- **Purpose:** Target-specific code generation
- **Representation:** Target-dependent (LLVM IR, bytecode, or JS AST)
- **Features:**
  - Explicit register allocation
  - Machine code generation
  - Target-specific optimizations
  - Backend integration
- **Target:** Code generation for JavaScript or Rust

**Justification:**
- HIR preserves Left-Right semantics for accurate analysis
- MIR enables powerful SSA-based optimizations
- LIR bridges to specific targets without carrying high-level semantics

### Lowering Strategy: AST → IR → Optimized IR

**Phase 1: AST → HIR**
- Parse source to AST
- Lower AST to HIR
- Preserve Left-Right semantics
- Represent operators, currying, maps
- Perform type inference and checking
- Validate semantic correctness

**Phase 2: HIR → MIR**
- Convert HIR to SSA form
- Flatten operator chains to temporaries
- Make control flow explicit (blocks, branches)
- Insert phi nodes at control flow merges
- Lower maps to explicit operations (lookups, updates)
- Normalize loops and iteration operators

**Phase 3: MIR Optimization Passes**
- Constant folding and propagation
- Dead code elimination
- Common subexpression elimination
- Inlining (small operators, curried functions)
- Loop optimizations (invariant code motion, unrolling, fusion)
- Array-oriented optimizations (map fusion, vectorization)
- Type specialization (monomorphization)
- Operator chain optimization

**Phase 4: MIR → LIR**
- Lower MIR to target-specific IR
- For JavaScript: Lower to JavaScript AST or bytecode
- For Rust: Lower to Rust AST (transpiler approach)
- For compilation: Lower to LLVM IR
- Perform target-specific optimizations
- Generate final code

**Phase 5: Code Generation**
- Generate JavaScript code
- Or generate Rust code (then compile to binary)
- Or generate machine code (via LLVM)
- Output final executable or library

### Optimization Passes Most Impactful for Our Language

Based on Left-Right's characteristics (operator-based, array-oriented, curried, dynamic), the most impactful optimization passes are:

**1. Constant Folding and Propagation**
- **Why:** Left-Right has many operator chains with constant arguments
- **Impact:** Eliminates redundant computations, simplifies code
- **Example:** `1 $plus 2 $times 3` → `9`

**2. Inlining for Operator Chains**
- **Why:** Currying and operator composition introduce overhead
- **Impact:** Eliminates call overhead, enables further optimization
- **Example:** Inline `$plus` and `$times` in `arr $map ($plus 1) $map ($times 2)`

**3. Map Fusion for Iteration Operators**
- **Why:** Array iteration is common, creates temporary arrays
- **Impact:** Eliminates temporary allocations, improves cache locality
- **Example:** Fuse `arr $map f $map g` → `arr $map (g . f)`

**4. Type Specialization (Monomorphization)**
- **Why:** Dynamic typing requires runtime type checks
- **Impact:** Eliminates type checks, improves performance
- **Example:** Specialize `$plus` for numbers, strings, arrays

**5. Inline Caching for Map Lookups**
- **Why:** Maps are used for functions, conditionals, control flow
- **Impact:** Eliminates hash table lookup overhead
- **Example:** Cache map lookup for `func["name"]` in function call

**6. Vectorization for Array Operations**
- **Why:** Array-oriented operations are common
- **Impact:** Leverages SIMD for 4-8x speedup on supported hardware
- **Example:** Vectorize `arr $map ($times 2)` → SIMD multiply

**7. Loop Optimization for Iteration Operators**
- **Why:** `$map`, `$filter`, `$reduce` are loops
- **Impact:** Reduces loop overhead, enables parallelization
- **Example:** Move invariant code out of `$map` loop

**8. Common Subexpression Elimination**
- **Why:** Operator chains may compute same expression multiple times
- **Impact:** Eliminates redundant computations
- **Example:** `arr $get 0 $plus arr $get 0` → `%tmp = arr $get 0; %tmp + %tmp`

**9. Dead Code Elimination**
- **Why:** Point-free style may create unused intermediates
- **Impact:** Reduces code size, eliminates unnecessary work
- **Example:** Remove unused temporary in complex operator chain

**10. Operator Chain Optimization**
- **Why:** Currying and composition are core to Left-Right
- **Impact:** Reduces overhead, improves code clarity
- **Example:** `f $compose g` → `g . f` (fusion)

**Optimization Pass Prioritization:**
1. **High impact, low cost:** Constant folding, DCE, CSE
2. **High impact, medium cost:** Inlining, type specialization, map fusion
3. **High impact, high cost:** Vectorization, loop optimization, operator chain optimization

**AOT Transpiler Optimization:**
- **Static Analysis:** Use type inference and semantic analysis
- **Profile-Guided Optimization:** Use execution profiles to guide optimizations
- **Aggressive Specialization:** Specialize for known patterns
- **Bytecode VM:** Optional VM-based execution with fast startup

## Conclusion

Intermediate representation design and optimization are critical for achieving maximum performance in the Left-Right compiler. The recommended 3-level IR design (HIR, MIR, LIR) balances semantic preservation with optimization opportunities. The most impactful optimization passes for Left-Right are constant folding, inlining, map fusion, type specialization, and vectorization.

By implementing a robust IR infrastructure with powerful optimization passes, the Left-Right compiler can achieve performance comparable to traditional languages while preserving its unique point-free, operator-based semantics. The dynamic typing and array-oriented nature of Left-Right require specialized optimizations, but the potential performance benefits are significant.