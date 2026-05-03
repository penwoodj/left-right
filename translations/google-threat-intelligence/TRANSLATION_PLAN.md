# Google Threat Intelligence → Left-Right Translation Plan

## Source
`translations/google-threat-intelligence-original/` — 4 JS files, 2164 lines in integration.js

## Output Structure
```
translations/google-threat-intelligence/
├── README.md                    # Translation notes
├── integration.lr               # Main file: doLookup, formatters, processing
├── constants.lr                 # Threat actor names (data file)
├── lookupUriByType.lr          # URI mapping config
├── formatVulnerabilities.lr    # CVSS/severity formatting pipeline
├── formatThreatActors.lr       # Threat actor formatting
├── formatThreats.lr            # Threat formatting
├── formatReports.lr            # Report formatting
├── groupByConfidence.lr        # Complex 8-field confidence grouping
├── flattenWithPaths.lr         # Recursive object flattening
├── sortByEntityPresence.lr     # Decorate/sort/group pipeline
├── processLookupItem.lr        # Scan result processing
├── detailFields.lr             # Config-driven detail fields
├── getWhois.lr                 # Historical whois lookup
├── getRelations.lr             # Referrer files lookup
├── getBehaviors.lr             # File behavior lookup
├── onMessage.lr                # Message router
├── validateOptions.lr          # Option validation
└── threatActorCache.lr         # Paginated cursor fetch
```

## Translation Notes

### What Translates Cleanly
- Every `flow()` chain → Left-Right pipeline
- Every `get('path', obj)` → `obj@`path``
- Every lodash collection method → Left-Right operator
- The entire data transformation layer is pure functional → perfect Left-Right fit

### What Needs Interop
- HTTP requests (`postman-request`) → JS interop
- `async.parallel` → parallel execution interop
- `showdown` (Markdown→HTML) → JS interop
- `node-schedule` (cron) → JS interop
- Ember.js components (block.js, summary.js) → NOT translated

### NOT Translated
- `components/block.js` — Ember.js UI (406 lines)
- `components/summary.js` — Ember.js UI (12 lines)
- `lib/constants.js` — Stays as data (1311 lines of threat actor names)

## Function-by-Function Plan

### 1. Constants and Config (lines 1-100)
```lr
// LOOKUP_URI_BY_TYPE
{
  ip: `https://www.virustotal.com/api/v3/ip_addresses`,
  domain: `https://www.virustotal.com/api/v3/domains`,
  hash: `https://www.virustotal.com/api/v3/files`,
  url: `https://www.virustotal.com/api/v3/urls`
}

// IGNORED_IPS
[`127.0.0.1`, `255.255.255.255`, `0.0.0.0`]

