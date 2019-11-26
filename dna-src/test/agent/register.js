const queries = require('../queries')
const { one } = require('../config')
module.exports = (scenario) => {

scenario('Can register a user and retrieve them again', async (s, t) => {
    const { alice } = await s.players({alice: one}, true)

    const result_ = await alice.callSync("app", "graphql", "graphql", {
      query: "query { apiVersion }",
      variables: {}
    })
    console.log(result_)
    t.equal(result_.Ok, '{"apiVersion":"0.0.1"}')

    const register_result = await alice.callSync("app", "graphql", "graphql", {
      query: queries.registerQuery,
      variables: {name: "wollum", avatarUrl: "//"}
    })
    console.log(register_result)
    t.equal(JSON.parse(register_result.Ok).registerUser.id, alice.info('app').agentAddress)

    const get_result = await alice.callSync("app", "graphql", "graphql", {
      query: queries.getPeopleQuery,
      variables: {id: alice.info('app').agentAddress}
    })
    console.log(get_result)
    t.assert(JSON.parse(get_result.Ok).people.items.length > 0)
    t.assert(JSON.parse(get_result.Ok).people.items.filter((person) => {
      return person.id === alice.info('app').agentAddress
    }).length > 0)
  })
}
