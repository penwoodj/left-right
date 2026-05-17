# Report 5: Code Generation

## Section 1: Code Generation Strategies

Code generation transforms the compiler's intermediate representation into executable code. This phase sits at the boundary between language semantics and target architecture. Choose the right strategy, and you get fast compilation, runtime performance, and maintainable code. Choose wrong, and you fight technical debt forever.

### Direct AST Walking vs IR-Based Codegen

**Direct AST walking** generates code by traversing the abstract syntax tree and emitting target language constructs on the fly. Simpler languages often use this approach. Babel for TypeScript does this, as do many transpilers.

```rust
fn gen_expr(ast: &Expr, ctx: &mut Context) -> String {
    match ast {
        Expr::Binary { left, op: ident, right } => {
            format!("{} {} {}", gen_expr(left, ctx), ident, gen_expr(right, ctx))
        }
        Expr::Call { func, args } => {
            format!("{}({})", gen_expr(func, ctx), args.iter()
                .map(|a| gen_expr(a, ctx))
                .collect::<Vec<_>>()
                .join(", "))
        }
        // ...
    }
}
```

**Note:** All operators (`+`, `@`, `><`, etc.) are `Identifier` tokens in Left-Right, not a separate `Operator` token type. Operator semantics emerge at runtime based on value types, not at lex/parse time.

Pros: Simple to implement, clear mapping from AST to output, good for transpilation to high-level targets like JavaScript.

Cons: Hard to optimize, target-specific logic scattered throughout, no separation of concerns. Optimization passes require mutating the AST or re-walking it. Adding a new target means touching every AST node.

**IR-based codegen** introduces an intermediate representation that abstracts away target details. The pipeline becomes: AST → IR → Target Code. LLVM, MLIR, and Cranelift follow this pattern.

```rust
// Generate IR first
fn gen_expr_ir(ast: &Expr, ctx: &mut IRContext) -> IRNode {
    match ast {
        Expr::Binary { left, op: ident, right } => {
            let left_val = gen_expr_ir(left, ctx);
            let right_val = gen_expr_ir(right, ctx);
            ctx.emit(IRNode::BinOp {
                op: ident.clone(),  // Identifier token, not Operator enum
                left: left_val,
                right: right_val,
            })
        }
        // ...
    }
}

// Then compile IR to target
fn compile_to_js(ir: &IRBlock) -> String {
    // IR-agnostic JS codegen
}

fn compile_to_llvm(ir: &IRBlock) -> String {
    // IR-agnostic LLVM IR codegen
}
```

Pros: Clear separation of concerns, optimizations can target IR instead of AST, easier to add new targets. IR can be validated, optimized, and reused.

Cons: More complex to implement, IR design matters profoundly. Bad IR creates friction. Overhead of translating twice.

For Left-Right, IR-based codegen wins. Multi-target (JavaScript, Rust, bytecode VM), optimization needs (curried functions, map-as-functions), and long-term maintainability demand the separation. The **primary target is a bytecode VM** — code generation focuses on emitting bytecode instructions that the VM executes. JavaScript and Rust backends can transpile bytecode or compile IR directly, but the bytecode VM is the core execution model. IR doesn't need to be LLVM or MLIR; a custom SSA-based IR suffices.

### Template-Based Code Generation

Template-based codegen uses string templates with placeholders filled during code generation. Common in web frameworks and some compilers.

```mustache
function {{name}}({{params}}) {
    {{#each body}}
    {{this}};
    {{/each}}
}
```

Pros: Fast to implement, output readability, non-programmers can tweak templates.

Cons: Hard to compose, error-prone for complex structures, limited optimization. Templates don't understand the structure they generate.

Template-based works for simple transpilers but breaks for complex cases. Left-Right's curried functions need structural understanding: partial application, closure capture, arity tracking. Templates can't express this cleanly.

Better: Structural code generation using AST construction in the target language, then pretty-printing. Or use IR where structure is explicit.

### Instruction Selection Algorithms

Instruction selection maps IR operations to target machine instructions. Simple targets (like JavaScript) have trivial selection. Native targets need algorithms.

**Tree parsing** builds a pattern matching tree for IR and selects best instruction sequence. BURG and DAG-based instruction selectors use this approach. Efficient but complex to implement.

**Tiling** covers the IR DAG with instruction patterns, selects minimal cost cover. Peephole optimization as a post-pass cleans up. Common in GCC.

**Simple peephole** uses pattern matching on generated code, replaces suboptimal sequences. Easy to implement, catches common patterns. Good first step.

For LLVM and Cranelift backends, instruction selection is handled by the infrastructure. For direct native codegen (e.g., x86 assembly), start with peephole optimization. Tree parsing becomes necessary only if performance demands it.

### Register Allocation for Native Targets

Register allocation maps temporaries to machine registers or stack slots. Critical for native codegen performance.

**Graph coloring** builds interference graph where nodes are temporaries, edges represent overlapping lifetimes. Colors registers. Chaitin's algorithm is classic. Expensive but produces good allocation. Briggs and George改进 spill strategy.

**Linear scan** allocates registers in linear pass over code. Simpler, faster, good for just-in-time compilers. LLVM's JIT uses variants.

**Register coalescing** eliminates unnecessary moves between registers. Reduced code size, better performance. Critical for x86 with few general-purpose registers.

**Spilling** registers to stack when insufficient. Spill code placement affects performance. Critical loops need aggressive allocation.

For Left-Right's native target via LLVM, register allocation is automatic. For direct codegen experiments, linear scan is pragmatic. Most performance gains come from better IR, not register allocation tricks.

### Stack Machine vs Register Machine Codegen

**Stack machine codegen** pushes operands to stack, operations pop and push results. Simple to implement, good for virtual machines and JavaScript. Most bytecode VMs (JVM, Python, WASM) are stack-based.

```javascript
// Stack-based IR for x + y * z
push x
push y
push z
mul
add
```

Pros: Simple codegen, fixed-size instructions, easy to verify.

Cons: More instructions, extra memory traffic, harder to optimize.

**Register machine codegen** uses virtual registers in IR. Fewer instructions, less memory traffic. LLVM, Cranelift, and modern VMs (V8, SpiderMonkey) use register-based IR internally.

```llvm
; Register-based IR
%tmp1 = mul %y, %z
%result = add %x, %tmp1
```

Pros: Fewer instructions, better for optimization, maps well to real hardware.

Cons: More complex codegen, register allocation needed.

Left-Right's IR should be register-based for optimization potential. Stack machine codegen to JavaScript is a post-processing step, not the core IR design. Curried functions benefit from SSA properties in register-based IR: easier to analyze closure capture and partial application.

**Trade-off summary:**

