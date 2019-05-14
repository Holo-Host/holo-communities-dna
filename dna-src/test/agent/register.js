const queries = require('../queries')
module.exports = (scenario) => {

scenario.runTape('Can register a user and retrieve them again', async (t, {alice}) => {
    const result_ = await alice.callSync("graphql", "graphql", {
      query: "query { apiVersion }",
      variables: {}
    })
    console.log(result_)
    t.equal(result_.Ok, '{"apiVersion":"0.0.1"}')

    const register_result = await alice.callSync("graphql", "graphql", {
      query: queries.registerQuery,
      variables: {name: "wollum", avatarUrl: "//"}
    })
    console.log(register_result)
    t.equal(JSON.parse(register_result.Ok).registerUser.id, alice.agentId)

    const get_result = await alice.callSync("graphql", "graphql", {
      query: queries.getPeopleQuery,
      variables: {id: alice.agentId}
    })
    console.log(get_result)
    t.assert(JSON.parse(get_result.Ok).people.items.length > 0)
    t.assert(JSON.parse(get_result.Ok).people.items.filter((person) => {
      return person.id === alice.agentId
    }).length > 0)
  })
}
