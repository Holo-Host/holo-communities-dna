module.exports = (scenario) => {

  const testComment1 = {
  	base: "base1",
  	content: "comment1",
  	timestamp: "2019-03-29T01:58:10+00:00"
  }
  const testComment2 = {
  	base: "base1",
  	content: "comment2",
  	timestamp: "2019-03-29T01:58:10+00:00"
  }

  scenario.runTape("Create and get single comment and all comments", async (t, { alice }) => {
      // define some helpers
  	let results = []
  	const lastResult = (back=0) => results[results.length-1-back]
  	const callComments = async (func, params) => {
  		const result = await alice.callSync("comments", func, params)
  		results.push(result)
  		return result
  	}

  	await callComments('create_comment', {
  		comment: testComment1
  	})
  	const address = lastResult().Ok
  	t.equal(lastResult().Ok.length, 46)

  	await callComments('create_comment', {
  		comment: testComment2
  	})
  	t.equal(lastResult().Ok.length, 46)

  	// get a single comment by its address
  	await callComments('get_comment', { address })
  	t.deepEqual(lastResult().Ok, { author: alice.agentId, ...testComment1 })

  	// get all the comments on a base
  	await callComments('get_comments', { base: 'base1' })
  	t.deepEqual(lastResult().Ok, [testComment1, testComment2].map(e => ({author: alice.agentId, ...e})))

  	results.forEach((r, i) => {
    		console.log(i, r)
  	})

  })
}
