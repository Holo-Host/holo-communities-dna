const queries = require('../queries')
module.exports = (scenario) => {

scenario.runTape('Can add a comment to a post via graphql', async (t, {alice}) => {
    let register_response = await alice.callSync("graphql", "graphql", {
      query: queries.registerQuery,
      variables: {name: "wollum", avatarUrl: "//"}
    })
    // console.log(register_response)

    // create a comment
    const addResult = await alice.callSync("graphql", "graphql", {
      query: queries.createCommentQuery,
      variables: {postId: '100', text: 'Holo Graph QL Comment'}
    })
    // console.log(addResult)
    let commentId = JSON.parse(addResult.Ok).createComment.id
    t.equal(commentId.length, 46) // thread was created and hash returned

    // create a comment
    const addResult2 = await alice.callSync("graphql", "graphql", {
      query: queries.createCommentQuery,
      variables: {postId: '100', text: '2nd Holo Graph QL Comment'}
    })

    // retrieve comments
    const getResult = await alice.callSync("graphql", "graphql", {
      query: queries.getCommentsQuery,
      variables: {id: '100'}
    })
    // console.log(JSON.parse(getResult.Ok).post.comments.items.length)
    t.deepEqual(JSON.parse(getResult.Ok).post.comments.items.length, 2)
    // console.log(getResult)

  })
}
