# **1\) Language Philosophy & Goals**

* What problems does PenroScript optimize for (e.g., data-transform pipelines, templateable DSLs, config \+ compute, functional composition)?  
  *   
* Ergonomics priorities: readability \> brevity? left-to-right flow over nesting? low ceremony over strictness?

* Intended runtime(s): compiled to JS/TS? interpreted VM? embedded as a JS library? CLI?

* Target audience: app devs, data engineers, config authors, power users?

# **2\) Core Evaluation Model**

* You say “expressions are evaluated left-to-right.” Does **every** operator obey this unless grouped by parentheses?

* Are operators **strict** (eager) or can some be lazy/short-circuiting?

* Do expressions return the last subexpression’s value by default?

* How do you sequence multiple top-level expressions in a file—implicit block? newline-terminated?

* What are the exact rules for grouping with `{ ... _< ... }`, `{ ... _> ... }`, and `{...endingNonKeyValueExpression }`?

* How does precedence interact with operator direction (`_<` vs `_>`) and with parentheses?

# **3\) Types & Values**

* Final, complete type set? (You list Operator, Hashmap, Array, String, Boolean, Number, Undefined.) Any others: Null, Symbol, Function, Date, BigInt?

* Distinction between **Undefined** vs **Null**? Truthiness rules?

* Are Maps ordered? Are keys strings only? Can keys be any value?

* Are Arrays heterogeneous? Sparse?

* Are Operators first-class values? Can they be stored in Maps/Arrays and passed as arguments?

# **4\) Variables, Names, and Assignment**

* How do you declare/assign? (You show computed maps: `{ a: 1, b: a+1 }`.)

* Are names immutable by default? Can reassignment occur? Shadowing?

* Name resolution order: lexical scope? dynamic scope?

* Is there a “let/const” equivalent or is assignment always via object literal evaluation?

* File-level bindings vs block-local—what creates a new scope?

# **5\) Functions & Calls**

* How are functions defined? (You show an “anonymous function” doc block; what is the code form?)

* Arity rules: fixed vs variadic? Default parameters?

* How do you call a function? With juxtaposition? With an explicit call operator?

* Is partial application built-in beyond “left-hungry currying” of diatic ops?

* Can functions be methods (bound to a receiver) or is everything free functions/operators?

# **6\) Operators (Design & Extensibility)**

* “Diatic operators are left hungry curried by default, but can be reversed.” Formal definition of **left hungry**?

* Operator associativity: left/right? per operator or uniform?

* Precedence table: Where do `@`, `+`, `-`, `?<`, `?>`, `#`, `~`, `^`, `^_`, etc. sit?

* Overloading: You mention “Core language operators behavior is input type dependent.” Provide a per-type dispatch table?

* Custom operators: how do users define **new symbols**, bind precedence, and left/right behavior?

* Aliases vs true new symbols (e.g., `~-` \=\> `{ _> - _< }`): how are they declared?

# **7\) The `_</_>` Directional Forms**

* Precise grammar for `{ _< op arg }` (left-section) and `{ _> op arg }` (right-section).

* Do these produce callable values (like Haskell sections), or macros expanded inline?

* Can `_</_>` nest? e.g., `{ _< + { _< * 2 } }`.

* How do they interact with pipelines or composition if those exist?

# **8\) Collections & Paths**

* The `@` (“get”) operator: full path syntax (strings, arrays of path segments, numeric indices).

* Behavior on missing keys/indices: return `undefined` or error? Optional chaining style?

* Can `@` set/update or only read? If set, what is the syntax?

* Deep update/merge operators in stdlib?

# **9\) Conditionals, Patterning & Control Flow**

* Conditional forms: is there an `if` operator, ternary operator, or predicate-driven filters inside `{ ... }` blocks (you show `$?{ … }`)?

* Looping/iteration: all via higher-order operators (map/filter/some/reduce), or explicit loops exist?

* Pattern matching: any destructuring match in maps/arrays? Guards?

* Error handling & control transfer: try/catch? error values? short-circuit ops?

# **10\) Strings & Templates**

* “All strings are template literals” — what’s the interpolation syntax exactly? `'{x}'`, `${x}`, or your own?

* How do `_</_>` turn strings into “operators” for templating? (You mention: operator if any template expressions use `_</_>`.)

* Escaping template delimiters and quotes?

