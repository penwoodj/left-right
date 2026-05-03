# Operator Reference — Left-Right

Comprehensive reference for all Left-Right operators, including syntax, type-dependent behavior, examples, and currying rules.

---

## Directional Operators

### `_<` — Left Variable Placeholder

Placeholder for the left variable in operator definition. Creates a lambda that takes a left variable.

**Syntax:**
```penroscript
{ _<, expression }
```

**Behavior:**
- Creates a unary operator waiting for left variable
- When applied, binds left variable to `_<`
- Executes expression with bound value

**Examples:**
```penroscript
// Add 10 to any left variable
add10: { _< + 10 }

// Usage
5 add10  // 15

// Equivalent to: 5 + 10
```

### `_>` — Right Variable Placeholder

Placeholder for the right variable in operator definition. Creates a lambda that takes a right variable.

**Syntax:**
```penroscript
{ expression, _> }
```

**Behavior:**
- Creates a unary operator waiting for right argument
- When applied, binds right argument to `_>`
- Executes expression with bound value

**Examples:**
```penroscript
// Subtract any right argument from 100
subtractFrom100: { 100 - _> }

// Usage
subtractFrom100 20  // 80

// Equivalent to: 100 - 20
```

### Combined `_< _>` — Two-Argument Operator

Both placeholders create a binary operator.

**Syntax:**
```penroscript
{ _<, _> }
```

**Behavior:**
- Creates a binary operator with two named variables
- Useful for clarity in complex expressions
- Enables explicit variable naming

**Examples:**
```penroscript
// Binary operator with named variables
add: { _< + _> }

// Usage
3 add 5  // 8

// Equivalent to: 3 + 5
```

---

## Access Operators

### `@` — Path Access

Access nested properties in maps and indices in arrays. Supports path-based access for deep structures.

**Syntax:**
```penroscript
value @[, , ]    // Array path (PRIMARY - idiomatic)
value @[index]                     // Index access
value @property                    // Single property access
```

**Behavior:**

| Input Type | Behavior | Example |
|------------|----------|----------|
| Map + Text | Property access | `obj @[]` → `obj.name` |
| Map + List | Nested path access (PRIMARY) | `data @[, , ]` → `data.user.profile.email` |
| List + Number | Index access | `arr @[0]` → `arr[0]` |
| List + List | Multiple indices | `arr @[0, 2, 4]` → `[arr[0], arr[2], arr[4]]` |
| List + Text | Invalid → returns `undefined` |  |

**Error Handling:**
- Missing key returns `undefined`
- Out of bounds returns `undefined`
- No exceptions thrown

**Examples:**
```penroscript
// Property access
{
  user: { name: `Alice`, email: `alice@example.com` },
  name: user @[],           // `Alice`
  email: user @[]          // `alice@example.com`
}

// Nested path access (PRIMARY/idiomatic - use array path)
{
  data: { user: { profile: { email: `alice@example.com` } } },
  email: data @[, , ]  // `alice@example.com`

  // Non-idiomatic alternative (chained @):
  // email: data @[] @[] @[]  // Same result but less idiomatic
}

// List indexing
{
  items: [`apple`, `banana`, `cherry`],
  first: items @[0],      // `apple`
  last: items @[-1],       // `cherry`
}

// Optional chaining with undefined
{
  user: { name: `Alice` },
  email: user @[] @[]  // undefined (profile missing)
}
```

**Note:** For nested access, always pass the full path as an array on the right side of `@`. This is the idiomatic approach. Chained `@` operations work but are non-idiomatic.

### `@+` — Pick

Extract specified keys from a map.

**Syntax:**
```penroscript
map @+[key1, key2, ...]
```

**Behavior:**
- Creates new map with only specified keys
- Preserves order of specified keys
- Returns empty map if no keys specified

**Examples:**
```penroscript
{
  user: {
    name: `Alice`,
    email: `alice@example.com`,
    password: `secret`,
    age: 30
  },
  publicUser: user @+['name', 'email']
  // { name: `Alice`, email: `alice@example.com` }
}
```

### `@-` — Omit

Remove specified keys from a map.

**Syntax:**
```penroscript
map @-[key1, key2, ...]
```

**Behavior:**
- Creates new map excluding specified keys
- Preserves order of remaining keys
- Returns copy if no keys specified

