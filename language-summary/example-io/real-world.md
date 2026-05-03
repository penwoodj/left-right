# Real-World Examples

Practical applications from ServiceNow integration, ETL patterns, data transformations, and real-world use cases in Left-Right.

## getResultForThisEntity

**Left-Right:**
```left-right
getResultForThisEntity: { entity: _<, options: _>,
  entityType: entity@`type`,
  queryFields: options@`queryFields`,

  entityType = `custom` & {
    customType: entity@`types`@0,
    result: queryFields?{ _<@`name` = customType }
      ${ _<@`query` }
  } | {
    result: queryFields?{ _<@`name` = entityType }
      ${ _<@`query` }
  }
}
```

**JavaScript:**
```javascript
const getResultForThisEntity = (entity, options) => {
  const entityType = entity.type;
  const queryFields = options.queryFields;

  if (entityType === `custom`) {
    const customType = entity.types[0];
    return queryFields
      .filter(field => field.name === customType)
      .map(field => field.query);
  } else {
    return queryFields
      .filter(field => field.name === entityType)
      .map(field => field.query);
  }
};
```

**Rust:**
```rust
fn get_result_for_this_entity(
  entity: serde_json::Value,
  options: serde_json::Value
) -> Vec<String> {
  let entity_type = entity["type"].as_str().unwrap();
  let query_fields = options["queryFields"].as_array().unwrap();

  if entity_type == `custom` {
    let custom_type = entity["types"].as_array().unwrap()[0].as_str().unwrap();
    query_fields
      .into_iter()
      .filter(|field| field["name"].as_str().unwrap() == custom_type)
      .map(|field| field["query"].as_str().unwrap().to_string())
      .collect()
  } else {
    query_fields
      .into_iter()
      .filter(|field| field["name"].as_str().unwrap() == entity_type)
      .map(|field| field["query"].as_str().unwrap().to_string())
      .collect()
  }
}
```

**Explanation:** Entity-specific query generation. Filter query fields by entity type, handle custom types specially. ServiceNow integration pattern.

## getKustoQueryResults

**Left-Right:**
```left-right
getKustoQueryResults: { query: _<, results: _<,
  parsed: results?{ _<@`status` = `success` },

  validData: parsed?{ _<@`data` | 0 },

  formatted: validData${ _<@`rows` }?{ _<@`value` | 0 }
}
```

**JavaScript:**
```javascript
const getKustoQueryResults = (query, results) => {
  const parsed = results.filter(result => result.status === `success`);
  const validData = parsed.filter(result => result.data || 0);
  const formatted = validData.map(data => data.rows).filter(row => row.value || 0);
  return formatted;
};
```

**Rust:**
```rust
fn get_kusto_query_results(
  query: String,
  results: serde_json::Value
) -> Vec<serde_json::Value> {
  let parsed = results
    .as_array()
    .unwrap()
    .into_iter()
    .filter(|result| result["status"].as_str().unwrap() == `success`)
    .collect::<Vec<_>>();

  let valid_data = parsed
    .into_iter()
    .filter(|result| !result["data"].is_null())
    .collect::<Vec<_>>();

  valid_data
    .into_iter()
    .map(|data| data["rows"].clone())
    .filter(|row| !row["value"].is_null())
    .collect()
}
```

**Explanation:** Kusto/Azure Data Explorer query processing. Filter successful responses, validate data presence, extract rows.

## mapObject

**Left-Right:**
```left-right
mapObject: { obj: _<, transform: _<,
  keys: obj@0... ,

  mapped: keys${ key =>
    transform(key, obj@key)
  }
}
```

**JavaScript:**
```javascript
const mapObject = (obj, transform) => {
  const keys = Object.keys(obj);

  const mapped = keys.map(key =>
    transform(key, obj[key])
  );

  return mapped;
};
```

**Rust:**
```rust
fn map_object(
  obj: serde_json::Map<String, serde_json::Value>,
  transform: fn(&str, &serde_json::Value) -> serde_json::Value
) -> Vec<serde_json::Value> {
  let keys: Vec<&str> = obj.keys().map(|k| k.as_str()).collect();

  keys
    .into_iter()
    .map(|key| transform(key, obj.get(key).unwrap()))
    .collect()
}
```

