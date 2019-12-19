const { Orchestrator, tapeExecutor, singleConductor, localOnly, combine, callSync  } = require('@holochain/tryorama')

process.on('unhandledRejection', error => {
  console.error('got unhandledRejection:', error);
});


const networkType = process.env.NETWORK_TYPE || 'memory'
const middleware = 
  ( networkType === 'websocket'
  ? combine(tapeExecutor(require('tape')), localOnly, callSync)

  : networkType === 'sim1h'
  ? combine(tapeExecutor(require('tape')), localOnly, callSync)

  : networkType === 'sim2h'
  ? combine(tapeExecutor(require('tape')), localOnly, callSync)

  : networkType === 'memory'
  ? combine(tapeExecutor(require('tape')), localOnly, singleConductor, callSync)

  : (() => {throw new Error(`Unsupported network type: ${networkType}`)})()
)

const orchestrator = new Orchestrator({
  middleware,
  waiter: {
    softTimeout: 10000,
    hardTimeout: 20000
  }
})

// require('./single_agent/communities')(orchestrator.registerScenario)
// require('./single_agent/posts')(orchestrator.registerScenario)
// require('./single_agent/comments')(orchestrator.registerScenario)
// require('./single_agent/threads')(orchestrator.registerScenario)
// require('./single_agent/messages')(orchestrator.registerScenario)
// require('./single_agent/people')(orchestrator.registerScenario)

require('./multi_agent/posts')(orchestrator.registerScenario)

orchestrator.run().then(stats => {
  console.log("All done.")
})
