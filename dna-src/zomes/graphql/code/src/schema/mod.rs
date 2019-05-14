use juniper::{FieldError, FieldResult, Value, ID};
use serde_json::json;
use crate::holochain_juniper::call_cached;
use crate::identity;
use crate::Context;

mod person;
mod message_thread;
mod message;
mod comment;
mod me;
mod post;
mod community;

use person::{Person, PersonQuerySet};
use message::{Message};
use comment::{Comment};
use message_thread::{MessageThread};
use me::Me;
use post::Post;
use community::Community;

use hdk::{
    AGENT_ADDRESS,
};

/*=====================================
=            Input Objects            =
=====================================*/

#[derive(GraphQLInputObject)]
struct MessageThreadInput {
    participant_ids: Option<Vec<Option<String>>>,
	created_at: Option<String>
}

#[derive(GraphQLInputObject)]
struct MessageInput {
    message_thread_id: Option<String>,
    text: Option<String>,
	created_at: Option<String>
}

#[derive(GraphQLInputObject)]
struct CommentInput {
    post_id: Option<String>,
    text: Option<String>,
	created_at: Option<String>
}

#[derive(GraphQLInputObject)]
struct PostInput {
    title: Option<String>,
    details: Option<String>,
    #[graphql(name="type", description="The post type")]
    r#type: Option<String>,
    community_slug: Option<String>,
    created_at: Option<String>
}

#[derive(GraphQLInputObject)]
struct CommunityInput {
    name: Option<String>,
	slug: Option<String>
}
/*=====  End of Input Objects  ======*/


/*
 * This is the macro for defining the query schema for the graphQL provider
 * Each field is something that can be queried. These take parameters to filter as needed
 */

pub struct Query;
graphql_object!(Query: Context |&self| {

    field apiVersion() -> FieldResult<String> {
        Ok("0.0.1".to_string())
    }

    field me() -> FieldResult<Me> {
    	Ok(Me)
    }

    field messageThread(id: Option<ID>) -> FieldResult<MessageThread> {
    	match id {
    		Some(id) => Ok(MessageThread{id: id.into()}),
    		None => Err(FieldError::new("Must call with an id parameter", Value::Null))
    	}
    }

    field people(
        &executor,
	    first: Option<i32>,
	    order: Option<String>,
	    sort_by: Option<String>,
	    offset: Option<i32>,
	    search: Option<String>,
	    autocomplete: Option<String>,
	    filter: Option<String>
	) -> FieldResult<PersonQuerySet> {
    	let people: Vec<Person> = identity::get_people()?
	    	.into_iter()
	    	.map(|id| {
        		Person {
	    			id: id.into(),
    			}
	    	})
    		.collect();
    	Ok(
    		PersonQuerySet{
    			total: people.len() as i32,
    			items: people,
    		}
    	)
	}

    field community(id: Option<ID>, slug: Option<String>) -> FieldResult<Community> {
        // TODO: look up community by slug optionally
        let id: Option<ID> = match slug {
            Some(slug) => {
                let address = call_cached("community", "get_community_address_by_slug", 
                    json!({
                        "slug": slug,
                    }).into())?;
                Some(address.as_str().unwrap().to_string().into())
            },
            None => id
        };

        match id {
            Some(id) => Ok(Community{id: id.into()}),
            None => Err(FieldError::new("Must call with either an id parameter or a slug", Value::Null))
        }

    }

    field post(id: Option<ID>) -> FieldResult<Post> {
        match id {
            Some(id) => Ok(Post{id: id.into()}),
            None => Err(FieldError::new("Must call with an id parameter", Value::Null))
        }
    }

    field person(id: Option<ID>) -> FieldResult<Person> {
        match id {
            Some(id) => Ok(Person{id: id.into()}),
            None => Err(FieldError::new("Must call with an id parameter", Value::Null))
        }
    }

    field comment(id: Option<ID>) -> FieldResult<Comment> {
        match id {
            Some(id) => Ok(Comment{id: id.into()}),
            None => Err(FieldError::new("Must call with an id parameter", Value::Null))
        }
    }
});

/*
 * This mutation object is what allows the consumer to change the data stored in the store
 * In holochain the store is the DHT. You also need to be sure you allow some pattern (such as links)
 * such that the values can be retrieved again later
 */

pub struct Mutation;
graphql_object!(Mutation: Context |&self| {

    field createMessage(data: Option<MessageInput>) -> FieldResult<Message> {
        let data = data.unwrap();
        let id = call_cached("chat", "post_message_to_thread", json!({
            "thread_addr": data.message_thread_id.unwrap(),
            "text": data.text.unwrap_or("".into()),
            "timestamp": data.created_at.unwrap_or("".into())
        }).into())?;
    	Ok(Message{
    		id: id.as_str().unwrap().to_string().into()
    	})
    }

    field findOrCreateThread(data: Option<MessageThreadInput>) -> FieldResult<MessageThread> {
    	let participant_agent_ids: Vec<String> = data.unwrap().participant_ids.unwrap().into_iter().map(|elem| elem.unwrap()).collect();

        let result_value = call_cached("chat", "get_or_create_thread", json!({"participant_ids": participant_agent_ids}).into())?;
        // hdk::debug(result_value.clone())?;
        return Ok(MessageThread {
            id: result_value.as_str().unwrap().to_string().into()
        })
    }

    field registerUser(name: Option<String>, avatar_url: Option<String>) -> FieldResult<Person> {
    	let id = identity::register_user(
    		name.unwrap_or("?".into()),
    		avatar_url.unwrap_or("".into())
    	)?;
    	Ok(Person { id: AGENT_ADDRESS.to_string().into() })
    }

    field createComment(data: Option<CommentInput>) -> FieldResult<Comment> {
        let data = data.unwrap();
        let id = call_cached("comments", "create_comment", json!({
            "comment": {
                "base": data.post_id.unwrap(),
                "text": data.text.unwrap_or("".into()),
                "timestamp": data.created_at.unwrap_or("2019-01-14T07:52:22+0000".into())
            }
        }).into())?;
    	Ok(Comment{
    		id: id.as_str().unwrap().to_string().into()
    	})
    }

    field createPost(data: Option<PostInput>) -> FieldResult<Post> {
        let data = data.unwrap();
        let id = call_cached("posts", "create_post", json!(
            {
                "base": data.community_slug.unwrap(),
                "post_type": data.r#type.unwrap(),
                "title": data.title.unwrap(),
                "details": data.details.unwrap_or("".into()),
                "announcement": false,
                "timestamp": data.created_at.unwrap_or("1970-01-01T00:00:00Z".into())
            }
        ).into())?;

        Ok(Post{
            id: id.as_str().unwrap().to_string().into()
        })
    }

    field createCommunity(data: Option<CommunityInput>) -> FieldResult<Community> {
        let data = data.unwrap();
        let id = call_cached("community", "create_community", json!(
            {
                "name": data.name.unwrap_or("".into()),
                "slug": data.slug.unwrap_or("".into())
            }
        ).into())?;

        Ok(Community{
            id: id.as_str().unwrap().to_string().into()
        })
    }
});


// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
pub type Schema = juniper::RootNode<'static, Query, Mutation>;
