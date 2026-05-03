``` TS
const getEntityTypes = (
  typesToGet: EntityType | EntityType[],
  entities: Entity[]
): Entity[] => {
  const lowerTypesToGet: string[] =
    typeof typesToGet === 'string' ? [toLower(typesToGet)] : map(toLower, typesToGet);

  const entitiesOfTypesToGet: Entity[] = filter((entity: Entity): boolean => {
    const lowerEntityTypes: string[] = map(toLower, entity.types);

    const entityTypesAreInTypesToGet: boolean = some(
      (typeToGet: string): boolean => lowerEntityTypes.includes(typeToGet),
      lowerTypesToGet
    );

    return entityTypesAreInTypesToGet;
  }, entities);

  return entitiesOfTypesToGet;
};

export default getEntityTypes;

``` PenroScript - file name "getEntityTypes.prsc"
{ typesToGet: _<@0, entities: _<@1,
  lowerTypesToGet: {
    typesToGet !?= `string`: [typesToGet],
    typesToGet ${'_}
  },

  entityTypesToGet: entities ?{
    lowerEntityTypes: entities@`types` ${'_},
    entityTypesAreInTypesToGet: lowerTypesToGet ?|{
      typeToGet: _<@0,
      lowerEntityTypes ?>< typeToGet
    },
    entityTypesAreInTypesToGet
  },

  entityTypesToGet
}

``` JS with Lodash FP
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

``` PenroScript anonymous function
{ threats: _<@[0,`threats'],
  maliciousThreatsCount: threats
    ?{ _<@[`AI Confidence Level`, `value`] = `malicious` }
    #,    
  threatClassifications: threats
    ${ _<@[`AI Confidence Level`, `value`] "^_}
    ~
    >< `, `
    { threatClassifications: _<,
      threatClassifications & `Threat Classifications: {threatClassifications}`
    },
  [] + maliciousThreatsCount + threatClassifications
}
Notes:
- Types: Operator, Hashmap, Array, String, Boolean, Number, Undefined
- Diatic operators are left hungry curried by default, but can be reversed
- Expressions are left to right evaluated, but can be grouped with parentheses
- {... endingNonKeyValueExpression } or {... _< ...} is an operator while {... key: value } is a JSON object
- All strings are template literals from an interface standpoint, but can be expressed as an operator if any the template expressions if the _< or _> operators are used
- Core language operators behavior is input type dependent
- Operators symbols can be overridden and extended
- Symbology is spacial &  and so   `asdf` "^   is toUpperCase while   `asdf` "^_   is capitalize
*/
