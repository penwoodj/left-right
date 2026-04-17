# Data Pipeline / ETL — Transform Patterns

Documents covering extract-transform-load (ETL) patterns and data pipeline operations.

## Contents

- [`Map Programming Language Syntax Brainstorming.txt`](../data-pipeline-etl/Map%20Programming%20Language%20Syntax%20Brainstorming.txt) - ServiceNow integration, data transformation examples
- [`01-chatgpt-designing-a-programming-language.md`](../language-design-comprehensive/01-chatgpt-designing-a-programming-language.md) - Operator table for data operations

## Key Topics

### Pipeline Operations
From [`01-chatgpt-designing-a-programming-language.md`](../language-design-comprehensive/01-chatgpt-designing-a-programming-language.md#lists-objects):

**Collection Operations:**
- `map` (`$`) — Transform each element
- `filter` (`$?`) — Predicate-based selection
- `flatMap` (`$_`) — Transform and flatten
- `reduce` (`$+`) — Accumulate to single value
- `group` (`$><`) — Partition by key
- `find` (`$?.`) — Locate first match
- `uniq` (`~`) — Deduplicate

**Utility Operations:**
- `size` (`#`) — Count elements
- `pick` (`@+`) — Select keys
- `omit` (`@-`) — Exclude keys
- `first/head` (`@0`) — First element
- `last` (`@-1`) — Last element
- `tail` (`@~`) — All but first
- `slice` (`@\`) — Extract range
- `reverse` (`~~`) — Invert order

### Real-World ETL Examples

#### ServiceNow Asset Tagging
From [`Map Programming Language Syntax Brainstorming.txt`](../data-pipeline-etl/Map%20Programming%20Language%20Syntax%20Brainstorming.txt):

```javascript
// Get table query data summary tags
getTotalAssetSummaryTag: {
  kbDocs: _<@1,
  tableQueryData: _<@0,
  
  tableQueryData
    $_{ @'result' }
    ${ @0 ~~ capitalize } // capitalize result field
    ~ // unique tags
    >< ', ' // join with commas
}
```

#### Lodash FP Integration
```javascript
// Complex pipeline with multiple operations
result >> getOr['tableQueryData', []]
      >> flatMap[[...] ==> {...}] // expand references
      >> compact // remove undefined
      >> uniq // deduplicate
```

### Transform Patterns

1. **Map-Transform-Map** — JSON-like structure for pipeline stages
2. **Implicit Iteration** — No explicit loops, use HOFs
3. **Left-to-Right Flow** — Data flows through transformations sequentially
4. **Deterministic Output** — Given same input, produces same output
5. **Composable Operations** — Each transformation can be used independently

### ETL Considerations

#### Data Extraction
- Dynamic path access via `@` operator
- Optional chaining for deep paths
- Graceful handling of missing keys

#### Transformation Stages
- Filter predicates (`$?{ condition }`)
- Map operations (`${ transformation }`)
- Aggregation (`reduce` with combiner)

#### Loading (Output)
- Template generation from transformed data
- JSON/YAML serialization
- Multiple output formats

## Related Concepts

- **ETL Pipelines** — Extract, Transform, Load pattern
- **Stream Processing** — Data flow through operations
- **Functional Pipelines** — Composable transformations
- **Immutable Data** — No mutation, create new structures
- **Lazy Evaluation** — Process only when needed
