module.exports = (scenario) => {

  scenario.runTape("Create and get single community", async (t, { alice }) => {
    const add_community_result = await alice.callSync("community", "create_community", {
      base: "Network 1",
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

    const get_communitys_result = await alice.callSync("community", "get_communitys", {
      base: "Network 1"
    })
    console.log(get_communitys_result)
    t.deepEqual(get_communitys_result.Ok, [address], "Could retrieve the added community from the base")
  })
}
