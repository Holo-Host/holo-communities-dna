const path = require('path')
const { Config, Scenario } = require('@holochain/holochain-nodejs')
Scenario.setTape(require('tape'))
const dnaPath = path.join(__dirname, "../dist/dna-src.dna.json")
const dna = Config.dna(dnaPath, 'hylo-messenger')

const agentAlice = Config.agent('alice')
const instanceAlice = Config.instance(agentAlice, dna)
const singleAgentScenario = new Scenario([instanceAlice], { debugLog: true })

require('./agent/community')(singleAgentScenario)
require('./agent/posts')(singleAgentScenario)
require('./agent/comments')(singleAgentScenario)
require('./agent/threads')(singleAgentScenario)
require('./agent/messages')(singleAgentScenario)

// disabled graphql tests
// require('./agent/register')(singleAgentScenario)
// require('./agent/gql_comments')(singleAgentScenario)
// require('./agent/gql_threads')(singleAgentScenario)
// require('./agent/gql_messages')(singleAgentScenario)
// require('./agent/gql_posts')(singleAgentScenario)
// require('./agent/gql_communities')(singleAgentScenario)

// const agentBob = Config.agent('bob')
// const instanceBob = Config.instance(agentBob, dna)
// const twoAgentScenario = new Scenario([instanceAlice, instanceBob], { debugLog: true })

// singleAgentScenario.runTape('Reference GraphQL schema matches the implementation', async (t, {alice}) => {

// 	const fs = require('fs');
// 	const { buildSchema, buildClientSchema, introspectionQuery } = require('graphql');
// 	require('graphql-schema-utils');

// 	const referenceSchemaDef = fs.readFileSync('../schema.graphql', "utf8");
// 	const referenceSchema = buildSchema(referenceSchemaDef);

// 	const getSchemaResult = await alice.callSync("graphql", "graphql", {
// 	  	query: introspectionQuery,
// 		variables: {}
// 	})
// 	const implSchemaDef = JSON.parse(getSchemaResult.Ok)
// 	const implSchema = buildClientSchema(implSchemaDef)

// 	const diffs = referenceSchema.diff(implSchema).filter(d => !d.backwardsCompatible)

// 	if(diffs.length > 0) {
// 		console.log(diffs)
// 	}

// 	t.equal(diffs.length, 0)
// })


// require('./scenarios/retrieve_agents_people_query')(twoAgentScenario)
