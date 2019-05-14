use juniper::{FieldResult, ID};
use hdk::AGENT_ADDRESS;

use crate::identity;
use crate::Context;
use crate::holochain_juniper::call_cached;
use serde_json::json;

use super::message_thread::{MessageThread, MessageThreadQuerySet};

/*
type Me {
  id: ID
  ...
  messageThreads(first: Int, offset: Int, order: String, sortBy: String): MessageThreadQuerySet
  ...
}
*/
pub struct Me;
graphql_object!(Me: Context |&self| {
	field id() -> FieldResult<ID> {
		Ok(AGENT_ADDRESS.to_string().into())		
	}

	field name() -> FieldResult<Option<String>> {
		match identity::get_identity(AGENT_ADDRESS.to_string().into()) {
			Ok(identity) => {Ok(Some(identity.name))},
			Err(err) => {Ok(None)}
		}
	}

	field avatarUrl() -> FieldResult<Option<String>> {
		match identity::get_identity(AGENT_ADDRESS.to_string().into()) {
			Ok(identity) => {Ok(Some(identity.avatar_url))},
			Err(err) => {Ok(None)}
		}
	}

	field isRegistered() -> FieldResult<bool> {
		Ok(identity::get_identity(AGENT_ADDRESS.to_string().into()).is_ok())
	}

	field messageThreads(first: Option<i32>, offset: Option<i32>, order: Option<String>, sort_by: Option<String>) -> FieldResult<MessageThreadQuerySet> {
		let result = call_cached("chat", "get_my_threads", json!({}).into())?;
		let result_vec = result.as_array().unwrap();
		Ok(MessageThreadQuerySet{
			total: result_vec.len() as i32,
			items: result_vec.into_iter().map(|id| MessageThread{id: id.as_str().unwrap().to_string().into()}).collect(),
		})
	}
});
