# Output Structure

## Directory Layout

All pulled Rosetta Code examples go to:

```
~/code/left-right/reports/rosetta-code/language-examples/
```

## Top-Level Structure

```
reports/rosetta-code/language-examples/
├── python/
├── rust/
├── javascript/
├── haskell/
├── ocaml/
├── elixir/
├── erlang/
├── clojure/
├── ruby/
├── r/
├── kotlin/
├── scala/
├── groovy/
├── coffeescript/
├── bqn/
├── j/
├── arturo/
├── wren/
├── mercury/
├── gap/
├── newlisp/
├── rebol/
├── oi/
├── ursala/
├── slate/
├── raven/
├── powershell/
├── lingo/
└── elm/
```

Each language has its own directory (26 total).

## Per-Language Structure

Each language directory contains 13 markdown files, one per problem category:

```
python/
├── 01-arithmetic-math.md
├── 02-string-processing.md
├── 03-collections-data-structures.md
├── 04-algorithms.md
├── 05-concurrency-parallelism.md
├── 06-file-io.md
├── 07-networking.md
├── 08-date-time.md
├── 09-recursion.md
├── 10-pattern-matching.md
├── 11-control-flow.md
├── 12-type-system.md
└── 13-metaprogramming.md
```

## File Naming Convention

### Category Files

Format: `{two_digit_prefix}-{category_name}.md`

Prefix corresponds to category numbering from `02-problem-categories.md`:

| Prefix | Category |
|--------|----------|
| 01 | Arithmetic & Math |
| 02 | String Processing |
| 03 | Collections & Data Structures |
| 04 | Algorithms |
| 05 | Concurrency & Parallelism |
| 06 | File I/O |
| 07 | Networking |
| 08 | Date/Time |
| 09 | Recursion |
| 10 | Pattern Matching |
| 11 | Control Flow |
| 12 | Type System |
| 13 | Metaprogramming |

Category names use kebab-case and lowercase.

## Markdown File Format

### Header

Each file starts with a header describing the language and category:

```markdown
# Python: Arithmetic & Math

**Language**: Python
**Category**: Arithmetic & Math
**Source**: Rosetta Code
**Generated**: 2024-01-15
**Tasks**: 8

---

## Task List

```

### Task Entry Format

Each task has the following structure:

```markdown
## Factorial

**URL**: https://rosettacode.org/wiki/Factorial#Python
**Solutions**: 2

### Version 1

```python
def factorial(n):
    return 1 if n <= 1 else n * factorial(n - 1)
```

**Notes**: Recursive implementation. Uses ternary operator for base case.

### Version 2

```python
import math

def factorial(n):
    return math.factorial(n)
```

**Notes**: Uses standard library. Delegates to optimized C implementation.

**Lines**: 1 vs 3

**Analysis**: The stdlib version is more efficient and handles larger numbers. Recursive version is illustrative but not production-grade.

---

```

### Metadata Fields

- **Task Name**: H2 heading
- **URL**: Link to Rosetta Code with language anchor
- **Solutions**: Count of distinct solutions/versions
- **Version**: H3 heading if multiple versions exist
- **Code**: Fenced code block with language identifier
- **Notes**: Observations about idiomatic patterns
- **Lines**: Line count comparison (if multiple versions)
- **Analysis**: Deeper insight about language capabilities

## Example Complete File

```markdown
# Python: Arithmetic & Math

**Language**: Python
**Category**: Arithmetic & Math
**Source**: Rosetta Code
**Generated**: 2024-01-15
**Tasks**: 8

---

## Factorial

**URL**: https://rosettacode.org/wiki/Factorial#Python
**Solutions**: 2

### Version 1

```python
def factorial(n):
    return 1 if n <= 1 else n * factorial(n - 1)
```

**Notes**: Recursive implementation. Uses ternary operator for base case.

### Version 2

```python
import math

def factorial(n):
    return math.factorial(n)