| Strategy | Best For | Left-Right |
|----------|----------|------------|
| Direct AST | Simple transpilers, single target | No |
| IR-based | Multi-target, optimizations | Yes |
| Template | Output readability, simple structures | Limited use |
| Tree parsing | Performance-critical native code | LLVM handles this |
| Graph coloring | Optimized native code | LLVM handles this |
| Stack machine | VMs, simple bytecode | **Left-Right bytecode VM** |
| Register machine | Optimization, multi-target | IR design |

## Section 2: Multi-Target Code Generation

Multi-target compilation demands careful IR design. The IR must capture language semantics abstract enough for all targets, specific enough to generate efficient code. Too abstract, and target-specific optimizations become impossible. Too concrete, and targets fight the IR's assumptions.

### Generating JavaScript from IR

JavaScript codegen transforms IR into idiomatic JavaScript. This is not just syntax translation. Idiomatic JavaScript uses closures, prototypes, and runtime typing. Direct translation produces verbose, slow code.

**Map operators to JavaScript equivalents.** Binary operators (all `Identifier` tokens in Left-Right) map directly to JavaScript operators. Curried closures (maps with `_<` references) need closure wrappers. Map-as-closures become object lookups or function calls.

```rust
// IR: apply a curried binary operator
IRNode::Apply { func: Box::new(IRNode::Add), arg: left, next_arg: right }

// JavaScript output
function add(a, b) { return a + b; }
add(x)(y)  // Curried application
```

**Runtime representation matters.** Left-Right has 7 types. JavaScript has primitives and objects. Map LR types to JS types:

- Number → JS `number` (or `bigint`)
- String → JS `string`
- Boolean → JS `boolean`
- Array → JS `Array` (nested)
- Map → JS `Object` or `Map` depending on use
- Function → JS `function`
- Null → JS `null`

**Dynamic typing runtime.** Every value carries type tag. JavaScript `typeof` works for primitives but objects need custom tagging.

```javascript
// Type tagging for all values
function tag(value, type) {
    return { value, type };
}

function untag(tagged) {
    return tagged.value;
}

function getType(tagged) {
    return tagged.type;
}
```

Boxing primitives adds overhead. Alternative: rely on JavaScript's dynamic typing for primitive types, custom tags for maps and functions. Trade-off: speed vs correctness guarantees.

**Closure representation.** Curried functions capture variables. JavaScript closures handle this naturally.

```javascript
// LR: map x => x + 1
// IR: closure capturing map iteration
function curry(fn, arity) {
    return function curried(...args) {
        if (args.length >= arity) {
            return fn.apply(this, args);
        }
        return curried.bind(this, ...args);
    };
}

// Generated code
const map = curry(function(fn, arr) {
    return arr.map(fn);
}, 2);
```

Curry wrapper adds overhead but enables partial application. Hot code paths might need monomorphic versions.

### Generating Native Code via LLVM IR

LLVM IR provides portable, typed intermediate representation. The compiler generates LLVM IR from its own IR, then LLVM compiles to machine code.

**Type mapping.** LLVM types are explicit. Map Left-Right types:

- Number → `i64` or `double`
- String → `i8*` (pointer to null-terminated string)
- Boolean → `i1`
- Array → `{ i64, [T x N]* }` (length and pointer)
- Map → `%Map*` (opaque struct pointer)
- Function → `%Function*` (function pointer struct)
- Null → `i8* null`

**Dynamic typing in static types.** LLVM is statically typed. Left-Right is dynamic. Solution: tagged unions.

```llvm
; Tagged value type
%Value = type { i64, %TaggedUnion }

%TaggedUnion = type {
    %Number,
    %String,
    %Array,
    %Map,
    %Function
}

%Number = type { double }
%String = type { i8*, i64 }  ; pointer and length
%Array = type { i64, %Value* }  ; length and elements
; ...
```

Every operation checks tag first, dispatches to handler.

```llvm
; Add operation with runtime dispatch
define %Value @add(%Value %a, %Value %b) {
    %tag_a = extractvalue %Value %a, 0
    %tag_b = extractvalue %Value %b, 0
    ; Check both are numbers
    %is_num = and i1 icmp eq i64 %tag_a, 0, icmp eq i64 %tag_b, 0
    br i1 %is_num, label %num_add, label %type_error

num_add:
    %num_a = extractvalue %Value %a, 1
    %num_b = extractvalue %Value %b, 1
    %result = fadd double %num_a, %num_b
    %tagged = insertvalue %Value undef, double %result, 1
    %final = insertvalue %Value %tagged, i64 0, 0
    ret %Value %final

type_error:
    call void @type_error()
    unreachable
}
```

**Garbage collection.** LLVM has GC integration but requires custom GC strategy. Simple: reference counting. More complex: tracing GC with stack maps. Left-Right needs closure capture, which complicates GC.

### Generating WebAssembly

WebAssembly compilation provides browser-native execution. WASM has linear memory and typed locals. Good for performance, bad for dynamic typing.

**Mapping dynamic types to WASM.** WASM supports `i32`, `i64`, `f32`, `f64`. Represent dynamic values as:

- Number → `f64` (or `i64` for integer)
- String → `i32` (index into string table)
- Array → `i32` (index into array table)
- Map → `i32` (index into map table)
- Function → `i32` (table index)
- Null → `-1` (sentinel)

**Runtime tables.** Allocate linear memory, maintain tables for each type. Garbage collection requires manual management or integrate with JavaScript GC.

```wat
;; WASM representation of dynamic values
;; Heap layout:
;; 0-1000: numbers (inline)
;; 1000-2000: string table
;; 2000-3000: array table
;; ...

;; Add operation
(func $add (param $a i64) (param $b i64) (result i64)
  ;; Extract type tag
  (local.get $a)
  (i64.const 0xFF)
  (i64.and)
  ;; Check if both numbers
  ;; ...
)
```

WASM codegen is complex for dynamic languages. Better path: compile to JavaScript first, let existing toolchains (JSC, V8) handle runtime. WASM for compute-heavy numeric kernels.

### Shared IR with Target-Specific Backends

Multi-target codegen needs shared IR. Each backend reads IR, emits target code.

```rust
pub trait CodeGenBackend {
    fn generate(&self, ir: &IRModule) -> Result<String>;
    fn target_name(&self) -> &str;
}

struct JavaScriptBackend;
struct LLVMBackend;
struct WASMBackend;

impl CodeGenBackend for JavaScriptBackend {
    fn generate(&self, ir: &IRModule) -> Result<String> {
        // JavaScript-specific codegen
    }
}

fn compile(ir: &IRModule, backends: &[Box<dyn CodeGenBackend>]) {
    for backend in backends {
        let output = backend.generate(ir)?;
        fs::write(format!("output.{}", backend.target_name()), output)?;
    }
}
```

**IR portability.** Avoid target-specific constructs. No `ptr_to_int` if WASM doesn't support. No `call_indirect` if JS lacks efficient call mechanisms. Use lowest common denominator, or IR variants per target family.