* Multiline strings? Raw strings?

# **11\) Standard Library (HOO & Data Ops)**

* Canonical names and signatures for core HOFs: `map`, `filter`, `some`, `every`, `reduce`, `size`, `uniq`, `join`, `flow`?

* Are these operators or functions? (You show them as operators in places.)

* Composition / pipe operators? (`|>`, `>>`, or your own)

* Equality & comparison semantics (strict vs loose; total ordering across types?)

* String ops: `'^'` to upper, `'^_'` capitalize—complete list, and are these extensible?

* Collection builders: ranges, zips, groupBy, sort—stable sort? comparator shape?

# **12\) Modules, Files, & Imports**

* File as module: How do you expose names? (You hint at `Files:` examples.)

* Import syntax? Named vs default? Relative paths? Extensions?

* Circular imports allowed?

* Execution model: top-level is executed on import or on demand?

# **13\) Interop (TS/JS & JSON)**

* How does PenroScript interop with JS objects (host data)? Is `@` enough?

* Can you call host functions? How do you marshal types (Map ⇄ Object, Array, Undefined/Null)?

* Do you guarantee deterministic serialization to JSON/YAML?

# **14\) Comments, Whitespace, and Layout**

* Single-line and block comments syntax?

* Significant whitespace? Are newlines statement separators? Can semicolons exist?

* Indentation or braces for blocks (beyond `{ ... }` operator groups)?

# **15\) Errors, Diagnostics, and Types (Static vs Dynamic)**

* Static typing at all? Type annotations? Gradual typing? Inference?

* Runtime type errors: message format and location info?

* Lints/warnings (unused names, shadowing, unreachable code)?

* Contract/check operators (e.g., assert, pre/postconditions)?

# **16\) Macros & Metaprogramming**

* Are doc-blocks like `/** PenroScript ... */` executable meta or just documentation?

* Can the language transform its own AST? Operator definition time vs run time?

* Symbol resolution at macro time vs runtime—hygiene rules?

# **17\) Performance & Semantics Guarantees**

* Determinism: any non-deterministic iteration (like hash map key order)?

* Tail-call guarantees? Optimization for pipelines?

* Big-O expectations for stdlib ops; are they lazy/fusion-friendly?

* Memory model: copy-on-write vs mutation? Are core collections persistent/immutable?

# **18\) Concurrency & Effects**

* Any concurrency primitives (futures, promises, channels)? Event loop semantics if targeting JS?

* IO model: pure core with effectful boundary ops? How are effects represented (operators vs monads/affinities)?

# **19\) Tooling & Packaging**

* CLI commands (run, fmt, check, doc, test, bundle)?

* Formatter rules (operator spacing, newline placement).

* Package manager integration (npm/pnpm) or custom registry?

# **20\) Syntax Reference & Grammar**

* Do you want a formal EBNF grammar? If yes, what are token classes and lexing rules?

* Operator tokenization: how to disambiguate `_>`, `_<`, `?>`, `?<`, `^_`, etc. from identifiers?

* Reserved words list.

# **21\) Examples & Canonical Patterns**

* “Hello World,” data-transforms, templating, module import/export, error handling, interop with JS—what should be the canonical showcase set?

* Idiomatic “pipeline” examples equivalent to your Lodash FP snippet (classification \+ counts).

* Operator override examples (e.g., define `~*` to mean right-sectioned multiply).

# **22\) Backwards/Forwards Compatibility**

* Stability policy for operator semantics and stdlib names.

* Versioning scheme for the language and standard library.

* Feature flags or “dialects” for experimental ops?

# **23\) Security & Sandboxing**

* Can untrusted PenroScript run safely? Resource/time limits? Disallowed host APIs?

* Path and file access—behind explicit capability objects?

# **24\) Testing & Documentation Conventions**

* Doctest style embedded in comments?

* Snapshot tests for evaluation results?

* Error message golden tests?

# **25\) Migration & Transpilation**

* TS/JS → PenroScript mechanical rewrite patterns (as in your examples)?

* PenroScript → JS/TS transpile guarantees (source maps, debuggability)?

* Interchange format for AST (JSON schema)?

---

If you fill these in—even in rough bullets—I can turn it into a first pass of the official spec: overview, tutorial, reference (grammar \+ operator tables), and a stdlib catalog with examples and law-like guarantees.

