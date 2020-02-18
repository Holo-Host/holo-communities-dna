const { one } = require('../config')
module.exports = (scenario) => {

scenario('Check for a nonexistent thread and then create it', async (s, t) => {
  const { alice } = await s.players({alice: one}, true)
  const timestamp = "2020-02-17T06:56:08+00:00"

  const createThreadZomeApiResult = await alice.callSync("app", "messages", "create_thread", {
    participant_ids: [],
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

scenario('create_thread, set last_read_time and confirm set with get_thread', async (s, t) => {
  const { alice } = await s.players({alice: one}, true)
  
  const timestamp = "2020-02-11T06:56:08+00:00"
  const { address } = (await alice.callSync("app", "messages", "create_thread", {
    participant_ids: [],
    timestamp
  })).Ok

  await s.consistency();
  const updatedTimestamp = "2020-02-17T06:56:08+00:00"
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
})

}