**Explanation:** Generic object transformation. Map over object keys, apply transformation function to each key-value pair.

## getSetCookies

**Left-Right:**
```left-right
getSetCookies: { headers: _<,
  setCookies: headers@`set-cookie`,

  cookies: setCookies${ cookie =>
    {
      name: cookie@0... `=`,
      value: cookie@`=...`,
      expires: cookie@`expires`,
      domain: cookie@`domain`
    }
  }
}
```

**JavaScript:**
```javascript
const getSetCookies = (headers) => {
  const setCookies = headers[`set-cookie`];

  const cookies = setCookies.map(cookie => {
    const [nameValue] = cookie.split('=');
    return {
      name: nameValue[0],
      value: cookie.split('=')[1],
      expires: cookie.match(/expires=([^;]+)/)?.[1],
      domain: cookie.match(/domain=([^;]+)/)?.[1]
    };
  });

  return cookies;
};
```

**Rust:**
```rust
fn get_set_cookies(headers: serde_json::Value) -> Vec<serde_json::Value> {
  let set_cookies = headers[`set-cookie`].as_array().unwrap();

  set_cookies
    .into_iter()
    .map(|cookie| {
      let cookie_str = cookie.as_str().unwrap();
      let parts: Vec<&str> = cookie_str.split('=').collect();

      serde_json::json!({
        "name": parts.get(0).unwrap_or(&""),
        "value": parts.get(1).unwrap_or(&""),
        `expires`: cookie_str.match("expires=([^;]+)").map(|m| m[1].to_string()).unwrap_or_default(String::new()),
        `domain`: cookie_str.match("domain=([^;]+)").map(|m| m[1].to_string()).unwrap_or_default(String::new())
      })
    })
    .collect()
}
```

**Explanation:** HTTP cookie parsing. Extract cookie attributes from Set-Cookie headers.

## allCombinations

**Left-Right:**
```left-right
allCombinations: { arrays: _<,
  first: arrays@0,

  combinations: first${ item =>
    arrays@1${ other =>
      [item, other]
    }
  }
}
```

**JavaScript:**
```javascript
const allCombinations = (arrays) => {
  const first = arrays[0];

  const combinations = first.flatMap(item =>
    arrays[1].map(other =>
      [item, other]
    )
  );

  return combinations;
};
```

**Rust:**
```rust
fn all_combinations(arrays: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  let first = &arrays[0];
  let second = &arrays[1];

  first
    .into_iter()
    .flat_map(|item| {
      second
        .into_iter()
        .map(|other| vec![*item, *other])
        .collect()
    })
    .collect()
}
```

**Explanation:** Generate all combinations between two lists. Cartesian product for pairing elements.

## millisToHoursMinutesAndSeconds

**Left-Right:**
```left-right
millisToHoursMinutesAndSeconds: { millis: _<,
  seconds: millis / 1000,

  minutes: seconds / 60,
  remainingSeconds: seconds % 60,

  hours: minutes / 60,
  remainingMinutes: minutes % 60,

  result: `Hours: {hours}, Minutes: {remainingMinutes}, Seconds: {remainingSeconds}`
}
```

**JavaScript:**
```javascript
const millisToHoursMinutesAndSeconds = (millis) => {
  const seconds = millis / 1000;

  const minutes = Math.floor(seconds / 60);
  const remainingSeconds = seconds % 60;

  const hours = Math.floor(minutes / 60);
  const remainingMinutes = minutes % 60;

  const result = `Hours: ${hours}, Minutes: ${remainingMinutes}, Seconds: ${remainingSeconds}`;

  return result;
};
```

**Rust:**
```rust
fn millis_to_hours_minutes_and_seconds(millis: i64) -> String {
  let seconds = millis / 1000;

  let minutes = seconds / 60;
  let remaining_seconds = seconds % 60;

  let hours = minutes / 60;
  let remaining_minutes = minutes % 60;

  format!(
    "Hours: {}, Minutes: {}, Seconds: {}",
    hours, remaining_minutes, remaining_seconds
  )
}
```

**Explanation:** Time conversion utility. Convert milliseconds to hours, minutes, seconds with proper modulo arithmetic.

