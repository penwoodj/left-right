const { map, get, getOr, filter, flow, negate, isEmpty, size } = require('lodash/fp');
const { parallelLimit } = require('async');

const {
  requests: { createRequestWithDefaults }
} = require('polarity-integration-utils');
const config = require('../config/config');

// Workaround required because v1 of the utils library is expecting a request object
// on the config
config.request = {
  cert: '',
  key: '',
  passphrase: '',
  ca: '',
  proxy: ''
};

const NodeCache = require('node-cache');
const tokenCache = new NodeCache({
  //Tokens expire after 24 hours so expiring 1 hour before
  stdTTL: 23 * 60 * 60
});

const requestForAuth = createRequestWithDefaults({
  config,
  roundedSuccessStatusCodes: [200],
  requestOptionsToOmitFromLogsKeyPaths: ['form.client_secret']
});

const requestWithDefaults = createRequestWithDefaults({
  config,
  roundedSuccessStatusCodes: [200],
  useLimiter: true,
  requestOptionsToOmitFromLogsKeyPaths: ['authorization', 'query'],
  preprocessRequestOptions: async ({ options, query }) => {
    const token = await getToken(options);

    return {
      method: 'POST',
      url: `https://api.${options.apiRegion}.app.wiz.io/graphql`,
      headers: {
        accept: 'application/json',
        authorization: `Bearer ${token}`,
        'content-type': 'application/json'
      },
      body: JSON.stringify({ query })
    };
  },
  postprocessRequestResponse: async (response) => {
    const bodyErrors = flow(get('body'), JSON.parse, get('errors'))(response);
    if (size(bodyErrors)) {
      const bodyError = new Error(get('0.message', bodyErrors));
      bodyError.status = response.status;

      throw bodyError;
    }

    return {
      /* 
      This is required as if you add `json: true` the request fails on this API.
      I've also found if you capitalize `content-type` to `Content-Type` it fails
      so manual parsing the response body is required.
    */
      ...response,
      body: JSON.parse(response.body)
    };
  },
  postprocessRequestFailure: (error) => {
    error.message = `${error.message} - (${error.status}) | ${error.description}`;

    throw error;
  }
});

const getToken = async (options) => {
  const tokenCacheKey = options.clientId + options.clientSecret;
  const cachedToken = tokenCache.get(tokenCacheKey);
  if (cachedToken) return cachedToken;

  const token = get(
    'body.access_token',
    await requestForAuth({
      method: 'POST',
      url: `https://${options.authTokenDomain}/oauth/token`,
      headers: {
        accept: 'application/json',
        'content-type': 'application/x-www-form-urlencoded'
      },
      form: {
        grant_type: 'client_credentials',
        audience: 'wiz-api',
        client_id: options.clientId,
        client_secret: options.clientSecret
      },
      json: true
    })
  );

  tokenCache.set(tokenCacheKey, token);

  return token;
};

const createRequestsInParallel =
  (requestWithDefaults) =>
  async (
    requestsOptions,
    responseGetPath,
    limit = 10,
    onlyReturnPopulatedResults = true
  ) => {
    const unexecutedRequestFunctions = map(
      ({ entity, ...requestOptions }) =>
        async () => {
          const response = await requestWithDefaults(requestOptions);
          const result = responseGetPath ? get(responseGetPath, response) : response;
          return entity ? { entity, result } : result;
        },
      requestsOptions
    );

    const results = await parallelLimit(unexecutedRequestFunctions, limit);

    return onlyReturnPopulatedResults
      ? filter(
          flow((result) => getOr(result, 'result', result), negate(isEmpty)),
          results
        )
      : results;
  };

const requestsInParallel = createRequestsInParallel(requestWithDefaults);

module.exports = {
  requestWithDefaults,
  requestsInParallel
};
