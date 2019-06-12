import React, { Component } from 'react'
import GraphiQL from 'graphiql'
import { connect } from '@holochain/hc-web-client'
import 'graphiql/graphiql.css'

// Uses conductor configured interface when in production
const WS_INTERFACE_URI = process.env.NODE_ENV === 'development'
  ? 'ws://localhost:3400'
  : null
const GRAPHQL_ZOME_CALL_OBJECT = {
  'instance_id': 'hylo',
  'zome': 'graphql',
  'function': 'graphql'
}
var callGraphqlZome = null

async function initOrGetCallGraphqlZome () {
  if (callGraphqlZome) return callGraphqlZome
  try {
    const wsClient = await connect(WS_INTERFACE_URI)
    const { callZome } = wsClient

    callGraphqlZome = callZome(
      GRAPHQL_ZOME_CALL_OBJECT['instance_id'],
      GRAPHQL_ZOME_CALL_OBJECT['zome'],
      GRAPHQL_ZOME_CALL_OBJECT['function']
    )

    return callGraphqlZome
  } catch (error) {
    console.log('👎 Error connecting to websocket interface', error)
  }
}

async function graphQLFetcher(graphQLParams) {
  callGraphqlZome = await initOrGetCallGraphqlZome()

  graphQLParams.variables = graphQLParams.variables || {}

  const unparsedResult = await callGraphqlZome(graphQLParams)
  const resultJSON = JSON.parse(unparsedResult)

  return resultJSON.Ok
    ? { data: JSON.parse(resultJSON.Ok) }
    : resultJSON
}

class App extends Component {
  render() {
    return (
      <div style={{ height: '100vh' }}>
        <GraphiQL fetcher={graphQLFetcher} />
      </div>
    )
  }
}

export default App
