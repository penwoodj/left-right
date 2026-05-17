# Type Systems and Semantic Analysis

## REPORT 7 OF 8

### For Building a Compiled Programming Language (Left-Right)

---

## 1. Semantic Analysis Overview

Semantic analysis sits between parsing and code generation in the compilation pipeline. After parsing confirms syntactic correctness, semantic analysis ensures that the program has meaning according to the language rules. This phase catches errors that the grammar cannot express, builds rich representations of program structure, and prepares information that later stages need for optimization and code generation.

### Purpose in Compilers

Semantic analysis serves several critical functions. First, it enforces semantic rules that go beyond syntax. A parsed expression like `x + 3` is syntactically valid, but semantically meaningless if `x` was never defined. Semantic analysis catches this. Second, it constructs symbol tables and type information that later stages need. Third, it performs transformations that simplify later compilation. Fourth, it provides warnings about potentially problematic code before runtime.

Static languages like Java or Rust rely heavily on semantic analysis to catch type errors, uninitialized variables, and other issues before the program runs. Dynamic languages like JavaScript or Python still perform semantic analysis, but it focuses more on name resolution, scope validation, and detecting obviously invalid code rather than type checking.

### Name Resolution and Scope Analysis

Name resolution determines what each identifier refers to in the program. This involves constructing scope hierarchies, resolving variable references to their declarations, and handling shadowing and closure capture. The compiler builds a symbol table that maps names to declarations, type information, and other metadata.

Scope analysis walks the abstract syntax tree to determine the scope boundaries in the program. Each scope contains the names visible in that region. When the compiler encounters an identifier reference, it looks up the name in the current scope, then outer scopes, until finding a declaration or reaching the global scope.

For Left-Right, scope works differently than most languages. Map keys become variable names after comma operators. A map like `{name: "John", age: 30}` creates bindings for `name` and `age` that subsequent code can reference. Nested maps create nested scopes. This means scope analysis must understand the structure of maps and when bindings come into existence.

### Type Checking

Type checking validates that operations are performed on compatible types. Static type checking happens at compile time and rejects programs that would cause runtime type errors. Dynamic type checking defers validation to runtime, but compilers can still perform type inference to generate better code.

Even for dynamic languages, type checking provides value. The compiler can infer likely types and generate warnings for obviously incorrect operations. It can also optimize code by specializing operations based on inferred types when the inference is confident.

Left-Right has seven types, but they are not statically declared. Operators behave differently based on their argument types. Type checking therefore focuses on detecting impossible operations and inferring likely types for optimization.

### Flow Analysis

Flow analysis tracks how values flow through the program. This includes control flow analysis (determining which code paths are possible), data flow analysis (tracking where values come from and go), and various specific analyses like definite assignment (ensuring variables are assigned before use) and reachability (finding unreachable code).

Flow analysis builds a control flow graph that represents all possible execution paths through the program. Each node in the graph represents a basic block, a sequence of instructions that always execute together without branching. Edges represent possible transitions between blocks.

For Left-Right, flow analysis must understand how map operators create control flow. When a map is used in a conditional context, its truthiness determines which path executes. The `!!!` operator throws errors, and `!!!?` catches them. These operators create control flow edges that the analysis must capture.

### Semantic Checks for Dynamic Languages

Dynamic languages have fewer semantic checks than static languages, but they still benefit from analysis. Common checks include detecting references to undefined variables, identifying unreachable code, flagging suspicious patterns, and inferring types for optimization.

The challenge for dynamic languages is that many semantic properties cannot be determined statically because they depend on runtime behavior. The compiler must balance catching errors early with avoiding false warnings for code that is technically valid but unusual.

Left-Right's dynamic nature means most semantic checks will be warnings rather than errors. The compiler can identify likely problems but should not reject code that might be valid at runtime. The goal is to help developers without overly constraining the language's flexibility.

---

## 2. Dynamic Language Type Analysis

