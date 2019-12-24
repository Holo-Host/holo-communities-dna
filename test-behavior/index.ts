import * as R from 'ramda'

import { parameterizedStages, stochasticPiecewise, periodically, configBatchSimple, Batch } from '@holochain/tryorama-stress-utils'
import behaviors from './behaviors'
import { Orchestrator, tapeExecutor, groupPlayersByMachine, compose } from '@holochain/tryorama'
import { Player } from '@holochain/tryorama'
import { batcher, userName } from './config'

// Hook these up to stress config:

const endpoints = null
const numMachines = endpoints ? endpoints.length : 1
const conductorsPerMachine = 3
const instancesPerConductor = 2

const stageDuration = 10000  // duration of each stage (ms)
const periodInitial = 1000  // initial interval between behavior runs (ms)
const periodExpBase = 0.5  // subsequent scaling at each new stage
const stageLimit = 1  // number of stages to run. If 0, will run until failure.

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
      period: t => periodInitial * Math.pow(periodExpBase, t)
    }
  })
})

orchestrator.run()