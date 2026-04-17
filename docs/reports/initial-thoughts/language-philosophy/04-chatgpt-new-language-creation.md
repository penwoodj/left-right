# New Language Creation — APL/J/K Inspired Exploration
**Source**: ChatGPT Chat History Conversation
**Conversation ID**: 770c6b02-3c36-431a-9e21-16250d32ee4c
**Relevance**: Earliest recorded exploration of what became left-right/penscript/penroscript

---

## Creating a New Programming Language

To create a new programming language, you would need to define its syntax, semantics, and features. The syntax determines the structure and grammar of the language, while the semantics specify how the language should behave. The features describe the capabilities and functionalities that the language should provide.

### General Steps

1. Define the purpose and audience of the language.
2. Choose a syntax that is easy to read and write.
3. Decide on the language's data types and structures.
4. Create a set of basic operations and functions that can be performed with the language.
5. Add more advanced features, such as object-oriented programming, concurrency, or functional programming.
6. Develop a compiler or interpreter to translate the code into machine language or bytecode.
7. Test the language extensively to ensure that it is efficient, reliable, and easy to use.

---

## Languages Using Glyph Abstract Functions

APL (A Programming Language) is a unique programming language that is known for its use of special characters and glyph abstract functions. These functions are represented by symbols or combinations of symbols, and they perform a wide variety of operations on data.

While APL is the most well-known language to use glyph abstract functions, there are a few other languages that also use similar symbols and operators to represent functions:

1. **J**: This is a language that is closely related to APL and uses many of the same symbols for its functions. J has a syntax that is based on verbs and nouns, with the verbs represented by symbols.

2. **K**: This is another language that is based on APL and uses symbols to represent functions. K has a simpler syntax than APL and is often used for financial applications.

3. **Wolfram Language**: This language, which is used in the Wolfram Mathematica software, uses a variety of symbols and operators to represent functions. The language is designed for symbolic computation and includes many mathematical functions.

---

## How Functions Work in APL

In APL (A Programming Language), functions are represented by special characters or symbols, known as glyph abstract functions. These functions can perform a wide variety of operations on data, and they are typically used in combination to create more complex functions.

APL functions can be used to manipulate arrays of data, and they can operate on entire arrays at once rather than having to iterate over each element. This makes APL programs concise and efficient, and it allows for complex calculations to be performed quickly and easily.

### Example: Adding Arrays

```
⍝ Add two arrays together
a ← 2 3 4
b ← 1 2 3
c ← a + b
```

In this example, the `+` symbol represents the addition function. The `a` and `b` arrays are added together using this function, and the result is stored in the `c` array. The resulting array `c` would be:

```
3 5 7
```

---

## Operator Composition in APL

Operator composition is a powerful feature of APL (A Programming Language) that allows you to combine multiple functions or operators into a single expression. This can make your code more concise and easier to read, and it allows for complex operations to be performed in a single step.

In APL, operators are represented by special symbols, such as `+` for addition or `×` for multiplication. These operators can be composed together using the function composition operator, which is represented by a small circle (○) in APL.

### Example: Squaring the Sum

```
⍝ Calculate the square of the sum of two arrays
a ← 1 2 3
b ← 4 5 6
result ← (a + b) × (a + b)
```

In this example, the `+` operator is used to add the `a` and `b` arrays together, and then the `×` operator is used to multiply the resulting array by itself. These two operations are composed together using the function composition operator (○), creating a single expression that performs both calculations in a single step.

```
result ← (a + b) ○× (a + b)
```

The resulting array `result` would be:

```
225 324 441
```

This is just a simple example, but operator composition can be used to create much more complex expressions that perform many operations at once. By combining multiple functions and operators into a single expression, you can create more concise and powerful APL programs.

---

## Language Design Concepts

The new language incorporates these key concepts:

- Loosely typed functional
- Hashmaps & arrays
- Simple
- Transpile to JS
- JSONish
- APL glyph style of programming
- Operators as functions
- Hashmaps as code organization

Additional concepts considered:

- Pattern matching
- Polymorphic
- Intuitive
- Scope
- Point free
- Abstraction

---

## Programming Language Name Suggestions

### First Set (7 names)

1. GlyphJS
2. HashFunc
3. APLite
4. JSONScript
5. TransPL
6. MapFunc
7. Simplify

These names play on the key concepts of the language, such as its use of APL-style glyph programming, hashmaps and arrays, and simple syntax. They also highlight the language's ability to transpile to JavaScript and its JSON-like structure. The names GlyphJS, HashFunc, and MapFunc also emphasize the language's use of operators and hashmaps. Finally, the name Simplify reflects the language's focus on simplicity and ease of use.

### Second Set (10 names)

1. GlyphicJS
2. Maplex
3. PolyMatch
4. AbstJS
5. SimpLam
6. ExpreMap
7. Scopix
8. Loosify
9. Jsonctive
10. LambdaGlyph

These names play on various aspects of the language's design and features, including its use of glyphs, operators, and pattern matching. Names like Maplex and ExpreMap emphasize the language's use of maps and lists, while Loosify and LambdaGlyph highlight its loosely-typed and functional nature. AbstJS and Jsonctive play on the language's abstraction and JSON-like structure, respectively. Finally, names like SimpLam and GlyphicJS focus on the language's simplicity and APL-style glyph programming.

### Left/Right Concept Names (5 names)

1. LeftRightScript
2. SideLogic
3. LeftRightLang
4. RightyTighty
5. TwofoldLang

These names play on the idea of left and right in various ways. Names like LeftRightScript and LeftRightLang are straightforward and emphasize the concept directly. SideLogic plays on the idea of two sides or perspectives, while RightyTighty emphasizes the idea of a tight and precise programming language. TwofoldLang plays on the idea of two distinct parts, which could be interpreted as left and right.

---

## Computer Science Terms

Here are ten terms that encompass these ideas in the context of computer science:

1. **MapReduce**: A programming model used to process and generate large data sets in a distributed computing environment.
2. **Hash Table**: A data structure used for implementing an associative array, also known as a hash map, hash table, or dictionary.
3. **Linked List**: A data structure consisting of a group of nodes that represent a sequence.
4. **Regular Expression**: A sequence of characters that define a search pattern, used for pattern matching.
5. **Polymorphism**: The ability of a variable, function, or object to take on multiple forms, typically through inheritance or interface implementation.
6. **Scope**: The region of a program where a variable or identifier is defined and accessible.
7. **Abstract Data Type**: A data type defined by its behavior rather than its implementation, allowing for more flexibility and abstraction in programming.
8. **Lambda Calculus**: A mathematical system used for formalizing the concept of functions and their evaluation.
9. **Higher-Order Functions**: Functions that can take other functions as arguments or return functions as their result.
10. **Type Inference**: The ability of a programming language to automatically deduce the data type of a variable or expression.

---

## Synonyms for Side and Left/Right

### Side:
- Edge
- Flank
- Border
- Periphery
- Margin
- Wing

### Left/Right:
- Port/Starboard
- Larboard/Starboard
- Sinister/Dexter
- Levorotatory/Dextrorotatory
- Near/Far
- East/West
- North/South (in some contexts)