Dynamic languages like Left-Right perform type checking at runtime, but compilers can still reason about types to generate better code and catch errors. This section explores techniques for type analysis in dynamic languages and how modern engines like V8 and SpiderMonkey optimize dynamic code.

### Type Inference for Dynamic Languages

Type inference in dynamic languages, often called soft typing, deduces likely types from how values are used. The compiler analyzes operations and assignments to infer what types values will have at different points in the program. These inferred types guide optimization decisions.

Basic type inference works forward through the program. When an operation requires specific compatible types, like `+` requiring two numbers (for addition) or two maps (for merge), the compiler infers that the operands likely have those types. When a value is assigned to a variable, the variable inherits the inferred type of the value.

More sophisticated inference uses constraint solving. The compiler generates type constraints from operations and solves them to find consistent type assignments. For example, if `x + 1` occurs and later `x[0]` occurs, the constraints require `x` to be both a number (for addition) and something indexable (for indexing), which is inconsistent. The compiler can report this as a likely error.

Left-Right's operators have type-dependent behavior, which complicates inference. The `@` operator can perform map lookup, list indexing, or string character access depending on its operand types. Inference must consider all possible behaviors and either warn about ambiguity or assume the most likely case.

### Gradual Typing Approaches

Gradual typing combines static and dynamic typing by allowing optional type annotations that the compiler enforces while still allowing untyped code. This provides the safety of static typing where developers want it and the flexibility of dynamic typing elsewhere.

In a gradually typed system, type annotations are optional but checked when present. Code without annotations runs dynamically. The boundary between typed and untyped code requires runtime checks to preserve safety. Performance impact can be significant, but research has developed techniques to minimize it.

Left-Right does NOT have type annotations, type signatures, or type declarations. All typing is dynamic at runtime. Compiler can infer likely types for optimization but cannot rely on explicit type information. Gradual typing concepts are not directly applicable to Left-Right, though the compiler could learn types from profiling for optimization.

### Type Specialization at Runtime

Many dynamic language engines specialize code at runtime based on observed types. When a function is called repeatedly with the same argument types, the engine generates a specialized version optimized for those types. If the types change, the engine falls back to a generic version.

V8's inline caching and hidden class optimizations work this way. When a property is accessed on an object, V8 remembers the object's shape. Subsequent accesses on objects with the same shape take a fast path. When a different shape appears, V8 generates a new cache entry.

SpiderMonkey uses similar techniques with shape-based property access and type-specialized JIT compilation. IonMonkey, its optimizing compiler, generates machine code specialized for observed types and deoptimizes if assumptions are violated.

Left-Right can benefit from similar approaches. When an operator is called repeatedly with consistent argument types, generate a specialized version. For example, if `@` is always called with a map and a string, specialize it to map lookup without checking for other combinations.

### Profile-Guided Type Information

Profile-guided optimization collects runtime type information and uses it to guide compilation. The program runs with instrumentation to record which types actually occur. The compiler then uses this data to make informed decisions about specialization and optimization.

Profile-guided optimization is particularly valuable for dynamic languages because static inference is limited. Real data about runtime behavior fills in gaps that static analysis cannot resolve. Engines often collect profiles continuously during normal execution, gradually improving optimization over time.

Left-Right's compiler could implement a profiling mode that records type information and operator usage patterns. Even simple statistics, like which operators are most common or what types typically flow through them, guide optimization priorities.

### How V8 and SpiderMonkey Optimize Dynamic Types

V8 and SpiderMonkey have developed sophisticated techniques to optimize dynamic languages without sacrificing flexibility. Understanding these techniques informs Left-Right's optimization strategy.

**V8's approach:**
- Hidden classes track object shapes for fast property access
- Inline caching remembers results of operations to skip lookups
- TurboFan optimizes hot code based on observed types
- Deoptimization safely falls back to slower code when assumptions fail

**SpiderMonkey's approach:**
- Shape-based property access similar to hidden classes
- Type-specific inline caches for arithmetic and comparisons
- IonMonkey performs type inference on SSA form
- WarpBuilder compiles to machine code with type guards