**Target-specific optimizations.** Each backend can add optimizations after codegen. JavaScript backend can inline small functions. LLVM backend gets all LLVM optimization passes. WASM backend can specialize numeric operations.

**Error handling.** Errors differ per target. JavaScript stack traces vs LLVM crash dumps. Unified error reporting in IR, target-specific formatting in backends.

**Testing strategy.** Test IR semantics independently. Then test each backend against same IR. Cross-compile: LR → IR → JS → IR → native, verify semantic equivalence.

Next section details JavaScript transpilation specifics.

## Section 3: JavaScript Transpilation

JavaScript transpilation transforms Left-Right into executable JavaScript. This is not mere syntax conversion. Idiomatic JavaScript differs fundamentally from point-free, curried, map-based code. Good transpilation produces code that reads like JavaScript, runs efficiently, and maintains Left-Right semantics.

### Mapping Language Constructs to Idiomatic JS

**Operators (identifiers) map directly.** All operators are `Identifier` tokens that translate to JavaScript operators with minimal overhead. Runtime dispatch handles operator semantics based on value types.

```javascript
// Left-Right: 1 + 2 * 3
// Transpiled: ((1 + 2) * 3)  // = 9, strict left-to-right, zero precedence
```

Parenthesization matters. Left-Right has ZERO precedence — all evaluation is strictly left-to-right curried. IR preserves this order, emit JS without assuming JavaScript's operator precedence.

**Maps-as-closures.** Left-Right treats maps as closures (unexecuted operators) that become functions at runtime. Maps with `_<` references are curried operators waiting for input. JavaScript objects are not callable. Two approaches:
1. Function wrapper: Wrap object in getter function.
2. Direct access: Map lookups become `obj.key` or `obj[key]`.

```javascript
// Left-Right: {x: 1, y: 2}.x
// Approach 1: wrapper
const map1 = {x: 1, y: 2};
const map1_func = (key) => map1[key];
map1_func('x');

// Approach 2: direct (better performance)
const map1 = {x: 1, y: 2};
map1.x;  // Direct property access
```

Direct access is faster. Function wrapper enables partial application (currying): `{x: 1, y: 2}['x']` vs `{x: 1, y: 2}('x')`. Left-Right's map-as-closure semantics require callable maps when maps contain `_<` references. Hybrid: static maps become objects, maps with operators become getter functions.

**Array operations.** Left-Right uses arrays pervasively. JavaScript arrays support iteration but need method calls for transformations.

```javascript
// Left-Right: [1, 2, 3] map (x => x * 2)
// Idiomatic JS: [1, 2, 3].map(x => x * 2)
```

Map, filter, reduce, and other array methods map directly. Left-Right's point-free style composes these naturally.

```javascript
// Left-Right: [1, 2, 3] map (x => x * 2) filter (x => x > 2)
// Transpiled: [1, 2, 3].map(x => x * 2).filter(x => x > 2)
```

Currying applies to array methods too. `[1, 2, 3].map(x => x * 2)` can become `map(x => x * 2)([1, 2, 3])`. Chaining maps to JavaScript's method chaining naturally.

**Conditionals.** Left-Right uses map-based conditionals with `_<` expression keys. When key evaluates truthy, `:` returns value; falls through if not. JavaScript has ternary operator and if-else.

```javascript
// Left-Right: {_<: yes, falseCase}
// Transpiled: condition ? yes : falseCase
```

Pattern match: detect map with `_<` expression key, emit ternary. General maps use property access or switch statement.

```javascript
// Left-Right: {a: 1, b: 2, c: 3}[key]
// Transpiled: ({a: 1, b: 2, c: 3})[key]
```

Small maps use object literal. Large maps use switch or function with case.

### Curried Closure Representation in JS

**Closures (maps-as-functions).** Curried closures take one argument at a time. JavaScript doesn't enforce this. Two approaches:

**Manual currying wrapper.** Wrap every function in currying logic.

```javascript
function curry(fn, arity) {
    return function curried(...args) {
        if (args.length >= arity) {
            return fn.apply(this, args);
        }
        return (...more) => curried.apply(this, args.concat(more));
    };
}

// Left-Right: x => y => x + y
// Transpiled: curry((x, y) => x + y, 2)
```

Currying wrapper adds overhead to every function call. Alternative: use arrow functions explicitly.

```javascript
// Direct currying with arrows
const add = x => y => x + y;
add(1)(2);  // 3
```

This is idiomatic, no wrapper needed. Trade-off: verbose for deeply curried functions, but faster runtime.

**Partial application.** Currying enables partial application. JavaScript's bind or default parameters can simulate.

```javascript
// Left-Right: map (x => x + 1)
// Transpiled (with curry): map(x => x + 1)
// Partial application happens naturally
```

When function is called with fewer args than arity, return function expecting remaining args.

**Arity tracking.** IR must encode expected arity. Curried functions have multiple arities: add has arity 2, but add(1) has arity 1.

```rust
enum IRNode {
    Function { name: String, params: Vec<String>, body: IRBlock },
    PartialApply { func: IRNode, args: Vec<IRNode> },
    // ...
}
```

During codegen, track how many args already provided. Emit arrow function or curry wrapper accordingly.

### Map-as-Closure Codegen Patterns

Left-Right's map-as-closure pattern maps well to JavaScript's property access, but needs callable semantics for maps with `_<` operator references.

**Static maps.** Known at compile time. Emit object literal, wrap in getter function if needed.

```javascript
// Left-Right: {x: 1, y: 2}
// Direct access (no callable needed):
const map1 = {x: 1, y: 2};
map1.x;

// Callable (for map-as-closure with `_<`):
const map1_get = key => ({x: 1, y: 2})[key];
map1_get('x');
```

Static maps with no operators: direct access faster. Static maps with `_<` references: getter function.

**Dynamic maps.** Created at runtime. Use `Map` object or plain object.

```javascript
// Left-Right: map from user input
// Dynamic map creation:
const map2 = new Map();
map2.set('x', 1);
map2.set('y', 2);

// Callable getter:
const map2_get = key => map2.get(key);
```

Plain objects faster for string keys, `Map` for non-string keys and insertion order preservation.

**Map composition.** Left-Right composes maps. JavaScript objects don't compose directly.

```javascript
// Left-Right: {a: 1} + {b: 2}  (map merge)
// Transpiled: Object.assign({}, {a: 1}, {b: 2})
// Or spread: {...{a: 1}, ...{b: 2}}
```

Spread syntax is idiomatic, handles merging. For deep merge, use recursive function.

**Conditional maps.** Maps used for conditionals translate to switch or lookup table.

```javascript
// Left-Right: {a: 1, b: 2, c: 3}[key]
// Small map: object access
const lookup = {a: 1, b: 2, c: 3};
lookup[key];

// Large map: switch (better performance)
function getMapValue(key) {
    switch(key) {
        case 'a': return 1;
        case 'b': return 2;
        case 'c': return 3;
        default: return undefined;
    }
}
```