**Examples:**
```penroscript
{
  user: {
    name: `Alice`,
    email: `alice@example.com`,
    password: `secret`
  },
  safeUser: user @-['password']
  // { name: `Alice`, email: `alice@example.com` }
}
```

### `@0` — First/Head

Get first element of a list or first character of text.

**Syntax:**
```penroscript
value @0
```

**Behavior:**

| Input Type | Behavior | Example |
|------------|----------|----------|
| List | First element | `[1,2,3] @0` → `1` |
| Text | First character | `` `hello` @0 `` → `` `h` `` |
| Map/Number | Invalid → returns `undefined` |  |

**Examples:**
```penroscript
{
  items: [`apple`, `banana`, `cherry`],
  first: items @0,      // `apple`

  text: `hello`,
  firstChar: text @0  // `h`
}
```

### `@-1` — Last

Get last element of a list or last character of text.

**Syntax:**
```penroscript
value @-1
```

**Behavior:**

| Input Type | Behavior | Example |
|------------|----------|----------|
| List | Last element | `[1,2,3] @-1` → `3` |
| Text | Last character | `` `hello` @-1 `` → `` `o` `` |
| Map/Number | Invalid → returns `undefined` |  |

**Examples:**
```penroscript
{
  items: [`apple`, `banana`, `cherry`],
  last: items @-1,       // `cherry`

  text: `hello`,
  lastChar: text @-1  // `o`
}
```

### `@~` — Tail

Get all elements except the first (tail of list or text).

**Syntax:**
```penroscript
value @~
```

**Behavior:**

| Input Type | Behavior | Example |
|------------|----------|----------|
| List | All except first | `[1,2,3] @~` → `[2,3]` |
| Text | All except first | `` `hello` @~ `` → `` `ello` `` |
| Map/Number | Invalid → returns `undefined` |  |

**Examples:**
```penroscript
{
  items: [`apple`, `banana`, `cherry`],
  rest: items @~,       // [`banana`, `cherry`]

  text: `hello`,
  restText: text @~  // `ello`
}
```

### `@\` — Slice

Extract a portion of a list or text.

**Syntax:**
```penroscript
value @\[start:end]
```

**Behavior:**
- Extracts elements from `start` (inclusive) to `end` (exclusive)
- Negative indices count from end
- Omitted `start` defaults to 0
- Omitted `end` defaults to length

**Examples:**
```penroscript
{
  items: [0, 1, 2, 3, 4, 5],
  slice1: items @\[1:4],   // [1, 2, 3]
  slice2: items @\[0:2],   // [0, 1]
  slice3: items @\[2:],     // [2, 3, 4, 5]
  slice4: items @\[:-2],    // [0, 1, 2, 3]

  text: `hello world`,
  subtext: text @\[0:5]    // `hello`
}
```

### `@>` — Values

Extract all values from a map.

**Syntax:**
```penroscript
map @>
```

**Behavior:**
- Returns list of map values
- Preserves insertion order
- Returns empty list for empty map

**Examples:**
```penroscript
{
  user: {
    name: `Alice`,
    email: `alice@example.com`,
    age: 30
  },
  keys: user @<    // [`name`, `email`, `age`]

  // Using in a pipeline
  userData: { name: `Bob`, age: 25 },
  keyList: userData @< @-  // [`age`, `name`]  (removed 'email')
}
```

### `@<` — Keys

Extract all keys from a map.

**Syntax:**
```penroscript
map @<
```

**Behavior:**
- Returns list of map keys
- Preserves insertion order
- Returns empty list for empty map

**Examples:**
```penroscript
{
  config: {
    host: `localhost`,
    port: 8080,
    debug: true
  },
  values: config @>   // [`localhost`, 8080, true]
}
```

---

## Collection Operators

### `$` — Map

Apply an operation to each element in a collection.

**Syntax:**
```penroscript
collection ${ operation }
collection ${_}              // Map with curry reversal
```

**Behavior:**
- Applies operation to each element
- Returns new collection with transformed elements
- Preserves collection type (list → list, map → map)
- `_` suffix reverses currying direction (curry reversal)