Both engines emphasize gradual optimization. Code starts interpreted, gets profiled, and eventually compiled to specialized machine code. When assumptions break, they deoptimize gracefully. This preserves correctness while achieving performance when types are stable.

Left-Right can adopt a similar strategy. Start with generic implementations, profile operator usage and type patterns, then specialize hot code paths. The difference is that Left-Right's operators are more uniformly data-driven, so specialization might focus on operator behavior rather than object shapes.

---

## 3. Scope and Binding Analysis

Scope and binding analysis determines what identifiers refer to throughout the program. This is fundamental for semantic analysis, optimization, and code generation. Left-Right's approach to scope differs from traditional languages, requiring specialized analysis techniques.

### Map Keys Become Variables After Comma

Left-Right uses commas to create bindings from map keys. When a map contains keys, those keys become available as variables in subsequent expressions. For example, `{x: 1, y: 2}, x + y` evaluates to 3. The map's keys bind the values, and later expressions can reference them.

This design means scope analysis must track which bindings are available at each point in the program. Unlike traditional block scoping where variables are visible within a lexical block, Left-Right's bindings flow forward through the program. The compiler must build a binding chain that tracks available variables as it progresses through the AST.

When the compiler encounters a map, it extracts the keys as new bindings. These bindings are added to the current environment. When the compiler encounters a comma operator, it processes the left side, then the right side with the bindings from the left available.

Binding analysis must also handle shadowing. If the same key appears in multiple maps, later bindings shadow earlier ones. The compiler tracks binding lifetimes to know when a shadowed binding becomes accessible again.

### Nested Scope via Nested Maps

Nested maps create nested scopes in Left-Right. When a map contains another map as a value, the inner map's keys become bindings available only when the outer map is accessed. This creates a hierarchical scope structure.

For example, `{outer: {inner: 5}, outer}` returns the inner map. The key `inner` is not directly available at the top level, but is available through the nested structure. This requires scope analysis to understand the nesting relationships and which bindings are accessible from which contexts.

The compiler builds a scope tree that mirrors the map nesting. Each map node has child scopes for any nested maps. When resolving an identifier, the compiler searches up the scope tree until finding a binding or reaching the root.

This approach differs from lexical scoping in most languages because the nesting is data-driven rather than syntax-driven. The scope structure depends on the runtime structure of maps, not on the syntactic structure of blocks. Static analysis must consider what map structures are statically determinable.

### Import Binding Resolution

Left-Right's import system uses a runtime variable (a map) called `imports`, not a keyword. Import syntax: `imports@[`lodash`, `fp`]` uses the `@` get operator with a list of strings for nested path access. `files@[`path`]` for local file imports.

Import binding analysis recognizes this pattern but cannot perform static import analysis because `imports` is a runtime variable. No static import validation is possible—imports are resolved at runtime.

When analyzing imports, the compiler can only recognize the import syntax pattern. It cannot validate that imported names exist in the source module because the module structure is only known at runtime. The compiler can detect circular dependencies in some cases when import patterns are statically analyzable, but this is limited.

Import resolution is entirely dynamic. Circular dependencies are handled at runtime, not detected statically.

### Operator Argument Binding

Left-Right has special reference mechanisms for operator arguments. The `_<` and `_>` identifiers reference the left argument and right argument of the enclosing operator. `_<@N` accesses the Nth element of the left argument (for lists or maps).

Argument reference analysis determines how operators access their arguments. This is crucial for operator implementation and understanding what an operator can access. `_<` and `_>` are NOT variable binding operators—they reference existing arguments, they do not create new bindings.

For extensible operators defined via map syntax, argument reference determines what the operator definition can access. If operators are defined as maps with specific keys, the compiler must ensure that invocations provide arguments in the expected form.

Argument reference also interacts with closure capture. If an operator is defined in a context with certain bindings, the operator might capture those bindings and use them later. The compiler must track which bindings each operator depends on.

### Closure Variable Capture Analysis

