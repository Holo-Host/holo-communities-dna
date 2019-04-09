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

    // retrieve comments
    const getResult = await alice.callSync("graphql", "graphql", {
      query: queries.getCommentsQuery,
      variables: {id: '100'}
    })

    console.log(getResult)

  })
}
