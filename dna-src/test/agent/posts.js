module.exports = (scenario) => {

const testPost = {
  title: "new post",
  details: "this is a details string",
  post_type: "a type",
  announcement: false,
  timestamp: ""
}

scenario.runTape('Can create a post', async (t, {alice}) => {
    const add_post_result = await alice.callSync("posts", "create_post", { base: "community1", ...testPost } )
    console.log(add_post_result)
    const address = add_post_result.Ok
    console.log(address)
    t.equal(address.length, 46)

    const get_post_result = await alice.callSync("posts", "get_post", {
      address
    })
    console.log(get_post_result)
    t.deepEqual(get_post_result.Ok, { ...testPost, creator: alice.agentId }, "Could retrieve the added post by address")

    const get_posts_result = await alice.callSync("posts", "get_posts", {
      base: "community1"
    })
    console.log(get_post_result)
    t.deepEqual(get_posts_result.Ok, [address], "Could retrieve the added post from the base")
  })
}
