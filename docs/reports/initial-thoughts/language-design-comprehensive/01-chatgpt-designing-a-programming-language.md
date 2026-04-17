# Designing a Programming Language
**Source**: ChatGPT Project Conversation
**Date**: December 22, 2024
**Conversation ID**: 67677555-a5c8-8001-a034-ccbfa41d02ca
**Messages**: 30

---

## Language Design Specification

The core idea is I'm building a programming language that uses operators similar to APL over functions, but transpiles back into Javascript, and runs in a similar fashion. This is done in a way that allows something that resembles point free style programming by chaining together operators that always get evaluated left to right in expressions. The other core concept is that the language uses a structure like JSON for the program expression structure, variables etc. To start, there are only these primitives String '', Number 0, Boolean false, and Undefined undefined. There are 2 Data Structures which are Lists [1,"a"], and Maps {a:1, b:2}. The syntax for functions/operators is similar to Maps however there are certain situations that distinguish a Map data structure, from an operator. First, if the last item in Map is an expression it is an unexecuted operator: { a: 1, b:2, a+b }. Second if the values of any keys includes the symbols _< or _>, which stand for left and right argument, then it is an unexecuted operator: { a: _<, b: a+1 }. If an operator has an expression as it's last item, it just returns the result of the evaluated expression if executed, but if an operator does not have an expression as an empty expression it just returns the Map with the values filled in when executed. With the keys of Maps and Operators each key becomes accessible as a variable to each subsequent Key and value: {a:1, b: a} or {a:_<, b:a, a+b}. Also, keys can be set as by variables with a string operator. Strings, like Maps can be turned into unexecuted operators, or interpolated string by the use of curly brackets in them. So an interpolated string would work like this: 'Thanks {nameVariable}', and an unexecuted operator would look the same except it has a _< or _> symbol in the curly brackets: 'Thanks {_<}'. Anything in those curly brackets in a strings is evaluated as an expression. With that in mind a Map and Operator key can be set with a variable using this technique: {a: 'key', '{a}': 2}. Also booleans can be used as the keys in an operator which only get executed if the boolean evaluates to true: {a > b: "greater", _: "less"}. And operators can be curried: if a single expression operator is given a static value to either side it returns a monadic function that takes the other side: {_> + 2}.

### Operators Table

**MATH**
  add or concat or combine maps: +,
  subtract or remove from list: -,
  divide: %,
  multiply or repeat list: *,
  exponent: **,
  modulus/remainder: %%,

