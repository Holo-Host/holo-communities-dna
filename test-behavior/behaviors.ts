/**
 * Defines a collection of behaviors that can be run many times using the same set of players
 * These are different from scenarios in that each behavior can be run against an existing state,
 * and the assertions should be such that there is no interference with previous or concurrent runs.
 */

import {Mutex} from 'async-mutex'
import { Player } from '@holochain/tryorama'

const random = require('random')
const seed = '0'
random.use(require('seedrandom')(seed))

type Players = Array<Player>

const randomElement = <D>(coll: Array<D>): D => {
  return coll[random.int(0, coll.length - 1)]
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

  const player = randomElement(players)
  const authorAddress = player.instance('app').agentAddress

  // define some helpers
  const callComments = (func, params) => player.call("app", "comments", func, params)
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

  const player = randomElement(players)
  const r = await nonce()
  const name = "Test Community " + r
  const slug = "test-" + r
  const add_community_result = await player.call("app", "communities", "create", {
    name,
    slug
  })
  await s.consistency()

  const { address } = add_community_result.Ok
  t.equal(address.length, 46)
  t.equal(add_community_result.Ok.name, name)
  t.equal(add_community_result.Ok.slug, slug)

  const communityResult = {address, name, slug}

  const get_community_result = await player.call("app", "communities", "get", {
    address
  })
  t.equal(get_community_result.Ok.address, address)
  t.equal(get_community_result.Ok.name, name)
  t.equal(get_community_result.Ok.slug, slug)

  const get_by_slug_result = await player.call("app", "communities", "get_by_slug", {
    slug
  })
  t.equal(get_by_slug_result.Ok.address, address)
  t.equal(get_by_slug_result.Ok.name, name)

  const get_communities_result = await player.call("app", "communities", "all", {})
  t.ok(get_communities_result.Ok.some(community => community.name === communityResult.name), "Could retrieve the added community from the base")
}

const messages = async (s, t, players: Players) => {

  const num = await nonce()
  const player = randomElement(players)
  const authorAddress = player.instance('app').agentAddress

  // add a thread
  const addResult = await player.call("app", "messages", "create_thread", {
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
  const postResult = await player.call("app", "messages", "create", testMessage)
  await s.consistency()

  const { address } = postResult.Ok

  t.deepEqual(postResult.Ok, {...testMessage, creator: authorAddress, address})

  // retrieve message from channel
  const get_result = await player.call("app", "messages", "get_thread_messages", {
    thread_address: threadAddress,
  })
  const texts = get_result.Ok.map(msg => msg.text)
  t.ok(texts.includes(text))

  const get_message_result = await player.call("app", "messages", "get", {
    message_addr: address
  })
  t.deepEqual(get_message_result.Ok, {...testMessage, creator: authorAddress, address})
}

const people = async (s, t, players: Players) => {
  const numPlayers = players.length
  const index1 = random.int(0, numPlayers - 1)
  const index2 = random.int(0, numPlayers - 1)
  const player1 = players[index1]
  const player2 = players[index2]
  const name1 = `player-${index1}`
  const name2 = `player-${index2}`

  const getResult = await player1.call("app", 'people', 'get', {agent_id: player2.instance('app').agentAddress})
  console.log('getResult', getResult)
  t.deepEqual(getResult.Ok, { name: name2, avatar_url: `${name2}.jpg`, address: player2.instance('app').agentAddress })

  const getMeResult = await player1.call("app", 'people', 'get_me', {})
  t.deepEqual(getMeResult.Ok, { name: name1, avatar_url: `${name1}.jpg`, address: player1.instance('app').agentAddress })

  const allResult = await player1.call("app", 'people', 'all', {})
  console.log('allResult', allResult)
  t.equal(allResult.Ok.length, players.length)
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

  const player = randomElement(players)

  const add_post_result = await player.call("app", "posts", "create", testPost )
  await s.consistency()
  const { address } = add_post_result.Ok
  console.log('add_post_result', add_post_result)
  t.equal(address.length, 46)
  t.deepEqual(add_post_result.Ok, { ...testPost, creator: player.instance('app').agentAddress, address })

  const get_post_result = await player.call("app", "posts", "get", {
    address
  })
  t.deepEqual(get_post_result.Ok, { ...testPost, creator: player.instance('app').agentAddress, address }, "Could retrieve the added post by address")

  const get_posts_result = await player.call("app", "posts", "all_for_base", {
    base: testPost.base
  })
  console.log(get_posts_result.Ok)
  t.deepEqual(get_posts_result.Ok, [{ ...testPost, creator: player.instance('app').agentAddress, address }], "Could retrieve the added post from the base")
}

export default [
  comments,
  communities,
  messages,
  people,
  posts,
]
