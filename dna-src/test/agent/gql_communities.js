const queries = require('../queries')
module.exports = (scenario) => {

  const slug = "/community1"

  scenario.runTape('Can create a new community via graphql', async (t, {alice}) => {
    let register_response = await alice.callSync("graphql", "graphql", {
      query: queries.registerQuery,
      variables: {name: "wollum", avatarUrl: "//"}
    })
    console.log(register_response)

    // create a community
    const addResult = await alice.callSync("graphql", "graphql", {
      query: queries.createCommunityQuery,
      variables: {
        name: "new graphql community",
        slug
      }
    })
    console.log(addResult)
    let communityId = JSON.parse(addResult.Ok).createCommunity.id
    t.equal(communityId.length, 46) // thread was created and hash returned

    // retrieve community
    const getResult = await alice.callSync("graphql", "graphql", {
      query: queries.getCommunityQuery,
      variables: {id: communityId}
    })
    console.log(getResult)
    let communityName = JSON.parse(getResult.Ok).community.name
    t.equal(communityName, "new graphql community") // thread was created and hash returned


    // retrieve community by slug
    const getResultSlug = await alice.callSync("graphql", "graphql", {
      query: queries.getCommunityQuery,
      variables: {slug}
    })
    console.log(getResultSlug)
    let communityNameSlug = JSON.parse(getResultSlug.Ok).community.name
    t.equal(communityNameSlug, "new graphql community") // thread was created and hash returned

    // add a post with the community as the base
    const addPostResult = await alice.callSync("graphql", "graphql", {
      query: queries.createPostQuery,
      variables: {
        communitySlug: slug,
        title: "new post 3000",
        details: "this is a details string",
        type: "a type"
      }
    })
    console.log(addPostResult)

    // retrieve all the posts from the community
    const getPostsResult = await alice.callSync("graphql", "graphql", {
      query: queries.getCommunityPostsQuery,
      variables: {id: communityId}
    })
    console.log('getPostsResult', getPostsResult)
    t.equal(JSON.parse(getPostsResult.Ok).community.posts.items.length, 1)
  })
}
