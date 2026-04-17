# Functional Programming Patterns — lodash/FP Point-Free Style
**Source**: ChatGPT Chat History Conversation
**Relevance**: Point-free functional programming style that directly influenced left-right's evaluation model

## Problem Statement

Write a function using lodash/fp in a point-free style that:
- Takes two arrays of objects as input
- Combines objects that have the same property value
- Merges different equivalent properties into an array
- Stores the combined array under the same property name in the original object

## Point-Free Implementation

```javascript
const _ = require('lodash/fp');

const combineObjects = (propName, arr1, arr2) =>
  _.flow(
    _.concat(arr1, arr2), // Concatenate both arrays
    _.groupBy(_.get(propName)), // Group objects by the specified property
    _.map(_.mapValues(_.map(_.omit(propName)))), // Remove the property and merge arrays
    _.map(_.mergeAll) // Merge the grouped objects
  )([]);

// Example usage:
const array1 = [
  { id: 1, value: 'A' },
  { id: 2, value: 'B' },
];

const array2 = [
  { id: 3, value: 'A', data: 'X' },
  { id: 4, value: 'C', data: 'Y' },
];

const result = combineObjects('value', array1, array2);
console.log(result);
```

## Pattern Breakdown

### 1. Function Composition with `_.flow`

```javascript
_.flow(
  _.concat(arr1, arr2),
  _.groupBy(_.get(propName)),
  _.map(_.mapValues(_.map(_.omit(propName)))),
  _.map(_.mergeAll)
)([])
```

Uses `_.flow` to compose a pipeline of operations. Data flows left-to-right through each function.

### 2. Array Concatenation

```javascript
_.concat(arr1, arr2)
```

Combines the two input arrays into a single array.

### 3. Grouping by Property

```javascript
_.groupBy(_.get(propName))
```

Groups objects by the specified property value. Creates key-value pairs where keys are property values and values are arrays of matching objects.

### 4. Property Removal

```javascript
_.omit(propName)
```

Removes the specified property from each object within a group.

### 5. Mapping Values

```javascript
_.mapValues(_.map(_.omit(propName)))
```

Applies the omit operation to values within each group.

### 6. Merging Objects

```javascript
_.mergeAll
```

Merges grouped objects, combining them into a single object structure.

## Key lodash/FP Functions Used

| Function | Purpose |
|----------|---------|
| `_.flow` | Composes functions left-to-right |
| `_.concat` | Concatenates arrays |
| `_.groupBy` | Groups objects by a key |
| `_.get` | Retrieves a property value |
| `_.map` | Maps over collections |
| `_.mapValues` | Maps over object values |
| `_.omit` | Removes properties from objects |
| `_.mergeAll` | Merges multiple objects |

## Point-Free Style Characteristics

- No explicit intermediate variables
- Function composition as primary control flow
- Data transformations expressed as pipeline
- Each function operates on the result of the previous
- Arguments flow implicitly through the pipeline
