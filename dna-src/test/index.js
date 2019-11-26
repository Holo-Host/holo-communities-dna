const { Orchestrator, tapeExecutor, singleConductor, localOnly, combine, callSync  } = require('@holochain/tryorama')

// This constant serves as a check that we haven't accidentally disabled scenario tests.
// Try to keep this number as close as possible to the actual number of scenario tests.
// (But never over)
const MIN_EXPECTED_SCENARIOS = 1

process.on('unhandledRejection', error => {
  console.error('got unhandledRejection:', error);
});


const networkType = process.env.APP_SPEC_NETWORK_TYPE
const middleware = 
  ( networkType === 'websocket'
  ? combine(tapeExecutor(require('tape')), localOnly, callSync)

  : networkType === 'sim1h'
  ? combine(tapeExecutor(require('tape')), localOnly, callSync)

  : networkType === 'sim2h'
  ? combine(tapeExecutor(require('tape')), localOnly, callSync)

  : networkType === 'memory'
  ? combine(tapeExecutor(require('tape')), localOnly, singleConductor, callSync)

  : (() => {throw new Error(`Unsupported memory type: ${networkType}`)})()
)

const orchestrator = new Orchestrator({
  middleware,
  waiter: {
    softTimeout: 10000,
    hardTimeout: 20000
  }
})

require('./agent/communities')(orchestrator.registerScenario)
require('./agent/posts')(orchestrator.registerScenario)
require('./agent/comments')(orchestrator.registerScenario)
require('./agent/threads')(orchestrator.registerScenario)
require('./agent/messages')(orchestrator.registerScenario)
require('./agent/people')(orchestrator.registerScenario)

orchestrator.run().then(stats => {
  console.log("All done.")
})