**Examples:**
```penroscript
// List mapping
{
  numbers: [1, 2, 3, 4, 5],
  doubled: numbers ${ _< * 2 }  // [2, 4, 6, 8, 10]

  names: [`alice`, `bob`, `charlie`],
  uppercased: names ${ ^ }  // [`ALICE`, `BOB`, `CHARLIE`]
}

// Map mapping (operates on values)
{
  scores: { alice: 85, bob: 92, charlie: 78 },
  adjusted: scores ${ _< + 5 }  // { alice: 90, bob: 97, charlie: 83 }
}

// Map with curry reversal
{
  typesToGet: [`String`, `Number`],
  lowerTypesToGet: typesToGet !?= `string`: [typesToGet],
    typesToGet ${'_}  // Reverse curry direction
}
```

### `?{` — Filter

Keep elements that satisfy a predicate condition.

**Syntax:**
```penroscript
collection ?{ predicate }
list ?{ condition } #,     // Filter with count suffix
```

**Behavior:**
- Tests each element with predicate
- Keeps elements where predicate returns truthy
- Returns collection of same type (filtered)
- Count suffix `#,` returns number of matching elements

**Examples:**
```penroscript
// List filtering
{
  numbers: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
  evens: numbers ?{ _< / 2 = 0 }  // [2, 4, 6, 8, 10]

  emails: [`alice@example.com`, `invalid`, `bob@test.org`, `x@y`],
  valid: emails ?{ _< ?/ '@' }  // [`alice@example.com`, `bob@test.org`]

  // Filter with count
  threats: [
    { level: `malicious` },
    { level: `suspicious` },
    { level: `malicious` }
  ],
  maliciousCount: threats ?{ _< @[] = `malicious` } #,  // 2
}
```

### `$+` — Reduce

Accumulate collection elements using a combining operation.

**Syntax:**
```penroscript
collection $+[ combiner ]
```

**Behavior:**
- Applies combiner to elements sequentially
- Uses first element as initial accumulator
- Returns final accumulated value

**Examples:**
```penroscript
// Sum numbers
{
  numbers: [1, 2, 3, 4, 5],
  sum: numbers $+[ + ]  // 15
}

// Concatenate text
{
  words: [`hello`, `world`, `!`],
  greeting: words $+[ + ]  // `helloworld!`
}

// Find maximum
{
  values: [10, 5, 20, 15],
  max: values $+[ @< > _> | _< ]  // 20
}
```

### `$_` — FlatMap

Map then flatten one level of nesting.

**Syntax:**
```penroscript
collection $_{ operation }
```

**Behavior:**
- Applies operation to each element
- Flattens resulting collection one level
- Useful for nested structures or one-to-many transformations

**Examples:**
```penroscript
{
  nested: [[1, 2], [3, 4], [5, 6]],
  flattened: nested $_{ _< }  // [1, 2, 3, 4, 5, 6]

  users: [
    { name: `Alice`, emails: [`alice@work.com`, `alice@home.com`] },
    { name: `Bob`, emails: [`bob@work.com`] }
  ],
  allEmails: users $_{ @[] }
  // [`alice@work.com`, `alice@home.com`, `bob@work.com`]
}
```

### `$><` — Group

Group collection elements by a key function.

**Syntax:**
```penroscript
collection $><{ keyFunction }
```

**Behavior:**
- Applies keyFunction to each element
- Groups elements with same key value
- Returns map where keys are group identifiers and values are element arrays

**Examples:**
```penroscript
{
  data: [
    { name: `Alice`, type: `admin` },
    { name: `Bob`, type: `user` },
    { name: `Charlie`, type: `admin` },
    { name: `Diana`, type: `user` }
  ],
  grouped: data $><{ @[] }

  // Result:
  // {
  //   `admin`: [
  //     { name: `Alice`, type: `admin` },
  //     { name: `Charlie`, type: `admin` }
  //   ],
  //   `user`: [
  //     { name: `Bob`, type: `user` },
  //     { name: `Diana`, type: `user` }
  //   ]
  // }
}
```

### `~` — Unique

Remove duplicate values from a collection.

**Syntax:**
```penroscript
~collection
```

**Behavior:**

| Input Type | Behavior | Example |
|------------|----------|----------|
| List | Remove duplicates | `~[1,2,1,3,2]` → `[1,2,3]` |
| Text | Remove duplicate characters | `` ~`aabbcc` `` → `` `abc` `` |
| Map | Not applicable |  |

**Examples:**
```penroscript
{
  numbers: [1, 2, 1, 3, 2, 4],
  unique: ~numbers  // [1, 2, 3, 4]

  text: `hello world`,
  uniqueChars: ~text  // `helo wrd`
}
```

