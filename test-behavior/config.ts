const path = require('path')
const { Config } = require('@holochain/tryorama')
import { commonConfig } from '../test/config'
import { configBatchSimple } from '@holochain/tryorama-stress-utils'

const dnaPath = path.join(__dirname, '../dist/holo-communities-dna.dna.json')
const chosenDna = Config.dna(dnaPath, 'app')

export const batcher = (numConductors, instancesPerConductor) => configBatchSimple(
  numConductors,
  instancesPerConductor,
  chosenDna,
  commonConfig
)

export const userName = (player, instance) => `user-${player.name}-${instance.id}`