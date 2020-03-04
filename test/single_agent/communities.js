const { one } = require('../config')
module.exports = scenario => {

  scenario("Create and get single community", async (s, t) => {
    const { alice } = await s.players({alice: one}, true)
    const name = "Test Community 1"
    const slug = "test1"

    const { Ok: addCommunityResult } = await alice.callSync("app", "communities", "create", {
      name,
      slug
    })
    t.equal(addCommunityResult.address.length, 46)
    t.equal(addCommunityResult.name, name)
    t.equal(addCommunityResult.slug, slug)

    const { Ok: getCommunityResult } = await alice.callSync("app", "communities", "get", {
      address: addCommunityResult.address
    })
    t.equal(getCommunityResult.address, address)
    t.equal(getCommunityResult.name, name)
    t.equal(getCommunityResult.slug, slug)

    const { getBySlugResult } = await alice.callSync("app", "communities", "get_by_slug", { slug })

    console.log('!!!!! getBySlugResult with error:', getBySlugResult)

    t.equal(getBySlugResult.Ok.address, address)
    t.equal(getBySlugResult.Ok.name, name)

    const { Ok: getCommunitiesResult } = await alice.callSync("app", "communities", "all", {})
    t.ok(getCommunitiesResult.some(community => community.name === addCommunityResult.name), "Could retrieve the added community from the base")
  }),

  scenario("Create 2 communities and get all communities back", async (s, t) => {
    const { alice } = await s.players({alice: one}, true)
    const communities = [
      { slug: 'test1', name: "Test Community 1" },
      { slug: 'test2', name: "Test Community 2" },
    ]
    for (const [index, community] of communities.entries()) {
      const { Ok: addCommunityResult } = await alice.callSync("app", "communities", "create", {
        name: community.name,
        slug: community.slug
      })
      console.log('!!!!! addCommunityResult:', addCommunityResult)

      communities[index].address = addCommunityResult.address

      t.equal(addCommunityResult.address.length, 46)
      t.equal(addCommunityResult.name, community.name)
      t.equal(addCommunityResult.slug, community.slug)

      const { Ok: getBySlugResult } = await alice.callSync("app", "communities", "get_by_slug", {
        slug: community.slug
      })

      console.log('!!!!! getBySlugResult:', getBySlugResult)
      t.equal(getBySlugResult.address, community.address)
      t.equal(getBySlugResult.name, community.name)
    }

    const { Ok: allCommunitiesResult } = await alice.callSync("app", "communities", "all", {})

    t.deepEqual(allCommunitiesResult, communities)

    // const getCommunitiesResult = await alice.callSync("app", "communities", "all", {})
    // t.ok(getCommunitiesResult.Ok.some(community => community.name === communityResult.name), "Could retrieve the added community from the base")
  })

}