### `~~` — Reverse

Reverse the order of elements in a collection.

**Syntax:**
```penroscript
~~collection
```

**Behavior:**

| Input Type | Behavior | Example |
|------------|----------|----------|
| List | Reverse element order | `~~[1,2,3]` → `[3,2,1]` |
| Text | Reverse character order | `` ~~`hello` `` → `` `olleh` `` |
| Map | Not applicable |  |

**Examples:**
```penroscript
{
  items: [`apple`, `banana`, `cherry`],
  reversed: ~~items  // [`cherry`, `banana`, `apple`]

  text: `hello world`,
  reversedText: ~~text  // `dlrow olleh`
}
```

---

## Text Operators

### `><` — Join / Includes

**Type-dependent operator** with two behaviors:

1. **List + Text**: Join list elements with separator
2. **List + Item**: Check if item exists in list (includes)

**Syntax:**
```penroscript
list >< separator      // Join
list >< item           // Includes check
text >< text           // Concatenation
```

**Behavior:**

| Input Types | Behavior | Example |
|-------------|----------|---------|
| List + Text | Join elements with separator | `[`a`,`b`] >< `,` → `a,b` |
| List + Item | Check if item in list (includes) | `[1,2,3] >< 2` → `true` |
| Text + Text | Concatenation | `` `a` + `b` `` → `` `ab` `` |

**Examples:**
```penroscript
// Join list with separator
{
  words: [`hello`, `world`, `from`, `left-right`],
  sentence: words >< ` `  // `hello world from left-right`

  // Join without separator
  concatenated: words >< ``  // `helloworldfromleft-right`
}

// Includes check
{
  numbers: [1, 2, 3, 4, 5],
  hasThree: numbers >< 3,       // true
  hasSix: numbers >< 6,        // false

  emails: [`alice@example.com`, `bob@test.org`],
  hasAlice: emails >< `alice@example.com`  // true

  // In filter context
  lowerTypes: [`string`, `number`, `boolean`],
  entityTypes: [`String`, `Number`],
  typesInSet: lowerTypes ?|{
    typeToGet: _<,
    entityTypes ?>< typeToGet
  }
}

// Text concatenation
{
  greeting: `hello`,
  name: `world`,
  message: greeting >< ` ` >< name  // `hello world`
}
```

### `<>` — Split

Split a text by a delimiter into a list.

**Syntax:**
```penroscript
string <> delimiter
```

**Behavior:**
- Splits text at each delimiter occurrence
- Returns list of text parts
- Empty delimiter splits between characters

**Examples:**
```penroscript
{
  text: `hello,world,from,left-right`,
  parts: text <> `,`  // [`hello`, `world`, `from`, `left-right`]

  // Split by characters
  chars: `hello` <> ``  // [`h`, `e`, `l`, `l`, `o`]
}
```

### `>`<` — Replace

Replace occurrences of a substring in a text.

**Syntax:**
```penroscript
string ><[target:replacement]
```

**Behavior:**
- Replaces all occurrences of target with replacement
- Returns new text with replacements applied
- Case-sensitive matching

**Examples:**
```penroscript
{
  text: `hello world`,
  replaced: text ><[world:universe]  // `hello universe`

  // Multiple replacements (chained)
  cleaned: `bad text` ><[bad:good] ><[text:data]  // `good data`
}
```

### `<"` — Trim

Remove whitespace from the beginning and end of a text.

**Syntax:**
```penroscript
<"string
```

**Behavior:**
- Removes leading and trailing whitespace
- Does not remove internal whitespace
- Handles spaces, tabs, newlines

**Examples:**
```penroscript
{
  text: `   hello world   `,
  trimmed: <"text  // `hello world`

  email: `  alice@example.com  `,
  cleanEmail: <"email  // `alice@example.com`
}
```

### `^` — toUpper

Convert all characters in a text to uppercase.

**Syntax:**
```penroscript
text "^
```

**Behavior:**
- Converts lowercase to uppercase
- Leaves non-letter characters unchanged
- Type-dependent: text only
- Base operator (no `_` suffix)

**Examples:**
```penroscript
{
  greeting: `hello world`,
  uppercased: greeting "^  // `HELLO WORLD`

  name: `alice`,
  upperName: name "^  // `ALICE`
}
```

### `^_` — Capitalize