## getHighestLowest

**Left-Right:**
```left-right
getHighestLowest: { values: _<,
  sorted: values${ _< } << { _< > _> },

  highest: sorted@0,
  lowest: sorted@-1,

  result: {
    highest: highest,
    lowest: lowest,
    range: highest - lowest
  }
}
```

**JavaScript:**
```javascript
const getHighestLowest = (values) => {
  const sorted = values.slice().sort((a, b) => b - a);

  const highest = sorted[0];
  const lowest = sorted[sorted.length - 1];

  const result = {
    highest,
    lowest,
    range: highest - lowest
  };

  return result;
};
```

**Rust:**
```rust
fn get_highest_lowest(mut values: Vec<i32>) -> serde_json::Value {
  values.sort_by(|a, b| b.cmp(a));

  let highest = values[0];
  let lowest = values[values.len() - 1];

  serde_json::json!({
    "highest": highest,
    "lowest": lowest,
    "range": highest - lowest
  })
}
```

**Explanation:** Find min/max values. Sort descending for O(n log n), extract first and last.

## ServiceNow-Style Data Processing

### Incident Query Builder

**Left-Right:**
```left-right
buildIncidentQuery: { filters: _<,
  base: `incident?sysparm_query=`,

  queryParts: filters?{ _<@`active` }
    ${ _<@`condition` },

  fullQuery: base & queryParts >< `^NQ`,
  encoded: fullQuery${ _<@`encodeURIComponent` }
}
```

**JavaScript:**
```javascript
const buildIncidentQuery = (filters) => {
  const base = `incident?sysparm_query=`;

  const queryParts = filters
    .filter(f => f.active)
    .map(f => f.condition);

  const fullQuery = base + queryParts.join(`^NQ`);
  const encoded = fullQuery.map(q => encodeURIComponent(q));

  return encoded;
};
```

**Rust:**
```rust
use percent_encoding;

fn build_incident_query(filters: Vec<serde_json::Value>) -> String {
  let base = "incident?sysparm_query=";

  let query_parts: Vec<String> = filters
    .into_iter()
    .filter(|f| f["active"].as_bool().unwrap())
    .map(|f| f["condition"].as_str().unwrap().to_string())
    .collect();

  let full_query = format!("{}{}", base, query_parts.join("^NQ"));
  let encoded = full_query
    .chars()
    .map(|c| percent_encoding::utf8_percent_encode(c))
    .collect();

  encoded
}
```

**Explanation:** ServiceNow incident query construction. Filter active conditions, join with ServiceNow operator, URL encode.

### Table Data Aggregation

**Left-Right:**
```left-right
aggregateTableData: { results: _<,
  grouped: results@`tableQueryData`${ _<@`category` }

  aggregated: grouped${ category =>
    {
      category: category@0,
      count: category@1$#,
      items: category@1${ _<@`name` }
    }
  }
}
```

**JavaScript:**
```javascript
const aggregateTableData = (results) => {
  const grouped = Object.groupBy(results.tableQueryData, item => item.category);

  const aggregated = Object.entries(grouped).map(([category, items]) => ({
    category: category,
    count: items.length,
    items: items.map(item => item.name)
  }));

  return aggregated;
};
```

**Rust:**
```rust
use std::collections::HashMap;

fn aggregate_table_data(results: serde_json::Value) -> Vec<serde_json::Value> {
  let table_data = results["tableQueryData"].as_array().unwrap();

  let mut grouped: HashMap<&str, Vec<serde_json::Value>> = HashMap::new();

  for item in table_data {
    let category = item["category"].as_str().unwrap();
    grouped.entry(category).or_insert(Vec::new()).push(item.clone());
  }

  grouped
    .into_iter()
    .map(|(category, items)| {
      serde_json::json!({
        "category": category,
        "count": items.len(),
        "items": items.iter().map(|i| i["name"].clone()).collect::<Vec<_>>()
      })
    })
    .collect()
}
```

**Explanation:** ServiceNow table data aggregation. Group by category, count items, extract names.

### User Permission Check

