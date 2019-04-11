const queries = require('../queries')
module.exports = (scenario) => {

scenario.runTape('Can create a new post', async (t, {alice}) => {
    let register_response = await alice.callSync("graphql", "graphql", {
      query: queries.registerQuery,
      variables: {id: "000", name: "wollum", avatarUrl: "//"}
    })
    console.log(register_response)

    // create a comment
    const addResult = await alice.callSync("graphql", "graphql", {
      query: queries.createPostQuery,
      variables: {
        title: "new post",
        details: "this is a details string",
        type: "a type"
      }
    })
    console.log(addResult)
    let postId = JSON.parse(addResult.Ok).createPost.id
    t.equal(postId.length, 46) // thread was created and hash returned

    // retrieve post
    // const getResult = await alice.callSync("graphql", "graphql", {
    //   query: queries.getCommentsQuery,
    //   variables: {id: '100'}
    // })
    //
    // console.log(getResult)

  })
}