Convert the first character of a text to uppercase. This is the `^` operator with the `_` curry reversal suffix.

**Syntax:**
```penroscript
text "^_
```

**Behavior:**
- Uppercases first character only
- Leaves rest of text unchanged
- Type-dependent: text only
- `_` suffix reverses operator behavior (curry reversal)

**Spatial Symbology:**
- `^` (up/uppercase) + `_` (underscore/lower) = capitalize first only
- The `_` suffix indicates reversal from "uppercase all" to "uppercase first"

**Curry Reversal Pattern:**
- `text "^` → uppercase all characters
- `text "^_` → uppercase first character only

**Examples:**
```penroscript
{
  text: `hello world`,
  capitalized: text "^_  // `Hello world`

  // In map context with curry reversal
  threats: [
    { classification: `malicious` },
    { classification: `suspicious` }
  ],
  threatClassifications: threats
    ${ _< @[] "^_}
    ~
    >< `, `
  // `Malicious, Suspicious`
}
```

### `"` — toLower

Convert all characters in a text to lowercase.

**Syntax:**
```penroscript
"text"
```

**Behavior:**
- Converts uppercase to lowercase
- Leaves non-letter characters unchanged
- Type-dependent: text only

**Examples:**
```penroscript
{
  greeting: `HELLO WORLD`,
  lowercased: "greeting  // `hello world`

  name: `ALICE`,
  lowerName: "name  // `alice`

  classification: `MALICIOUS`,
  lowerClassification: "classification  // `malicious`
}
```

---

## Logical Operators

### `!` — Not

Negate a truthy/falsy value or expression.

**Syntax:**
```penroscript
!expression
```

**Behavior:**
- Converts `true` to `false`
- Converts `false` to `true`
- Works on any value with truthy/falsy coercion

**Examples:**
```penroscript
{
  isValid: false,
  isInvalid: !isValid  // true

  value: 42,
  isZero: !(value = 0)  // true (42 ≠ 0)

  // Coercion
  hasValue: !undefined  // true
  isEmpty: ![]  // false (empty list is falsy)
}
```

### `&` — And / Conditional Append

**Type-dependent operator** with two behaviors:

1. **Logical AND**: Boolean logic between expressions
2. **Conditional Append**: Append template if value is truthy

**Syntax:**
```penroscript
expression1 & expression2          // Logical AND
value & 'template {var}'            // Conditional append
```

**Behavior:**

| Input Types | Behavior | Example |
|-------------|----------|---------|
| Boolean + Boolean | Logical AND | `true & false` → `false` |
| Truthy + Template | Append if truthy | `` `name` & `hello {name}` `` |

**Logical AND:**
- Returns `true` only if all expressions are truthy
- Short-circuits: returns `false` on first falsy value
- Works on booleans and other truthy/falsy values

**Conditional Append:**
- Appends template string to value if value is truthy
- Supports template interpolation with `{var}` syntax
- Returns value + template if truthy, otherwise undefined

