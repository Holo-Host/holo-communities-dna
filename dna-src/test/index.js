const path = require('path')
const { Config, Scenario } = require('@holochain/holochain-nodejs')
Scenario.setTape(require('tape'))
const dnaPath = path.join(__dirname, "../dist/dna-src.dna.json")
const dna = Config.dna(dnaPath, 'hylo-messenger')

const agentAlice = Config.agent('alice')
const instanceAlice = Config.instance(agentAlice, dna)
const singleAgentScenario = new Scenario([instanceAlice], { debugLog: true })

const agentBob = Config.agent('bob')
const instanceBob = Config.instance(agentBob, dna)
const twoAgentScenario = new Scenario([instanceAlice, instanceBob], { debugLog: true })


// require('./agent/register')(singleAgentScenario)
// require('./agent/threads')(singleAgentScenario)
// require('./agent/messages')(singleAgentScenario)
// require('./agent/comments')(singleAgentScenario)
// require('./agent/posts')(singleAgentScenario)
// require('./agent/community')(singleAgentScenario)
//
// require('./agent/gql_comments')(singleAgentScenario)
// require('./agent/gql_threads')(singleAgentScenario)
// require('./agent/gql_messages')(singleAgentScenario)
// require('./agent/gql_posts')(singleAgentScenario)
require('./agent/gql_communitys')(singleAgentScenario)
//
// require('./scenarios/retrieve_agents_people_query')(twoAgentScenario)
