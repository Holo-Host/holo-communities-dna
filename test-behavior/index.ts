import * as R from 'ramda'

import { parameterizedStages, stochasticPiecewise, periodically, configBatchSimple, Batch } from '@holochain/tryorama-stress-utils'
import behaviors from './behaviors'
import { Orchestrator, tapeExecutor, groupPlayersByMachine, compose } from '@holochain/tryorama'
import { Player } from '@holochain/tryorama'
import { batcher, userName } from './config'

const defaultConfig = {
    nodes: endpoints ? endpoints.length : 1,
    conductors: 1,
    instances: 10,
    endpoints: null,
    stageDuration: 10000,  // duration of each stage (ms)
    periodInitial: 1000,  // initial interval between behavior runs (ms)
    periodExpBase: 0.5,  // subsequent scaling at each new stage
    stageLimit: 1,  // number of stages to run. If 0, will run until failure.
    initialStage: 0,  // start at a higher stage
}

const runName = process.argv[2] || ""+Date.now()  // default exam name is just a timestamp
const config = process.argv[3] ? require(process.argv[3]) : defaultConfig

console.log(`Running behavior test id=${runName} with:\n`, config)

const endpoints = config.endpoints
const numMachines = config.nodes
const conductorsPerMachine = config.conductors
const instancesPerConductor = config.instances
const stageDuration = config.stageDuration  // duration of each stage (ms)
const periodInitial = config.periodInitial  // initial interval between behavior runs (ms)
const periodExpBase = config.periodExpBase  // subsequent scaling at each new stage
const stageLimit = config.stageLimit  // number of stages to run. If 0, will run until failure.
const initialStage = config.initialStage  // start at a higher stage

// Below this line should not need changes

const numConductors = numMachines * conductorsPerMachine

const middleware = endpoints
  ? compose(tapeExecutor(require('tape')), groupPlayersByMachine(endpoints, conductorsPerMachine))
  : undefined

const orchestrator = new Orchestrator({middleware})

orchestrator.registerScenario('behavior tests', async (s, t) => {

  const init = async () => {
    const configs = await batcher(numConductors, instancesPerConductor)
    // create and spawn some players
    const players: Array<Player> = Object.values(await s.players(configs, true))
    const makeUser = name => ({
      name,
      avatar_url: `${name}.jpg`
    })
    const batch = new Batch(players)
    // then register a user for each instance of each player
    await batch.mapInstances(async i => {
      const name = userName(i.player, i)
      const result = await i.call('people', 'register_user', makeUser(name))
      t.ok(result.Ok)
    })
    return players
  }

  const stage = async (players, {period}) => {
    const behaviorRunners = Object.entries(behaviors).map(([name, runner]) => () => {
      console.log(`Running behavior ${name}`)
      return runner(s, t, players)
    })
    // create piecewise function of behaviors where all branches are called with equal probability
    const randomBehavior = stochasticPiecewise(behaviorRunners.map(runner => [runner, 1]))
    // call a random behavior at regular intervals for a certain duration
    await periodically({period, duration: stageDuration, awaitAll: true}, () => Promise.resolve(randomBehavior()))

    return players
  }

  await parameterizedStages<Array<Player>>({
    init,
    stage,
    fail: s.fail,
    failHandler: s.onFail,
    stageLimit,
    parameters: {
      period: t => periodInitial * Math.pow(periodExpBase, t + initialStage)
    }
  })
})

orchestrator.run()
