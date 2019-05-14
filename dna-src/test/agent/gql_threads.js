const queries = require('../queries')
module.exports = (scenario) => {

scenario.runTape('Check for a non existent thread and then create it', async (t, {alice}) => {
    const result = await alice.callSync("graphql", "graphql", {
      query: queries.registerQuery,
      variables: {name: "wollum", avatarUrl: "//"}
    })

    const get_result = await alice.callSync("graphql", "graphql", {
      query: queries.getMessageThreadsQuery,
      variables: {first: 10, offset: 0}
    })

    console.log(get_result)
    t.equal(JSON.parse(get_result.Ok).me.messageThreads.total, 0) // no threads have been created yet

    // add a thread
    const add_result_str = await alice.callSync("graphql", "graphql", {
      query: queries.findOrCreateThreadQuery,
      variables: {participantIds: []}
    })
    console.log(add_result_str)
    const add_result = JSON.parse(add_result_str.Ok)
    t.equal(add_result.findOrCreateThread.id.length, 46) // thread was created and hash returned
    t.equal(add_result.findOrCreateThread.participants.length, 1) // thread was created and hash returned
    t.assert(add_result.findOrCreateThread.participants
      .map(person => person.id)
      .includes(alice.agentId)
    )

    const get_result_post = await alice.callSync("graphql", "graphql", {
      query: queries.getMessageThreadsQuery,
      variables: {first: 10, offset: 0}
    })

    console.log(get_result_post)
    t.equal(JSON.parse(get_result_post.Ok).me.messageThreads.total, 1) // created a single thread
  })
}
