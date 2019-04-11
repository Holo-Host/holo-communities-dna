module.exports = (scenario) => {

scenario.runTape('Can create a post', async (t, {alice}) => {
    const add_post_result = await alice.callSync("posts", "create_post", {
      title: "new post",
      details: "this is a details string",
      post_type: "a type",
      announcement: false,
      timestamp: ""
    })
    const address = add_post_result.Ok
    console.log(address)
    t.equal(address.length, 46)

    const get_post_result = await alice.callSync("posts", "get_post", {
      address
    })
    console.log(get_post_result)
    t.equal(get_post_result.Ok.title, "new post")
  })
}
