# Collections and Paths — Maps, Lists, and Access

Left-Right uses maps as the primary data structure, with lists and operators providing powerful collection manipulation. This document covers collections, path access, collection operations, and ETL patterns.

## Maps — Primary Data Structure

### Map Syntax

Maps are the fundamental data structure in Left-Right:

```javascript
// Basic map
{
  name: `Alice`,
  age: 30,
  active: true
}

// Nested maps
{
  user: {
    name: `Bob`,
    profile: {
      email: `bob@example.com`,
      settings: {
        theme: `dark`
      }
    }
  }
}
```

### Map Properties

- **Ordered** — Key insertion order preserved
- **Text-Keyed** — Keys are strings
- **Heterogeneous Values** — Any type as value
- **JSON-Compatible** — Serializable to JSON

### Map Evaluation

Maps evaluate keys sequentially, allowing references to earlier keys:

```javascript
{
  base: 10,
  offset: 5,
  result: base + offset
}
// base = 10, offset = 5, result = 15
```

## Lists — Ordered Collections

### List Syntax

Lists are ordered, heterogeneous collections:

```javascript
// Simple array
[1, 2, 3, 4, 5]

// Heterogeneous array
[`Alice`, 30, true, { active: true }]

// Nested arrays
[[1, 2], [3, 4], [5, 6]]
```

### List Properties

- **Ordered** — Index order preserved
- **Heterogeneous** — Any type as element
- **Zero-Indexed** — First element at index 0
- **JSON-Compatible** — Serializable to JSON

## Path Access with @ Operator

### @ — Get Operator

The `@` operator provides path access for both maps and arrays:

```javascript
// Map path access
user@`name`              // Get `name` key
user@[`profile`, `email`] // Nested path access (idiomatic)

// List index access
items@0                  // First element
items@-1                 // Last element
items@[2, 4]             // Slice from index 2 to 4
```

**Array Path is Primary**: While chained `@` operators work (e.g., user@`profile`@`name`), the array path syntax user@[`profile`, `name`] is the idiomatic approach for nested access.

### Path Syntax Variants

#### Single Key Access

```javascript
// Direct key access
obj@`key`

// Equivalent to: obj.key
```

#### List of Path Segments

```javascript
// Multiple segments
obj@[`nested`, `path`, `value`]

// Equivalent to: obj.nested.path.value
```

#### Numeric Indexing

```javascript
// List indices
items@0       // First item
items@-1      // Last item
items@[1, 3]   // Index 1 to 3 (exclusive)
```

### Dynamic Path Construction

Paths can be constructed dynamically:

```javascript
{
  field: `name`,
  value: obj@[field]    // Dynamic key
}
```

### Missing Key Behavior

Accessing missing paths returns `undefined` (graceful degradation):

```javascript
obj@`missingKey` // Returns: undefined
obj@[`nested`, `nonexistent`] // Returns: undefined
```

## Collection Operations

### Transform Operations

#### $ — Map

Transform each element in a collection:

```javascript
// Double each number
[1, 2, 3] ${ _< * 2 }
// Result: [2, 4, 6]

// Transform object field
users ${ @`name` }
// Result: List of names
```

#### $_ — FlatMap

Transform and flatten one level:

```javascript
// Flatten nested arrays
[[1, 2], [3, 4]] $_{ _< }
// Result: [1, 2, 3, 4]

// Transform and flatten
users $_{ @`tags` }
// Result: All tags from all users
```

### Filter Operations

#### ?{ — Filter

Select elements matching predicate:

```javascript
// Filter numbers
[1, 2, 3, 4, 5] ?{ _< > 3 }
// Result: [4, 5]

// Filter objects
items ?{ @`active` }
// Result: Active items only

// Complex predicate
data ?{
  @`age` > 18 &
  @`status` = `active`
}
```

#### $?. — Find

Find first matching element:

```javascript
// Find by property
items $?.{ @`id` = 42 }
// Result: First item with id 42

// Find by predicate
numbers $?.{ _< % 2 = 0 }
// Result: First even number
```

#### ?|! — Every

Check if all elements satisfy condition (reversed due to LTR evaluation):

