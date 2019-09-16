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

  scenario("Create and get single comment and all comments", async (s, t, { alice }) => {
      // define some helpers
  	const callComments = (func, params) => alice.callSync("comments", func, params)

  	const createResult = await callComments('create', testComment1)

		await callComments('create', testComment2)

  	const { address } = createResult.Ok
		t.equal(address.length, 46)
		t.deepEqual(createResult.Ok, { ...testComment1, address, creator: alice.agentId })

  	// get a single comment by its address
  	const getResult = await callComments('get', { address })
  	t.deepEqual(getResult.Ok, { ...testComment1, address, creator: alice.agentId })

  	// get all the comments on a base
  	const allResult = await callComments('all_for_base', { base })
  	t.deepEqual(allResult.Ok.length, 2)
  })
}
