const { one } = require('../config')
module.exports = (scenario) => {

	const base = 'base1'

  const testComment1 = {
  	base,
  	text: "comment1",
  	timestamp: "2019-03-29T01:58:10+00:00"
  }
  const testComment2 = {
  	base,
  	text: "comment2",
  	timestamp: "2019-03-29T01:58:10+00:00"
  }

  scenario("Create and get single comment and all comments", async (s, t) => {
    const { alice } = await s.players({alice: one}, true)
    // define some helpers
  	const callComments = (func, params) => alice.callSync("app", "comments", func, params)

  	const createResult = await callComments('create', testComment1)

		await callComments('create', testComment2)

  	const { address } = createResult.Ok
		t.equal(address.length, 46)
		t.deepEqual(createResult.Ok, { ...testComment1, address, creator: alice.info('app').agentAddress })

  	// get a single comment by its address
  	const getResult = await callComments('get', { address })
  	t.deepEqual(getResult.Ok, { ...testComment1, address, creator: alice.info('app').agentAddress })

  	// get all the comments on a base
  	const allResult = await callComments('all_for_base', { base })
  	t.deepEqual(allResult.Ok.length, 2)
  })
}