Switch tables are faster for many keys, V8 optimizes them. Emit switch when map has >5 keys, all literal.

### Source Map Generation

Source maps link transpiled JavaScript back to Left-Right source. Critical for debugging.

**Source map format.** JSON file mapping output positions to source positions.

```json
{
  "version": 3,
  "file": "output.js",
  "sourceRoot": "",
  "sources": ["input.lr"],
  "names": [],
  "mappings": "AAAA,SAASA,..."
}
```

Mappings encode position mappings in base64 VLQ format. Libraries handle this: `source-map` npm, Rust's `source-map-mappings`.

**Generating mappings.** During codegen, track source position for each generated line.

```rust
fn emit_expr_js(expr: &Expr, ctx: &mut CodeGenCtx, src_pos: SourcePos) -> String {
    let output = match expr {
        // ...
    };
    ctx.source_map.add_mapping(output_pos, src_pos);
    output
}
```

Column-level mapping gives precise error messages. Line-level mapping suffices for debugging, simpler to implement.

**Inlining.** IR optimizations may inline functions. Source map must preserve original position, not inlined location.

```rust
// When inlining, preserve original source position
fn inline_function(call: &Call, func: &Function, ctx: &mut IRContext) {
    // Copy func body, track original call position
    for stmt in func.body {
        let stmt_with_pos = stmt.with_position(call.position);
        ctx.emit(stmt_with_pos);
    }
}
```

Source maps enable browser DevTools to show Left-Right source, set breakpoints, and trace errors. Essential for developer experience.

Next section covers native code generation.

## Section 4: Native Code Generation

Native code generation transforms IR into machine code. This path offers maximum performance but introduces complexity: runtime typing, garbage collection, and ABI considerations. For Left-Right, native compilation targets LLVM or Cranelift as backend, avoiding direct machine code emission.

### Using LLVM as Backend

LLVM provides portable IR, optimization passes, and code generation. The compiler generates LLVM IR from its own IR, then LLVM compiles to machine code.

**LLVM IR basics.** LLVM IR is in SSA form, typed, with infinite virtual registers.

```llvm
; Example: Left-Right add operation
define %Value @add(%Value %a, %Value %b) {
entry:
    %tag_a = extractvalue %Value %a, 0
    %tag_b = extractvalue %Value %b, 0
    %is_num_a = icmp eq i64 %tag_a, 0
    %is_num_b = icmp eq i64 %tag_b, 0
    %both_num = and i1 %is_num_a, %is_num_b
    br i1 %both_num, label %num_add, label %type_error

num_add:
    %val_a = extractvalue %Value %a, 1
    %val_b = extractvalue %Value %b, 1
    %result = fadd double %val_a, %val_b
    %tagged_result = insertvalue %Value undef, double %result, 1
    %final = insertvalue %Value %tagged_result, i64 0, 0
    ret %Value %final

type_error:
    call void @runtime_type_error()
    unreachable
}
```

**Type definitions.** Define tagged union structure for dynamic types.

```llvm
; Type tag enum
%Tag = type i64  ; 0=number, 1=string, 2=boolean, 3=array, 4=map, 5=function, 6=null

; Tagged value
%Value = type { %Tag, %TaggedUnion }

; Tagged union (one field active based on tag)
%TaggedUnion = type { %Number, %String, %Boolean, %Array, %Map, %Function }

%Number = type { double }
%String = type { i8*, i64 }  ; pointer and length
%Boolean = type { i1 }
%Array = type { i64, %Value* }  ; length and elements
%Map = type %ValueMap*  ; opaque pointer
%Function = type %Value (*)(%Value)  ; function pointer taking one Value
```

**Operator dispatch.** Every operation checks tags, dispatches to handler.

```llvm
; Generic add dispatch
define %Value @add_dispatch(%Value %a, %Value %b) {
entry:
    %tag_a = extractvalue %Value %a, 0
    %tag_b = extractvalue %Value %b, 0

    ; Dispatch matrix: number+number, string+string (concat), array+array (concat)
    switch i64 %tag_a, label %type_error [
        i64 0, label %maybe_num  ; number
        i64 1, label %maybe_str  ; string
        i64 3, label %maybe_arr  ; array
    ]

maybe_num:
    switch i64 %tag_b, label %type_error [
        i64 0, label %num_add
    ]

num_add:
    %num_a = extractvalue %Value %a, 1
    %num_b = extractvalue %Value %b, 1
    %result = fadd double %num_a, %num_b
    ; ... tag and return
    ; ...

type_error:
    call void @runtime_type_error()
    unreachable
}
```

Dispatch adds overhead. JIT can specialize monomorphic paths. AOT can inline type checks where types are known from IR analysis.

**Function representation.** Left-Right functions take one argument (curried). LLVM functions take one `%Value`.

```llvm
; Left-Right: x => y => x + y
; LLVM IR:

; Closure struct
%Closure = type { %Value, %Value (*)(%Value, %Closure*) }

; Outer function (creates closure)
define %Value @lambda_x(%Value %x) {
    %closure = allocate %Closure
    %outer_fn = bitcast %Value (%Value, %Closure*)* @add_curried to %Value (*)(%Value, %Closure*)
    ; Store captured x in closure
    ; Return closure as function
    %func_ptr = bitcast %Closure* %closure to %Value
    ret %Value %func_ptr
}

; Inner function (applies y)
define %Value @add_curried(%Value %y, %Closure* %closure) {
    %x = load %Value, %Closure* %closure
    %result = call %Value @add_dispatch(%Value %x, %Value %y)
    ret %Value %result
}
```

Closures heap-allocate captured variables. Small closures can stack-allocate if no escaping.

**Garbage collection integration.** LLVM has GC infrastructure but requires custom strategy. Options:

1. **Reference counting.** Simple, immediate reclamation. Cycles cause leaks. Use for initial implementation.

```llvm
; Increment ref count
define void @retain(%Value %val) {
    %tag = extractvalue %Value %val, 0
    %needs_gc = icmp ule i64 %tag, 4  ; string, array, map, function need GC
    br i1 %needs_gc, label %do_retain, label %done

do_retain:
    %obj_ptr = extractvalue %Value %val, 1
    %refcount = load i32, i32* %obj_ptr
    %new_count = add i32 %refcount, 1
    store i32 %new_count, i32* %obj_ptr
    br label %done

done:
    ret void
}

; Decrement and free
define void @release(%Value %val) {
    %obj_ptr = extractvalue %Value %val, 1
    %refcount = load i32, i32* %obj_ptr
    %new_count = sub i32 %refcount, 1
    store i32 %new_count, i32* %obj_ptr
    %is_zero = icmp eq i32 %new_count, 0
    br i1 %is_zero, label %free, label %done

free:
    call void @free_obj(%Value %val)
    br label %done

done:
    ret void
}
```

