module.exports.registerQuery = `
mutation ($id: ID, $name: String, $avatarUrl: String) {
  registerUser(id: $id, name: $name, avatarUrl: $avatarUrl) {
    success
  }
}
`

module.exports.getPeopleQuery = `
query PeopleContacts {
  people {
    items {
      id
      name
      avatarUrl
      memberships (first: 1) {
        id
        community {
          id
          name
        }
      }
    }
  }
}
`

module.exports.getMessageThreadsQuery = `
query {
  me {
    id
    messageThreads {
      total
      hasMore
      items {
        id
        createdAt
        updatedAt
        participants {
          id
          name
          avatarUrl
        }
        messages {
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
}`

module.exports.findOrCreateThreadQuery =`
mutation ($participantIds: [String]) {
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

module.exports.createMessageQuery = `
mutation ($messageThreadId: String, $text: String) {
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
}`

module.exports.getMessagesQuery = `
  query ($id: ID) {
    messageThread (id: $id) {
      id
      messages {
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
`

module.exports.createCommentQuery = `
mutation ($postId: String, $text: String) {
    createComment(data: {postId: $postId, text: $text}) {
      id
      text
      post {
        id
      }
      createdAt
      creator {
        id
      }
    }
  }
`

module.exports.getCommentsQuery = `
query ($id: ID) {
    post(id: $id) {
      id
      comments {
        items {
          id
          text
          creator {
            id
            name
            avatarUrl
          }
          createdAt
          attachments {
            id
            url
          }
        }
        total
        hasMore
      }
    }
  }
`

module.exports.createPostQuery = `
mutation (
  $communitySlug: String,
  $type: String,
  $title: String,
  $details: String
) {
  createPost(data: {
    communitySlug: $communitySlug,
    type: $type,
    title: $title,
    details: $details
  })
  {
    id
    title
    details
    type
    creator {
      id
      name
      avatarUrl
    }
    createdAt
    updatedAt
  }
}`

module.exports.getPostQuery = `
query ($id: ID) {
  post(id: $id) {
    id
    title
    details
    type
    creator {
      id
      name
      avatarUrl
    }
    createdAt
    updatedAt
  }
}
`


module.exports.getPostsQuery = `
query {
  posts {
    hasMore
    items {
      id
      title
      details
      post_type
      creator {
        id
        name
        avatarUrl
      }
      createdAt
      updatedAt
    }
  }
}
`

module.exports.createCommunityQuery =`
mutation (
  $name: String,
  $slug: String
) {
  createCommunity(data: {
    name: $name,
    slug: $slug
  })
  {
    id
    name
  }
}
`

module.exports.getCommunityQuery = `
query ($id: ID) {
  community(id: $id) {
    id
    name
  }
}
`

module.exports.getCommunityPostsQuery = `
query ($id: ID) {
  community(id: $id) {
    posts {
      hasMore
      items {
        id
        title
        details
        type
        creator {
          id
          name
          avatarUrl
        }
        createdAt
        updatedAt
      }
    }
  }
}
`