Closures in Left-Right occur when a function or operator references bindings from its enclosing scope. Closure capture analysis determines which variables each closure captures and how those captured values are accessed.

When analyzing a closure, the compiler identifies all free variables, variables that are referenced but not defined within the closure. These free variables must come from the enclosing scope. The compiler records this dependency for later stages.

Closure capture affects code generation strategies. Captured variables must be stored in a way that allows the closure to access them after the enclosing scope has exited. This typically involves heap allocation of captured environments.

Left-Right's map-based closures might use map structures to store captured bindings. The compiler must determine when to create closure maps and what bindings to include in them. Static analysis can optimize by capturing only the bindings that are actually used.

---

## 4. Operator Type Analysis

Left-Right's operators have type-dependent behavior, making operator type analysis critical for both correctness and optimization. This section explores how to analyze operator types, build type flow graphs, infer result types, and identify specialization opportunities.

### Each Operator Has Type-Dependent Behavior

In Left-Right, operator behavior varies based on argument types. The `@` operator performs map lookup when given a map and string, list indexing when given a list and number, and string character access when given a string and number. This polymorphism makes type analysis more complex but also more powerful.

Type-dependent behavior means the compiler cannot determine operator behavior purely from syntax. It must infer or track types to understand what each operator invocation does. This requires building type information that flows through the program.

The challenge is that in a dynamic language, types are not statically known. The compiler must consider all possible type combinations that an operator might encounter and either handle them generically or specialize based on inferred types.

Operator type analysis starts by enumerating all operators and their behavior for each type combination. This creates a dispatch table that maps operator plus argument types to behavior. The compiler then uses this table during analysis and code generation.

### Building Type Flow Graphs

Type flow graphs track how types propagate through the program. Each node represents a value or operation, and edges represent how types flow from one node to another. Type flow analysis computes the likely types for each node based on its inputs and operations.

For Left-Right, type flow analysis considers how operators transform their input types to output types. For example, if the `+` operator receives two numbers, its output type is a number. If it receives two maps, its output type is a map (through merging). The compiler builds this information into the type flow graph.

Type flow analysis can be performed in multiple passes. A forward pass propagates types from inputs to outputs. A backward pass refines types by considering how values are used. Iterative passes can resolve circular dependencies and improve precision.

The type flow graph supports optimization by identifying opportunities for specialization. When a value always has a specific type throughout a region, the compiler can specialize operations on that value. When types are uncertain, the compiler generates generic code that handles all possibilities.

### Inferring Operator Result Types

Given operator behavior rules and input types, the compiler can infer the likely result type of each operator invocation. This inference guides optimization and can catch errors.

Simple inference looks at operator dispatch rules. If the compiler infers that an operator is called with specific argument types, it looks up the corresponding behavior and uses the result type from that behavior.

More sophisticated inference considers multiple possible type combinations. When argument types are uncertain, the result type is the union of possible result types. The compiler might track probability information to prioritize likely cases.

Left-Right's operators sometimes return different types based on runtime conditions. For example, a map lookup might return any type depending on the stored value. Inference must handle this by tracking the most general type or by using type sets that represent multiple possibilities.

When inference detects inconsistency, like an operator receiving a type combination it does not support, the compiler can generate a warning. This catches likely errors without rejecting potentially valid code.

### Multi-Method Dispatch Analysis

Multi-method dispatch, where method selection depends on multiple argument types rather than just the receiver, is central to Left-Right's operator system. Dispatch analysis determines which operator implementation will be called for each invocation.

Traditional single dispatch languages select methods based on the receiver type only. Left-Right's multi-method dispatch considers all argument types, making dispatch more complex but more expressive.

Dispatch analysis must consider the order of argument types in dispatch resolution. When multiple operator implementations could match a given argument combination, the language needs rules to select one. Common approaches include specificity (more specific implementations win), declaration order, or explicit priority.

The compiler can optimize dispatch by caching the results of previous dispatch decisions. When the same argument types occur repeatedly, use the cached implementation directly. This is analogous to inline caching in V8.

