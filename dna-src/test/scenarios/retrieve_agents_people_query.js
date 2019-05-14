const queries = require('../queries')

module.exports = (scenario) => {
  scenario.runTape('Check for a non existent thread and then create it', async (t, {alice, bob}) => {
    
    register(alice, 'alice')
    register(bob, 'bob')

    t.equal(await checkFor(alice, 'alice'), 1, 'alice can see alice')
    t.equal(await checkFor(alice, 'bob'), 1, 'alice can see bob')

    t.equal(await checkFor(bob, 'bob'), 1, 'bob can see bob')
    t.equal(await checkFor(bob, 'alice'), 1, 'bob can see alice')

  })
}


const register = async (agent, name) => {
  return agent.callSync("graphql", "graphql", {
    query: queries.registerQuery,
    variables: {name, avatarUrl: "//"}
  })
}

const checkFor = async (agent, name) => {
    const result = await agent.callSync("graphql", "graphql", {
      query: queries.getPeopleQuery,
      variables: {}
    })
    console.log(result)
    return JSON.parse(result.Ok).people.items.filter(p => p.name === name).length
}