2. **Tracing GC.** Complex, handles cycles. Requires stack maps for root identification. LLVM generates these automatically with statepoints.

```llvm
; Stack map example
call void @some_gc_func() [ "deopt"(...) ]
```

Use reference counting first. Upgrade to tracing GC if profiling shows GC overhead.

**Optimization passes.** LLVM provides optimization passes: inlining, constant folding, dead code elimination. Enable standard optimization levels (O0, O1, O2, O3).

```bash
# LLVM optimization passes
opt -O2 input.ll -o optimized.ll
llc -O2 optimized.ll -o output.s
```

Left-Right-specific optimizations exist: currying fusion, map specialization. Implement as LLVM passes or as IR passes before LLVM generation.

### Using Cranelift for Faster Compilation

Cranelift is Rust-native codegen library. Faster than LLVM for JIT, less mature. Good for server-side WASM or Rust-based toolchains.

**Cranelift basics.** Cranelift uses CLIF (Cranelift IR), similar to LLVM but simpler.

```rust
// Cranelift function builder
let mut func = Function::new();

// Entry block
let mut builder = FunctionBuilder::new(&mut func);

// Emit add
let a = builder.use_param(param_index(0));
let b = builder.use_param(param_index(1));
let sum = builder.ins().iadd(a, b);
builder.ins().return_(&[sum]);
```

**Type mapping to Cranelift.** Cranelift supports integer and floating types. Represent dynamic values as:

- Number → `f64`
- String → `i64` (handle pointer to string struct)
- Boolean → `i8`
- Array, Map, Function → `i64` (opaque pointers)

No tagged unions in Cranelift. Manually manage type tags.

**Advantages of Cranelift.**

- Faster compilation than LLVM. Useful for JIT or serverless.
- Rust-native, easier integration with Rust compiler.
- Good for WASM (Cranelift WASM backend exists).

**Disadvantages.**

- Less mature than LLVM.
- Fewer optimization passes.
- Smaller ecosystem, less tooling support.

For Left-Right, Cranelift is viable if compiler speed matters more than runtime performance. Use LLVM for production, Cranelift for development/iterative compilation.

### Dynamic Typing Representation

Native code must represent Left-Right's 7 types. Two main approaches: tagged unions and boxing.

**Tagged unions.** Every value carries type tag and data. Scheme and OCaml use this approach.

```c
// C representation of tagged union
typedef enum {
    TAG_NUMBER,
    TAG_STRING,
    TAG_BOOLEAN,
    TAG_ARRAY,
    TAG_MAP,
    TAG_FUNCTION,
    TAG_NULL
} Tag;

typedef struct {
    Tag tag;
    union {
        double number;
        struct { char* ptr; size_t len; } string;
        int boolean;
        struct { size_t len; Value* elems; } array;
        void* map;  // opaque
        Value (*func)(Value);
    } data;
} Value;
```

Tagged unions require runtime checks on every operation. Fast for type-known code (compiler can specialize), slow for polymorphic code.

**NaN boxing.** Use double NaN payload space to store non-double values. Compact (pointer sized), fast (no separate allocation for numbers). Complex to implement. Note: Left-Right uses decimal-only numbers, but boxing strategy remains relevant for native codegen.

**Type tag placement.**

- Tag in high bits: simple masking. Check `tag = val >> TAG_BITS`.
- Tag in low bits: clear low bits for values, set for pointers. Check `val & TAG_MASK`.
- Separate header: objects have header with tag. Numbers don't.

High bits easiest for portability. Low bits efficient for pointer-heavy code.

**Operator dispatch.** Every operation checks tag, dispatches to handler. Switch statement or computed goto.

```c
Value add(Value a, Value b) {
    Tag tag_a = get_tag(a);
    Tag tag_b = get_tag(b);

    if (tag_a == TAG_NUMBER && tag_b == TAG_NUMBER) {
        return box_number(unbox_number(a) + unbox_number(b));
    } else if (tag_a == TAG_STRING && tag_b == TAG_STRING) {
        return box_string(concat(unbox_string(a), unbox_string(b)));
    } else {
        runtime_error("Type error in add");
    }
}
```

Dispatch table with function pointers speeds up polymorphic dispatch.

```c
typedef Value (*BinaryOp)(Value, Value);

BinaryOp add_handlers[TAG_COUNT][TAG_COUNT] = {
    // Initialized at runtime
    [TAG_NUMBER][TAG_NUMBER] = add_number_number,
    [TAG_STRING][TAG_STRING] = add_string_string,
    // ...
};

Value add(Value a, Value b) {
    Tag tag_a = get_tag(a);
    Tag tag_b = get_tag(b);
    return add_handlers[tag_a][tag_b](a, b);
}
```

Dispatch table lookup is O(1), faster than switch. Requires TAG_COUNT² handlers. Manageable for 7 types (49 handlers), but grows with language evolution.

### Operator Dispatch for Native Code

Operator dispatch selects handler based on operand types. Critical for performance in dynamic languages.

**Monomorphic inlining.** When type analysis shows monomorphic call site, inline handler.

```rust
// IR analysis: add always called with numbers
// Codegen: emit fast path without checks

fn emit_add_fast(a: IRNode, b: IRNode) -> IRNode {
    // Direct number add, no tag checks
    IRNode::FAdd { left: a, right: b }
}
```

Type inference at IR level identifies monomorphic sites. JIT can speculatively inline, add guard for type mismatch, deopt if guard fails.

**Polymorphic inline cache (PIC).** Cache handler at call site based on observed types.

```c
struct InlineCache {
    Value* (*handler)(Value, Value);
    Tag expected_a;
    Tag expected_b;
};

Value add_with_pic(Value a, Value b, InlineCache* cache) {
    Tag tag_a = get_tag(a);
    Tag tag_b = get_tag(b);

    if (tag_a == cache->expected_a && tag_b == cache->expected_b) {
        return cache->handler(a, b);
    }

    // Cache miss: lookup and update cache
    cache->handler = find_handler(tag_a, tag_b);
    cache->expected_a = tag_a;
    cache->expected_b = tag_b;
    return cache->handler(a, b);
}
```

PIC speeds up polymorphic code by caching handlers. Common pattern in V8, SpiderMonkey. Requires JIT to update cache at runtime.

**Type feedback.** Collect runtime type information, use for optimization.

```c
void record_add_types(Tag tag_a, Tag tag_b) {
    add_type_feedback[tag_a][tag_b]++;
}

if (add_type_feedback[TAG_NUMBER][TAG_NUMBER] > HOT_THRESHOLD) {
    // Specialize number path
}
```

Profiler instrument code, collect type frequencies. Re-optimize hot code with specialization.

Next section covers curried function and map compilation in detail.

## Section 5: Curried Closure and Map Compilation

