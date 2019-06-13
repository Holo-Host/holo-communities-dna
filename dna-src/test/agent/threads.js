module.exports = (scenario) => {

scenario.runTape('Check for a non existent thread and then create it', async (t, {alice}) => {
    
    // add a thread
    const add_result_str = await alice.callSync("messages", "create_thread", {
      participant_ids: []
    })

    const address = add_result_str.Ok
    t.equal(address.length, 46) // thread was created and hash returned

    const get_result_post = await alice.callSync("messages", "get_threads", {})

    console.log(get_result_post)
    t.equal(get_result_post.Ok.length, 1) // created a single thread    
  })
}
