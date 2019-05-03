import React, { Component } from 'react';
import 'graphiql/graphiql.css';

import GraphiQL from 'graphiql';
import fetch from 'isomorphic-fetch';


function graphQLFetcher(graphQLParams) {

  graphQLParams.variables = graphQLParams.variables || {}

  const payload = {
    instance_id: 'hylo-chat',
    zome: 'graphql',
    function: 'graphql',
    params: graphQLParams,
  }

  return fetch('http://localhost:4001', {
    method: 'post',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      jsonrpc: '2.0',
      id: 123,
      method: 'call',
      params: payload
    })
  })
  .then(response => response.json())
  .then(json => json.result)
  .then(result => {
    const resultJson = JSON.parse(result)
    return resultJson.Ok ? { data: JSON.parse(resultJson.Ok) } : resultJson
  })
}

class App extends Component {
  render() {
    return (
      <div style={{ height: '100vh' }}>
        <GraphiQL fetcher={graphQLFetcher} />
      </div>
    );
  }
}

export default App;
