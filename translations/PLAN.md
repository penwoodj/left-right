# Left-Right Translation Plan

## Source of Truth
`PenroScript-clean.md` — 87 lines, shows exact Left-Right syntax patterns. Protected (never edit).

## JS → Left-Right Operator Mapping

| JavaScript (lodash/fp) | Left-Right | Notes |
|---|---|---|
| `flow(...fns)(data)` | pipeline: `data fn1 fn2 fn3` | Left-to-right evaluation |
| `get('path', obj)` | `obj@`path`` | Path access with backtick text |
| `get(['a','b'], obj)` | `obj@[`a`, `b`]` | Array path (idiomatic) |
| `map(fn, arr)` | `arr ${ operator }` | Map operator |
| `filter(fn, arr)` | `arr ?{ condition }` | Filter operator |
| `some(fn, arr)` | `arr ?|{ condition }` | Some/any operator |
| `includes(val, arr)` | `arr >< val` | Includes operator |
| `uniq(arr)` | `arr ~` | Unique operator |
| `compact(arr)` | `arr ?{ _< }` | Filter truthy |
| `size(arr)` | `arr #` | Count operator |
| `join(sep, arr)` | `arr >< sep` | Join operator (same symbol as includes) |
| `reduce(fn, init, arr)` | `{ agg: _<@1, ...body, agg }` | Operator with accumulator |
| `groupBy(key, arr)` | operator | Custom groupBy operator |
| `sortBy(fn, arr)` | operator | Custom sortBy operator |
| `flatMap(fn, arr)` | `arr ${ operator } .` (flatten) | Map then flatten |
| `toLower(str)` | `str `'_` | String toLower |
| `capitalize(str)` | `str "^_` | String capitalize (curry reversal) |
| `startCase(str)` | string op | StartCase operator |
| `replace(pattern, str)` | string op | Replace operator |
| `split(sep, str)` | string op | Split operator |
| `trim(str)` | string op | Trim operator |
| `isString(x)` | `x ?= `text`` | Type check |
| `isPlainObject(x)` | `x ?= `map`` | Type check |
| `isArray(x)` | `x ?= `list`` | Type check |
| `isEmpty(x)` | `x # = 0` | Empty check |
| `keys(obj)` | `obj ${ _< }` + extract | Map to keys |
| `values(obj)` | `obj ${ _> }` + extract | Map to values |
| `toPairs(obj)` | `obj entries` | Entries operator |
| `entries(obj)` | `obj entries` | Entries operator |
| `assign(a, b)` | `a + b` (for maps) | Merge |
| `curry(fn)` | default behavior | All operators curry by default |
| `negate(fn)` | `!` prefix | Negation |

## Translation Principles

1. **Flow chains → pipelines**: `flow(fn1, fn2, fn3)(data)` becomes `data fn1 fn2 fn3`
2. **Lodash get → @**: `get('last_analysis_stats', attributes)` becomes `attributes@`last_analysis_stats``
3. **Callback functions → operators**: `(x) => x.value` becomes `{ _<@`value` }`
4. **Type checks → ?=**: `typeof x === 'string'` becomes `x ?= `text``
5. **Conditional logic → ternary**: `x ? y : z` stays as `x ? y : z` or uses `!?=` for type-check ternary
6. **Array methods → operators**: `.map()`, `.filter()`, `.reduce()` all become Left-Right operators
7. **Async/Promise → kept as-is for interop**: HTTP requests, async operations still need interop layer
8. **Ember.js components → NOT translated**: UI components (block.js, summary.js) are framework-specific

## File-by-File Plans

### Wiz Repository (simpler, ~600 lines)

See `wiz/TRANSLATION_PLAN.md`

### Google Threat Intelligence Repository (complex, 2164 lines)

See `google-threat-intelligence/TRANSLATION_PLAN.md`

## Protected Files (NEVER edit)
- `PenroScript-clean.md`
- `PenroScript.md`
- `Penscript_LeftRight brainstorm.md`
- `Map Programming Language Syntax Brainstorming.txt`
