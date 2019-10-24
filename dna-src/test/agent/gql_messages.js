const queries = require('../queries')
const { one } = require('../config')
module.exports = (scenario) => {

scenario('Can register a user and retrieve them again', async (s, t) => {
  const { alice } = await s.players({alice: one('alice')}, true)

    let register_response = await alice.callSync("app", "graphql", "graphql", {
      query: queries.registerQuery,
      variables: {name: "wollum", avatarUrl: "//"}
    })
    console.log(register_response)

    // add a thread
    const add_result = await alice.callSync("app", "graphql", "graphql", {
      query: queries.findOrCreateThreadQuery,
      variables: {participantIds: []}
    })
    let threadId = JSON.parse(add_result.Ok).findOrCreateThread.id
    t.equal(threadId.length, 46) // thread was created and hash returned

    const post_result = await alice.callSync("app", "graphql", "graphql", {
      query: queries.createMessageQuery,
      variables: {messageThreadId: threadId, text: "Hello hylo+holo!"}
    })
    console.log(post_result)
    // t.notEqual(JSON.parse(post_result.Ok).createMessage.text, "Hello hylo+holo!")

    // retrieve message from channel
    await alice.callSync("app", "graphql", "graphql", {
      query: queries.getMessagesQuery,
      variables: {id: threadId, cursor: "0"}
    })
    await alice.callSync("app", "graphql", "graphql", {
      query: queries.getMessagesQuery,
      variables: {id: threadId, cursor: "0"}
    })
    const get_result = await alice.callSync("app", "graphql", "graphql", {
      query: queries.getMessagesQuery,
      variables: {id: threadId, cursor: "0"}
    })
    console.log(get_result)
    t.equal(JSON.parse(get_result.Ok).messageThread.messages.total, 1)
    t.equal(JSON.parse(get_result.Ok).messageThread.messages.items[0].text, "Hello hylo+holo!")

  })
}