**Examples:**
```penroscript
{
  // Logical AND
  a: true,
  b: true,
  c: false,

  result1: a & b & c,     // false (short-circuits at c)
  result2: a & b,         // true
  result3: c & b,         // false

  // Conditional append
  threatClassifications: `malicious, suspicious`,
  message: threatClassifications & `Threat Classifications: {threatClassifications}`
  // `Threat Classifications: malicious, suspicious`

  // Conditional append with falsy value
  empty: undefined,
  noMessage: empty & `This won't appear`  // undefined
}
```

### `|` — Or

Logical OR of two or more truthy/falsy expressions.

**Syntax:**
```penroscript
expression1 | expression2
```

**Behavior:**
- Returns `true` if any expression is truthy
- Short-circuits: returns `true` on first truthy value
- Type-dependent: works on booleans and other truthy/falsy values

**Examples:**
```penroscript
{
  a: false,
  b: false,
  c: true,

  result1: a | b | c,     // true (returns from c)
  result2: a | b,         // false
  result3: a | c,         // true
}
```

### `?|` — Some

Returns `true` if any element in a collection satisfies a predicate.

**Syntax:**
```penroscript
collection ?|{ predicate }
```

**Behavior:**
- Tests each element with predicate
- Returns `true` if any element satisfies predicate
- Returns `false` if no element satisfies or collection is empty

**Examples:**
```penroscript
{
  numbers: [2, 4, 6, 8],
  hasOdd: numbers ?|{ _< / 2 = 1 }  // false

  numbers2: [1, 2, 3, 4],
  hasEven: numbers2 ?|{ _< / 2 = 0 }  // true
}
```

### `?|!` — Every

Returns `true` if every element in a collection satisfies a predicate.

**Syntax:**
```penroscript
collection ?|!{ predicate }
```

**Behavior:**
- Tests each element with predicate
- Returns `true` if all elements satisfy predicate
- Returns `false` if any element fails predicate
- Returns `true` if collection is empty
- Does NOT reverse condition (unlike `?|`)

**Examples:**
```penroscript
{
  numbers: [2, 4, 6, 8],
  allEven: numbers ?|!{ _< / 2 = 0 }  // true

  numbers2: [1, 2, 3, 4],
  allEven: numbers2 ?|!{ _< / 2 = 0 }  // false

  // Every check with negative condition
  negative: [1, 2, 3, 4, 5] ?|!{ _< <= 0 }!  // Checks every element <= 0
}
```

---

## Comparison Operators

### `=` — Loose Equality

Check if two values are equal, ignoring order for collections. Performs type coercion before comparison.

**Syntax:**
```penroscript
value1 = value2
```

**Behavior:**
- Primitive types: standard equality with type coercion
- Lists: equality regardless of element order
- Maps: equality regardless of key order
- Type-dependent: coerces types before comparison
- Loose equality (like JS `==`)

**Examples:**
```penroscript
{
  // Primitives
  equal1: 5 = 5,           // true
  equal2: `hello` = `hello`,  // true
  equal3: 5 = `5`,           // true (type coercion)

  // Lists (unordered)
  list1: [1, 2, 3],
  list2: [3, 2, 1],
  listsEqual: list1 = list2,  // true

  // Maps (unordered)
  map1: { a:1, b:2 },
  map2: { b:2, a:1 },
  mapsEqual: map1 = map2,  // true
}
```

### `==` — Strict Equality

Check if two values are equal, including order for collections. Performs strict type checking.

**Syntax:**
```penroscript
value1 == value2
```

**Behavior:**
- Primitive types: strict type checking (no coercion)
- Lists: equality including element order
- Maps: equality including key order
- Strict type comparison (like JS `===`)
- Example: `0 == `0`` → `false` (different types)

**Examples:**
```penroscript
{
  // Strict type checking
  strict1: 0 == `0`,         // false (number vs text)
  strict2: `5` == `5`,        // true (same type and value)
  strict3: 5 == 5,           // true

  // Lists (ordered)
  list1: [1, 2, 3],
  list2: [3, 2, 1],
  listsEqual: list1 == list2,  // false (different order)

  list3: [1, 2, 3],
  list4: [1, 2, 3],
  listsEqual2: list3 == list4,  // true (same order)

  // Maps (ordered)
  map1: { a:1, b:2 },
  map2: { b:2, a:1 },
  mapsEqual: map1 == map2,  // false (different order)
}
```

### `<` — Less Than

Check if left value is less than right value.

**Syntax:**
```penroscript
left < right
```

**Behavior:**
- Numbers: numeric comparison
- Strings: lexicographic comparison
- Other types: coerced or undefined

**Examples:**
```penroscript
{
  result1: 3 < 5,        // true
  result2: `apple` < `banana`,  // true
  result3: 10 < 10,       // false
}
```

### `<=` — Less Than or Equal

Check if left value is less than or equal to right value.

**Syntax:**
```penroscript
left <= right
```

**Examples:**
```penroscript
{
  result1: 3 <= 5,        // true
  result2: 5 <= 5,         // true
  result3: 10 <= 10,       // true
}
```

### `>` — Greater Than

Check if left value is greater than right value.

**Syntax:**
```penroscript
left > right
```

**Examples:**
```penroscript
{
  result1: 5 > 3,        // true
  result2: `banana` > `apple`,  // true
  result3: 10 > 10,       // false
}
```

### `>=` — Greater Than or Equal

Check if left value is greater than or equal to right value.

**Syntax:**
```penroscript
left >= right
```

**Examples:**
```penroscript
{
  result1: 5 >= 3,        // true
  result2: 5 >= 5,         // true
  result3: 10 >= 10,       // true
}
```

---

## Mathematical Operators

### `+` — Add/Concat

Add numbers or concatenate strings/arrays.

**Syntax:**
```penroscript
left + right
```

**Behavior:**

| Input Type | Behavior | Example |
|------------|----------|----------|
| Number + Number | Addition | `1 + 2` → `3` |
| Text + Text | Concatenation | `` `a` + `b` `` → `` `ab` `` |
| List + List | Concatenation | `[1] + [2]` → `[1,2]` |
| Map + Map | Merge | `{a:1} + {b:2}` → `{a:1, b:2}` |

**Identity Elements:**
- `undefined` is identity for numbers: `1 + undefined` → `1`
- `undefined` is identity for text: `` `hello` + undefined `` → `` `hello` ``
- `undefined` is identity for maps: `{a:1} + undefined` → `{a:1}`
- `undefined` APPENDS to lists: `[1,2] + undefined` → `[1,2,undefined]`
- Non-identity from different set: value disappears
- Text concat when either side is text
- List concat/append when either side is list

**Examples:**
```penroscript
{
  // Numbers
  sum: 10 + 20,  // 30

  // Text
  greeting: `hello` + ` ` + `world`,  // `hello world`

  // Lists
  combined: [1, 2] + [3, 4],  // [1, 2, 3, 4]

  // Maps
  merged: {a:1} + {b:2, c:3},  // {a:1, b:2, c:3}

  // Identity elements
  numIdentity: 5 + undefined,  // 5
  strIdentity: 1 + ``,          // `1` (type coercion)
  arrIdentity: [] + [42],       // [42]
}
```

### `-` — Subtract

Subtract numbers.

**Syntax:**
```penroscript
left - right
```

**Behavior:**
- Performs numeric subtraction
- Returns `undefined` for invalid inputs

**Examples:**
```penroscript
{
  result1: 10 - 3,   // 7
  result2: 5 - 8,     // -3
}
```

### `*` — Multiply/Repeat

Multiply numbers or repeat arrays.

**Syntax:**
```penroscript
left * right
```

**Behavior:**

| Input Type | Behavior | Example |
|------------|----------|----------|
| Number * Number | Multiplication | `3 * 4` → `12` |
| List * Number | Repeat list | `[1,2] * 2` → `[1,2,1,2]` |

**Examples:**
```penroscript
{
  // Numbers
  product: 3 * 4,  // 12

  // Lists
  repeated: [1, 2] * 3,  // [1, 2, 1, 2, 1, 2]
}
```

### `%` — Divide

Divide numbers.

**Syntax:**
```penroscript
left % right
```

**Behavior:**
- Performs numeric division
- Returns `undefined` for division by zero
- Returns number or undefined

**Examples:**
```penroscript
{
  result1: 10 % 2,   // 5
  result2: 10 % 0,   // undefined (division by zero)
}
```

### `%%` — Modulus

Get remainder of division.

**Syntax:**
```penroscript
left %% right
```

**Behavior:**
- Performs modulo operation
- Returns `undefined` for division by zero
- Returns remainder or undefined

**Examples:**
```penroscript
{
  result1: 10 %% 3,   // 1
  result2: 10 %% 0,   // undefined
}
```

### `**` — Exponent

Raise a number to a power.

**Syntax:**
```penroscript
left ** right
```

**Behavior:**
- Performs exponentiation
- Returns number or undefined for invalid inputs

**Examples:**
```penroscript
{
  result1: 2 ** 3,   // 8
  result2: 10 ** 2,  // 100
  result3: 4 ** 0.5, // 2 (square root)
}
```

---

## Utility Operators

### `#` — Size/Count

