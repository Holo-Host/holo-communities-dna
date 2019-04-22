const queries = require('../queries')
module.exports = (scenario) => {

scenario.runTape('Can create a new post', async (t, {alice}) => {
    let register_response = await alice.callSync("graphql", "graphql", {
      query: queries.registerQuery,
      variables: {id: "000", name: "wollum", avatarUrl: "//"}
    })
    console.log(register_response)

    // create a post
    const addResult = await alice.callSync("graphql", "graphql", {
      query: queries.createPostQuery,
      variables: {
        communitySlug: "base to link from",
        title: "new post",
        details: "this is a details string",
        type: "a type"
      }
    })
    console.log(addResult)
    let postId = JSON.parse(addResult.Ok).createPost.id
    t.equal(postId.length, 46) // thread was created and hash returned

    // retrieve post
    const getResult = await alice.callSync("graphql", "graphql", {
      query: queries.getPostQuery,
      variables: {id: postId}
    })
    console.log(getResult)
    let post = JSON.parse(getResult.Ok).post
    t.equal(post.title, "new post") // thread was created and hash returned
    t.equal(post.commentersTotal, 0)
    t.deepEqual(post.commenters, [])

    // come a comments
    await alice.callSync("graphql", "graphql", {
      query: queries.createCommentQuery,
      variables: {postId, text: 'first comment'}
    })
    await alice.callSync("graphql", "graphql", {
      query: queries.createCommentQuery,
      variables: {postId, text: 'another comment'}
    })

    // retrieve post after comment
    const getResult2 = await alice.callSync("graphql", "graphql", {
      query: queries.getPostQuery,
      variables: {id: postId}
    })
    console.log(getResult2)
    let post2 = JSON.parse(getResult2.Ok).post
    t.equal(post2.commentersTotal, 1)
    t.deepEqual(post2.commenters, [{id: "000", name: "wollum"}])
  })
}
