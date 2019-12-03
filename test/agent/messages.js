const { one } = require('../config')
module.exports = (scenario) => {

scenario('Can create a message and retrieve it', async (s, t) => {
  const { alice } = await s.players({alice: one}, true)

    // add a thread
    const addResult = await alice.callSync("app", "messages", "create_thread", {
      participant_ids: []
    })

    const threadAddress = addResult.Ok

    t.equal(threadAddress.length, 46) // thread was created and hash returned

    // post a message
    const testMessage = {
      thread_address: threadAddress,
      text: "Hello hylo+holo!",
      timestamp: "000"
    }
    const postResult = await alice.callSync("app", "messages", "create", testMessage)
    const { address } = postResult.Ok

    t.deepEqual(postResult.Ok, {...testMessage, creator: alice.info('app').agentAddress, address})

    // retrieve message from channel
    const get_result = await alice.callSync("app", "messages", "get_thread_messages", {
      thread_address: threadAddress,
    })
    t.equal(get_result.Ok.length, 1)
    t.deepEqual(get_result.Ok[0], {...testMessage, creator: alice.info('app').agentAddress, address})

    const get_message_result = await alice.callSync("app", "messages", "get", {
      message_addr: address
    })
    t.deepEqual(get_message_result.Ok, {...testMessage, creator: alice.info('app').agentAddress, address})

  })
}