**RETURNS BOOLEAN**
  Comparison
    lt: <
    lte: <=
    gt: >
    gte: >=
    eq: =

  Conjunctions
    not: !
    and: &
    or: |

  List and String
    includes/contains: >.<
    startsWith: <.
    endsWith: >.

  List
    some: $|
    all: $&
    applyAnd: $<[] $+`&
    applyOr: $<[] $+`|

  Type Check
    isEmpty: .!
    isUndefined: .__
    isString: ."
    isBoolean: .?
    isNumber .#
    isList: .[
    isMap: .{
    isFunction: .(

**GETTERS**
  size: #
  get: @
  pick: @+
  omit: @-
  first/head: @0
  last: @-1
  tail: @~
  slice: @\
  values: @>
  keys: @<
  identity: {_<}

**String**
  join: ><
  split: <>
  replace: >"<
  trim: <">
  toUpper: "^
  toLower: "_
  capitalize: "^_
    $<[@0"^, @~"_] $+`+

**Lists & Objects**
  map: $
  flatten: _,
  flatMap: $_
  reduce: $+
  group: $><
  filter: $?
  find: $?.
  compact: $?{_<}
  chunk: $#
  applyToEachLeft: $<
  applyToEachRight: $>
  unique: ~
  uniqWith: $~
  reverse: ~~
  orderBy/sort: ~?
  shuffle: ??

**Type Conversion**
  toString: ^"
  toBoolean: ^?
  toNumber: ^#
  toList: ^]
  listsToMap: ^}

---

## Code Examples: JavaScript → PenroScript Translations

### getResultForThisEntity

```javascript
const getResultForThisEntity = (
  entity: Entity,
  results: any[],
  onlyReturnUniqueResults: boolean = false
): any =>
  flow(
    filter(flow(get('resultId'), eq(entity.value))),
    flatMap(get('result')),
    onlyReturnUniqueResults ? uniqWith(isEqual) : identity
  )(results);
```

```javascript
{ entity: _< get 0, results: _< get 1, onlyReturnUniqueResults: _< get 2 or false,
  results
    filter { get 'resultId' eq entity get 'value' }
    flatMap { get 'result' }
    { onlyReturnUniqueResults toBoolean: _< uniq, _< }
}
```

```javascript
{ entity: _<@0, results: _<@1, onlyReturnUniqueResults: _<@2 | false,
  results
    $?{ @'resultId' = entity@'value' }
    $_{ @'result' }
    { onlyReturnUniqueResults ^?: _<~, _< }
}
```

**TODO: Think through way to do conditionals with just boolean variables**

---

### getKustoQueryResults

```javascript
const getKustoQueryResults = (entity, kustoQueryResults, options) => {
  const { schema, results: tableRows } =
    getResultForThisEntity(entity, kustoQueryResults) || {};

  const tableFields = flow(
    map(getTableFields(schema, options)),
    groupBy(flow(find(flow(get('name'), eq('SourceTable'))), get('value'))),
    entries,
    map(([tableName, tableRowFields]) => ({
      tableName,
      tableRowCount: size(tableRows),
      tableRowFields: flatten(tableRowFields)
    }))
  )(tableRows);

  return tableFields;
};
```

```javascript
{ entity: _<@0, kustoQueryResults: _<@1, options: _<@2,
  resultForThisEntity: [entity, kustoQueryResults] getResultForThisEntity
  schema: resultForThisEntity@'schema',
  tableRows: resultForThisEntity@'results',
  tableFields: tableRows
    ${ tableRow: _<, [schema, options, tableRow] getTableFields }
    $><{ $?.{ @'name' = 'SourceTable' } @'value' }
    ^]
    ${ tableName: _<@[0,0], tableRowFields: _<@[0,1],
      {
        tableName,
        tableRowCount: tableRows #,
        tableRowFields: tableRowFields _
      }
    },

  tableFields
}
```

**TODO: $><{ _< $?.{ _<@'name' = 'SourceTable' } @'value' } vs $><{ $?.{ @'name' = 'SourceTable' } @'value' }**

---

### Threat Analysis

```javascript
({ threats }) => {
  const maliciousThreatsCount = flow(
      filter((threat) => get(`['AI Confidence Level'].value`, threat) === 'malicious'),
      size
    )(threats);

  const threatClassifications = flow(
    map(flow(get(`['Classification'].value`), capitalize)),
    uniq,
    join(', '),
    (threatClassifications) =>
      threatClassifications && `Threat Classifications: ${threatClassifications}`
  )(threats);

  return []
    .concat(maliciousThreatsCount)
    .concat(threatClassifications)
}
```

```javascript
{ threats: _<@[0,'threats'],
  maliciousThreatsCount: threats
    $?{ @['AI Confidence Level', 'value'] = 'malicious' }
    #,
  threatClassifications: threats
    ${ @['AI Confidence Level', 'value'] "^_}
    ~
    >< ', '
    { threatClassifications: _<,
      threatClassifications & 'Threat Classifications: {threatClassifications}'
    },

  [] + maliciousThreatsCount + threatClassifications
}
```

**TODO:**
**TODO figure out way of doing uniqueWith without running into threatClassifications problem above**
**TODO figure out way of doing And Ternaries cleaner**
**TODO: currying**
diadic functions if given a static type to either side return a monadic function that takes the other side

---

### mapObject

```javascript
const mapObject = curry((func, obj) =>
  flow(
    Object.entries,
    map(([key, value]) => func(value, key)),
    filter(and(negate(isEmpty), flow(size, eq(2)))),
    transpose2DArray,
    ([keys, values]) => zipObject(keys, values)
  )(obj)
);
```

```javascript
{func: _<@0, obj: _<@1,
  obj
    ^]
    ${ @0 ~~ func }
    $?{ $<[.! !, # = 2] $+`& }
    transpose2DArray
    { @0 zipObject }
  )(obj)
}
```

```javascript
{ func: _<@0, obj: _<@1,
  obj ${ value: _<@0, key: _<@1, [value, key] func }
}
```

```javascript
{ func: _<@0, obj: _<@1,
  obj ${ ~~ func }
}
```

---

### getSetCookies

```javascript
const getSetCookies = flow(get('set-cookie'), map(flow(split('; '), first)), join('; '));
```

```javascript
{ @'set-cookie' ${ split '; ' @0 } ><'; '
```

---

### allCombinations

```javascript
// allCombinations(['a', 'b'], [[1,2],[4,5]]) --> outputs [ { a: 1, b: 4 }, { a: 1, b: 5 }, { a: 2, b: 4 }, { a: 2, b: 5 } ]
const allCombinations = (
  [dimensionName, ...remainingDimensionNames],
  [arrayToCombine, ...remainingArraysToCombine],
  agg = {}
) =>
  flatMap(
    (item) =>
      size(remainingArraysToCombine)
        ? allCombinations(remainingDimensionNames, remainingArraysToCombine, {
            ...agg,
            [dimensionName]: item
          })
        : {
            ...agg,
            [dimensionName]: item
          },
    arrayToCombine
  );
```

```javascript
{
  allCombinations: {
    dimensionName: _<@[0,0], remainingDimensionNames: _<@0@~,
    arrayToCombine: _<@[1,0], remainingArraysToCombine: _<@1@~,
    agg: _< | {},

    arrayToCombine $_{ item: _<,
      remainingArraysToCombine # ^?: [
        remainingDimensionNames,
        remainingArraysToCombine,
        agg + {'{dimensionName}': item }
      ] allCombinations,

      agg + {'{dimensionName}': item }
    }
  }
} @ 'allCombinations'
```

---

### millisToHoursMinutesAndSeconds

```javascript
const millisToHoursMinutesAndSeconds = (millis) => {
  let remainingMillis = millis;

  const seconds = Math.floor((remainingMillis / 1000) % 60);
  remainingMillis -= seconds * 1000;

  const minutes = Math.floor((remainingMillis / 60000) % 60);
  remainingMillis -= minutes * 60000;

  const hours = Math.floor(remainingMillis / 3600000);

  return (
    (hours ? `${hours} hours, ` : '') +
    (minutes ? `${minutes} minutes, ` : '') +
    (seconds ? `${seconds} seconds` : '') +
    (!hours && !minutes && !seconds ? `${millis}ms` : '')
  );
};
```

```javascript
{ millis: _<,
  remainingMillis: millis,

  seconds: (remainingMillis % 1000) %% 60 Math@'floor',
  remainingMillis:  remainingMillis - (seconds * 1000),

  minutes: (remainingMillis % 60000) %% 60 Math@'floor',
  remainingMillis: remainingMillis - (minutes * 60000),

  hours: (remainingMillis % 3600000) Math@'floor',

  ''{
    ''{ hours ^?: '{hours} hours, ', '' } +
    ''{ minutes ^?: '{minutes} minutes, ', '' } +
    ''{ seconds ^?: '{seconds} seconds', '') +
    ''{ hours! & minutes! & seconds!: '{millis}ms', '' }
  }

  OR


  hours! & minutes! & seconds!: '{millis}ms',

  (hours ^? & '{hours} hours, ' | '') +
  (minutes ^? & '{minutes} minutes, ' | '') +
  (seconds ^? & '{seconds} seconds' | '')
}
```

---

### getHighestLowest

```javascript
const getHighestLowest = (companyInsights, sortOrder) =>
  flow(
    orderBy(
      (insight) =>
        insight.start_score > insight.end_score ? insight.start_score : insight.end_score,
      sortOrder
    ),
    first,
    ({ start_score, end_score, ...insight }) => ({
      ...insight,
      start_score,
      end_score,
      score:
        sortOrder === 'asc'
          ? start_score > end_score
            ? start_score
            : end_score
          : start_score < end_score
          ? start_score
          : end_score
    })
  )(companyInsights);
```

```javascript
{ companyInsights: _<@0, sortOrder: _<@1,
  [companyInsights, sortOrder]
    ~?{ insight: _<,
      insight@'start_score' > insight@'end_score' & insight@'start_score' | insight@'end_score'
    }
    @0
    { insight: _<,
      start_score: insight@'start_score',
      end_score: insight@'end_score',

      insight + {
        score:
          sortOrder = 'asc'
            & (start_score > end_score
              & start_score
              | end_score
            )
            | (start_score < end_score
              & start_score
              | end_score
            )
      }
}
```

```javascript
{ sortOrder: _<@1,

  _< ~?{ _<@'start_score' > _<@'end_score' & (_<@'start_score') | (_<@'end_score') }
  @0
  { start_score: _<@'start_score', end_score: _<@'end_score',
    _< + {
      score:
        sortOrder = 'asc'
          & (start_score > end_score
            & start_score
            | end_score
          )
          | (start_score < end_score
            & start_score
            | end_score
          )
    }

  }
}
```

---

## Symbol Design Rationale

When choosing the design for the symbology in the more native operators, I used these notes:

### Single Chars
/ \ | ( ) [ ] { } < > ! @ # $ % ^ & * ~ - _ ` ' " = + ? . ,

### Double Chars
// || !! \\ )) (( }} {{ ]] [[ >> << __ .. ,, -- ~~ ^^

### Two Chars
/| \| |/ \| |> >| <| |<

|/_ |/. |/-

### Symbol indications

|/\ <> ~-_= indicate direction
\\ // || = indicates parallel direction
()[]{}<> indicates encapsulation

### Per-Symbol Analysis

**/** indicates
  down left up right
  math division,
  regex delimiter,
  separation,
  in other words,
  comments,
  OR or choices or alternatives,
  in J reduce or insert operator between each element in a list and give final result,
  file folder separation

**\** indicates
  down right up left
  escape character in string,
  in J scan or insert operator between each element in a list and give a running result for each element,
  in J replicate or filter with bitmap,
  file folder separation

**|** indicates
  OR or choices or alternatives,
  Remainder in math,
  modulus,
  absolute value,
  reverse in J,
  table divider in markdown,
  pipe in bash,

**(** indicates
  left
  start of function application
  changes expression operation order
  start of argument/parameters list,
  insertion of extraneous details or context
  asides

**)** indicates
  right
  end of function application
  changes expression operation order
  end of argument/parameters list,
  insertion of extraneous details or context
  asides

**[** indicates
  left
  start of list
  dynamic property access in JS
  dynamic object key in JS
  indexing
  in J left arg or identity
  optional context
  clarification

**]** indicates
  right
  end of list
  dynamic property access in JS
  dynamic object key in JS
  indexing
  in J right arg or identity
  optional context
  clarification

**{** indicates
  left
  Start of object
  Start of function, class, if, loop, try/catch scope/code block
  start of a template literal in JS
  set in math
  group of related ideas
  in J get index

**}** indicates
  right
  end of object
  end of function, class, if, loop, try/catch scope/code block
  end of a template literal in JS
  set in math
  group of related ideas
  in J replace index

**<** indicates
  left
  open of a tag in HTML
  comparison
  left arrow <- <= <_ <~ <<
  less than < <=
  in J box

**>** indicates
  right
  close of a tag in HTML
  comparison
  right arrow -> => _> ~> >>
  greater than > >=
  in J box

**!** indicates
  negation of a boolean,
  exclamation/importance,

**@** indicates
  at
  location
  decorator in Strongly typed languages
  in J Adding verbs together / operator composition / adverbs

**#** indicates
  number of/size/length
  in J tally or duplication or bitmap filtering

**$** indicates
  value
  end of a string in regex
  start of template literal in JS
  function application in haskell
  variable reference in bash
  in J shape or reshape

**%** indicates
  percentage
  modulus/remainder
  pattern matching
  proportions
  ratios
  in J reciprocal or divide

**^** indicates
  exponent/ log with negative numbers
  exponential
  beginning of string in regex
  XOR
  caret symbol
  anchor
  power

**&** indicates
  'and' or 'then'
  function composition in J
  address in C
  background process in bash

**** indicates
  multiply or repeat
  wildcard in regex
  all in database
  footnote or annotation
  ** exponentiation
  block comment
  little star
  in J multiply or signum (0 for 0, 1 for positive, -1 for negative)

**~** indicates
  reverse or not or operator composition in J
  approximation
  top of T
  home folder in bash

**-** indicates
  minus or subtract
  range in regex
  negative or negate
  dash case in css
  stick for arrow ->
  combining/compound words  e.g. context-specific
  flags in bash

**_** indicates
  placeholder or default value
  blank space
  undefined
  private or hidden variable
  locally scoped variable
  in J negative or infinity
  snake_case

**`** indicates
  backtick
  quotes
  template literal in JS
  code block in markdown
  reversed apostrophe
  in J evaluate code string

**'** indicates
  string delimiter
  apostrophe
  single quote
  possession
  contractions e.g. don't, can't, won't

**"** indicates
  string delimiter
  double quote
  inches
  in J apply at rank

**=** indicates
  equality
  assignment
  arrow line <= =>

**+** indicates
  add or concat
  positive
  increment
  plus
  one or more matches in regex

**?** indicates
  randomness,
  uncertainty,
  uniqueness,
  a question,
  ternary and option chaining in JS,
  optional character in regex,

**** indicates
  get property in JS,
  single character in Regex,
  dot product in math,
  composition in functional programming,
  end of a statement
  reorder/reverse function application @.
  abbreviation e.g. Mr. Dr. St. Ave. Rd. etc.

**,** indicates
  separates items in a list
  pause in sentence
  in J concatenate or append or flatten

---

## Operator Symbol Explanation

### Mathematical and Structural Operators

| Operator | Explanation of Symbology |
|-----------|---------------------------|
| / | Division, separation, or choices/alternatives. Inspired by regex delimiters, file paths, or reductions in functional programming. |
| \ | Escape, scan, or reverse directionality. Reflects its use in programming (e.g., escaping characters in strings) or functional scanning. |
| \| | OR, pipe, or absolute value. Inspired by its use in pipelines (Bash, JavaScript) and modulus/remainder in math. |
| + | Addition or concatenation. Universal meaning for math and string concatenation. |
| - | Subtraction, negation, or range. Commonly used for numerical operations, ranges (e.g., `[1-5]`), and flags. |
| % | Modulus, ratio, or reciprocal. Represents remainder in math and proportions. |
| * | Multiplication or repetition. Also serves as a wildcard or "all" operator in databases. |
| ^ | Exponentiation or XOR. Represents power in math or caret for anchoring in regex. |
| = | Equality or assignment. Commonly used for comparison and setting values. |
| ~ | Reverse, not, or approximation. Reflects its use in lists and approximation contexts. |

### Logical and Conditional Operators

| Operator | Explanation of Symbology |
|-----------|---------------------------|
| ! | Negation or importance. Represents logical NOT in Boolean contexts or exclamation in language. |
| & | AND or composition. Represents conjunctions in logic and function composition in functional programming. |
| \| | OR or pipe. Inspired by logical disjunction and functional pipelines. |
| ? | Question, randomness, or optionality. Represents ternary expressions, uncertainty, or randomness. |

### Encapsulation and Access

| Operator | Explanation of Symbology |
|-----------|---------------------------|
| () | Encapsulation or grouping. Represents parentheses for function application and expression precedence. |
| [] | Indexing or dynamic access. Inspired by arrays in programming and indexing in lists. |
| {} | Grouping, objects, or functions. Represents JSON-like objects, function blocks, or scope. |
| <> | Direction or comparisons. Inspired by HTML tags and comparison operators. |

### Functional Programming Operators

| Operator | Explanation of Symbology |
|-----------|---------------------------|
| $ | Value or application. Reflects reshaping or functional application. |
| $_ | FlatMap or functional composition. Represents mapping and flattening in lists. |
| $? | Filter or conditional mapping. Inspired by predicates and filtering lists. |
| $+ | Reduce or accumulation. Represents combining elements in a list. |

### Dynamic Keys and Interpolation

| Operator | Explanation of Symbology |
|-----------|---------------------------|
| @ | Dynamic key access. Represents property access in objects or paths in nested structures. |
| ' | String delimiter or possession. Represents string literals or ownership in language. |
| " | String delimiter or rank. Represents text and specific functional ranks in programming languages. |
| {key} | Interpolation or dynamic keys. Inspired by template literals for constructing keys or strings. |

### Comparison and Directional Operators

| Operator | Explanation of Symbology |
|-----------|---------------------------|
| < | Left, less than, or input. Represents directionality or numerical comparison. |
| > | Right, greater than, or output. Represents directionality or numerical comparison. |
| << | Shift left. Represents leftward movement or precedence. |
| >> | Shift right. Represents rightward movement or precedence. |

### List and String Manipulation

| Operator | Explanation of Symbology |
|-----------|---------------------------|
| >< | Join or grouping. Inspired by combining lists or strings. |
| <> | Split or separation. Represents breaking down strings or lists. |
| ~ | Reverse or uniqueness. Reflects operations on lists or strings (e.g., reversing order). |

### Special Patterns and Double Characters

| Operator | Explanation of Symbology |
|-----------|---------------------------|
| // | Comments or alternatives. Inspired by inline comments in JavaScript or paths in file systems. |
| \\ | Escaping or scanning. Reflects escape sequences or running computations. |
| \|\| | Parallel alternatives. Represents logical OR or parallel operations. |
| !! | Double negation or importance. Represents strong negation or emphasis. |
| __ | Placeholder or undefined. Represents default values or hidden details. |
| ~~ | Negation or reversal. Represents list reversal or approximation. |
| .. | Range or continuation. Represents range of values or ellipsis. |

---

## Execution Walkthroughs

### $<[>2, <5] $+`&

To explain execution of your code `$<[>2, <5] $+`&`, I'll break it into steps of evaluation based on operators and their functionality in your described language.

**Code Components and Interpretation**

1. **`$<`**:
   - This operator represents **applyToEachLeft**. It applies provided operator to each element in left input.
   - In this case, it's applying operators `[>2, <5]` to elements of left input (which isn't explicitly stated but would typically be a list).

2. **`[>2, <5]`**:
   - This is an array of two operators:
     - `>2`: A shorthand operator meaning "_is greater than 2_".
     - `<5`: A shorthand operator meaning "_is less than 5_".
   - These operators are used to check conditions on input list.

3. **`$+`**:
   - This operator represents **reduce**. It reduces a list by applying specified operator between elements.
   - The operator to reduce with is specified as ``` `& ```, which means **logical AND**.

4. ``` `& ```**:
   - This is **logical AND** operator, which takes two Boolean values and returns `true` only if both are `true`.

**Execution Steps**

**Step 1: Initial Input and ApplyToEachLeft (`$<`)**
- Assume an **input list** for `$<` operator (e.g., `[1, 3, 4]`).
- The operators `[>2, <5]` are **applied to each element** of list.

**Step 2: Evaluate Operators for Each Element**
For each element in the input list:
1. `>2`: Checks if element is greater than 2.
2. `<5`: Checks if element is less than 5.

Example evaluations for input `[1, 3, 4]`:
- For `1`:
  - `>2` evaluates to `false`.
  - `<5` evaluates to `true`.
- For `3`:
  - `>2` evaluates to `true`.
  - `<5` evaluates to `true`.
- For `4`:
  - `>2` evaluates to `true`.
  - `<5` evaluates to `true`.

This results in a **Boolean matrix** for each condition across all elements:
```
[
  [false, true],  // Results for element 1
  [true, true],   // Results for element 3
  [true, true]    // Results for element 4
]
```

**Step 3: Reduce with Logical AND (`$+` `&`)**
- The `reduce` operator combines Boolean results **row-wise** using `&` (logical AND).
- For each element, it evaluates:
  - `[false, true]` → `false & true` → `false`.
  - `[true, true]` → `true & true` → `true`.
  - `[true, true]` → `true & true` → `true`.

This results in a **reduced list** of Boolean values:
```
[false, true, true]
```

**Step 4: Final Output**
- The final result of expression is reduced list: `[false, true, true]`.

**Summary of Execution**
1. **Input List**: `[1, 3, 4]`.
2. **Operators Applied to Each Element**:
   - `>2` → Checks if greater than 2.
   - `<5` → Checks if less than 5.
3. **Results of Operators**:
   ```
   [
     [false, true],  // Element 1
     [true, true],   // Element 3
     [true, true]    // Element 4
   ]
   ```
4. **Row-Wise Reduction with AND**:
   ```
   [false, true, true]
   ```
5. **Final Output**: `[false, true, true]`.

---

### 3 $<[>2, <5] $+`&

Let's break down the expression `3 $<[>2, <5] $+`&` step by step, explaining how it is evaluated based on your language's semantics.

**Expression**
`3 $<[>2, <5] $+`&`

**Key Operators and Their Roles**
1. `3` – A **number literal**, initial value.
2. `$<[>2, <5]` – **ApplyToEachLeft operator**:
   - Iteratively applies each condition (`>2` and `<5`) to left input (here, `3`).
3. `$+` – **Reduce/Accumulator operator**:
   - Combines results from preceding steps using a specified function (`&` in this case, representing logical AND).
4. ``` ` ``` – **Function composition**:
   - Chains together multiple operations or applies them in sequence.

**Step-by-Step Execution**

**Step 1: Start with Input**
- Initial input is **number `3`**.

**Step 2: ApplyToEachLeft (`$<[>2, <5]`)**
- **Breakdown**:
  - The array `[>2, <5]` contains two predicates:
    1. `>2`: Checks if the input is greater than 2.
    2. `<5`: Checks if the input is less than 5.
  - The `$<[ ]` operator takes the **input `3`** and applies each predicate to it.

- **Execution**:
  - `3 > 2` → `true`
  - `3 < 5` → `true`

- **Result**:
  - This step produces `[true, true]`.

**Step 3: Reduce with `$+` and `&`**
- **Breakdown**:
  - The `$+` operator reduces a list of values using the specified function.
  - Here, the function is logical AND (`&`), which computes `true` only if all inputs are `true`.

- **Execution**:
  - Reduce `[true, true]` using AND:
    - `true & true` → `true`

- **Result**:
  - This step produces the final value: `true`.

**Final Result**
- The expression evaluates to **`true`**.

**Summary of Execution**
1. Start with number `3`.
2. Apply each predicate (`>2` and `<5`) from the array to `3`, resulting in `[true, true]`.
3. Reduce results with logical AND (`&`), which outputs `true`.

---

### 10 $<[>2, <5] $+`&

Let's evaluate expression `10 $<[>2, <5] $+`&` step by step, following the same logic as before.

**Expression**
`10 $<[>2, <5] $+`&`

**Key Operators and Their Roles**
1. `10` – A **number literal**, initial value.
2. `$<[>2, <5]` – **ApplyToEachLeft operator**:
   - Applies each condition (`>2` and `<5`) to left input (`10`).
3. `$+` – **Reduce/Accumulator operator**:
   - Combines results using the logical AND operator (`&`).
4. ``` ` ``` – **Function composition**:
   - Chains together multiple operations or applies them in sequence.

**Step-by-Step Execution**

**Step 1: Start with Input**
- Initial input is **number `10`**.

**Step 2: ApplyToEachLeft (`$<[>2, <5]`)**
- **Breakdown**:
  - The array `[>2, <5]` contains two predicates:
    1. `>2`: Checks if the input is greater than 2.
    2. `<5`: Checks if the input is less than 5.
  - The `$<[ ]` operator takes the **input `10`** and applies each predicate to it.

- **Execution**:
  - `10 > 2` → `true`
  - `10 < 5` → `false`

- **Result**:
  - This step produces `[true, false]`.

**Step 3: Reduce with `$+` and `&`**
- **Breakdown**:
  - The `$+` operator reduces a list of values using the specified function.
  - Here, the function is logical AND (`&`), which computes `true` only if all inputs are `true`.

- **Execution**:
  - Reduce `[true, false]` using AND:
    - `true & false` → `false`

- **Result**:
  - This step produces the final value: `false`.

**Final Result**
- The expression evaluates to **`false`**.

**Summary of Execution**
1. Start with number `10`.
2. Apply each predicate (`>2` and `<5`) from the array to `10`, resulting in `[true, false]`.
3. Reduce results with logical AND (`&`), which outputs `false`.

**Key Difference from Previous Example**
- Unlike the previous example, `10` does not satisfy all predicates (`<5` is `false`), so the result of the reduction is `false`.

---

### 10 $<[>2, <5] $+`|

Let's evaluate expression `10 $<[>2, <5] $+`|` step by step.

**Expression**
`10 $<[>2, <5] $+`|`

**Key Operators and Their Roles**
1. `10` – A **number literal**, initial value.
2. `$<[>2, <5]` – **ApplyToEachLeft operator**:
   - Applies each condition (`>2` and `<5`) to left input (`10`).
3. `$+` – **Reduce/Accumulator operator**:
   - Combines results using the logical OR operator (`|`).
4. ``` ` ``` – **Function composition**:
   - Chains together multiple operations or applies them in sequence.

**Step-by-Step Execution**

**Step 1: Start with Input**
- Initial input is **number `10`**.

**Step 2: ApplyToEachLeft (`$<[>2, <5]`)**
- **Breakdown**:
  - The array `[>2, <5]` contains two predicates:
    1. `>2`: Checks if the input is greater than 2.
    2. `<5`: Checks if the input is less than 5.
  - The `$<[ ]` operator takes the **input `10`** and applies each predicate to it.

- **Execution**:
  - `10 > 2` → `true`
  - `10 < 5` → `false`

- **Result**:
  - This step produces `[true, false]`.

**Step 3: Reduce with `$+` and `|`**
- **Breakdown**:
  - The `$+` operator reduces a list of values using the specified function.
  - Here, the function is logical OR (`|`), which computes `true` if at least one input is `true`.

- **Execution**:
  - Reduce `[true, false]` using OR:
    - `true | false` → `true`

- **Result**:
  - This step produces the final value: `true`.

**Final Result**
- The expression evaluates to **`true`**.

**Summary of Execution**
1. Start with number `10`.
2. Apply each predicate (`>2` and `<5`) from the array to `10`, resulting in `[true, false]`.
3. Reduce results with logical OR (`|`), which outputs `true`.

**Key Difference from Previous Examples**
- The logical OR (`|`) operator only requires one `true` value in the list, so the final result is `true` despite one predicate (`<5`) being `false`.

---

### Operator Encapsulation

Here's how you can encapsulate this logic into an operator that takes a left argument. The operator applies a series of conditions to the left argument and combines results using logical OR (`|`).

**Encapsulated Operator Definition**

```
// Encapsulate logic
{ _<[conditions, combiner]:
    _ $<[conditions] $+`combiner
}
```

- **`_`**: Placeholder for left argument.
- **`conditions`**: An array of predicates (e.g., `[>2, <5]`).
- **`combiner`**: The function used to combine results (e.g., `|` for logical OR).

**Example Encapsulation for OR**
Here's a specific version for logical OR:

```
// Specific operator for OR combination
{ _<[conditions]:
    _ $<[conditions] $+`|
}
```

This defines an operator that:
1. Applies each condition in `conditions` to the left argument.
2. Combines results with OR (`|`).

**Examples**

**Example 1: Input `10`**
**Expression**:
```
// Define the operator
{ _<[>2, <5] }

// Apply operator to input 10
10 { _<[>2, <5] }
```

**Execution**:
1. Apply each condition in `[>2, <5]` to `10`:
   - `10 > 2` → `true`
   - `10 < 5` → `false`
2. Combine results with OR:
   - `true | false` → `true`

**Result**:
`true`

**Example 2: Input `3`**
**Expression**:
```
// Apply operator to input 3
3 { _<[>2, <5] }
```

**Execution**:
1. Apply each condition in `[>2, <5]` to `3`:
   - `3 > 2` → `true`
   - `3 < 5` → `true`
2. Combine results with OR:
   - `true | true` → `true`

**Result**:
`true`

**Example 3: Input `1`**
**Expression**:
```
// Apply operator to input 1
1 { _<[>2, <5] }
```

**Execution**:
1. Apply each condition in `[>2, <5]` to `1`:
   - `1 > 2` → `false`
   - `1 < 5` → `true`
2. Combine results with OR:
   - `false | true` → `true`

**Result**:
`true`

**Example 4: Input `0`**
**Expression**:
```
// Apply operator to input 0
0 { _<[>2, <5] }
```

**Execution**:
1. Apply each condition in `[>2, <5]` to `0`:
   - `0 > 2` → `false`
   - `0 < 5` → `false`
2. Combine results with OR:
   - `false | false` → `false`

**Result**:
`false`

**Correct Encapsulation**
You are absolutely right, the correct encapsulation for the operator should indeed be:

```
// With explicit left argument placeholder
{ _< $<[>2, <5] $+`| }

// Using shorthand for operators with left argument (_< is implicit)
{ $<[>2, <5] $+`| }
```

**Why This Works**
1. **`_<' is implicit** in single-expression operators in shorthand form.
   - `{ $<[>2, <5] $+`| }` assumes the left input as `_<' without needing explicit declaration.
2. **Left-to-right evaluation** ensures that the left argument flows through `$<[ ]`, applies the conditions (`>2` and `<5`), and reduces with `$+` using `|`.

**Examples Revisited**

**Example 1: Input `10`**
**Expression**:
```javascript
10 { $<[>2, <5] $+`| }
```

**Execution**:
1. Apply `>2` and `<5`:
   - `10 > 2` → `true`
   - `10 < 5` → `false`
2. Reduce with OR:
   - `true | false` → `true`

**Result**: `true`

**Example 2: Input `3`**
**Expression**:
```javascript
3 { $<[>2, <5] $+`| }
```

**Execution**:
1. Apply `>2` and `<5`:
   - `3 > 2` → `true`
   - `3 < 5` → `true`
2. Reduce with OR:
   - `true | true` → `true`

**Result**: `true`

**Example 3: Input `0`**
**Expression**:
```javascript
0 { $<[>2, <5] $+`| }
```

**Execution**:
1. Apply `>2` and `<5`:
   - `0 > 2` → `false`
   - `0 < 5` → `true`
2. Reduce with OR:
   - `false | true` → `true`

**Result**: `true`

**Example 4: Input `-1`**
**Expression**:
```javascript
-1 { $<[>2, <5] $+`| }
```

**Execution**:
1. Apply `>2` and `<5`:
   - `-1 > 2` → `false`
   - `-1 < 5` → `true`
2. Reduce with OR:
   - `false | true` → `true`

**Result**: `true`

**Key Takeaway**
- **Explicit Left Argument**: `{ _< $<[>2, <5] $+`| }`
- **Shorthand**: `{ $<[>2, <5] $+`| }`

**Input that Returns False**
To construct an example where the operator `{ $<[>2, <5] $+`| }` returns **`false`**, we need an input that fails **both predicates**:
1. **Condition 1**: `>2` (greater than 2).
2. **Condition 2**: `<5` (less than 5).

For the result to be `false`, the input must:
- Be **less than or equal to 2**, or
- Be **greater than or equal to 5**, or
- **Fail both conditions**.

**Summary Table of Inputs and Results**
| Input | `>2` Result | `<5` Result | Final Output |
|-------|-------------|-------------|--------------|
| `7`   | `true`      | `false`     | `true`       |
| `1`   | `false`     | `true`      | `true`       |

---

### getResultForThisEntity Walkthrough

Let's break down the operator and explain how it works step by step.

**Operator Breakdown**

**Full Operator**:
```javascript
{ entity: _<@0, results: _<@1, onlyReturnUniqueResults: _<@2 | false,
  results
    $?{ @'resultId' = entity@'value' }
    $_{ @'result' }
    { onlyReturnUniqueResults ^?: _<~, _< }
}
```

**Components of Operator**
1. **Input Arguments**:
   - `@0`: First argument, `entity`.
   - `@1`: Second argument, `results`.
   - `@2`: Third argument, `onlyReturnUniqueResults` (defaults to `false` if not provided).

2. **Structure**:
   - **Step 1**: Extracts `entity`, `results`, and `onlyReturnUniqueResults` from inputs.
   - **Step 2**: Processes `results` through a sequence of filters and transformations:
     1. **Filter**: `results` is filtered where `resultId` matches `entity.value`.
     2. **FlatMap**: Extracts `result` from the filtered results.
     3. **Conditional Handling**: If `onlyReturnUniqueResults` is `true`, return unique results (`~`); otherwise, return results as-is.

**Step-by-Step Execution**

**Step 1: Assign Inputs**
- Inputs are assigned to respective keys:
  - `entity = @0` (the first argument).
  - `results = @1` (the second argument).
  - `onlyReturnUniqueResults = @2 | false` (third argument or `false` by default).

**Step 2: Filter Results**
- Use `$?` to filter `results`:
  ```javascript
  $?{ @'resultId' = entity@'value' }
  ```
  - For each item in `results`:
    - Check if `resultId` matches `entity.value`.
  - Keeps only the items where the condition is `true`.

**Step 3: FlatMap Results**
- Use `$_` to extract the `result` field from the filtered results:
  ```javascript
  $_{ @'result' }
  ```
  - Maps over the filtered results and collects the `result` field from each item.
  - Flattens results if they are nested lists.

**Step 4: Conditional Unique Filtering**
- Use a conditional operator to handle `onlyReturnUniqueResults`:
  ```javascript
  { onlyReturnUniqueResults ^?: _<~, _< }
  ```
  - If `onlyReturnUniqueResults` is `true`, return unique results (`~` operator).
  - Otherwise, return the results unchanged (`_<'`).

**Output**
- The final output is the processed `results`, either unique or as-is, depending on `onlyReturnUniqueResults`.

**Example Inputs and Results**

**Input 1: `onlyReturnUniqueResults = true`**
**Inputs**:
```javascript
@0 = { value: '123' } // entity
@1 = [
  { resultId: '123', result: 'A' },
  { resultId: '123', result: 'A' },
  { resultId: '123', result: 'B' },
  { resultId: '456', result: 'C' }
] // results
@2 = true // onlyReturnUniqueResults
```

**Execution**:
1. **Filter**:
   - Keep only items where `resultId = entity.value ('123')`:
     ```javascript
     [
       { resultId: '123', result: 'A' },
       { resultId: '123', result: 'A' },
       { resultId: '123', result: 'B' }
     ]
     ```

2. **FlatMap**:
   - Extract `result` from each item:
     ```javascript
     ['A', 'A', 'B']
     ```

3. **Conditional Unique Filtering**:
   - Since `onlyReturnUniqueResults = true`, return unique results:
     ```javascript
     ['A', 'B']
     ```

**Output**:
```javascript
['A', 'B']
```

**Input 2: `onlyReturnUniqueResults = false`**
**Inputs**:
```javascript
@0 = { value: '123' } // entity
@1 = [
  { resultId: '123', result: 'A' },
  { resultId: '123', result: 'A' },
  { resultId: '123', result: 'B' },
  { resultId: '456', result: 'C' }
] // results
@2 = false // onlyReturnUniqueResults
```

**Execution**:
1. **Filter**:
   - Same filtered results:
     ```javascript
     [
       { resultId: '123', result: 'A' },
       { resultId: '123', result: 'A' },
       { resultId: '123', result: 'B' }
     ]
     ```

2. **FlatMap**:
   - Same flat-mapped results:
     ```javascript
     ['A', 'A', 'B']
     ```

3. **Conditional Unique Filtering**:
   - Since `onlyReturnUniqueResults = false`, return results unchanged:
     ```javascript
     ['A', 'A', 'B']
     ```

**Output**:
```javascript
['A', 'A', 'B']
```

**Summary**
- **Purpose**: Filters and extracts `results` based on a matching `resultId` to `entity.value`, optionally ensuring uniqueness.
- **Outputs**:
  - **With `onlyReturnUniqueResults = true`**: Unique results.
  - **With `onlyReturnUniqueResults = false`**: All matching results, unfiltered for duplicates.

---

### mapObject Walkthrough

Let's break down this operator step by step, explain its mechanics, and analyze its output for different inputs.

**Operator Breakdown**

**Full Operator**:
```javascript
{ func: _<@0, obj: _<@1,
  obj ${ value: _<@0, key: _<@1, [value, key] func }
}
```

**Components of Operator**
1. **Input Arguments**:
   - `@0`: First argument, `func` – a function to be applied.
   - `@1`: Second argument, `obj` – an object whose key-value pairs are processed.

2. **Structure**:
   - **Step 1**: Extracts `func` and `obj` from the input arguments.
   - **Step 2**: Iterates over `obj` using `${}`, which applies a mapping operation:
     - For each `key-value` pair in `obj`:
       - Bind `value` to the value of the current key.
       - Bind `key` to the current key.
       - Apply `[value, key]` as an argument array to `func`.

**How It Works**
1. **Input Object Traversal**:
   - The `${}` operator iterates over the `obj` key-value pairs.
   - `value` is the value of the current key.
   - `key` is the current key.

2. **Function Application**:
   - `[value, key]` is passed as an argument array to `func`.
   - The result of applying `func([value, key])` becomes the transformed value for each `key-value` pair.

3. **Output**:
   - A transformed list or object, depending on the mapping context.

**Step-by-Step Execution**

**Example Input**
```javascript
@0 = ([value, key]) => `${key}: ${value}`; // func
@1 = { a: 1, b: 2, c: 3 }; // obj
```

**Expression**:
```javascript
{ func: @0, obj: @1,
  obj ${ value: _<@0, key: _<@1, [value, key] func }
}
```

**Step 1: Assign Inputs**
- `func = ([value, key]) => `${key}: ${value}``
- `obj = { a: 1, b: 2, c: 3 }`

**Step 2: Iterate Over `obj`**
Use `${}` to iterate through the key-value pairs of `obj`:
1. For the first key-value pair (`a: 1`):
   - `value = 1`
   - `key = 'a'`
   - Apply `func([value, key]) → func([1, 'a']) → 'a: 1'`.

2. For the second key-value pair (`b: 2`):
   - `value = 2`
   - `key = 'b'`
   - Apply `func([value, key]) → func([2, 'b']) → 'b: 2'`.

3. For the third key-value pair (`c: 3`):
   - `value = 3`
   - `key = 'c'`
   - Apply `func([value, key]) → func([3, 'c']) → 'c: 3'`.

**Step 3: Collect Results**
- Collect the transformed results:
  ```javascript
  ['a: 1', 'b: 2', 'c: 3']
  ```

**Output**
For the given input:
```javascript
@0 = ([value, key]) => `${key}: ${value}`;
@1 = { a: 1, b: 2, c: 3 };
```

**Result**:
```javascript
['a: 1', 'b: 2', 'c: 3']
```

**Additional Examples**

**Example 1: Square Values**
**Inputs**:
```javascript
@0 = ([value, key]) => value * value; // func
@1 = { a: 1, b: 2, c: 3 }; // obj
```

**Execution**:
1. For `a: 1` → `1 * 1 = 1`.
2. For `b: 2` → `2 * 2 = 4`.
3. For `c: 3` → `3 * 3 = 9`.

**Output**:
```javascript
[1, 4, 9]
```

**Example 2: Key-Value Concatenation**
**Inputs**:
```javascript
@0 = ([value, key]) => `${key}-${value}`; // func
@1 = { x: 10, y: 20 }; // obj
```

**Execution**:
1. For `x: 10` → `'x-10'`.
2. For `y: 20` → `'y-20'`.

**Output**:
```javascript
['x-10', 'y-20']
```

**Summary**
The operator:
1. Iterates through an object's key-value pairs.
2. Applies a function `func` to each pair, treating them as `[value, key]`.
3. Returns a transformed list based on `func`.

---

## Key Design Decisions & Open Questions

### TODO Items Identified

1. **Conditionals**: Think through way to do conditionals with just boolean variables
2. **uniqueWith**: Figure out way of doing uniqueWith without running into threatClassifications problem above
3. **And Ternaries**: Figure out way of doing And Ternaries cleaner
4. **Currying**: Diadic functions if given a static type to either side return a monadic function that takes the other side
5. **$>< operator**: $><{ _< $?.{ _<@'name' = 'SourceTable' } @'value' } vs $><{ $?.{ @'name' = 'SourceTable' } @'value' }

### Key Design Principles

1. **Point-Free Style**: Language enables point-free programming by chaining operators left-to-right
2. **JSON-like Structure**: Uses Map/Map-like structures for both data and operators
3. **Implicit Left Argument**: Single-expression operators can assume `_<` without explicit declaration
4. **Variable Scope**: Keys in Maps/Operators become accessible as variables to subsequent keys
5. **String Interpolation**: Curly braces in strings enable both interpolation and operator creation
6. **Boolean Keys in Operators**: Boolean expressions as keys enable conditional execution
7. **Operator Distinguishing**:
   - Last item is an expression → unexecuted operator returning evaluated result
   - Contains `_<' or `_>` → unexecuted operator (with placeholders)
   - Otherwise → Map data structure
8. **Static Type Currying**: Operators with static values on one side return monadic functions
9. **Type System**: Four primitive types (String, Number, Boolean, Undefined) and two data structures (List, Map)

### Open Questions

- How to implement boolean-only conditionals cleanly
- How to handle `uniqueWith` without interference in chained transformations
- How to make ternary/and combinations more concise
- Currying semantics for diadic functions with static values
- Clarification on `$><` operator syntax for nested conditions
