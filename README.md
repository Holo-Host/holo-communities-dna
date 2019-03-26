# holo-hylo

DNAs required for running Hylo on Holochain.

! This is a work in progress and only limited functionality is available at this stage !

## Building and running Holochain instance

Instructions for installing Rust and building the holochain container can be found at [developer.holochain](https://developer.holochain.org/start.html).

After this is installed the DNA can be build using
`npm run build`
and started using
`npm run start`

If you plan to develop the DNA code further the automated tests can be run using
`npm run test`

At the current time the container config does not include configuration of the network. This is because at the time of writing you must hard code the IPs in the local network. Defauls about this can be found at [developer.holochain](https://developer.holochain.org/start.html) and will likely be updated soon.

## Example queries
Handy to have these here for reference. These are the 4 queries that are redirected to holochain.

Test the holochain container calls with
`curl -X POST -H "Content-Type: application/json" -d @create-thread-query.json http://localhost:3400`

Retrieving all threads
```javascript
graphql: {
      query: `query ($first: Int, $offset: Int) {
        me {
          id
          messageThreads(sortBy: "updatedAt", order: "desc", first: $first, offset: $offset) {
            total
            hasMore
            items {
              id
              unreadCount
              lastReadAt
              createdAt
              updatedAt
              participants {
                id
                name
                avatarUrl
              }
              messages(first: 1, order: "desc") {
                items {
                  id
                  createdAt
                  text
                  creator {
                    id
                    name
                  }
                }
              }
            }
          }
        }
      }`,
      variables: {
        first,
        offset
      }
    }
```

Fetching all the messages from a thread
```javascript
graphql: {
      query: `
        query ($id: ID, $cursor: ID) {
          messageThread (id: $id) {
            id
            messages(first: 80, cursor: $cursor, order: "desc") {
              items {
                id
                createdAt
                text
                creator {
                  id
                  name
                  avatarUrl
                }
              }
              total
              hasMore
            }
          }
        }
      `,
      variables: opts.cursor ? {id, cursor: opts.cursor} : {id}
    }
```

Creating a message in a thread
```javascript
graphql: {
      query: `mutation ($messageThreadId: String, $text: String) {
        createMessage(data: {messageThreadId: $messageThreadId, text: $text}) {
          id
          text
          createdAt
          creator {
            id
          }
          messageThread {
            id
          }
        }
      }`,
      variables: {
        messageThreadId,
        text
      }
    }
```

Find or create a thread
```javascript
const findOrCreateThreadQuery =
`mutation ($participantIds: [String]) {
  findOrCreateThread(data: {participantIds: $participantIds}) {
    id
    createdAt
    updatedAt
    participants {
      id
      name
      avatarUrl
    }
  }
}`
```
