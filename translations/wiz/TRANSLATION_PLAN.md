# Wiz → Left-Right Translation Plan

## Source
`translations/wiz-original/` — 14 JS files, ~600 lines total

## Output Structure
```
translations/wiz/
├── README.md                    # Translation notes + mapping reference
├── integration.lr               # Main entry: doLookup
├── constants.lr                 # MAX_PAGE_SIZE constant
├── dataTransformations.lr       # Core data functions
├── assembleLookupResults.lr     # Result assembly
├── request.lr                   # HTTP auth + parallel requests
├── queries/
│   ├── queryEntities.lr         # Generic query runner (chunks → GraphQL → merge → associate)
│   ├── queryIssues.lr           # Issues GraphQL builder
│   ├── queryVulnerabilities.lr  # Vulnerabilities GraphQL builder
│   └── queryAssets.lr           # Assets GraphQL builder
└── userOptions/
    ├── validateOptions.lr       # Option validation
    └── utils.lr                 # Validation utilities
```

## Translation Notes

### What Translates Cleanly
- **dataTransformations.js**: `getEntityTypes` is literally the PenroScript-clean.md example! Perfect 1:1 translation.
- All `flow()` chains → Left-Right pipelines
- All `get('path', obj)` → `obj@`path``
- All `filter()` → `?{`, `map()` → `${`, `some()` → `?|{`

### What Needs Interop Layer
- `request.js`: HTTP requests + OAuth token caching — needs JS interop
- `async/await`: Promise.all → parallel execution interop
- `require()`: Module imports → Left-Right module system

### Function-by-Function Plan

#### 1. `isPrivateIP` (dataTransformations.js:20-29)
```js
// JS
const isPrivateIP = (ip) => {
  var parts = ip.split('.');
  return parts[0] === '10' || (parts[0] === '172' && ...) || (parts[0] === '192' && parts[1] === '168');
};
```
```lr
// Left-Right
{ ip: _<,
  parts: ip split `.`,
  parts@0 = `10` | (parts@0 = `172` & parts@1 >= 16 & parts@1 <= 31) | (parts@0 = `192` & parts@1 = `168`)
}
```

#### 2. `addIdsToEntities` (dataTransformations.js:31-38)
```js
map((entity) => ({ ...entity, id: `a${Math.random().toString(36).slice(2)}` }), entities)
```
```lr
entities ${ { ..._<, id: `a` + Math.random().toString(36).slice(2) } }
```

#### 3. `removePrivateIps` (dataTransformations.js:40-41)
```js
filter(({ isIP, value }) => !isIP || (isIP && !isPrivateIP(value)), entities)
```
```lr
entities ?{ { isIP: _<@`isIP`, value: _<@`value` }, !isIP | (isIP & !{value}isPrivateIP) }
```

