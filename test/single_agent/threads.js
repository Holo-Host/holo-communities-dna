const { one } = require('../config')
module.exports = (scenario) => {

scenario('Can create a new Message Thread, and retrieve it in my list of all Message Threads', async (s, t) => {
  const { alice } = await s.players({alice: one}, true)
  const timestamp = "2020-02-17T06:56:08+00:00"

  const createThreadZomeApiResult = await alice.callSync("app", "messages", "create_thread", {
    participant_addresses: [],
    timestamp
  })

  const createThreadResult = createThreadZomeApiResult.Ok
  const expectedCreateThreadResult = {
    address: createThreadResult.address,
    participant_addresses: [alice.info('app').agentAddress],
    last_read_time: timestamp
  }
  t.deepEqual(createThreadResult, expectedCreateThreadResult)

  const allThreadsResult = await alice.callSync("app", "messages", "all_threads", {})

  t.equal(allThreadsResult.Ok.length, 1)
  t.deepEqual(allThreadsResult.Ok[0], expectedCreateThreadResult)
}),

scenario('Can set a Message Threads last read time', async (s, t) => {
  const { alice } = await s.players({alice: one}, true)
  
  // TODO: becomes Iso8601 once core regex tagging issue fixed
  // const timestamp = "2020-02-11T06:56:08+00:00"
  const timestamp = "old"
  const { address } = (await alice.callSync("app", "messages", "create_thread", {
    participant_addresses: [],
    timestamp
  })).Ok

  await s.consistency();

  // TODO: becomes Iso8601 once core regex tagging issue fixed
  // const updatedTimestamp = "2020-02-17T06:56:08+00:00"
  const updatedTimestamp = "new"
  const setLastReadTimeApiResult = await alice.callSync("app", "messages", "set_last_read_time", {
    thread_address: address,
    last_read_time: updatedTimestamp
  })
  const setLastReadTimeResult = setLastReadTimeApiResult.Ok
  const expectedSetLastReadTimeResult = {
    address: address,
    participant_addresses: [alice.info('app').agentAddress],
    last_read_time: updatedTimestamp
  }

  await s.consistency();

  t.deepEqual(setLastReadTimeResult, expectedSetLastReadTimeResult)

  const getThreadResult = (await alice.callSync("app", "messages", "get_thread", {
    thread_address: address
  })).Ok
  await s.consistency();

  t.deepEqual(getThreadResult, expectedSetLastReadTimeResult)

  const allThreadsResult = await alice.callSync("app", "messages", "all_threads", {})
  await s.consistency();

  t.equal(allThreadsResult.Ok.length, 1)
})

}