```

**Notes**: Uses standard library. Delegates to optimized C implementation.

**Lines**: 1 vs 3

**Analysis**: The stdlib version is more efficient and handles larger numbers. Recursive version is illustrative but not production-grade.

---

## Fibonacci Sequence

**URL**: https://rosettacode.org/wiki/Fibonacci_sequence#Python
**Solutions**: 3

### Version 1

```python
def fibonacci(n):
    a, b = 0, 1
    for _ in range(n):
        a, b = b, a + b
    return a
```

**Notes**: Iterative approach. Uses tuple unpacking for swap.

### Version 2

```python
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
```

**Notes**: Naive recursive implementation. Exponential time complexity.

### Version 3

```python
from functools import lru_cache

@lru_cache(maxsize=None)
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
```

**Notes**: Memoized recursion using decorator. Linear time.

**Lines**: 6 vs 4 vs 6

**Analysis**: Memoized version is practical. Pure recursion is O(2^n) and useless for n > 30. Iterative is best for clarity and performance.

---

## Greatest Common Divisor

**URL**: https://rosettacode.org/wiki/Greatest_common_divisor#Python
**Solutions**: 2

### Version 1

```python
def gcd(a, b):
    while b:
        a, b = b, a % b
    return a
```

**Notes**: Euclidean algorithm. Iterative.

### Version 2

```python
import math

def gcd(a, b):
    return math.gcd(a, b)
```

**Notes**: Standard library implementation.

**Lines**: 4 vs 2

**Analysis**: Both versions are O(log(min(a,b))). Stdlib is preferred in production.

---

## Prime Numbers

**URL**: https://rosettacode.org/wiki/Prime_numbers#Python
**Solutions**: 4

### Version 1: Trial Division

```python
def is_prime(n):
    if n < 2:
        return False
    for i in range(2, int(n**0.5) + 1):
        if n % i == 0:
            return False
    return True

def primes_up_to(n):
    return [i for i in range(2, n + 1) if is_prime(i)]
```

**Notes**: Basic trial division. O(n^1.5) for primes up to n.

### Version 2: Sieve of Eratosthenes

```python
def primes_up_to(n):
    sieve = [True] * (n + 1)
    sieve[0] = sieve[1] = False
    for p in range(2, int(n**0.5) + 1):
        if sieve[p]:
            for multiple in range(p*p, n + 1, p):
                sieve[multiple] = False
    return [i for i, is_prime in enumerate(sieve) if is_prime]
```

**Notes**: Sieve algorithm. O(n log log n). Memory O(n).

### Version 3: Generator

```python
def prime_generator():
    D = {}
    q = 2
    while True:
        if q not in D:
            yield q
            D[q*q] = [q]
        else:
            for p in D[q]:
                D.setdefault(p + q, []).append(p)
            del D[q]
        q += 1
```

**Notes**: Incremental sieve using dictionary. Memory efficient.

### Version 4: SymPy

```python
from sympy import primerange

def primes_up_to(n):
    return list(primerange(2, n + 1))
```

**Notes**: Uses sympy library. Optimized C implementation.

**Lines**: 13 vs 11 vs 12 vs 4

**Analysis**: SymPy version is shortest. Sieve is best for bounded ranges. Generator is best for unbounded streaming.

---

## Pascal's Triangle

**URL**: https://rosettacode.org/wiki/Pascal's_triangle#Python
**Solutions**: 2

### Version 1

```python
def pascals_triangle(n):
    triangle = []
    for row in range(n + 1):
        row_vals = [1]
        if row > 0:
            for i in range(1, row):
                row_vals.append(triangle[row - 1][i - 1] + triangle[row - 1][i])
            row_vals.append(1)
        triangle.append(row_vals)
    return triangle
```

**Notes**: Iterative construction. Builds triangle row by row.

### Version 2

```python
def pascals_triangle(n):
    from math import comb
    return [[comb(r, k) for k in range(r + 1)] for r in range(n + 1)]