#### 4. `getEntityTypes` (dataTransformations.js:43-59) — EXACT PenroScript example!
```lr
// Directly from PenroScript-clean.md:
{ typesToGet: _<@0, entities: _<@1,
  lowerTypesToGet: {
    typesToGet ?= `text`: [typesToGet],
    typesToGet `'_ 
  },
  entityTypesToGet: entities ?{
    lowerEntityTypes: entities@`types` `'_,
    entityTypesAreInTypesToGet: lowerTypesToGet ?|{
      typeToGet: _<@0,
      lowerEntityTypes >< typeToGet
    },
    entityTypesAreInTypesToGet
  },
  entityTypesToGet
}
```

#### 5. `splitCommaSeparatedUserOption` (dataTransformations.js:61-62)
```js
flow(get(key), split(','), map(trim), compact, uniq)(options)
```
```lr
options@key split `,` ${ trim } ?{ _< } ~
```

#### 6. `getResultForThisEntity` (dataTransformations.js:64-69)
```js
flow(
  filter(flow(get('resultId'), eq(entity.value))),
  flatMap(get('result')),
  onlyReturnUniqueResults ? uniqWith(isEqual) : identity
)(results)
```
```lr
results ?{ _<@`resultId` = entity@`value` } ${ _<@`result` } .(flatten)
// with unique option:
{ entity: _<@0, results: _<@1, onlyUnique: _<@2,
  filtered: results ?{ _<@`resultId` = entity@`value` } ${ _<@`result` },
  onlyUnique ? filtered ~ : filtered
}
```

#### 7. `validateOptionsToDoLookupOptions` (dataTransformations.js:71-79)
```js
reduce((agg, optionObj, optionKey) => ({ ...agg, [optionKey]: get('value', optionObj) }), {}, options)
```
```lr
options { agg: _<@1, optionObj: _<@0@1, optionKey: _<@0@0, ...agg, [optionKey]: optionObj@`value` }
```

#### 8. `assembleLookupResults` (assembleLookupResults.js:5-56)
```lr
{ entities: _<@0, issues: _<@1, vulnerabilities: _<@2, assets: _<@3, options: _<@4,
  results: entities ${
    { entity: _<,
      resultsForThisEntity: {
        issues: getResultForThisEntity(entity, issues),
        vulnerabilities: getResultForThisEntity(entity, vulnerabilities),
        assets: getResultForThisEntity(entity, assets)
      },
      resultsFound: resultsForThisEntity ?|{ _< # > 0 },
      lookupResult: {
        entity: entity,
        data: resultsFound ? {
          summary: createSummaryTags(resultsForThisEntity),
          details: resultsForThisEntity
        } : undefined
      },
      lookupResult
    }
  },
  results
}
```

#### 9. `createSummaryTags` (assembleLookupResults.js:36-54)
```lr
{ data: _<,
  issuesTag: data@`issues` # > 0 & `Issues: {data@`issues` #}{data@`issues` # = MAX_PAGE_SIZE & `+`}`,
  vulnsTag: data@`vulnerabilities` # > 0 & `Vulns: {data@`vulnerabilities` #}{data@`vulnerabilities` # = MAX_PAGE_SIZE & `+`}`,
  assetsTag: data@`assets` # > 0 & `Assets: {data@`assets` #}{data@`assets` # = MAX_PAGE_SIZE & `+`}`,
  [] + issuesTag + vulnsTag + assetsTag
}
```

#### 10. `buildAggregateQueries` (queryEntitiesWithQueryStringBuilder.js:30-46)
```lr
{ entities: _<@0, builder: _<@1, chunkSize: _<@2, options: _<@3,
  entities chunk chunkSize ${
    { chunkEntities: _<,
      queries: chunkEntities ${ { entity: _<, builder entity options } } >< ``,
      `query {{queries}}`
    }
  }
}
```

#### 11. `validateOptions` (validateOptions.js:6-64)
```lr
{ options: _<@0, callback: _<@1,
  stringErrors: validateStringOptions(stringOptionsErrorMessages, options),
  stringErrors # > 0 ? callback(undefined, stringErrors) : {
    querySelected: options@`queryIssues`@`value` | options@`queryVulnerabilities`@`value` | options@`queryAssets`@`value`,
    queryErrors: !querySelected ? [
      { key: `queryIssues`, message: `At least one of these Query options must be checked` },
      { key: `queryVulnerabilities`, message: `At least one of these Query options must be checked` },
      { key: `queryAssets`, message: `At least one of these Query options must be checked` }
    ] : [],
    queryErrors # > 0 ? callback(undefined, queryErrors) : {
      domainValue: options@`authTokenDomain`@`value`,
      slashError: domainValue endsWith `/` ? [{ key: `authTokenDomain`, message: `Only Domain allowed, no trailing slash / allowed` }] : [],
      protocolError: domainValue startsWith `http` ? [{ key: `authTokenDomain`, message: `Only Domain allowed, http(s) not allowed` }] : [],
      callback(undefined, slashError + protocolError)
    }
  }
}
```

## Not Translated
- `request.js`: HTTP/OAuth interop (needs JS runtime)
- GraphQL query strings: Stay as template text
- `polarity-integration-utils` imports: External interop
