const { one } = require('../config')
module.exports = (scenario) => {

const postFactory = (title) => ({
    title,
    details: "this is a details string",
    post_type: "a type",
    announcement: false,
    timestamp: "",
    base: "community1",
})

scenario('Can create and retrieve post', async (s, t) => {
    const { alice } = await s.players({alice: one}, true)
    const testPost = postFactory("test")
    const add_post_result = await alice.callSync("app", "posts", "create", testPost )
    console.log(add_post_result)
    const { address } = add_post_result.Ok
    t.equal(address.length, 46)
    t.deepEqual(add_post_result.Ok, { ...testPost, creator: alice.info('app').agentAddress, address })

    const get_post_result = await alice.callSync("app", "posts", "get", {
      address
    })
    t.deepEqual(get_post_result.Ok, { ...testPost, creator: alice.info('app').agentAddress, address }, "Could retrieve the added post by address")

    const get_posts_result = await alice.callSync("app", "posts", "all_for_base", {
      base: testPost.base,
      // TODO: becomes Iso8601 once core regex tagging issue fixed
      _from_time: "new",
      _limit: "10"  
    })
    console.log(get_posts_result.Ok)
    t.deepEqual(get_posts_result.Ok, {posts: [{ ...testPost, creator: alice.info('app').agentAddress, address }], more: false}, "Could retrieve the added post from the base")
  })

scenario('Can create multiple posts', async (s, t) => {
    const { alice } = await s.players({alice: one}, true)

    const nTestPosts = 10
    let postAddrs = []
    for(let i = 0; i < nTestPosts; i++) {
      const testPost = postFactory("test"+i)
      const add_post_result = await alice.callSync("app", "posts", "create", testPost )
      t.equal(add_post_result.Ok.address.length, 46)
      postAddrs.push(add_post_result.Ok.address)
    }

    // try getting all of them
    const get_posts_result = await alice.callSync("app", "posts", "all_for_base", {
      base: postFactory("").base,
      _limit: "10",
      _from_time: "new"
      // _from_time: "2020-02-17T06:56:08+00:00"
    })
    t.deepEqual(get_posts_result.Ok.posts.length, nTestPosts)

    // // try getting only the first slicePoint
    // const slicePoint = 4;
    // const get_posts_result_limit = await alice.callSync("app", "posts", "all_for_base", {
    //   base: postFactory("").base,
    //   limit: slicePoint
    // })
    // t.deepEqual(get_posts_result_limit.Ok.posts.length, slicePoint)  
    // t.deepEqual(get_posts_result_limit.Ok.more, true)  
    // t.deepEqual(get_posts_result_limit.Ok.posts.map(p => p.address), postAddrs.slice(0, slicePoint))  

    // // try getting the rest
    // const get_posts_result_since = await alice.callSync("app", "posts", "all_for_base", {
    //   base: postFactory("").base,
    //   since: postAddrs[slicePoint-1]
    // })
    // t.deepEqual(get_posts_result_since.Ok.posts.length, nTestPosts - slicePoint)  
    // t.deepEqual(get_posts_result_since.Ok.more, false)  
    // t.deepEqual(get_posts_result_since.Ok.posts.map(p => p.address), postAddrs.slice(slicePoint, nTestPosts))  
  })

}