```javascript
// All numbers positive?
[1, 2, 3, 4, 5]?|{ _< > 0 }!
// Result: true

// All text non-empty?
[`a`, `b`, ``]?|{ _< ?= `text` }!
// Result: false (empty text found)

// Complex condition
items ?|{
  @`active` &
  @`age` > 18
}!
```

**Note:** The `!` at the end reverses the condition, allowing normal "every" semantics despite LTR evaluation.

### Reduction Operations

#### $+ — Reduce

Accumulate to single value:

```javascript
// Sum with OR (initial 0)
[1, 2, 3] $+ |
// Result: 6

// Product with AND (initial 1)
[2, 3, 4] $+ &
// Result: 24

// Custom reducer
items $+ {
    sum: _<@0,
    item: _>@1,
    sum + item
  }
```

### Utility Operations

#### # — Size

Count elements or properties:

```javascript
// List length
[1, 2, 3] #
// Result: 3

// Map key count
{ a: 1, b: 2, c: 3 } #
// Result: 3
```

#### ~ — Unique

Deduplicate elements:

```javascript
// Unique numbers
[1, 2, 2, 3, 1] ~
// Result: [1, 2, 3]

// Unique objects by property
items ~@`id`
// Result: Items with unique ids
```

#### ~? — OrderBy / Sort

Sort elements:

```javascript
// Sort numbers
[3, 1, 4, 2] ~? <
// Result: [1, 2, 3, 4]

// Sort by property
items ~? @`age`
// Result: Items sorted by age
```

#### ~? — UniqWith

Deduplicate by custom key:

```javascript
// Unique by custom field
items ~? @`name`
// Result: First item with each unique name
```

#### ~~ — Reverse

Invert order:

```javascript
// Reverse array
[1, 2, 3, 4] ~~
// Result: [4, 3, 2, 1]
```

#### >< — Join

Concatenate with separator:

```javascript
// Join strings
[`a`, `b`, `c`] >< `, `
// Result: `a, b, c`

// Join numbers
[1, 2, 3] >< ``
// Result: `123`
```

#### <> — Split

Break string into parts:

```javascript
// Split by delimiter
`a,b,c` <> `,`
// Result: [`a`, `b`, `c`]

// Split by whitespace
`hello world` <> ` `
// Result: [`hello`, `world`]
```

### Accessor Operations

#### @0 — First

Get first element:

```javascript
items@0 // First item
```

#### @-1 — Last

Get last element:

```javascript
items@-1 // Last item
```

#### @~ — Tail

Get all but first element:

```javascript
items@~ // All except first
```

#### @\ — Slice

Extract range:

```javascript
items@[1, 4] // Elements 1 to 4 (exclusive)
items@[2]     // Elements from index 2 to end
```

#### @+ — Pick

Select specific keys:

```javascript
user@+[`name`, `email`]
// Result: { name: `Alice`, email: `alice@example.com` }
```

#### @- — Omit

Exclude specific keys:

```javascript
user@-[`password`, `secret`]
// Result: User object without sensitive fields
```

### Grouping Operations

#### $>< — Group

Partition by key:

```javascript
// Group by category
items $>< @`category`
// Result: { category1: [items...], category2: [items...] }
```

## ETL Pipeline Patterns

### Extract Phase

Pull data from sources:

```javascript
// Extract from API
{
  url: `https://api.example.com/data`,
  data: url >> fetch >> @`body`
}

// Extract from database
{
  query: `SELECT * FROM users`,
  results: query >> dbExecute
}
```

### Transform Phase

Apply transformations:

```javascript
// Complex transformation pipeline
rawData
  $_{ @`result` }           // Extract nested results
  ${ @`value` }             // Get value field
  ${ _< ~~ capitalize }       // Clean and capitalize
  ~                          // Deduplicate
  @[0, 10]                  // Limit to 10
```

### Load Phase

Output transformed data:

```javascript
// Load to file
transformedData >> writeFile >> `output.json`

// Load to API
results >> post >> `https://api.example.com/submit`