**Left-Right:**
```left-right
checkUserPermissions: { user: _<, required: _<,
  userRoles: user@`roles`,

  hasPermission: required?{ permission =>
    userRoles?|{ role =>
      role@`permissions`?|{ perm =>
        perm@`name` = permission
      }
    }
  },

  authorized: hasPermission | false
}
```

**JavaScript:**
```javascript
const checkUserPermissions = (user, required) => {
  const userRoles = user.roles;

  const hasPermission = required.every(permission =>
    userRoles.some(role =>
      role.permissions.some(perm =>
        perm.name === permission
      )
    )
  );

  return hasPermission || false;
};
```

**Rust:**
```rust
fn check_user_permissions(
  user: serde_json::Value,
  required: Vec<String>
) -> bool {
  let user_roles = user["roles"].as_array().unwrap();

  let has_permission = required
    .iter()
    .all(|permission| {
      user_roles
        .iter()
        .any(|role| {
          let permissions = role["permissions"].as_array().unwrap();
          permissions
            .iter()
            .any(|perm| perm["name"].as_str().unwrap() == permission)
        })
    });

  has_permission || false
}
```

**Explanation:** ServiceNow RBAC permission check. Nested iteration over user roles and permissions.

### Asset Discovery Pipeline

**Left-Right:**
```left-right
discoverAssets: { entity: _<,
  assetQuery: `asset?sysparm_query=^name={entity@`name`}CONTAINS`,

  assetResults: assetQuery >> query,

  enriched: assetResults${ asset =>
    asset + {
      discovered: true,
      source: `discovery`,
      timestamp: Date.now
    }
  }
}
```

**JavaScript:**
```javascript
const discoverAssets = (entity) => {
  const assetQuery = `asset?sysparm_query=^name=${entity.name}CONTAINS`;

  const assetResults = query(assetQuery);

  const enriched = assetResults.map(asset => ({
    ...asset,
    discovered: true,
    source: `discovery`,
    timestamp: Date.now()
  }));

  return enriched;
};
```

**Rust:**
```rust
use chrono;

fn discover_assets(entity: serde_json::Value) -> Vec<serde_json::Value> {
  let entity_name = entity["name"].as_str().unwrap();
  let asset_query = format!("asset?sysparm_query=^name={}CONTAINS", entity_name);

  let asset_results = query(asset_query);

  asset_results
    .into_iter()
    .map(|mut asset| {
      let obj = asset.as_object_mut().unwrap();
      obj.insert("discovered".to_string(), serde_json::Value::Bool(true));
      obj.insert("source".to_string(), serde_json::json!("discovery"));
      obj.insert("timestamp".to_string(), serde_json::json!(chrono::Utc::now().timestamp()));
      serde_json::Value::Object(obj.clone())
    })
    .collect()
}
```

**Explanation:** ServiceNow CMDB asset discovery. Build CONTAINS query, fetch results, enrich with metadata.

### Validation Pipeline

**Left-Right:**
```left-right
validateIncident: { incident: _<,
  errors: [],

  validated: {
    !incident@`number` & errors + [`Incident number required`],
    !incident@`description` & errors + [`Description required`],
    !incident@`priority` ?= `number` & errors + [`Priority must be number`],
    ![`active`, `pending`, `resolved`] >< incident@`state` & errors + [`Invalid state`]
  },

  isValid: errors$# == 0,

  result: {
    incident: incident,
    isValid: isValid,
    errors: isValid | errors
  }
}
```

**JavaScript:**
```javascript
const validateIncident = (incident) => {
  let errors = [];

  if (!incident.number) errors.push(`Incident number required`);
  if (!incident.description) errors.push(`Description required`);
  if (typeof incident.priority !== `number`) errors.push(`Priority must be number`);
  if (![`active`, `pending`, `resolved`].includes(incident.state)) {
    errors.push(`Invalid state`);
  }

  const isValid = errors.length === 0;

  return {
    incident,
    isValid,
    errors: isValid ? errors : []
  };
};
```

