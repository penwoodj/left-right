# Problem Categories

## Overview

Rosetta Code contains 1000+ programming tasks. We categorize them into 13 semantic groups for systematic Left-Right validation. Each category exercises different language features and patterns.

## Category 1: Arithmetic & Math

**Tests**: Basic operators, numeric types, precision, math functions

### Representative Tasks

| Task | Rosetta Code URL | Why It Matters |
|-------|-------------------|----------------|
| Arithmetic operations | `/wiki/Arithmetic_operations` | Operator precedence, infix notation |
| Factorial | `/wiki/Factorial` | Recursion vs iteration, big integers |
| Fibonacci sequence | `/wiki/Fibonacci_sequence` | Multiple solution approaches, recursion |
| Prime numbers | `/wiki/Prime_numbers` | Algorithm complexity, optimization |
| Greatest common divisor | `/wiki/Greatest_common_divisor` | Euclidean algorithm |
| Perfect squares | `/wiki/Perfect_squares` | Integer predicates, ranges |
| Pascal's triangle | `/wiki/Pascal's_triangle` | 2D arrays, binomial coefficients |
| Continued fraction | `/wiki/Continued_fraction` | Rational arithmetic, iteration |

### Left-Right Validation Points

- Operator direction: Does `_<` vs `_>` make math expressions clear?
- Currying: How easy is partial application for math functions?
- Numeric types: Can Left-Right handle arbitrary precision integers?
- Array math: Do array-oriented operators work for vectorized arithmetic?

**Priority**: HIGH (core language feature)

## Category 2: String Processing

**Tests**: String literals, interpolation, regex, encoding, parsing

### Representative Tasks

| Task | Rosetta Code URL | Why It Matters |
|-------|-------------------|----------------|
| String operations | `/wiki/String_operations` | Concatenation, slicing, length |
| String length | `/wiki/String_length` | Unicode handling |
| Reverse a string | `/wiki/Reverse_a_string` | Array conversion, iteration |
| Palindrome detection | `/wiki/Palindrome_detection` | String comparison, normalization |
| Regular expressions | `/wiki/Regular_expressions` | Regex integration, pattern matching |
| String interpolation | `/wiki/String_interpolation` | Template literals (key for Left-Right) |
| ROT-13 | `/wiki/ROT-13` | Character manipulation |
| Tokenize a string | `/wiki/Tokenize_a_string` | Splitting, whitespace handling |
| Longest common prefix | `/wiki/Longest_common_prefix` | Array comparison |
| Levenshtein distance | `/wiki/Levenshtein_distance` | Dynamic programming, edit operations |

### Left-Right Validation Points

- Template syntax: Are `_<` and `_>` useful for interpolation?
- String as operator: Does treating strings as operators work in practice?
- Unicode: Does Left-Right handle multi-byte characters correctly?
- Regex operators: Should there be dedicated regex operators?

**Priority**: HIGH (Left-Right treats strings as operators)

## Category 3: Collections & Data Structures

**Tests**: Arrays, lists, maps, sets, trees, graphs

### Representative Tasks

| Task | Rosetta Code URL | Why It Matters |
|-------|-------------------|----------------|
| Arrays | `/wiki/Arrays` | Indexing, slicing, mutability |
| Associative arrays | `/wiki/Associative_arrays` | Map access, key types |
| Sets | `/wiki/Sets` | Membership, union, intersection |
| Stacks | `/wiki/Stacks` | LIFO operations |
| Queues | `/wiki/Queues` | FIFO operations |
| Priority queue | `/wiki/Priority_queue` | Heap operations |
| Linked list | `/wiki/Linked_list` | Pointer-based structures |
| Binary tree | `/wiki/Binary_tree` | Recursive structures, traversal |
| Hash table | `/wiki/Hash_table` | Map implementation, collision handling |
| Graph traversal | `/wiki/Graph_traversal` | BFS/DFS algorithms |

### Left-Right Validation Points

- Array operators: Are `+` for concat, `@` for get intuitive?
- Map syntax: Is `key: value` syntax consistent and readable?
- Collection literals: Can you express complex structures declaratively?
- Mutability: Does Left-Right distinguish mutable vs immutable collections?

**Priority**: HIGH (collections are core to Left-Right)

## Category 4: Algorithms

**Tests**: Sorting, searching, algorithmic thinking

### Representative Tasks

| Task | Rosetta Code URL | Why It Matters |
|-------|-------------------|----------------|
| Sorting algorithms | `/wiki/Sorting_algorithms` | Multiple approaches, stability |
| Binary search | `/wiki/Binary_search` | Divide and conquer |
| Linear search | `/wiki/Linear_search` | Simple iteration |
| Merge sort | `/wiki/Merge_sort` | Recursion, divide-conquer |
| Quick sort | `/wiki/Quick_sort` | In-place algorithms |
| Topological sort | `/wiki/Topological_sort` | Graph algorithms, dependencies |
| Dijkstra's algorithm | `/wiki/Dijkstra's_algorithm` | Shortest path, priority queues |
| Knapsack problem | `/wiki/Knapsack_problem` | Dynamic programming |
| Permutations | `/wiki/Permutations` | Combinatorics, recursion |
| Combinations | `/wiki/Combinations` | Combinatorics |