// Generate template
processedData >> generateTemplate >> `template.html`
```

### Real-World ETL Example

ServiceNow Asset Tagging:

```javascript
// Transform table query data
getTotalAssetSummaryTag: {
  kbDocs: _<@1,
  tableQueryData: _<@0,

  tableQueryData
    $_{ @`result` }           // Extract and flatten
    ${ @0 ~~ capitalize }       // Capitalize result field
    ~                          // Unique tags
    >< `, `                    // Join with commas
}
```

Complex ETL with Multiple Stages:

```javascript
// Process multiple transformations
results
  >> getOr['tableQueryData', []]    // Get or default
  >> flatMap[                       // Flatten and map
    { tableQueryDataResult } => {
      summaryTagPathsForThisType: getTableQuerySummaryTagPathsType[entity.type],
      allTableQueryPathValuesForThisResult: SUMMARY_TAG_DEFAULT_PATHS
        >> concat[summaryTagPathsForThisType]
        >> map[<< get[tableQueryDataResult] >> capitalize]
        >> uniq
        >> slice[0, 5]
    }
  ]
  >> compact                        // Remove undefined
  >> uniq                           // Deduplicate
```

## Operator Behavior by Type

### + Operator with Undefined

The `+` operator has specific behavior with `undefined`:

```javascript
// List + undefined appends undefined
[1, 2] + undefined // Result: [1, 2, undefined]

// Other types + undefined return original
`text` + undefined // Result: `text` (no change)
42 + undefined // Result: 42 (no change)
true + undefined // Result: true (no change)
```

### Map-Specific Operations

```javascript
// @ for path access
map@`key`
map@[`nested`, `path`]

// # for key count
map# // Number of keys

// @+ for picking keys
map@+[`key1`, `key2`]

// @- for omitting keys
map@-[`key1`, `key2`]
```

### List-Specific Operations

```javascript
// @ for index access
array@0       // First
array@-1      // Last
array@[1, 4]   // Slice

// ~ for unique
array~

// ~~ for reverse
array~~
```

### Text-Specific Operations

```javascript
// >< for join
[`a`, `b`, `c`] >< `, ` // `a, b, c`

// <> for split
`a,b,c` <> `, ` // [`a`, `b`, `c`]

// ^ for uppercase
`hello`^ // `HELLO`

// ^_ for capitalize
`hello`^_ // `Hello`
```

## Comparison with Other Languages

### JavaScript Array Methods

| Operation | JavaScript | Left-Right |
|-----------|------------|-------------|
| Map | `arr.map(x => x * 2)` | `arr ${ _< * 2 }` |
| Filter | `arr.filter(x => x > 5)` | `arr ?{ _< > 5 }` |
| Reduce | `arr.reduce((a, b) => a + b, 0)` | `arr $+ |` |
| Find | `arr.find(x => x.id === 42)` | `arr $?.{ @`id` = 42 }` |
| Unique | `[...new Set(arr)]` | `arr ~` |
| Join | `arr.join(', ')` | `arr >< ', '` |

### Lodash FP

| Operation | Lodash FP | Left-Right |
|-----------|-----------|-------------|
| Map | `map(x => x * 2)` | `${ _< * 2 }` |
| Filter | `filter(x => x > 5)` | `?{ _< > 5 }` |
| Reduce | `reduce(add, 0)` | `$+ |` |
| Flow | `flow(f, g, h)` | Sequential chaining |
| Compose | `compose(f, g)` | `<<` operator |

## Design Principles

1. **Map as Primary** — Maps are the fundamental structure
2. **Ordered Collections** — Predictable iteration order
3. **Path Access** — Uniform @ operator for all paths
4. **Graceful Degradation** — Missing keys return undefined
5. **Composable Operations** — Pipeline-friendly transformations
6. **Type-Adaptive** — Operators work differently by type
7. **ETL-Optimized** — Natural data transformation patterns

## Related Concepts

- **Map** — Key-value data structure
- **List** — Ordered collection
- **Path Access** — Nested structure navigation
- **Collection Operations** — Map, filter, reduce patterns
- **ETL Pipelines** — Extract, transform, load
- **Data Transformation** — Converting data formats
- **Lazy Evaluation** — Process on demand
- **Immutable Data** — Copy-on-write semantics