Get the size of a collection.

**Syntax:**
```penroscript
#collection
```

**Behavior:**

| Input Type | Behavior | Example |
|------------|----------|----------|
| Text | Character count | `` #`hello` `` → `5` |
| List | Element count | `#[1,2,3]` → `3` |
| Map | Key count | `#{a:1, b:2}` → `2` |

**Examples:**
```penroscript
{
  text: `hello world`,
  length: #text,  // 11

  items: [1, 2, 3, 4, 5],
  count: #items,  // 5

  config: { a:1, b:2, c:3 },
  keysCount: #config,  // 3
}
```

### `!` — Not (Logical)

Negate a truthy/falsy value.

**Note:** Same symbol as mathematical negation, but context determines logical vs. mathematical use.

**Syntax:**
```penroscript
!expression
```

**Examples:**
```penroscript
{
  isActive: true,
  isInactive: !isActive,  // false
}
```

### `?` — Type Check

Returns the type of a value.

**Syntax:**
```penroscript
value ?
```

**Behavior:**
- Returns type name as string
- Similar to JavaScript's `typeof` operator
- Default output matches JS `typeof`

**Configurability:**
- **Global:** Default output matches JS `typeof`
- **Per-project:** Override type names for specific projects
- **Per-file:** Configure type names for individual files

