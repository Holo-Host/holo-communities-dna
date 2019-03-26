const queries = require('../queries')
module.exports = (scenario) => {

scenario.runTape('Can register a user and retrieve them again', async (t, {alice}) => {

    // add a thread
    const add_result_str = await alice.callSync("chat", "get_or_create_thread", {
      participant_ids: []
    })

    console.log(add_result_str)
    const add_result = add_result_str.Ok
    t.equal(add_result.length, 46) // thread was created and hash returned

    // post a message
    const post_result = await alice.callSync("chat", "post_message_to_thread", {
      thread_addr: add_result, 
      text: "Hello hylo+holo!",
      timestamp: "000"
    })
    console.log(post_result)

    // retrieve message from channel
    const get_result = await alice.callSync("chat", "get_thread_messages", {
      thread_addr: add_result,
    })
    console.log(get_result)
    t.equal(get_result.Ok.length, 1)
    // t.equal(JSON.parse(get_result.Ok).messageThread.messages.items[0].text, "Hello hylo+holo!")

    const get_message_result = await alice.callSync("chat", "get_message", {
      message_addr: get_result.Ok[0]
    })
    t.equal(get_message_result.Ok.text, "Hello hylo+holo!")

  })
}