**Rust:**
```rust
fn validate_incident(incident: serde_json::Value) -> serde_json::Value {
  let mut errors = Vec::<String>::new();

  if incident["number"].is_null() {
    errors.push(`Incident number required`.to_string());
  }
  if incident["description"].is_null() {
    errors.push(`Description required`.to_string());
  }
  if !matches!(incident["priority"], serde_json::Value::Number(_)) {
    errors.push(`Priority must be number`.to_string());
  }
  match incident["state"].as_str() {
    Some(s) if ["active", "pending", "resolved"].contains(&s) => {},
    _ => errors.push(`Invalid state`.to_string())
  }

  let is_valid = errors.is_empty();

  serde_json::json!({
    "incident": incident,
    "isValid": is_valid,
    "errors": if is_valid { vec![] } else { errors }
  })
}
```

**Explanation:** Multi-field validation with error collection. Accumulate all errors, determine validity, return structured result.

## Identity Element Behavior in Real-World Context

Left-Right's `+` operator handles identity elements gracefully: `undefined`, empty text, and empty lists act as neutral elements.

### Handling Missing Configuration Values

**Left-Right:**
```left-right
safeMergeConfig: { baseConfig: _<, overrideConfig: _<,
  merged: {
    timeout: baseConfig@`timeout` + overrideConfig@`timeout`,
    retries: baseConfig@`retries` + overrideConfig@`retries`,
    endpoint: baseConfig@`endpoint` + overrideConfig@`endpoint`,
    headers: baseConfig@`headers` + overrideConfig@`headers`
  }
}
```

**JavaScript:**
```javascript
const safeMergeConfig = (baseConfig, overrideConfig) => {
  return {
    timeout: baseConfig.timeout ?? overrideConfig.timeout,
    retries: baseConfig.retries ?? overrideConfig.retries,
    endpoint: baseConfig.endpoint ?? overrideConfig.endpoint,
    headers: baseConfig.headers ?? overrideConfig.headers
  };
};
```

**Rust:**
```rust
fn safe_merge_config(
  base_config: serde_json::Value,
  override_config: serde_json::Value
) -> serde_json::Value {
  serde_json::json!({
    "timeout": add_identity(base_config["timeout"].clone(), override_config["timeout"].clone()),
    "retries": add_identity(base_config["retries"].clone(), override_config["retries"].clone()),
    "endpoint": add_identity(base_config["endpoint"].clone(), override_config["endpoint"].clone()),
    "headers": add_identity(base_config["headers"].clone(), override_config["headers"].clone())
  })
}

fn add_identity(left: Value, right: Value) -> Value {
  match (left, right) {
    (Value::Undefined, r) => r,
    (l, Value::Undefined) => l,
    _ => add_lr(left, right)
  }
}
```