Curried closures (maps with `_<` references) and maps are central to Left-Right. Compiling these efficiently requires careful representation. Currying affects call overhead, closure capture, and optimization. Map compilation impacts data layout, access patterns, and runtime dispatch.

### Closure Representation Strategies

Closures capture free variables from enclosing scope. Different targets handle closures differently.

**JavaScript closures.** JavaScript's native closure mechanism handles capture automatically.

```javascript
// Left-Right: { arg: _<@0, body }  (map-as-closure capturing arg)
// Transpiled:
function outer(arg) {
    return function(input) {
        // body uses captured arg
        return arg + input;  // arg captured
    };
}
```

JavaScript closures heap-allocate captured variables. Every closure creation allocates new function object. Overhead for hot loops.

**Native closures.** Need explicit closure representation.

```c
// Closure struct
typedef struct {
    void* (*code)(Value, Closure*);  // function pointer
    Value* captured;                 // captured variables
    size_t capture_count;
} Closure;

// Example: { arg: _<@0, body }
Value outer(Value arg) {
    // Allocate closure
    Closure* closure = malloc(sizeof(Closure));
    closure->code = &inner;
    closure->captured = malloc(sizeof(Value));
    closure->captured[0] = x;
    closure->capture_count = 1;

    // Wrap closure as function value
    return box_function(closure);
}

Value inner(Value y, Closure* closure) {
    Value x = closure->captured[0];
    return add_dispatch(x, y);
}
```

Closure allocation overhead. Small closures can stack-allocate if non-escaping.

**Closure pooling.** Reuse closure objects for identical captures.

```c
// Pool for common closures
Closure* get_closure(void* code, Value* captured, size_t count) {
    // Hash code and capture values
    uint64_t hash = hash_closure(code, captured, count);

    // Check pool
    Closure* cached = closure_pool_lookup(hash);
    if (cached) {
        retain(cached);
        return cached;
    }

    // Allocate new
    Closure* new_closure = create_closure(code, captured, count);
    closure_pool_insert(hash, new_closure);
    return new_closure;
}
```

Pooling reduces allocation for repeated patterns. Useful for functions returning same closures (e.g., map iterators).

**Closure flattening.** Eliminate closures by inlining captured variables.

```javascript
// Before currying
const add = x => y => x + y;

// Flatten: generate monomorphic versions
function add_xy(x, y) {
    return x + y;
}

function add_x(x) {
    return function(y) {
        return add_xy(x, y);
    };
}
```

Flat closures reduce indirection. Compiler analyzes capture patterns, flattens when safe.

### Partial Application Codegen

Partial application happens when curried function receives fewer arguments than arity. Must return function expecting remaining arguments.

**JavaScript partial application.** Arrow functions handle naturally.

```javascript
// Left-Right: + 1 (partial application of + operator)
// Transpiled:
const add = x => y => x + y;
const add_one = add(1);  // Returns y => 1 + y
add_one(2);  // 3
```

Curry wrapper handles generic case.

```javascript
function curry(fn, arity) {
    return function curried(...args) {
        if (args.length >= arity) {
            return fn.apply(this, args);
        }
        return (...more) => curried.apply(this, args.concat(more));
    };
}

const add = curry((x, y) => x + y, 2);
const add_one = add(1);
```

Curry wrapper adds overhead. Optimization: emit direct arrow functions when partial application points are known at compile time. Left-Right's explicit currying via `_<` eliminates most need for generic wrappers.

**Native partial application.** Need explicit partial function struct.

```c
// Partial application struct
typedef struct {
    void* (*code)(Value, PartialApp*);
    Value* applied_args;      // already provided args
    size_t applied_count;     // number of applied args
    size_t total_arity;       // total expected args
} PartialApp;

// Example: add 1
Value add_one() {
    PartialApp* partial = malloc(sizeof(PartialApp));
    partial->code = &add_partial;
    partial->applied_args = malloc(sizeof(Value));
    partial->applied_args[0] = box_number(1);
    partial->applied_count = 1;
    partial->total_arity = 2;

    return box_function(partial);
}

Value add_partial(Value next_arg, PartialApp* partial) {
    // Combine applied args with new arg
    Value args[2];
    args[0] = partial->applied_args[0];
    args[1] = next_arg;

    // Call original function
    return add_xy(args[0], args[1]);
}
```

Partial application creates new struct every time. Pooling helps for repeated patterns.

**Arity-based specialization.** Generate specialized functions for each partial application arity.

```rust
// IR: add has arity 2
// Codegen: generate 3 versions
// - add_xy(x, y): fully applied
// - add_x(x): partially applied, expects y
// - add: expects x

fn gen_curried_fn(name: &str, arity: usize) -> Vec<Function> {
    let mut fns = Vec::new();

    // Full arity version
    fns.push(gen_full_fn(name, arity));

    // Partial versions (arity-1 down to 1)
    for partial_arity in (1..arity).rev() {
        fns.push(gen_partial_fn(name, arity, partial_arity));
    }

    fns
}
```

Specialization reduces overhead but increases code size. Use for hot functions.

### Map Compilation

Left-Right treats maps as fundamental data structure. Compilation must map to efficient target representation.

**Static maps.** Known at compile time. Emit literal or constant data.

```javascript
// Left-Right: {x: 1, y: 2}
// JavaScript: object literal
const map1 = {x: 1, y: 2};

// Access
map1.x;  // 1
```

Static maps fast, no overhead. Used for configuration, constant data.

**Native static maps.** Emit constant struct or global array.

```c
// Static map as struct
typedef struct {
    const char* key;
    Value value;
} MapEntry;

MapEntry static_map[] = {
    {"x", box_number(1)},
    {"y", box_number(2)},
    {NULL, box_null()}  // terminator
};

// Access: linear search or binary search
Value map_lookup(const char* key) {
    for (MapEntry* entry = static_map; entry->key != NULL; entry++) {
        if (strcmp(key, entry->key) == 0) {
            return entry->value;
        }
    }
    return box_null();
}
```

Linear search O(n). Binary search O(log n) for sorted maps. Perfect hash O(1) for known keys.

**Dynamic maps.** Created at runtime. Need efficient data structure.

```javascript
// JavaScript: Map or object
const map2 = new Map();
map2.set('x', 1);
map2.set('y', 2);
```

`Map` in JavaScript handles any key type, preserves insertion order. Objects faster for string keys.

**Native dynamic maps.** Use hash table.

```c
// Hash map implementation
typedef struct {
    Value* keys;
    Value* values;
    size_t size;
    size_t capacity;
    size_t (*hash)(Value);
} HashMap;

HashMap* hash_map_new(size_t capacity) {
    HashMap* map = malloc(sizeof(HashMap));
    map->keys = calloc(capacity, sizeof(Value));
    map->values = calloc(capacity, sizeof(Value));
    map->size = 0;
    map->capacity = capacity;
    map->hash = &value_hash;
    return map;
}

Value hash_map_get(HashMap* map, Value key) {
    size_t index = map->hash(key) % map->capacity;

    // Linear probing for collisions
    while (!is_null(map->keys[index])) {
        if (value_eq(map->keys[index], key)) {
            return map->values[index];
        }
        index = (index + 1) % map->capacity;
    }

    return box_null();  // not found
}
```

