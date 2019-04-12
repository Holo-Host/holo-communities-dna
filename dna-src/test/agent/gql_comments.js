const queries = require('../queries')
module.exports = (scenario) => {

scenario.runTape('Can add a comment to a post', async (t, {alice}) => {
    let register_response = await alice.callSync("graphql", "graphql", {
      query: queries.registerQuery,
      variables: {id: "000", name: "wollum", avatarUrl: "//"}
    })
    console.log(register_response)

    // create a comment
    const addResult = await alice.callSync("graphql", "graphql", {
      query: queries.createCommentQuery,
      variables: {postId: '100', text: 'Holo Comment'}
    })
    console.log(addResult)
    let commentId = JSON.parse(addResult.Ok).createPost.id
    t.equal(commentId.length, 46) // thread was created and hash returned

    // retrieve comments
    const getResult = await alice.callSync("graphql", "graphql", {
      query: queries.getCommentsQuery,
      variables: {id: '100'}
    })
    t.deepEqual(getResult.Ok.length, 2)
    console.log(getResult)

  })
}