### Left-Right Validation Points

- Higher-order functions: Are `map`, `filter`, `reduce` ergonomic?
- Partial application: Can you easily curry comparison functions?
- Point-free style: Can you express algorithms without named variables?
- Algorithm expression: Is the operator syntax conducive to algorithmic thinking?

**Priority**: MEDIUM (tests expressiveness and functional patterns)

## Category 5: Concurrency & Parallelism

**Tests**: Threads, async, message passing, synchronization

### Representative Tasks

| Task | Rosetta Code URL | Why It Matters |
|-------|-------------------|----------------|
| Create a task | `/wiki/Create_a_task` | Thread spawning |
| Mutex | `/wiki/Mutex` | Mutual exclusion |
| Semaphore | `/wiki/Semaphore` | Resource counting |
| Parallel processing | `/wiki/Parallel_processing` | Work distribution |
| Map-reduce | `/wiki/Map-reduce` | Distributed patterns |
| Race condition | `/wiki/Race_condition` | Shared state bugs |
| Producer-consumer | `/wiki/Producer-consumer` | Queue synchronization |
| Fork | `/wiki/Fork` | Process spawning |
| Channel | `/wiki/Channel` | Message passing |

### Left-Right Validation Points

- Concurrency model: Does Left-Right have a concurrency story?
- Async operators: Should there be async-aware operators?
- Parallel operators: Can array operators parallelize automatically?

**Priority**: LOW (Left-Right may defer this to target languages)

## Category 6: File I/O

**Tests**: Reading, writing, streams, paths

### Representative Tasks

| Task | Rosetta Code URL | Why It Matters |
|-------|-------------------|----------------|
| Read a file | `/wiki/Read_a_file` | File system interaction |
| Write a file | `/wiki/Write_a_file` | File system interaction |
| List directory | `/wiki/List_directory` | Directory traversal |
| File modification time | `/wiki/File_modification_time` | Metadata |
| Check that file exists | `/wiki/Check_that_file_exists` | File system queries |
| Copy a file | `/wiki/Copy_a_file` | Streaming |
| Delete a file | `/wiki/Delete_a_file` | File operations |
| Temporary file | `/wiki/Temporary_file` | Resource management |

### Left-Right Validation Points

- I/O operators: Should file I/O be operator-based or function-based?
- Path handling: Are paths strings or a special type?
- Streaming: Can Left-Right stream files efficiently?

**Priority**: MEDIUM (practical necessity)

## Category 7: Networking

**Tests**: HTTP, sockets, APIs

### Representative Tasks

| Task | Rosetta Code URL | Why It Matters |
|-------|-------------------|----------------|
| HTTP request | `/wiki/HTTP_request` | Web interaction |
| HTTPS | `/wiki/HTTPS` | SSL/TLS |
| URL parsing | `/wiki/URL_parsing` | String parsing, URI spec |
| TCP socket | `/wiki/TCP_socket` | Low-level networking |
| UDP socket | `/wiki/UDP_socket` | Low-level networking |
| DNS lookup | `/wiki/DNS_lookup` | Network queries |
| Web scraping | `/wiki/Web_scraping` | HTML parsing |
| JSON parsing | `/wiki/JSON_parsing` | Data serialization |

### Left-Right Validation Points

- Async I/O: Does Left-Right handle network I/O well?
- Streaming: Can Left-Right stream network responses?

**Priority**: LOW (defer to target runtime)

## Category 8: Date/Time

**Tests**: Temporal operations, formatting, timezones

### Representative Tasks

| Task | Rosetta Code URL | Why It Matters |
|-------|-------------------|----------------|
| Current date | `/wiki/Current_date` | System time |
| Date format | `/wiki/Date_format` | String formatting |
| Time | `/wiki/Time` | System time |
| Add/subtract days | `/wiki/Add_subtract_days` | Date arithmetic |
| Leap year | `/wiki/Leap_year` | Logic, predicates |
| Day of week | `/wiki/Day_of_week` | Calendar calculations |
| Unix timestamp | `/wiki/Unix_timestamp` | Epoch conversion |
| Sleep | `/wiki/Sleep` | Timing |

### Left-Right Validation Points

- Date types: Should dates be a primitive or library?
- Operator overloading: Can you do `date + 7.days`?

**Priority**: LOW (library concern)

## Category 9: Recursion

**Tests**: Recursive patterns, tail recursion, self-reference

### Representative Tasks

| Task | Rosetta Code URL | Why It Matters |
|-------|-------------------|----------------|
| Recursive functions | `/wiki/Recursive_functions` | Basic recursion |
| Tail recursion | `/wiki/Tail_recursion` | Stack optimization |
| Mutual recursion | `/wiki/Mutual_recursion` | Indirect recursion |
| Tree traversal | `/wiki/Tree_traversal` | Recursive data structures |
| Depth-first search | `/wiki/Depth-first_search` | Recursive algorithms |
| Ackermann function | `/wiki/Ackermann_function` | Deep recursion |
| Quine | `/wiki/Quine` | Self-referential programs |
| Sierpinski triangle | `/wiki/Sierpinski_triangle` | Fractal recursion |