Hash table O(1) average, O(n) worst case (collisions). Use robin hood hashing or cuckoo hashing for better performance.

**Map specialization.** Analyze map usage patterns, optimize.

- **Small maps (<5 keys):** Use linear array, faster than hash.
- **String-keyed maps:** Use string interning, compare pointers.
- **Integer-keyed maps:** Use array/dense vector.
- **Fixed keys:** Compile to struct with named fields.

```rust
// Map usage analysis
fn analyze_map_usage(ir: &IRModule) -> MapStrategy {
    if map_size <= 4 {
        return MapStrategy::LinearArray;
    }
    if all_keys_are_strings() {
        return MapStrategy::StringMap;
    }
    if all_keys_are_integers() {
        return MapStrategy::DenseVector;
    }
    MapStrategy::HashMap
}
```

Specialization reduces overhead. Requires runtime dispatch to map operations.

### Conditional Map Compilation

Left-Right uses maps for conditionals with `_<` expression keys. Compile to efficient control flow.

**Expression key maps.** Maps with `_<` keys evaluate expression at runtime, dispatch based on truthiness.

```javascript
// Left-Right: {_<: trueCase, falseCase}
// Transpiled: conditional dispatch
const dispatch = (input) => {
    if (input) {
        return trueCase;
    } else {
        return falseCase;
    }
};
```

Ternary idiomatic for simple cases, if-else for multi-branch.

**Multi-key maps.** Maps with enum-like keys compile to switch.

```javascript
// Left-Right: {a: 1, b: 2, c: 3}[key]
// Transpiled to switch
switch (key) {
    case 'a': return 1;
    case 'b': return 2;
    case 'c': return 3;
    default: return null;
}
```

Switch optimized by JS engines (jump tables). Native compilers also optimize switch.

**Native conditional maps.** Use jump table or computed goto.

```c
// Native switch
Value map_lookup(Value key) {
    Tag tag = get_tag(key);

    switch (tag) {
        case TAG_STRING:
            return lookup_string(key);
        case TAG_NUMBER:
            return lookup_number(key);
        default:
            return box_null();
    }
}

// Jump table (GCC extension)
void map_dispatch(Value key) {
    static void* dispatch_table[] = {
        &&handle_string,
        &&handle_number,
        &&handle_null,
    };

    Tag tag = get_tag(key);
    goto *dispatch_table[tag];

handle_string:
    // string lookup
    goto done;

handle_number:
    // number lookup
    goto done;

done:
    return;
}
```

Jump table O(1), faster than switch for dense keys. Requires compiler support (GCC/Clang).

**Pattern matching compilation.** Left-Right maps can represent pattern matching with expression keys. Compile to decision tree.

```rust
// Map as pattern match
// {x: 1, y: 2} -> handle_xy
// {x: 1} -> handle_x
// _ -> handle_default

// Compile to decision tree
fn compile_map_as_pattern(map: &Map) -> IRNode {
    let branches = map.branches();

    if branches.len() == 1 {
        // Direct match
        return emit_direct_match(&branches[0]);
    }

    // Split on first key
    let first_key = branches[0].keys[0];
    let matching = branches.iter().filter(|b| b.keys.contains(&first_key));
    let default = branches.iter().filter(|b| !b.keys.contains(&first_key));

    IRNode::If {
        cond: emit_key_check(first_key),
        then: compile_map_as_pattern(&collect(matching)),
        else: compile_map_as_pattern(&collect(default)),
    }
}
```

Decision tree reduces comparisons from O(n) to O(log n). Critical for large maps.

Next section covers production examples and final recommendations.

## Section 6: Production Examples and Recommendations

Production compilers face trade-offs between compilation speed, runtime performance, and complexity. Study existing systems to understand what works.

### Rustc Codegen

Rustc uses LLVM as backend with MIR (Mid-level IR) before LLVM IR.

**MIR design.** MIR is control-flow graph based, type-carrying, relatively low-level but higher than LLVM IR. Handles Rust-specific features like borrow checking.

```
// Rust: x + y
// MIR:
let _1: i32;    // x
let _2: i32;    // y
let _3: i32;    // result

_3 = Add(_1, _2);
return _3;
```

MIR provides type information for borrow checker, then lowers to LLVM IR where types are erased. Left-Right's bytecode VM follows similar pattern: high-level IR for semantic checks, lowers to bytecode for execution.

**Codegen pipeline.**

1. **AST → HIR (High-level IR):** Desugaring, macro expansion, name resolution.
2. **HIR → MIR:** Type checking, borrow checking, monomorphization.
3. **MIR → LLVM IR:** Lowering, optimization passes.
4. **LLVM IR → Machine code:** LLVM codegen passes.

Rustc's layering enables each phase to focus on specific concerns. HIR handles high-level semantics, MIR handles middle-level analysis, LLVM handles low-level optimization.

**Lesson for Left-Right:** Use multiple IR layers. High-level IR for language-specific analysis (currying, maps), low-level IR for codegen (LLVM, Cranelift). Don't try to do everything in one IR.

**Monomorphization.** Rust generics are monomorphized at compile time. For each generic use, generate specialized code.

```rust
fn add<T: Add>(a: T, b: T) -> T {
    a + b
}

// Two uses: add::<i32> and add::<f64>
// Generates two specialized functions:
fn add_i32(a: i32, b: i32) -> i32 { a + b }
fn add_f64(a: f64, b: f64) -> f64 { a + b }
```

Monomorphization increases compile time and binary size, but runtime performance is optimal.

Left-Right can monomorphize operator calls when type analysis shows monomorphic use.

### TypeScript Compiler

TypeScript compiler transpiles TypeScript to JavaScript. No native backend, but multi-target (ES5, ES6, ESNext, etc.).

**Type erasure.** TypeScript types erased at compile time.

```typescript
// TypeScript
function add(a: number, b: number): number {
    return a + b;
}

// Emits JavaScript (types erased)
function add(a, b) {
    return a + b;
}
```

Type checking occurs before codegen. Codegen works on untyped AST.

Left-Right should adopt similar pattern: runtime type inference in IR phase, emit untyped JavaScript, type dispatch for bytecode VM/native codegen.

**Target selection.** TypeScript supports multiple ECMAScript targets.

```bash
tsc --target ES5        # Older browsers
tsc --target ESNext     # Modern browsers, Node.js
tsc --target ES2020     # Specific version
```

Each target has feature detection, fallbacks for unsupported syntax.

Left-Right's multi-target needs similar selection: ES5 for broad support, ES2020+ for modern environments. Bytecode VM provides consistent runtime across all targets.

