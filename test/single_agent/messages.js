const { one } = require('../config')
module.exports = (scenario) => {

  scenario('Can create a message and retrieve it', async (s, t) => {
    const { alice } = await s.players({alice: one}, true)
    const timestamp = "2020-01-09T06:56:08+00:00"
    // add a thread
    const createThreadResult = await alice.callSync("app", "messages", "create_thread", {
      participant_addresses: [],
      timestamp
    })

    const { address: threadAddress } = createThreadResult.Ok

    t.equal(threadAddress.length, 46) // thread was created and hash returned

    const createMessageInput = {
      thread_address: threadAddress,
      text: "Hello hylo+holo!",
      timestamp
    }
    const createMessageZomeApiResult = await alice.callSync("app", "messages", "create_message", createMessageInput)
    const createMessageResult = createMessageZomeApiResult.Ok
    const expectedCreateMessageResult = {
      address: createMessageResult.address,
      timestamp: createMessageInput['timestamp'],
      text: createMessageInput['text'],
      thread_address: createMessageInput['thread_address'],
      creator: alice.info('app').agentAddress
    }

    t.deepEqual(createMessageResult, expectedCreateMessageResult)

    const get_result = await alice.callSync("app", "messages", "all_messages_for_thread", {
      thread_address: threadAddress,
    })
    t.equal(get_result.Ok.length, 1)
    t.deepEqual(get_result.Ok[0], expectedCreateMessageResult)
  })

}