For extensible operators defined via map syntax, dispatch analysis must consider dynamically defined operators. The compiler may need to emit generic dispatch code that searches through available operator definitions at runtime.

### Specialization Opportunities from Type Analysis

Type analysis reveals opportunities to specialize code for specific types. Specialization improves performance by eliminating runtime type checks and using type-specific fast paths.

Common specialization opportunities include:
- Arithmetic operations on numbers
- Collection operations on maps vs lists vs strings
- Comparison operations that have type-specific semantics
- Property access patterns that are consistent

When type analysis determines that a variable always has a specific type within a region, the compiler can replace generic operations with specialized ones. For example, if a variable is always a number, use native addition instead of checking for string concatenation.

Specialization requires guards to ensure assumptions hold at runtime. If a specialized assumption is violated, the code must deoptimize to a generic implementation. V8 and SpiderMonkey use this approach extensively.

Left-Right's operator-centric design might benefit from specializing entire operator chains rather than individual operators. When a sequence of operators consistently receives certain types, generate a specialized chain that avoids intermediate type checks.

---

## 5. Type Coercion Analysis

Type coercion occurs when values are implicitly converted between types. Left-Right includes explicit coercion operators like `?` for toBoolean conversion, but operators may also perform implicit coercion. Analysis of coercion helps catch errors and optimize code.

### Our `?` ToBoolean Coercion Rules

The `?` operator converts values to booleans, which is crucial for conditional logic. Understanding the coercion rules is essential for both semantic analysis and optimization.

Left-Right likely follows JavaScript-like coercion rules:
- Numbers: 0 and NaN coerce to false, other numbers to true
- Strings: Empty string coerces to false, non-empty to true
- Booleans: Identity (true stays true, false stays false)
- Maps: Non-empty maps coerce to true, empty maps to false
- Lists: Non-empty lists coerce to true, empty lists to false
- Undefined: Always coerces to false

Coercion analysis tracks when `?` is applied and what the likely result is. This enables dead code elimination (removing code in unreachable branches) and branch prediction.

The compiler can also optimize `?` by using fast paths for common cases. For example, checking if a number is zero is faster than calling a generic toBoolean function.

### Implicit Coercion in Operators

Left-Right does NOT support implicit type coercion. Operators fail if types are incompatible. No automatic string→number, number→boolean, or other implicit conversions.

Analysis of type compatibility helps catch errors and optimize code. The compiler can warn when operations are called with incompatible types that will fail at runtime.

For Left-Right, type compatibility requirements apply to:
- Arithmetic operators that ONLY accept numbers
- Comparison operators that work across compatible types
- Collection operations that expect specific collection types
- Operators that require boolean context (use `?` for explicit conversion)

Analysis must determine whether type combinations are compatible or will cause runtime failures. When types are compatible, the compiler can optimize. When incompatible, the compiler should warn.

### Type Mismatch Detection at Compile Time

Even in a dynamic language, some type mismatches are statically detectable. If an operator only supports certain type combinations and static analysis determines that incompatible types will be passed, this is a likely error.

Type mismatch detection involves comparing inferred types against operator requirements. When the intersection is empty, no valid execution exists. When it is non-empty but narrow, the compiler can specialize. When it is broad, the compiler must handle all cases.

For example, if Left-Right's `@` operator for map lookup only accepts a string key, and analysis determines that the key argument will always be a number, this is a type mismatch. The compiler can warn or error.

Type mismatch detection must balance precision with false positives. Being too strict rejects valid dynamic code. Being too lenient misses real errors. Left-Right's design should define which mismatches are errors, warnings, or acceptable.

### Warning Generation for Suspicious Coercions

When coercion occurs, the compiler can evaluate whether it is intentional or potentially accidental. Suspicious coercions might indicate bugs and warrant warnings.

Suspicious coercion patterns include:
- Coercing undefined to boolean in a conditional (always false)
- Coercing complex values like maps to strings
- Mixing types in operations where one type clearly dominates
- Chains of coercion that suggest confusion