**Example Usage:**
```javascript
// Left-Right
safeMergeConfig({ timeout: 5000 }, {}) // → { timeout: 5000 }
safeMergeConfig({ endpoint: `` }, `https://api.example.com`) // → { endpoint: `https://api.example.com` }
safeMergeConfig({ headers: [] }, [`Authorization: Bearer token`]) // → { headers: [`Authorization: Bearer token`] }
```

### Accumulating Metrics with Safe Defaults

**Left-Right:**
```left-right
accumulateMetrics: { currentMetrics: _<, newMetrics: _<,
  total: {
    requests: currentMetrics@`requests` + newMetrics@`requests`,
    errors: currentMetrics@`errors` + newMetrics@`errors`,
    latency: currentMetrics@`latency` + newMetrics@`latency`,
    warnings: currentMetrics@`warnings` + newMetrics@`warnings`
  }
}
```

**JavaScript:**
```javascript
const accumulateMetrics = (currentMetrics, newMetrics) => {
  return {
    requests: (currentMetrics.requests ?? 0) + (newMetrics.requests ?? 0),
    errors: (currentMetrics.errors ?? 0) + (newMetrics.errors ?? 0),
    latency: (currentMetrics.latency ?? 0) + (newMetrics.latency ?? 0),
    warnings: (currentMetrics.warnings ?? 0) + (newMetrics.warnings ?? 0)
  };
};
```

**Example Usage:**
```javascript
// Left-Right
accumulateMetrics({ requests: 100, errors: 5 }, { requests: 50 }) // → { requests: 150, errors: 5 }
accumulateMetrics({ latency: 0 }, { latency: 123 }) // → { latency: 123 }
accumulateMetrics({}, { warnings: 3 }) // → { warnings: 3 }
```

### Building Query Strings with Safe Concatenation

**Left-Right:**
```left-right
buildQueryString: { baseParams: _<, additionalParams: _<,
  query: baseParams + additionalParams,
  encoded: query${ pair =>
    pair@0 & `=` & encodeURIComponent(pair@1)
  } >< `&`
}
```

**JavaScript:**
```javascript
const buildQueryString = (baseParams, additionalParams) => {
  const merged = baseParams ?? additionalParams;
  const query = Object.entries(merged).map(([key, value]) =>
    `${key}=${encodeURIComponent(value)}`
  ).join('&');

  return query;
};
```

**Example Usage:**
```javascript
// Left-Right
buildQueryString({ limit: 10 }, {}) // → `limit=10`
buildQueryString({}, { offset: 20, filter: `active` }) // → `offset=20&filter=active`
buildQueryString({ limit: 10 }, { offset: 20 }) // → `limit=10&offset=20`
```

### Processing Optional Data Streams

**Left-Right:**
```left-right
processDataStreams: { primaryStream: _<, secondaryStream: _<,
  combined: primaryStream + secondaryStream,

  processed: combined${ item =>
    {
      id: item@`id`,
      timestamp: item@`timestamp` ?? Date.now,
      value: item@`value` ?? 0,
      metadata: item@`metadata` ?? {}
    }
  }
}
```

**JavaScript:**
```javascript
const processDataStreams = (primaryStream, secondaryStream) => {
  const combined = primaryStream ?? secondaryStream;

  return combined.map(item => ({
    id: item.id,
    timestamp: item.timestamp ?? Date.now(),
    value: item.value ?? 0,
    metadata: item.metadata ?? {}
  }));
};
```

**Rust:**
```rust
fn process_data_streams(
  primary_stream: Vec<serde_json::Value>,
  secondary_stream: Vec<serde_json::Value>
) -> Vec<serde_json::Value> {
  let combined = if primary_stream.is_empty() {
    secondary_stream
  } else {
    primary_stream
  };

  combined
    .into_iter()
    .map(|item| {
      serde_json::json!({
        "id": item["id"],
        "timestamp": item["timestamp"].as_i64().unwrap_or_else(|_| chrono::Utc::now().timestamp()),
        "value": item["value"].as_i64().unwrap_or(0),
        "metadata": if item["metadata"].is_null() { serde_json::json!({}) } else { item["metadata"].clone() }
      })
    })
    .collect()
}
```

**Example Usage:**
```javascript
// Left-Right
processDataStreams([{ id: 1, value: 100 }], []) // → [{ id: 1, value: 100, timestamp: ..., metadata: {} }]
processDataStreams([], [{ id: 2, value: 200 }]) // → [{ id: 2, value: 200, timestamp: ..., metadata: {} }]
processDataStreams([{ id: 1 }], [{ id: 2, value: 200 }]) // → [{ id: 1, value: 0 }, { id: 2, value: 200 }]
```

### Aggregating Partial Responses

**Left-Right:**
```left-right
aggregatePartialResponses: { responses: _<,
  totals: responses${ response =>
    response@`data` ?? {}
  }${ {
    sum: _<@`value` + 0,
    count: _<@`count` + 0,
    errors: _<@`errors` + []
  }
}
```

**JavaScript:**
```javascript
const aggregatePartialResponses = (responses) => {
  return responses.map(response => response.data ?? {}).reduce((acc, data) => ({
    sum: acc.sum + (data.value ?? 0),
    count: acc.count + (data.count ?? 0),
    errors: [...acc.errors, ...(data.errors ?? [])]
  }), { sum: 0, count: 0, errors: [] });
};
```

**Example Usage:**
```javascript
// Left-Right
aggregatePartialResponses([
  { data: { value: 100, count: 1 } },
  { data: { count: 2 } },
  { data: { value: 50, errors: [`timeout`] } }
])
// → { sum: 150, count: 3, errors: [`timeout`] }
```

**Explanation:** Identity elements (`undefined`, ``, `[]`) act as neutral values for `+`. This enables graceful handling of missing data, optional parameters, and partial responses without explicit null checks. The behavior mirrors JavaScript's null coalescing but integrated into operator semantics.
