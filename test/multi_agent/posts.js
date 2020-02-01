const { one } = require('../config')

module.exports = (scenario) => {
  const postFactory = (title) => ({
      title,
      details: "this is a details string",
      post_type: "a type",
      announcement: false,
      timestamp: "",
      base: "community1",
  })

  // TODO: Remove pagination scheme params (since, etc)
  scenario('Can create multiple posts with two agents', async (s, t) => {
      const { alice, bob } = await s.players({alice: one, bob: one}, true)

      const nTestPosts = 10
      let postAddrs = []

      for(let i = 0; i < nTestPosts; i++) {
        const testPost = postFactory("test"+i)
        // switch agents every second post
        const add_post_result = i % 2 == 0
          ? await alice.callSync("app", "posts", "create", testPost)
          : await bob.callSync("app", "posts", "create", testPost)
        await s.consistency()
        t.equal(add_post_result.Ok.address.length, 46)
        postAddrs.push(add_post_result.Ok.address)
      }

      await s.consistency()
  })
}