### Left-Right Validation Points

- Recursion syntax: Can Left-Right express recursion clearly?
- Tail call optimization: Should Left-Right guarantee TCO?
- Self-reference: Can functions reference themselves easily?

**Priority**: HIGH (tests core language design)

## Category 10: Pattern Matching

**Tests**: Destructuring, guards, algebraic types

### Representative Tasks

| Task | Rosetta Code URL | Why It Matters |
|-------|-------------------|----------------|
| Pattern matching | `/wiki/Pattern_matching` | Pattern syntax |
| Regular expressions | `/wiki/Regular_expressions` | Text patterns |
| Glob patterns | `/wiki/Glob_patterns` | File patterns |
| Match simple | `/wiki/Match_simple` | Literal matching |
| Conditional structures | `/wiki/Conditional_structures` | Predicates, guards |

### Left-Right Validation Points

- Pattern operators: Should Left-Right have pattern operators?
- Destructuring: Can you destructure maps/arrays easily?
- Guard syntax: Are conditional guards ergonomic?

**Priority**: MEDIUM (important for expressiveness)

## Category 11: Control Flow

**Tests**: Conditionals, loops, early exit, exceptions

### Representative Tasks

| Task | Rosetta Code URL | Why It Matters |
|-------|-------------------|----------------|
| Conditional structures | `/wiki/Conditional_structures` | If/else, ternary |
| Loops | `/wiki/Loops` | Iteration patterns |
| For loop | `/wiki/For_loop` | C-style loops |
| While loop | `/wiki/While_loop` | Predicate loops |
| Break | `/wiki/Break` | Early exit |
| Continue | `/wiki/Continue` | Skip iteration |
| Exceptions | `/wiki/Exceptions` | Error handling |
| Assertions | `/wiki/Assertions` | Preconditions |

### Left-Right Validation Points

- Loop operators: Should loops be operator-based?
- Short-circuiting: Do `&` and `|` short-circuit as intended?
- Error operators: How does Left-Right handle errors?

**Priority**: HIGH (Left-Right uses `&`/`|` for conditionals)

## Category 12: Type System

**Tests**: Type inference, polymorphism, constraints

### Representative Tasks

| Task | Rosetta Code URL | Why It Matters |
|-------|-------------------|----------------|
| Type inference | `/wiki/Type_inference` | Static vs dynamic |
| Polymorphism | `/wiki/Polymorphism` | Generic functions |
| Type casting | `/wiki/Type_casting` | Conversion |
| Type introspection | `/wiki/Type_introspection` | Reflection |
| Dynamic dispatch | `/wiki/Dynamic_dispatch` | Method selection |
| Function overloading | `/wiki/Function_overloading` | Multiple signatures |

### Left-Right Validation Points

- Dynamic typing: How well does duck typing work in practice?
- Type coercion: Are implicit conversions safe and predictable?
- Operator dispatch: Does type-dependent operator dispatch work as intended?

**Priority**: MEDIUM (Left-Right is loosely typed)

## Category 13: Metaprogramming

**Tests**: Code generation, macros, eval, reflection

### Representative Tasks

| Task | Rosetta Code URL | Why It Matters |
|-------|-------------------|----------------|
| Eval | `/wiki/Eval` | Code execution |
| Macro | `/wiki/Macro` | Code transformation |
| Code generation | `/wiki/Code_generation` | Dynamic programming |
| Reflection | `/wiki/Reflection` | Self-inspection |
| Quine | `/wiki/Quine` | Self-reproduction |
| Symbolic computation | `/wiki/Symbolic_computation` | AST manipulation |

### Left-Right Validation Points

- Operator definition: How ergonomic is defining new operators?
- Metaprogramming: Can Left-Right manipulate its own syntax?
- Template operators: Are operator strings useful for metaprogramming?

**Priority**: MEDIUM (Left-Right strings-as-operators is a metaprogramming feature)

## Priority Ranking for Left-Right

### Phase 1 (Must validate first)

1. **Control Flow** - Tests `&`/`|` conditional operators
2. **Arithmetic & Math** - Tests operator direction and precedence
3. **String Processing** - Tests template operators
4. **Collections** - Tests map/array syntax and `@` operator

### Phase 2 (Important patterns)

5. **Recursion** - Tests function syntax and self-reference
6. **Algorithms** - Tests higher-order functions
7. **Pattern Matching** - Tests destructuring potential

### Phase 3 (Library concerns)

8. **File I/O** - Tests practicality
9. **Type System** - Tests type coercion
10. **Metaprogramming** - Tests operator definition

### Phase 4 (Deferred to future)

11. **Concurrency** - Likely target runtime concern
12. **Networking** - Likely target runtime concern
13. **Date/Time** - Library-level concern

---

**Next**: See `03-output-structure.md` for file organization and formatting.
