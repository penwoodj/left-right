# Dyadic Operator Terminology & Properties
**Source**: ChatGPT Chat History Conversation
**Conversation ID**: 68ba1df7-af04-8328-a839-7ddb714cb922
**Relevance**: Mathematical foundations for operator design in left-right language

# Key terms

- **Arity**: how many inputs an operator takes.  
  - **Unary/monadic** (1 input), **binary/dyadic** (2), **ternary**, **n-ary/variadic**.

- **Commutative**: order doesn't matter: `a ⊕ b = b ⊕ a`.  
  - Example: `a + b = b + a`.  
  - **Non-commutative**: order matters (e.g., subtraction, matrix mult).  
  - **Anti-commutative (skew-commutative)**: swapping flips the sign: `a ⊗ b = −(b ⊗ a)` (e.g., vector cross product, Lie bracket).

- **Associative**: grouping doesn't matter: `(a ⊕ b) ⊕ c = a ⊕ (b ⊕ c)`.  
  - Example: addition, multiplication, set union/intersection.  
  - **Non-associative**: subtraction, division, exponentiation, cross product.

- **Identity (neutral) element**: an element `e` with `e ⊕ a = a ⊕ e = a`.  
  - Examples: `0` for `+`, `1` for `×`, `∅` for `∪`, universal set `U` for `∩`.  
  - Sometimes identities are only one-sided (left/right identity).

- **Inverse**: for each `a`, an element `b` with `a ⊕ b = e`.  
  - Example: `a + (−a) = 0`, `a × (a⁻¹) = 1` (where defined).

- **Idempotent**: applying it to the same element does nothing new: `a ⊕ a = a`.  
  - Examples: `max(a,a)=a`, set union/intersection.

- **Distributive**: one operation distributes over another:  
  - `×` distributes over `+`: `a×(b+c)=a×b + a×c`.  
  - In sets: `∩` distributes over `∪` and vice versa.

- **Absorbing (annihilator) element**: `z ⊕ a = z`.  
  - Examples: `0` for multiplication (`0×a=0`), `∅` for intersection (`∅∩A=∅`).

- **Cancellative**: from `a ⊕ c = b ⊕ c` infer `a=b`.  
  - True for integer `+` and for `×` by a nonzero `c`; false for `max`, `∪`.

- **Closure**: result stays in the set (e.g., integers closed under `+`).

> Note: In programming, **operator associativity/precedence** (parsing rules like "`^` is right-associative") is different from mathematical **associativity** (the algebraic law above).

# Quick examples (binary/dyadic)

| Operation (on…) | Commutative | Associative | Identity | Idempotent | Notes |
|---|---|---|---|---|---|
| Integer `+` | ✓ | ✓ | 0 | ✗ | Cancellative; inverses exist (negatives) |
| Integer `×` | ✓ | ✓ | 1 | ✗ | 0 is absorbing |
| Integer `−` | ✗ | ✗ | — | — | Neither commutative nor associative |
| Integer `÷` | ✗ | ✗ | — | — | On rationals, inverses for nonzero |
| Exponentiation `a^b` | ✗ | ✗ | 1 (when `b=0`) | — | Not commutative/associative; parses right-assoc in many langs |
| `max`, `min` (reals) | ✓ | ✓ | `−∞`/`+∞` | ✓ | Not cancellative |
| Sets `∪`, `∩` | ✓ | ✓ | `∅` / `U` | ✓ | Distribute over each other; `∅` absorbs for `∩` |
| Function composition `∘` | ✗ | ✓ | `id` | ✗ | Core example of assoc., non-commutative |
| Matrix mult | ✗ | ✓ | `I` | ✗ | Distributes over matrix `+` |
| XOR on bits | ✓ | ✓ | 0 | ✗ | Self-inverse: `a ⊕ a = 0` |
| Vector cross `×` | **anti-comm.** | ✗ | — | — | `a×b = −(b×a)` |
