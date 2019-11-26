const { one } = require('../config')
module.exports = (scenario) => {

const testPost = {
  title: "new post",
  details: "this is a details string",
  post_type: "a type",
  announcement: false,
  timestamp: "",
  base: "community1",
}

scenario('Can create a post', async (s, t) => {
    const { alice } = await s.players({alice: one}, true)

    const add_post_result = await alice.callSync("app", "posts", "create", testPost )
    const { address } = add_post_result.Ok
    t.equal(address.length, 46)
    t.deepEqual(add_post_result.Ok, { ...testPost, creator: alice.info('app').agentAddress, address })

    const get_post_result = await alice.callSync("app", "posts", "get", {
      address
    })
    t.deepEqual(get_post_result.Ok, { ...testPost, creator: alice.info('app').agentAddress, address }, "Could retrieve the added post by address")

    const get_posts_result = await alice.callSync("app", "posts", "all_for_base", {
      base: testPost.base
    })
    console.log(get_posts_result.Ok)
    t.deepEqual(get_posts_result.Ok, [{ ...testPost, creator: alice.info('app').agentAddress, address }], "Could retrieve the added post from the base")
  })
}
