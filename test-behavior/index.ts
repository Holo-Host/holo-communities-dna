import * as R from 'ramda'

import { one } from '../test/config'
import { parameterizedStages, stochasticPiecewise, periodically } from '@holochain/tryorama-stress-utils'
import behaviors from './behaviors'
import { Orchestrator } from '@holochain/tryorama'
import { Player } from '@holochain/tryorama'


const N = 5

const orchestrator = new Orchestrator()

orchestrator.registerScenario('behavior tests', async (s, t) => {

  const init = async () => {
    const configs = R.repeat(one, N)
    const players: Array<Player> = Object.values(await s.players(configs, true))
    const makeUser = name => ({
      name,
      avatar_url: `${name}.jpg`
    })
    Promise.all(
      players.map(async (player, i) => {
        const name = `player-${i}`
        const result = await player.call("app", 'people', 'register_user', makeUser(name))
        t.ok(result.Ok)
      })
    )
    return players
  }

  await parameterizedStages<Array<Player>>({
    init,
    stage: async (players, {period}) => {
      const behaviorRunners = Object.entries(behaviors).map(([name, runner]) => () => {
        console.log(`Running behavior ${name}`)
        return runner(s, t, players)
      })
      const randomBehavior = stochasticPiecewise(behaviorRunners.map(runner => [runner, 0.5]))
      await periodically({period, duration: 10000, awaitAll: true}, () => Promise.resolve(randomBehavior()))

      return players
    },
    fail: s.fail,
    failHandler: s.onFail,
    stageLimit: 1,
    parameters: {
      period: t => 1000 * Math.pow(0.5, t)
    }
  })
})

orchestrator.run()