const path = require('path')
const tape = require('tape')

const { Diorama, tapeExecutor, backwardCompatibilityMiddleware } = require('@holochain/diorama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/dna-src.dna.json")
const dna = Diorama.dna(dnaPath, 'hylo')

const singleInstance = new Diorama({
  instances: {
    alice: dna,
  },
  debugLog: false,
  executor: tapeExecutor(require('tape')),
  middleware: backwardCompatibilityMiddleware,
})

const multiInstance = new Diorama({
  instances: {
    alice: dna,
    bob: dna,
  },
  debugLog: false,
  executor: tapeExecutor(require('tape')),
  middleware: backwardCompatibilityMiddleware,
})

// require('./agent/communities')(singleInstance.registerScenario)
// require('./agent/posts')(singleInstance.registerScenario)
// require('./agent/comments')(singleInstance.registerScenario)
require('./agent/threads')(singleInstance.registerScenario)
// require('./agent/messages')(singleInstance.registerScenario)
// require('./agent/people')(multiInstance.registerScenario)

singleInstance.run()
multiInstance.run()



// disabled graphql tests
// require('./agent/register')(singleAgentScenario)
// require('./agent/gql_comments')(singleAgentScenario)
// require('./agent/gql_threads')(singleAgentScenario)
// require('./agent/gql_messages')(singleAgentScenario)
// require('./agent/gql_posts')(singleAgentScenario)
// require('./agent/gql_communities')(singleAgentScenario)

// singleAgentscenario('Reference GraphQL schema matches the implementation', async (s, t, { alice }) => {

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