The compiler can use heuristics to identify these patterns. For example, if a boolean test always receives values that coerce to false, the code is likely buggy.

Warning generation should be tunable. Developers might want stricter warnings in development but fewer warnings in production. Left-Right could provide configuration options to adjust warning sensitivity.

---

## 6. Flow Analysis

Flow analysis tracks how control and data flow through the program. This enables optimizations, error detection, and ensures program correctness. Left-Right's unique control flow based on map operators requires specialized analysis techniques.

### Control Flow Graph Construction

The control flow graph (CFG) represents all possible execution paths through the program. Each node is a basic block, a sequence of operations that always executes together. Edges represent possible transfers of control between blocks.

CFG construction involves identifying points where control can branch, such as conditionals, loops, and error handling. For each branch point, the compiler creates edges for each possible path.

Left-Right's conditionals are based on map truthiness rather than explicit boolean expressions. When a map appears in a conditional context, its truthiness determines which path executes. The compiler must understand map truthiness rules to build accurate CFGs.

CFG construction also handles exceptional control flow. The `!!!` operator throws errors, and `!!!?` catches them. These create edges from throw sites to catch handlers. The compiler must ensure that error paths are represented in the CFG.

### Maps as Conditionals Create Branches

In Left-Right, maps used in conditional contexts create branches based on their truthiness. This differs from traditional languages that have explicit boolean expressions.

Conditional analysis must determine when a map is used in a conditional context. Common contexts include:
- As the condition in an if-statement equivalent
- As the test in a loop condition
- As the operand of logical operators
- After applying the `?` toBoolean operator

When the compiler encounters a map in a conditional context, it analyzes whether the map's truthiness can be statically determined. If the map is a literal with known contents, its truthiness is known. If the map is computed, its truthiness is unknown at compile time.

For maps with unknown truthiness, the compiler creates branches in the CFG. One branch assumes the map is truthy, the other assumes it is falsy. Subsequent analysis must consider both possibilities.

This approach enables dead code elimination. When analysis determines that a branch is never reachable, the compiler can remove the code in that branch.

### Error Handling Flow (`!!!` throw, `!!!?` catch)

Left-Right provides explicit error handling with `!!!` for throwing and `!!!?` for catching. Error handling flow analysis tracks how errors propagate and where they are caught.

Throw analysis identifies all throw sites and determines what exceptions might be thrown. This includes direct throws via `!!!` and implicit throws from failed operations.

Catch analysis identifies catch handlers and what exceptions they handle. When `!!!?` is used, it catches the most recent exception and makes it available as a binding.

The compiler builds an exception flow graph that connects throws to catches. This graph ensures that exceptions reach appropriate handlers and enables optimizations like exception table construction for runtime dispatch.

Error handling analysis also detects uncaught exceptions. When a throw has no corresponding catch in the call chain, this might indicate a bug or be intentional for top-level error reporting.

### Async Flow (`///` / `\\\`)