**Source maps.** TypeScript generates source maps linking output JS to TS source. Critical for debugging. Left-Right must generate source maps for JavaScript output. For bytecode VM, debug info provides source-to-bytecode mapping for breakpoints and error traces.

Left-Right is dynamically typed with no type syntax, so type declaration files aren't applicable. JavaScript backends rely on JSDoc comments for optional IDE type hints. For native backends, runtime type checking handles all operations.

### ClojureScript

ClojureScript transpiles Clojure to JavaScript. Handles dynamic typing, macros, immutable data structures.

**Closure compiler integration.** ClojureScript optionally uses Google Closure Compiler for advanced optimizations.

```bash
clj --optimizations advanced
```

Advanced optimizations rename functions, inline code, dead-code eliminate based on whole-program analysis.

ClojureScript's use of external optimizer is smart. Left-Right could integrate with terser for JavaScript, LLVM passes for native code.

**JavaScript interop.** ClojureScript provides clean interop with JavaScript libraries.

```clojure
;; Call JavaScript Math.max
(Math/max 1 2 3)

;; Access JavaScript object property
(. js/window location)

;; Create JavaScript object
#js {:key "value"}
```

Interop layer maps ClojureScript types to JavaScript types seamlessly.

Left-Right needs interop layer for JavaScript targets. For native targets, FFI bridges to C/Rust libraries.

**Persistent data structures.** ClojureScript implements persistent vectors, maps, sets. Compiles to efficient JavaScript.

```clojure
;; Persistent vector
(def v [1 2 3])
(conj v 4)  ;; [1 2 3 4] (new vector, v unchanged)
```

Implementation uses structural sharing: new vectors share structure with old, modifying only path to updated element.

Left-Right's arrays are mutable by default. Could add persistent variants as library.

### Recommended Strategy for Left-Right

Based on research, Left-Right should adopt hybrid strategy combining IR-based codegen with target-specific optimization.

**IR design.**

1. **HIR (High-level IR):** Close to Left-Right syntax, curried closures (maps with `_<`) explicit, map-as-closure semantics preserved. Runtime type inference performed here.

2. **LIR (Low-level IR):** Lowered, closer to bytecode VM model. Currying flattened, maps specialized, SSA form.

```rust
enum HIRNode {
    Closure { name: String, captures: Vec<HIRNode>, body: HIRBlock },
    Apply { func: HIRNode, arg: HIRNode },
    Map { entries: Vec<(HIRNode, HIRNode)> },
    // ...
}

enum LIRNode {
    Closure { name: String, arity: usize, captures: Vec<LIRNode> },
    Call { func: LIRNode, args: Vec<LIRNode> },
    BinOp { op: String, left: LIRNode, right: LIRNode },  // op is identifier
    // ...
}
```

Layering enables phase-specific optimizations. HIR optimizes currying, map patterns. LIR optimizes for bytecode VM/LLVM codegen targets.

**Backend selection.**

- **Bytecode VM backend:** Primary target. Emit bytecode instructions for the Left-Right VM. Consistent semantics across all environments.
- **JavaScript backend:** Emit ES2020 by default, ES5 as fallback. Use terser for minification. Generate source maps.
- **LLVM backend:** Generate LLVM IR from LIR, use LLVM optimization passes, compile to machine code.

Multi-target enabled by shared LIR. Bytecode VM execution model ensures consistent behavior.

**Dynamic typing strategy.**

- **JavaScript:** Use JavaScript's dynamic typing for primitives, custom tags for maps/functions.
- **Native:** Tagged union representation, type tag in high bits, operator dispatch with inline cache.
- **WASM:** Linear memory with type tables, GC integration via JavaScript.

Target-specific representation chosen based on capabilities.

**Currying implementation.**

- **HIR:** Explicit currying via `_<` references, partial application tracked.
- **LIR:** Currying flattening, arity-based specialization.
- **Codegen:**
  - Bytecode VM: Stack-based or register-based instructions, closure bytecode.
  - JavaScript: Arrow functions, curry wrappers for dynamic cases.
  - Native: Closure structs, partial application structs, closure pooling.

**Map implementation.**

- **HIR:** Map-as-closure semantics preserved, map patterns analyzed.
- **LIR:** Map specialization (static/dynamic, string/int keys, small/large maps).
- **Codegen:**
  - Bytecode VM: Map bytecode instructions, lookup tables.
  - JavaScript: Objects for static maps, Map for dynamic maps, direct access optimization.
  - Native: Hash tables for dynamic maps, perfect hash for static maps, switch for conditional maps.

**Optimization passes.**

1. **HIR passes:**
   - Currying fusion: f (g x) → (f ∘ g) x
   - Map pattern analysis: detect static maps, small maps, enum-like maps.
   - Type inference: monomorphic call sites identified.

2. **LIR passes:**
    - Dead code elimination.
    - Constant folding.
    - Closure inlining for hot paths.
    - Map iteration optimization.

3. **Target-specific:**
    - Bytecode VM: Bytecode optimization, peephole patterns.
    - JavaScript: Terser optimization, tree shaking.
    - Native: LLVM optimization passes (O2 by default).

**Testing strategy.**

- **IR validation:** Separate tests for HIR and LIR semantics.
- **Backend testing:** Same IR compiled to all targets, outputs tested for semantic equivalence.
- **Cross-compilation:** LR → HIR → LIR → JS → HIR → LIR → native, verify round-trip.

**Development phases.**

1. **Phase 1 (MVP):** Bytecode VM only, direct AST→bytecode codegen, no IR. Fast to prototype, verify language design.
2. **Phase 2 (IR):** Add HIR, LIR. Bytecode VM uses LIR, preserves semantics across code changes.
3. **Phase 3 (JavaScript):** Add JavaScript backend, generate ES2020 code. Compare performance to bytecode VM.
4. **Phase 4 (Optimization):** Add optimization passes, type inference, monomorphization.
5. **Phase 5 (Native):** Add LLVM backend, generate native code. Compare performance to bytecode/JS.

Incremental approach reduces risk. Each phase builds on previous, enabling early feedback.

**Conclusion.**

Code generation is the bridge between language design and execution. Left-Right's multi-target requirements demand careful IR design and backend strategy. IR-based codegen with HIR/LIR layering enables language-specific optimizations while maintaining target flexibility. **The bytecode VM is the primary target** — code generation focuses on emitting bytecode instructions that ensure consistent semantics. Target-specific backends (JavaScript, LLVM) transpile bytecode or compile IR directly, but the bytecode VM execution model is the foundation. Dynamic typing handled via runtime type tags. Currying and maps compiled with arity-based specialization and map analysis respectively. Production systems like Rustc, TypeScript, and ClojureScript provide proven patterns for multi-target compilation. Left-Right should adopt these patterns, adapting to point-free, curried, map-based semantics with the bytecode VM as the core execution model.