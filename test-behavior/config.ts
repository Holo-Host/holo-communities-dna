const path = require('path')
const { Config } = require('@holochain/tryorama')
import { commonConfig } from '../test/config'
import { configBatchSimple } from '@holochain/tryorama-stress-utils'

const dnaPath = path.join(__dirname, '../dist/holo-communities-dna.dna.json')
const dnaUri = 'https://github.com/Holo-Host/holo-communities-dna/releases/download/holoscape-bundle-v0.0.4/holo-communities.dna.json'

export const batcher = (isRemote, numConductors, instancesPerConductor) => configBatchSimple(
  numConductors,
  instancesPerConductor,
  Config.dna(isRemote ? dnaUri : dnaPath, 'app'),
  commonConfig
)

export const userName = (player, instance) => `user-${player.name}-${instance.id}`