Async operators in Left-Right (likely `///` for async spawn and `\\\` for async await) create asynchronous control flow that standard CFGs cannot represent.

Async flow analysis requires extended models that track parallel execution paths, synchronization points, and dependencies between async operations. The compiler must understand which operations can run concurrently and which must wait for previous results.

When `///` spawns an async operation, the compiler notes that subsequent code can execute before the async operation completes. This affects data flow analysis because values from the async operation are not immediately available.

When `\\\` awaits an async operation, the compiler marks a synchronization point. Code after the await can only execute after the awaited operation completes. The compiler must ensure proper ordering and handle potential exceptions.

Async analysis also detects deadlocks, race conditions, and other concurrency issues. While static detection of these problems is undecidable in general, the compiler can identify common patterns that are likely problematic.

### Dead Code Detection in Map Operators

Map operators can create dead code when some branches are never executed. Dead code detection identifies and removes this unreachable code, reducing program size and improving maintainability.

Dead code occurs when:
- A conditional branch has a known condition
- An operator always produces a specific result
- A binding is always overwritten before use
- A map key is always the same value

For map operators, dead code often appears in the form of unreachable map branches. If a map's keys can be statically determined and only some keys are ever used, the unused branches represent dead code.

The compiler performs dead code elimination after building the CFG. Starting from the entry point, it marks reachable blocks. Any blocks not marked are unreachable and can be removed.

Dead code elimination must be conservative in dynamic languages. Just because a branch looks unreachable based on static analysis does not mean it is unreachable at runtime. The compiler should only remove code when it can prove unreachability.

---

## 7. Linting and Static Analysis

Linting and static analysis provide developers with feedback about code quality, potential bugs, and style issues. For a point-free language like Left-Right, traditional linting rules do not apply directly, requiring language-specific analysis.

### Undefined Variable Detection

Undefined variable detection identifies references to names that are not defined in any accessible scope. This catches typos and missing declarations.

In Left-Right, undefined variable detection is complicated by map-based scope. Variables come from map keys, so the compiler must track which keys have been defined at each point in the program.

When the compiler encounters an identifier reference, it looks up the name in the current binding chain. If the name is not found, this is a potential undefined variable.

However, dynamic languages sometimes intentionally reference undefined names for metaprogramming. Left-Right might allow this for operator lookup or map key access. The compiler must distinguish between intentional dynamic access and accidental undefined references.

Undefined variable detection can be configured as error or warning depending on strictness. In strict mode, all variables must be statically defined. In permissive mode, dynamic access is allowed but flagged for review.

### Unused Operator Detection

Unused operator detection identifies operators that are defined or imported but never called. This helps remove dead code and understand what operators are actually used.

Detection involves tracking which operators are called and which are defined but not called. For extensible operators defined via maps, the compiler analyzes map definitions to determine which operators they provide.

Unused detection must consider indirect uses. An operator might be passed as a value and called indirectly. The compiler must track higher-order uses to avoid false positives.

Operators might also be used dynamically via string names. For example, an operator might be looked up from a map using a string key. Static analysis cannot always detect these uses, so unused detection should be conservative.

### Complexity Analysis

Complexity analysis measures how complex code is, often to encourage maintainability. Common metrics include cyclomatic complexity (number of independent paths), nesting depth, and operator count.

For Left-Right, complexity analysis focuses on operator nesting and map depth. Deeply nested operators are harder to understand and maintain. The compiler can measure nesting depth and flag excessively complex code.

Map complexity is also important. Large maps with many keys or nested structures are harder to reason about. The compiler can measure map size and depth.

Point-free languages like Left-Right can be dense, making complexity metrics less straightforward. A single line of Left-Right might express substantial logic that would be multiple lines in another language. Complexity analysis should account for this density.

### Style Checking for a Point-Free Language

Traditional style rules focus on formatting, naming, and code organization. Point-free languages need different style checks because the code structure is fundamentally different.

Style checks for Left-Right might include:
- Consistent operator usage patterns
- Appropriate use of implicit vs explicit coercion
- Map organization and key naming
- Avoiding unnecessarily complex operator chains
- Consistent use of binding mechanisms

Style checking can help enforce conventions that make Left-Right code more readable. For example, the compiler might suggest breaking up long operator chains or using intermediate bindings for clarity.

Because Left-Right is unconventional, style conventions will emerge from community practice. The compiler should be configurable to support evolving style preferences.

---

## 8. Implementation Recommendations

This section provides concrete recommendations for implementing semantic analysis for Left-Right, balancing practical feasibility with the language's unique requirements.

### What Semantic Analysis is Feasible for Our Dynamic Language

Not all semantic checks are practical for a dynamic language. Left-Right's flexibility means some properties cannot be determined statically without rejecting valid programs.

Feasible analysis includes:
- Scope and binding resolution, since map-based scope is mostly static
- Operator type inference for hot paths with stable types
- Control flow graph construction for optimization
- Dead code detection for provably unreachable code
- Basic linting for undefined variables and unused operators

Less feasible analysis includes:
- Full type checking, since types are dynamic by design
- Precise type inference for all code paths
- Data flow analysis that tracks all possible values
- Comprehensive error detection that eliminates runtime errors

The recommendation is to start with feasible checks and gradually add more sophisticated analysis as experience shows what is valuable. Prioritize analysis that enables optimization and catches common errors over analysis that attempts to prove program correctness.

### Scope Resolution Algorithm

Scope resolution for Left-Right should work as follows:

1. **Initialize environment** with pre-defined bindings for built-in operators and types.

2. **Walk the AST** in execution order, maintaining a current environment that tracks available bindings.

3. **When encountering a map**, extract its keys and add them to the environment for subsequent expressions. Handle shadowing by keeping track of previous bindings.

4. **When encountering a comma operator**, process the left expression, then the right expression with the bindings from the left available.

5. **When encountering a nested map**, create a new environment scope for the nested keys. The nested scope inherits bindings from the parent but does not add bindings to the parent.

6. **When encountering an identifier reference**, look up the name in the current environment. Search from innermost scope outward. If not found, report as potential undefined variable.

7. **When encountering closures**, capture the current environment for the closure's use. Determine which bindings the closure actually accesses to optimize closure capture.

This algorithm handles Left-Right's map-based scope while supporting nested structures and closures. Implementation can use a linked environment structure where each scope points to its parent scope.

### Type Flow Analysis Approach

Type flow analysis should adopt a pragmatic approach that balances precision with practicality:

1. **Profile first**: Before attempting sophisticated type inference, collect runtime type profiles. This data guides where static analysis is most valuable.

2. **Local inference**: Perform type inference locally within expressions where types can be deduced with high confidence. For example, literal values have known types, and operators with literal arguments have deducible result types.

3. **Flow-sensitive but not path-sensitive**: Track types as they flow through the program, but don't attempt to track separate types for each control flow path. This reduces complexity while capturing most useful information.

4. **Use union types**: When a value might have multiple types, represent this as a union type rather than trying to track each path separately. Union types allow operations that work across multiple types while still catching incompatibilities.

5. **Guard specialization**: When specializing code based on inferred types, generate guards that check assumptions at runtime. This ensures correctness even when inference is imperfect.

6. **Iterative refinement**: Run type flow analysis multiple times to improve precision. Each pass can refine type bounds based on how values are used.

This approach provides useful type information for optimization without requiring sophisticated type systems or rejecting dynamic code.

### Warnings vs Errors in Semantic Phase

For a dynamic language like Left-Right, most semantic issues should be warnings rather than errors. The language's flexibility means code that looks problematic might be intentionally dynamic.

**Errors** (must fix):
- Syntactically invalid code (already caught in parsing)
- Import of non-existent modules when module name is static
- Unresolvable circular dependencies

**Warnings** (should investigate):
- Undefined variable references
- Unreachable code
- Suspicious type coercions
- Unused operators and bindings
- Excessive complexity

**Informational** (might be useful):
- Type inference results
- Optimization opportunities
- Performance suggestions

The compiler should provide configuration options to adjust warning severity and turn specific warnings on or off. Strict modes can promote some warnings to errors for developers who want tighter enforcement.

This approach preserves Left-Right's flexibility while providing developers with useful feedback to catch mistakes and improve code quality.

---

## Conclusion

Semantic analysis for Left-Right requires adapting traditional techniques to the language's map-based scope and dynamic operator system. While full static type checking is not feasible, significant analysis is still valuable for optimization and error detection.

The key insights are:
1. Map-based scope requires specialized resolution algorithms but is mostly analyzable statically
2. Operator type analysis enables powerful optimizations through specialization
3. Flow analysis must account for map-based conditionals and error handling
4. Linting rules must be adapted for point-free syntax
5. Profile-guided analysis complements static inference in dynamic languages

The next report will explore code generation strategies, including how to translate Left-Right's abstract syntax into efficient target code for JavaScript and Rust backends.

---

**NEXT: REPORT 8 — Code Generation and Target Compilation**