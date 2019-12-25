const _ = require('lodash')
const path = require('path')
const { Config } = require('@holochain/tryorama')

const dnaPath = path.join(__dirname, '../dist/holo-communities-dna.dna.json')
const dna = Config.dna(dnaPath, 'app')

let network;

if (process.env.HC_TRANSPORT_CONFIG) {
  network=require(process.env.HC_TRANSPORT_CONFIG)
  console.log("setting network from:"+process.env.HC_TRANSPORT_CONFIG)
} else {
  console.log("setting network from:"+process.env.NETWORK_TYPE)
  const networkType = process.env.NETWORK_TYPE
  network =
    ( networkType === 'websocket'
  ? Config.network('websocket')

  : networkType === 'memory'
  ? Config.network('memory')

  : networkType === 'sim1h'
  ? {
    type: 'sim1h',
    dynamo_url: 'http://localhost:8000'
  }

  : networkType === 'sim2h'
  ? {
    type: 'sim2h',
    sim2h_url: 'wss://localhost:9000'
  }

  : (() => {throw new Error(`Unsupported network type: ${networkType}`)})()
    )
}

const logger = {
  type: 'debug',
  rules: {
    rules: [
      {
        exclude: true,
        pattern: '.*parity.*'
      },
      {
        exclude: true,
        pattern: '.*mio.*'
      },
      {
        exclude: true,
        pattern: '.*tokio.*'
      },
      {
        exclude: true,
        pattern: '.*hyper.*'
      },
      {
        exclude: true,
        pattern: '.*rusoto_core.*'
      },
      {
        exclude: true,
        pattern: '.*want.*'
      },
      {
        exclude: true,
        pattern: '.*rpc.*'
      }
    ]
  },
  state_dump: false
}

const commonConfig = { logger, network }

module.exports = {
  one: Config.gen({
      app: dna
    },
    commonConfig
  ),
  commonConfig
}
