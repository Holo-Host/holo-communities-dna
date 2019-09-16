module.exports = (scenario) => {
  scenario('Can create a message and retrieve it', async (s, t, { alice, bob }) => {
    const aliceUser = {
      name: 'Alice',
      avatar_url: 'alice.png'
    }

    const bobUser = {
      name: 'Bob',
      avatar_url: 'bob'
    }

    const isRegisteredResult1 = await alice.call('people', 'is_registered', {})
    t.deepEqual(isRegisteredResult1.Ok, false)

    const registerResult = await alice.call('people', 'register_user', aliceUser)
    console.log("-->",registerResult);
    t.deepEqual(registerResult.Ok, { ...aliceUser, address: alice.agentId })

    const isRegisteredResult2 = await alice.call('people', 'is_registered', {})
    t.deepEqual(isRegisteredResult2.Ok, true)

    await bob.call('people', 'register_user', bobUser)
    const getResult = await alice.call('people', 'get', {agent_id: bob.agentId})
    t.deepEqual(getResult.Ok, { ...bobUser, address: bob.agentId })

    const getMeResult = await alice.call('people', 'get_me', {})
    t.deepEqual(getMeResult.Ok, { ...aliceUser, address: alice.agentId })

    const allResult = await alice.call('people', 'all', {})
    t.deepEqual(allResult.Ok.length, 2)
    t.deepEqual(allResult.Ok.sort().map(p => p.name), [aliceUser, bobUser].sort().map(p => p.name))

  })
}
