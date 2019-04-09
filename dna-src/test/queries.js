module.exports.registerQuery = `
mutation ($id: ID, $name: String, $avatarUrl: String) {
  registerUser(id: $id, name: $name, avatarUrl: $avatarUrl) {
    success
  }
}
`

module.exports.getPeopleQuery = `
query PeopleContacts ($first: Int) {
  people (first: $first) {
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
query ($first: Int, $offset: Int) {
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
query ($id: ID, $cursor: ID) {
    post(id: $id) {
      id
      comments(first: 10, cursor: $cursor, order: "desc") {
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


module.exports.getPostQuery = `
query ($id: ID) {  
  post(id: $id) {
    id  
    announcement  
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
    commenters(first: 3) {    
      id    
      name    
      avatarUrl  
    }  
    commentersTotal  
    comments(first: 10, order: "desc") {    
      items {      
        id      
        text      
        creator {        
          id        
          name        
          avatarUrl      
        }      
        attachments {        
          id        
          url      
        }      
        createdAt    
      }    
      total    
      hasMore  
    }  
    linkPreview {   
     id    
     title    
     url    
     imageUrl  
   }  
   votesTotal  
   myVote  
   communities {    
     id    
     name    
     slug  
   }  
   attachments {    
     id    
     position    
     type    
     url  
   }  
   postMemberships {    
     id    
     pinned    
     community {      
       id    
     }  
   }  
   topics {    
     id    
     name    
     postsTotal    
     followersTotal  
   }  
   members {    
     total    
     hasMore    
     items {      
       id      
       name      
       avatarUrl      
       bio      
       tagline      
       location      
       skills (first: 100) {        
         items {          
           id          
           name        
         }      
       }    
     }  
   }  
 }
}
`


module.exports.getPostsQuery = `
query (  $sortBy: String,  $offset: Int,  $search: String,  $filter: String,  $topic: ID,  $first: Int) {  
  posts(  first: $first,  offset: $offset,  sortBy: $sortBy,  search: $search,  filter: $filter,  topic: $topic,  order: "desc") {  
    hasMore  
    items {      
      id  
      announcement  
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
      commenters(first: 3) {    
        id    
        name    
        avatarUrl  
      }  
      commentersTotal    
      linkPreview {    
        id    
        title    
        url    
        imageUrl  
      }  
      votesTotal  
      myVote  
      communities {    
        id    
        name    
        slug  
      }  
      attachments {    
        id    
        position    
        type    
        url  
      }  
      postMemberships {    
        id    
        pinned    
        community {      
          id    
        }  
      }  
      topics {    
        id    
        name    
        postsTotal    
        followersTotal  
      }  
      members {    
        total    
        hasMore    
        items {      
          id      
          name      
          avatarUrl      
          bio      
          tagline      
          location      
          skills (first: 100) {        
            items {          
              id          
              name        
            }      
          }    
        }  
      }  
    }
  }
}
`
