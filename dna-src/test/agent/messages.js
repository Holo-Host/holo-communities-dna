module.exports = (scenario) => {

scenario.runTape('Can create a message and retrieve it', async (t, {alice}) => {

    // add a thread
    const addResult = await alice.callSync("messages", "create_thread", {
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
    const postResult = await alice.callSync("messages", "create", testMessage)
    const { address } = postResult.Ok

    t.deepEqual(postResult.Ok, {...testMessage, creator: alice.agentId, address})

    // retrieve message from channel
    const get_result = await alice.callSync("messages", "get_thread_messages", {
      thread_address: threadAddress,
    })
    t.equal(get_result.Ok.length, 1)
    t.deepEqual(get_result.Ok[0], {...testMessage, creator: alice.agentId, address})

    const get_message_result = await alice.callSync("messages", "get", {
      message_addr: address
    })
    t.deepEqual(get_message_result.Ok, {...testMessage, creator: alice.agentId, address})

  })
}
