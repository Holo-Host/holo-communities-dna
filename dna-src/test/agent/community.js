module.exports = (scenario) => {

  scenario.runTape("Create and get single community", async (t, { alice }) => {
    const add_community_result = await alice.callSync("community", "create_community", {
      name: "Test Community 1",
      slug: "url/slug1"
    })
    const address = add_community_result.Ok
    console.log(address)
    t.equal(address.length, 46)

    const get_community_result = await alice.callSync("community", "get_community", {
      address
    })
    console.log(get_community_result)
    t.equal(get_community_result.Ok.name, "Test Community 1")

    const get_community_result_by_slug = await alice.callSync("community", "get_community_address_by_slug", {
      slug: "url/slug1"
    })
    console.log(get_community_result_by_slug)
    t.equal(get_community_result_by_slug.Ok, address)

    const get_communities_result = await alice.callSync("community", "get_communities", {
    })
    console.log(get_communities_result)
    t.deepEqual(get_communities_result.Ok, [address], "Could retrieve the added community from the base")
  })
}