**LTR Evaluation:**
`` `hello` ? = `text` `` — Full LTR: `` `hello` `` evaluates first, `?` outputs `text`, `=` compares

**Examples:**
```penroscript
{
  // Type checking
  textType: `hello` ?,       // `text`
  numType: 42 ?,            // `number`
  truthyType: true ?,        // `undefined` (truthy/falsy values are not types)
  undefType: undefined ?,    // `undefined`
  listType: [1,2,3] ?,     // `list`
  mapType: {a:1} ?,        // `map`
  operatorType: + ?,        // `operator`

  // Type comparison
  isText: `hello` ? = `text`,      // true
  isNumber: 42 ? = `number`,        // true
}
```

### `?"` — Conditional/Type Check

Conditional type check or ternary operator.

**Syntax:**
```penroscript
expression ?truthyValue:falsyValue
```

**Behavior:**
- Returns first value if expression is truthy
- Returns second value if expression is falsy
- Equivalent to ternary operator in other languages

**Examples:**
```penroscript
{
  value: 42,
  result1: value ?is number:not number,

  isEmpty: [],
  result2: !isEmpty ?array is empty:has items,

  age: 25,
  canVote: age >= 18 ?yes:no,
}
```

---

## Left-Hungry Currying

### Automatic Currying from Left

When operators receive insufficient arguments, they automatically curry from the left side.

**Syntax:**
```penroscript
{
  operator leftArg,    // Partial application
  full: leftArg operator rightArg  // Full application
}
```

**Behavior:**
- Binary operators become unary when given one argument
- Left argument is bound, right argument is awaited
- Enables point-free pipelines

**Examples:**
```penroscript
{
  // Partial application
  add10: + 10,        // Returns function waiting for right arg
  double: * 2,          // Returns function waiting for right arg

  // Full application
  sum: 5 add10,          // 5 + 10 = 15
  quadrupled: 4 double,    // 4 * 2 = 8

  // Directional sections
  greaterThan5: _< > 5,
  result: 10 greaterThan5,    // 10 > 5 = true

  addTo5: _> + 5,
  result2: 3 addTo5,       // 3 + 5 = 8
}
```

---

## Spatial Compounding Examples

### Text Case Operators

| Operator | Name | Spatial Pattern | Example | Result |
|-----------|------|----------------|----------|----------|
| `^` | toUpper | Upward caret (raise all) | `` `hello` ^ `` | `` `HELLO` `` |
| `^_` | Capitalize | Upward caret + underscore (raise first only) | `` `hello` ^_ `` | `` `Hello` `` |
| `"` | toLower | Downward quote (lower all) | `` `HELLO` " `` | `` `hello` `` |

### Directional Operators

| Operator | Name | Spatial Pattern | Example |
|-----------|------|----------------|----------|
| `_ <` | Left section | Arrow pointing left (left arg placeholder) | `{ _< + 5 }` |
| `_ >` | Right section | Arrow pointing right (right arg placeholder) | `{ 10 + _> }` |

---

## Operator Precedence

### Left-to-Right Evaluation

All operators evaluate left-to-right unless grouped by parentheses.

**Example:**
```penroscript
{
  result: 3 + 4 * 2  // (3 + 4) * 2 = 14
}

// With grouping
{
  result: 3 + (4 * 2)  // 3 + 8 = 11
}
```

### Grouping Rules

1. Parentheses `()` create explicit groups
2. Operators inside groups evaluate before outer operators
3. Nested groups evaluate from inner to outer

**Example:**
```penroscript
{
  result: (1 + 2) * (3 + 4)  // 3 * 7 = 21
}
```

---

## Related Documentation

- [Type System](./02-type-system.md) — Type system and type-dependent behavior
- [Design Philosophy](./01-design-philosophy.md) — Language philosophy and design principles
- [Language Overview](./00-language-overview.md) — Complete language overview
- [Master Index](./README.md) — Complete documentation suite
