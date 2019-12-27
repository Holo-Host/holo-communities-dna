/**
 * Defines a collection of behaviors that can be run many times using the same set of players
 * These are different from scenarios in that each behavior can be run against an existing state,
 * and the assertions should be such that there is no interference with previous or concurrent runs.
 */

import {Mutex} from 'async-mutex'
import { Player, Instance } from '@holochain/tryorama'
import { Batch } from '@holochain/tryorama-stress-utils'

const random = require('random')
const seed = '0'
random.use(require('seedrandom')(seed))

type Players = Array<Player>

const randomElement = <D>(coll: Array<D>): D => {
  return coll[random.int(0, coll.length - 1)]
}

const randomInstance = (players: Array<Player>): Instance => {
  return randomElement(randomElement(players).instances())
}

let _nonce = 0
const nonceMutex = new Mutex()
const nonce = () => nonceMutex.runExclusive(() => _nonce++)

const comments = async (s, t, players: Players) => {
  const num = await nonce()
  const base = 'base-' + num
  const testComment1 = {
  	base,
  	text: "comment1",
  	timestamp: "2019-03-29T01:58:10+00:00"
  }
  const testComment2 = {
  	base,
  	text: "comment2",
  	timestamp: "2019-03-29T01:58:10+00:00"
  }

  const instance = randomInstance(players)
  const authorAddress = instance.agentAddress

  // define some helpers
  const callComments = (func, params) => instance.call("comments", func, params)
  const createResult = await callComments('create', testComment1)
  await s.consistency()
  await callComments('create', testComment2)
  await s.consistency()

  const { address } = createResult.Ok
  t.equal(address.length, 46)
  t.deepEqual(createResult.Ok, { ...testComment1, address, creator: authorAddress })

  // get a single comment by its address
  const getResult = await callComments('get', { address })
  t.deepEqual(getResult.Ok, { ...testComment1, address, creator: authorAddress })

  // get all the comments on a base
  const allResult = await callComments('all_for_base', { base })
  t.deepEqual(allResult.Ok.length, 2)
}

const communities = async (s, t, players: Players) => {

  const instance = randomInstance(players)
  const r = await nonce()
  const name = "Test Community " + r
  const slug = "test-" + r
  const add_community_result = await instance.call("communities", "create", {
    name,
    slug
  })
  await s.consistency()

  const { address } = add_community_result.Ok
  t.equal(address.length, 46)
  t.equal(add_community_result.Ok.name, name)
  t.equal(add_community_result.Ok.slug, slug)

  const communityResult = {address, name, slug}

  const get_community_result = await instance.call("communities", "get", {
    address
  })
  t.equal(get_community_result.Ok.address, address)
  t.equal(get_community_result.Ok.name, name)
  t.equal(get_community_result.Ok.slug, slug)

  const get_by_slug_result = await instance.call("communities", "get_by_slug", {
    slug
  })
  t.equal(get_by_slug_result.Ok.address, address)
  t.equal(get_by_slug_result.Ok.name, name)

  const get_communities_result = await instance.call("communities", "all", {})
  t.ok(get_communities_result.Ok.some(community => community.name === communityResult.name), "Could retrieve the added community from the base")
}

const messages = async (s, t, players: Players) => {

  const num = await nonce()
  const instance = randomInstance(players)
  const authorAddress = instance.agentAddress

  // add a thread
  const addResult = await instance.call("messages", "create_thread", {
    participant_ids: []
  })
  await s.consistency()

  const threadAddress = addResult.Ok

  t.equal(threadAddress.length, 46) // thread was created and hash returned

  const text = "Hello hylo+holo! " + num
  // post a message
  const testMessage = {
    thread_address: threadAddress,
    text,
    timestamp: String(num),
  }
  const postResult = await instance.call("messages", "create", testMessage)
  await s.consistency()

  const { address } = postResult.Ok

  t.deepEqual(postResult.Ok, {...testMessage, creator: authorAddress, address})

  // retrieve message from channel
  const get_result = await instance.call("messages", "get_thread_messages", {
    thread_address: threadAddress,
  })
  const texts = get_result.Ok.map(msg => msg.text)
  t.ok(texts.includes(text))

  const get_message_result = await instance.call("messages", "get", {
    message_addr: address
  })
  t.deepEqual(get_message_result.Ok, {...testMessage, creator: authorAddress, address})
}

const people = async (s, t, players: Players) => {
  const batch = new Batch(players)
  const instance1 = randomInstance(players)
  const instance2 = randomInstance(players)

  const getResult = await instance1.call('people', 'get', {agent_id: instance2.agentAddress})
  console.log('getResult', getResult)
  t.deepEqual(getResult.Ok.address, instance2.agentAddress )

  const getMeResult = await instance1.call('people', 'get_me', {})
  t.deepEqual(getMeResult.Ok.address, instance1.agentAddress )

  const allResult = await instance1.call('people', 'all', {})
  console.log('allResult', allResult)
  t.equal(allResult.Ok.length, batch.instances().length)
}

const posts = async (s, t, players: Players) => {

  const num = await nonce()

  const testPost = {
    title: "new post " + num,
    details: "this is a details string",
    post_type: "a type",
    announcement: false,
    timestamp: "",
    base: "community" + num,
  }

  const instance = randomInstance(players)

  const add_post_result = await instance.call("posts", "create", testPost )
  await s.consistency()
  const { address } = add_post_result.Ok
  console.log('add_post_result', add_post_result)
  t.equal(address.length, 46)
  t.deepEqual(add_post_result.Ok, { ...testPost, creator: instance.agentAddress, address })

  const get_post_result = await instance.call("posts", "get", {
    address
  })
  t.deepEqual(get_post_result.Ok, { ...testPost, creator: instance.agentAddress, address }, "Could retrieve the added post by address")

  const get_posts_result = await instance.call("posts", "all_for_base", {
    base: testPost.base
  })
  console.log(get_posts_result.Ok)
  t.deepEqual(get_posts_result.Ok, [{ ...testPost, creator: instance.agentAddress, address }], "Could retrieve the added post from the base")
}

export default [
  comments,
  communities,
  messages,
  people,
  posts,
]
