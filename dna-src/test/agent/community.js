module.exports = (scenario) => {

  scenario.runTape("Create and get single community", async (t, { alice }) => {
    const add_community_result = await alice.callSync("community", "create_community", {
      name: "Test Community 1",
      slug: "url/slug1"
    })
    const address = add_community_result.Ok
    console.log(address)
    t.equal(address.length, 46)

    // const get_post_result = await alice.callSync("posts", "get_post", {
    //   address
    // })
    // console.log(get_post_result)
    // t.equal(get_post_result.Ok.title, "new post")
  })
}