// GTI_LOOKUP_LIMIT
5
```

### 2. `doLookup` (lines 100-250) — Entity categorization + parallel lookups
```lr
// Entity categorization pipeline (simplified)
{ entities: _<@0, options: _<@1, callback: _<@2,
  
  // Categorize entities by type
  ipEntities: entities ?{ _<@`isIP` } ?{ !(_<@`value` >< IGNORED_IPS) },
  domainEntities: entities ?{ _<@`isDomain` },
  hashEntities: entities ?{ _<@`isHash` },
  urlEntities: entities ?{ _<@`isURL` },
  cveEntities: entities ?{ _<@`types` >< `cve` },
  threatActorEntities: entities ?{ _<@`types` >< `threatActor` | (_<@`value` >< threatActorNames) },
  
  // Parallel lookups (interop layer)
  ipResults: ipEntities ${ _lookupEntityType(`ip`, _<, options) },
  domainResults: domainEntities ${ _lookupEntityType(`domain`, _<, options) },
  hashResults: hashEntities ${ _lookupEntityType(`hash`, _<, options) },
  urlResults: urlEntities ${ _lookupEntityType(`url`, _<, options) },
  cveResults: cveEntities ${ _lookupEntityType(`cve`, _<, options) },
  
  // Threat actor + GTI lookups in parallel
  threatActorResults: threatActorEntities ${ lookupThreatActorsAsync(_<, options) },
  gtiResults: entities ${ lookupGtiAsync(_<, options) },
  
  // Merge and return
  allResults: [] + ipResults + domainResults + hashResults + urlResults + cveResults + threatActorResults + gtiResults,
  allResults ?{ _<@`data` } // filter out misses
}
```

### 3. `formatVulnerabilities` (lines 400-600) — Massive CVSS pipeline
```lr
// This is the showcase function — massive flow chain becomes clean pipeline
{ entity: _<@0, responseBody: _<@1,
  vulns: responseBody@`data` ${
    { vuln: _<,
      cve: vuln@`id`,
      attributes: vuln@`attributes`,
      
      // CVSS scoring
      cvss: attributes@[`cvssv3`, `cvssv2`] ?{ _< # > 0 },
      severity: cvss ?{ _<@[`cvssv3`, `severity`] } | attributes@`severity`,
      score: cvss ?{ _<@[`cvssv3`, `score`] } | attributes@`score`,
      
      // EPSS metrics
      epssProbability: attributes@`epss`,
      epssPercentile: attributes@`epss_percentile`,
      
      // Time to mitigation
      timeToMitigation: attributes@`time_to_mitigation`,
      
      // Dates (convert epoch to milliseconds)
      creation_date: attributes@`creation_date` * 1000,
      last_modification_date: attributes@`last_modification_date` * 1000,
      
      // HTML description (interop)
      htmlDescription: convertMarkdownToHtml(attributes@`description`),
      
      // Confidence grouped data
      confidenceGroupedData: {attributes} groupByConfidence,
      
      // Full formatted vulnerability object
      ...attributes,
      id: cve,
      cvssScore: score,
      severityLevel: severity
    }
  },
  vulns
}
```

### 4. `formatThreatActors` (lines 800-920)
```lr
{ responseBody: _<,
  actors: responseBody@`data` ${
    { threatActor: _<, attributes: _<@`attributes`,
      targetedIndustryNames: attributes@`targeted_industries_tree` ${
        { ig: _<@`industry_group`, i: _<@`industry` },
        ig # > i # ? ig : i
      },
      targetedRegionNames: attributes@`targeted_regions_hierarchy` ${
        { country: _<@`country` | _<@`country_iso2` | ``, sub: _<@`sub_region` | _<@`region` | `` },
        country & sub ? `{country}, {sub}` : country | sub
      } ?{ _< },
      htmlDescription: convertMarkdownToHtml(attributes@`description`),
      confidenceGroupedData: {attributes} groupByConfidence,
      first_seen: attributes@`first_seen` ? attributes@`first_seen` * 1000 : attributes@`first_seen`,
      last_seen: attributes@`last_seen` ? attributes@`last_seen` * 1000 : attributes@`last_seen`,
      ...attributes,
      id: threatActor@`id`
    }
  },
  actors
}
```

### 5. `groupByConfidence` (lines 1151-1306) — Complex 8-field grouping
```lr
{ association: _<,
  // Check if array has confidence objects
  hasConfidenceArray: { property: _<,
    association@`property` ?= `list` & 
    association@`property` # > 0 &
    association@`property`@0 ?= `map` &
    association@`property` ?|{ _<@`confidence` }
  },
  
  // Fields to check for confidence data
  confidenceFields: [`motivations`, `tags_details`, `malware_roles`, `available_mitigation`,
                     `vendor_fix_references`, `source_regions_hierarchy`, 
                     `targeted_regions_hierarchy`, `targeted_industries_tree`, `merged_actors`],
  
  // Group each field by confidence value
  groupedData: confidenceFields ${
    { field: _<,
      hasData: {association} hasConfidenceArray field,
      hasData ? association@field groupBy `confidence` : undefined
    }
  },
  
  // Collect unique confidence values
  uniqueConfidences: groupedData ${ _<@`value` } ?{ _< } ~,
  
  // For each confidence, collect values from each field
  result: uniqueConfidences ${
    { confidence: _<,
      getUniqueValues: { groupedField: _<,
        groupedField@confidence ${ _<@`value` } ~ ?{ !(_< >< `null` | _< >< `undefined`) }
      },
      motivations: getUniqueValues groupedData@`motivations`,
      tags: getUniqueValues groupedData@`tags_details`,
      malwareRoles: getUniqueValues groupedData@`malware_roles`,
      availableMitigation: getUniqueValues groupedData@`available_mitigation`,
      vendorFixReferences: getUniqueValues groupedData@`vendor_fix_references`,
      sourceRegions: groupedData@`source_regions_hierarchy` formatRegion confidence,
      targetedRegions: groupedData@`targeted_regions_hierarchy` formatRegion confidence,
      targetedIndustries: getUniqueValues groupedData@`targeted_industries_tree`,
      mergedActors: getUniqueValues groupedData@`merged_actors`,
      
      { 
        motivations & { motivations } | undefined,
        tags & { tags } | undefined,
        malwareRoles & { malwareRoles } | undefined,
        availableMitigation & { availableMitigation } | undefined,
        vendorFixReferences & { vendorFixReferences } | undefined,
        sourceRegions & { sourceRegions } | undefined,
        targetedRegions & { targetedRegions } | undefined,
        targetedIndustries & { targetedIndustries } | undefined,
        mergedActors & { mergedActors } | undefined
      }
    }
  },
  result
}
```

### 6. `flattenWithPaths` (lines 1373-1422) — Recursive traversal
```lr
// Recursive operator — walks object tree, lifts leaves to flat list with paths
{ entity: _<@0, obj: _<@1,
  // Check if empty object
  obj keys # = 0 ? undefined : {
    traverse: { path: _<@0, val: _<@1,
      // Plain object with no nested → leaf
      val ?= `map` & !(val values ?|{ _< ?= `map` | _< ?= `list` })
        ? [{ ...val, path: path >< `.` }]
      // Array with no nested → leaf
      : val ?= `list` & !(val ?|{ _< ?= `map` | _< ?= `list` })
        ? [{ values: val, path: path >< `.` }]
      // Plain object with nested → recurse
      : val ?= `map` & val values ?|{ _< ?= `map` | _< ?= `list` }
        ? val entries ${ { [k, v]: _<, traverse([path + k], v) } } .
      // Array with nested → recurse
      : val entries ${ { [_, v]: _<, traverse(path, v) } } .
      // Primitive → leaf
      : [{ value: val, path: path >< `.` }]
    },
    
    flattened: traverse([], obj),
    sorted: {entity} sortByEntityPresenceAndPrevalence flattened,
    sorted
  }
}
```

### 7. `sortByEntityPresenceAndPrevalence` (lines 1449-1479)
```lr
{ entity: _<@0, objects: _<@1,
  result: objects
    ?{ _< }                          // compact (remove falsey)
    ~                                // uniq
    ${ { ..._<, readablePath: _<@`path` makeHumanReadable, matchesSubstring: _< value entity@`value` } }
    sort { -(_<@`prevalence` | 0) }  // high prevalence first
    sort { _<@`matchesSubstring` ? 0 : 1 }  // entity matches first
    groupBy `readablePath`           // group by pretty path
    ${ { records: _<,
         lengths: records ?{ _<@`value` } ${ String(_<) # },
         avg: lengths # > 0 ? Math.ceil(mean(lengths)) : 0,
         { data: records, averageLength: avg }
       }
    },
  result
}
```

### 8. `makeHumanReadable` (line 1509)
```lr
// flow(replace(/\./g, ' '), replace(/_/g, ' '), startCase)
{ path: _<,
  path replace `.` ` ` replace `_` ` ` "^"
}
```

### 9. `_processLookupItem` (lines 1512-1628) — Scan result processing
```lr
{ type: _<@0, result: _<@1, entity: _<@2, showNoDetections: _<@3, showNoInfo: _<@4,
  
  // Key limit check
  result@`__keyLimitReached` ? { entity, data: undefined } : {
    data: result@`data`,
    attributes: data@`attributes`,
    lastAnalysisStats: attributes@`last_analysis_stats`,
    totalResults: lastAnalysisStats values sum,
    totalMalicious: lastAnalysisStats@`malicious`,
    
    // No data check
    !result | !data | !totalResults ?
      showNoInfo ? { entity, data: { summary: [`has not seen or scanned`], details: { noInfoMessage: true } } }
      : { entity, data: undefined }
    : !totalMalicious & !showNoDetections ? { entity, data: undefined } : {
      // Scan results
      scans: attributes@`last_analysis_results` ${
        { scanResult: _<@1, scanName: _<@0,
          name: scanName,
          detected: scanResult@`category` = `malicious`,
          result: !scanResult@`result` & scanResult@`category` = `type-unsupported` 
            ? `type-unsupported`
            : [`clean`, `suspicious`, `malware`, `malicious`, `unrated`] >< scanResult@`result`
              ? scanResult@`result` "^_    // capitalize
              : scanResult@`result`
        }
      },
      
      verdict: attributes@[`gti_assessment`, `verdict`, `value`] replace `VERDICT_` `` "^_,
      coreLink: `https://www.virustotal.com/gui/{data@`type` replace `_` `-`}/{data@`id`}`,
      detailsTab: {DETAILS_FORMATS@type} getDetailFields attributes,
      
      { entity,
        data: {
          summary: [`Verdict: {verdict}`],
          details: {
            type,
            detectionsLink: `{coreLink}/detection`,
            relationsLink: `{coreLink}/relations`,
            detailsLink: `{coreLink}/details`,
            communityLink: `{coreLink}/community`,
            behaviorLink: `{coreLink}/behavior`,
            total: totalResults,
            reputation: attributes@`reputation`,
            scan_date: new Date(attributes@`last_modification_date` * 1000),
            positives: totalMalicious,
            positiveScans: scans ?{ _<@`detected` } sort { _<@`result` } desc,
            names: attributes@`names`,
            negativeScans: scans ?{ !(_<@`detected`) } sort { _<@`result` } desc,
            detailsTab,
            tags: attributes@`tags`,
            gtiAssessment: attributes@`gti_assessment`,
            verdict
          }
        }
      }
    }
  }
}
```

### 10. `DETAILS_FORMATS` + `getDetailFields` (lines 1630-1725)
```lr
// Config-driven detail fields — stays as data structure
DETAILS_FORMATS: {
  file: [
    { key: `Basic Properties`, isTitle: true },
    { key: `File type`, path: `type_description` },
    { key: `File size`, path: `size`, transform: { _< / 1049295 / 0.01 * 0.01 } & ` MB ({_<} bytes)` },
    { key: `MD5`, path: `md5` },
    { key: `SHA-1`, path: `sha1` },
    { key: `SHA-256`, path: `sha256` }
    // ... more fields
  ],
  url: [...],
  domain: [...],
  ip: [...]
}

// getDetailFields
{ fields: _<@0, attributes: _<@1,
  fields ${
    { field: _<,
      value: attributes@field@`path`,
      transformedValue: field@`transform` ? {value} field@`transform` : value,
      { ...field, value & { value: transformedValue } | undefined }
    }
  }
}
```

### 11. `getWhois` (lines 1839-1890) — Historical whois
```lr
{ entity: _<@0, options: _<@1,
  entity@`isIP` | entity@`isDomain` ? {
    type: entity@`isIP` ? `ip` : `domain`,
    result: // HTTP interop — request historical_whois
    result@`data` ?
      result@`data` ${
        { whois: _<,
          whois@`attributes`@`last_updated` ?
          {
            last_updated: new Date(whois@`attributes`@`last_updated` * 1000),
            ...whois@`attributes`@`whois_map`
          } : undefined
        }
      } ?{ _< }
    : []
  } : []
}
```

### 12. `onMessage` (lines 2011-2070) — Message router
```lr
{ payload: _<@0, options: _<@1, callback: _<@2,
  entity: payload@`entity`,
  action: payload@`action`,
  
  action = `GET_RELATIONS` ? {entity} getRelations options callback
  : action = `GET_BEHAVIORS` ? {entity} getBehaviors options callback
  : action = `GET_WHOIS` ? {entity} getWhois options callback
  : action = `GET_THREATS` ? {
    threatResults: {entity} lookupThreatsAsync options,
    associationLink: `https://www.virustotal.com/gui/{entity getUiUrlByEntityType}`,
    callback(undefined, { threatResults, associationLink })
  }
  : action = `GET_REPORTS` ? {
    reportResults: {entity} lookupReportsAsync options,
    associationLink: `https://www.virustotal.com/gui/{entity getUiUrlByEntityType}`,
    callback(undefined, { reportResults, associationLink })
  }
  : undefined
}
```

### 13. `validateOptions` (lines 2072-2102)
```lr
{ userOptions: _<@0, callback: _<@1,
  apiKeyErrors: userOptions@`apiKey`@`value` ?= `text` & userOptions@`apiKey`@`value` # > 0
    ? []
    : [{ key: `apiKey`, message: `You must provide a GTI API key` }],
  
  maxHashErrors: userOptions@`maxHashesPerGroup`@`value` <= 0
    ? [{ key: `maxHashesPerGroup`, message: `Maximum number of hashes per lookup request must be greater than 0` }]
    : [],
  
  errors: apiKeyErrors + maxHashErrors,
  errors # = 0 ? scheduleJob(getAndCacheAllThreatActorNames) : undefined,
  callback(undefined, errors)
}
```

### 14. `getAndCacheAllThreatActorNames` (lines 2104-2136) — Paginated cursor fetch
```lr
// Recursive pagination with cursor
{ options: _<@0, cursor: _<@1, agg: _<@2,
  searchResult: // HTTP interop — GET /collections with cursor
  names: searchResult@`data` ${ _<@`attributes`@`name` },
  nextCursor: searchResult@`meta`@`cursor`,
  nextAgg: agg + names,
  
  nextCursor ? {options} getAndCacheAllThreatActorNames nextCursor nextAgg
  : nextAgg ~ sort { _< "' }
}
```

## Translation Order (recommended)

1. ✅ `constants.lr` — Simple data
2. ✅ `makeHumanReadable` — 1-line flow chain
3. ✅ `getEntityTypes` — Already in PenroScript-clean.md
4. `sortByEntityPresenceAndPrevalence` — Clean pipeline
5. `flattenWithPaths` — Recursive traversal
6. `groupByConfidence` — Complex reduce/groupBy
7. `formatThreatActors` — Medium complexity pipeline
8. `formatThreats` / `formatReports` — Same pattern as #7
9. `formatVulnerabilities` — Most complex pipeline
10. `_processLookupItem` — Scan result processing
11. `getDetailFields` + `DETAILS_FORMATS` — Config-driven
12. `doLookup` — Entity categorization + orchestration
13. `getWhois` / `getRelations` / `getBehaviors` — HTTP lookups
14. `onMessage` — Message router
15. `validateOptions` — Validation
16. `getAndCacheAllThreatActorNames` — Paginated fetch
