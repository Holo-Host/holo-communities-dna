const queries = require('../queries')
const { one } = require('../config')

module.exports = (scenario) => {
  scenario('Check for a non existent thread and then create it', async (s, t) => {
    const { alice, bob } = await s.players({alice: one('alice'), bob:one('bob')}, true)

    register(alice, 'alice')
    register(bob, 'bob')

    t.equal(await checkFor(alice, 'alice'), 1, 'alice can see alice')
    t.equal(await checkFor(alice, 'bob'), 1, 'alice can see bob')

    t.equal(await checkFor(bob, 'bob'), 1, 'bob can see bob')
    t.equal(await checkFor(bob, 'alice'), 1, 'bob can see alice')

  })
}


const register = async (agent, name) => {
  return agent.app.callSync("app", "graphql", "graphql", {
    query: queries.registerQuery,
    variables: {name, avatarUrl: "//"}
  })
}

const checkFor = async (agent, name) => {
    const result = await agent.app.callSync("app", "graphql", "graphql", {
      query: queries.getPeopleQuery,
      variables: {}
    })
    console.log(result)
    return JSON.parse(result.Ok).people.items.filter(p => p.name === name).length
}
