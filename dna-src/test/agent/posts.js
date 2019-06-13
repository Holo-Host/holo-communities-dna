module.exports = (scenario) => {

const testPost = {
  title: "new post",
  details: "this is a details string",
  post_type: "a type",
  announcement: false,
  timestamp: "",
  base: "community1",
}

scenario.runTape('Can create a post', async (t, {alice}) => {
    const add_post_result = await alice.callSync("posts", "create", testPost )
    const { address } = add_post_result.Ok
    t.equal(address.length, 46)
    t.deepEqual(add_post_result.Ok, { ...testPost, creator: alice.agentId, address })

    const get_post_result = await alice.callSync("posts", "get", {
      address
    })
    t.deepEqual(get_post_result.Ok, { ...testPost, creator: alice.agentId, address }, "Could retrieve the added post by address")

    const get_posts_result = await alice.callSync("posts", "all_for_base", {
      base: testPost.base
    })
    console.log(get_posts_result.Ok)
    t.deepEqual(get_posts_result.Ok, [{ ...testPost, creator: alice.agentId, address }], "Could retrieve the added post from the base")
  })
}
