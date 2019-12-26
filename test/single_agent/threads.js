const { one } = require('../config')
module.exports = (scenario) => {

scenario('Check for a non existent thread and then create it', async (s, t) => {
    const { alice } = await s.players({alice: one}, true)

    // add a thread
    const add_result_str = await alice.callSync("app", "messages", "create_thread", {
      participant_ids: []
    })

    const address = add_result_str.Ok
    t.equal(address.length, 46) // thread was created and hash returned

    const get_result_post = await alice.callSync("app", "messages", "get_threads", {})

    console.log(get_result_post)
    t.equal(get_result_post.Ok.length, 1) // created a single thread
  })
}
