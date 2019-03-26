module.exports = (scenario) => {

scenario.runTape('Check for a non existent thread and then create it', async (t, {alice}) => {
    
    // add a thread
    const add_result_str = await alice.callSync("chat", "get_or_create_thread", {
      participant_ids: []
    })

    console.log(add_result_str)
    const add_result = add_result_str.Ok
    t.equal(add_result.length, 46) // thread was created and hash returned

    const get_result_post = await alice.callSync("chat", "get_my_threads", {})

    console.log(get_result_post)
    t.equal(get_result_post.Ok.length, 1) // created a single thread

    
  })
}
