const { one } = require('../config')
module.exports = (scenario) => {
  scenario('Can create a message and retrieve it', async (s, t) => {
    const { alice, bob } = await s.players({alice: one, bob: one}, true)

    const aliceUser = {
      name: 'Alice',
      avatar_url: 'alice.png'
    }

    const bobUser = {
      name: 'Bob',
      avatar_url: 'bob'
    }

    const isRegisteredResult1 = await alice.call("app", 'people', 'is_registered', {})
    t.deepEqual(isRegisteredResult1.Ok, false)

    const registerResult = await alice.call("app", 'people', 'register_user', aliceUser)
    console.log("-->",registerResult);
    t.deepEqual(registerResult.Ok, { ...aliceUser, address: alice.info('app').agentAddress })

    const isRegisteredResult2 = await alice.call("app", 'people', 'is_registered', {})
    t.deepEqual(isRegisteredResult2.Ok, true)

    await s.consistency()

    await bob.call("app", 'people', 'register_user', bobUser)
    const getResult = await alice.call("app", 'people', 'get', {agent_id: bob.info('app').agentAddress})
    t.deepEqual(getResult.Ok, { ...bobUser, address: bob.info('app').agentAddress })

    const getMeResult = await alice.call("app", 'people', 'get_me', {})
    t.deepEqual(getMeResult.Ok, { ...aliceUser, address: alice.info('app').agentAddress })

    const allResult = await alice.call("app", 'people', 'all', {})
    t.deepEqual(allResult.Ok.length, 2)
    t.deepEqual(allResult.Ok.sort().map(p => p.name), [aliceUser, bobUser].sort().map(p => p.name))

  })
}