```

**Notes**: List comprehension with combinatorics. `comb` is Python 3.8+.

**Lines**: 11 vs 3

**Analysis**: List comprehension version is idiomatic Python. Iterative is clearer for beginners.

---

## Perfect Squares

**URL**: https://rosettacode.org/wiki/Perfect_squares#Python
**Solutions**: 2

### Version 1

```python
def is_perfect_square(n):
    return int(n**0.5)**2 == n
```

**Notes**: Square root comparison. Works for integers.

### Version 2

```python
import math

def is_perfect_square(n):
    root = math.isqrt(n)
    return root * root == n
```

**Notes**: Uses integer square root function. More accurate for large n.

**Lines**: 2 vs 3

**Analysis**: `math.isqrt` is more precise. Both are O(1).

---

## Continued Fraction

**URL**: https://rosettacode.org/wiki/Continued_fraction#Python
**Solutions**: 1

### Version 1

```python
def continued_fraction(n):
    result = []
    while n:
        integer_part = int(n)
        result.append(integer_part)
        fractional = n - integer_part
        if fractional == 0:
            break
        n = 1 / fractional
    return result
```

**Notes**: Converts rational to continued fraction representation.

**Analysis**: Iterative approach. Terminates for rationals, continues indefinitely for irrationals.

---

## Summary Statistics

- **Total Tasks**: 8
- **Total Solutions**: 17
- **Average Solutions per Task**: 2.1
- **Most Solutions**: Prime Numbers (4)
- **Fewest Solutions**: Continued Fraction (1)
- **Line Count Range**: 1-13
- **Average Lines**: 5.4

## Language Patterns

1. **Strong standard library**: Most problems have a one-line stdlib solution.
2. **Multiple approaches**: Python supports imperative, functional, and iterative styles.
3. **Readability preferred**: Idiomatic Python favors clarity over conciseness.
4. **Comprehensions powerful**: List/dict comprehensions reduce boilerplate.

## Left-Right Comparison Points

- Can Left-Right express these solutions more concisely?
- How does operator-based syntax compare to Python's function calls?
- Are array-oriented operations as clear as Python comprehensions?
- Does Left-Right need a standard library for these tasks?

---
```

## Index Files

### Language-Level Index

Each language directory should have an `index.md` file summarizing all categories:

```markdown
# Python Examples Summary

**Total Categories**: 13
**Total Tasks**: 120
**Total Solutions**: 285
**Generated**: 2024-01-15

## Categories

1. [Arithmetic & Math](01-arithmetic-math.md) - 8 tasks, 17 solutions
2. [String Processing](02-string-processing.md) - 10 tasks, 24 solutions
3. [Collections & Data Structures](03-collections-data-structures.md) - 9 tasks, 22 solutions
...
```

### Root-Level Index

Create `reports/rosetta-code/language-examples/index.md`:

```markdown
# Rosetta Code Language Examples

**Languages**: 26
**Total Categories**: 13
**Total Tasks**: 3,100+
**Generated**: 2024-01-15

## Languages

| Language | Categories | Tasks | Solutions |
|-----------|-------------|--------|-----------|
| [Python](python/) | 13 | 120 | 285 |
| [Rust](rust/) | 13 | 115 | 230 |
| [JavaScript](javascript/) | 13 | 125 | 290 |
...
```

## Validation

### File Validation

Each generated file must:

- [ ] Be valid markdown (no broken fences)
- [ ] Have all required metadata fields
- [ ] Include Rosetta Code URLs
- [ ] Have at least one code block per task
- [ ] Have notes section for each task
- [ ] Use correct language identifier in code fences

### Directory Validation

Each language directory must:

- [ ] Have all 13 category files
- [ ] Have an index.md
- [ ] Use kebab-case for filenames
- [ ] Be empty or only contain markdown files

### Content Validation

Each task entry must:

- [ ] Have URL field
- [ ] Have at least one solution
- [ ] Have code fence with correct language
- [ ] Have notes section
- [ ] Have analysis if multiple versions

---

**Next**: See `04-implementation-steps.md` for tooling and automation.
