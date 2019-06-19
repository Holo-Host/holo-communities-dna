use juniper::{FieldResult, ID};
use hdk::AGENT_ADDRESS;
use hdk::error::ZomeApiResult;
use hdk::holochain_core_types::{
	error::HolochainError,
	json::JsonString,
};
use crate::Context;
use crate::holochain_juniper::call_cached;
use serde_json::json;
// use holochain_core_types_derive::{ DefaultJson };
use std::convert::TryFrom;

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

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct PersonEntry {
	pub name: String,
	pub avatar_url: String,
}

impl Me {
	fn retrieve_entry(&self) -> ZomeApiResult<PersonEntry> {
		let id: String = AGENT_ADDRESS.to_string().into();
		let result = JsonString::from(call_cached("identity", "get_identity", json!({"agent_id": id}).into())?);
		let person_entry = PersonEntry::try_from(result)?;
		Ok(person_entry)
	}
}

graphql_object!(Me: Context |&self| {
	field id() -> FieldResult<ID> {
		Ok(AGENT_ADDRESS.to_string().into())		
	}

field name() -> FieldResult<Option<String>> {
		match self.retrieve_entry() {
			Ok(identity) => {Ok(Some(identity.name))},
			Err(err) => {Ok(None)}
		}
	}

	field avatarUrl() -> FieldResult<Option<String>> {
		match self.retrieve_entry() {
			Ok(identity) => {Ok(Some(identity.avatar_url))},
			Err(err) => {Ok(None)}
		}
	}

	field isRegistered() -> FieldResult<bool> {
		let result = serde_json::from_value(call_cached("identity", "is_registered", json!({}).into())?).unwrap();
		Ok(result)
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
