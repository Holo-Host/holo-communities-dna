use hdk::holochain_core_types::error::HolochainError;
use hdk::holochain_core_types::json::JsonString;
use hdk::error::ZomeApiResult;
use juniper::{FieldResult, ID};
use serde_json::json;
use std::convert::TryFrom;

use crate::Context;
use crate::holochain_juniper::{HID, call_cached};

use super::person::Person;
use super::message_thread::MessageThread;


#[derive(Constructor, Clone)]
pub struct Message {
    pub id: HID,
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct MessageEntry {
    pub timestamp: String,
    pub text: String,
    pub thread_id: String,
    pub creator: String,
}

impl Message {
	fn retrieve_entry(&self) -> ZomeApiResult<MessageEntry> {
		let id: String = self.id.clone().into();
		let result = JsonString::from(call_cached("chat", "get_message", json!({"message_addr": id}).into())?);
		let message_entry = MessageEntry::try_from(result)?;
		Ok(message_entry)
	}
}

graphql_object!(Message: Context |&self| {
	field id() -> ID {
		self.id.clone().into()
	}

	field text() -> FieldResult<String> {
		Ok(self.retrieve_entry()?.text)
	}

	field creator() -> FieldResult<Person> {
		let id: String = self.retrieve_entry()?.creator;
		Ok(Person{id: id.into()})
	}

	field messageThread() -> FieldResult<MessageThread> {
		Ok(MessageThread{id: self.retrieve_entry()?.thread_id.into()})
	}

	field createdAt() -> FieldResult<String> {
		Ok(self.retrieve_entry()?.timestamp)
	}
});

/*
type MessageQuerySet {
  total: Int
  hasMore: Boolean
  items: [Message]
}
*/
#[derive(Constructor, Clone)]
pub struct MessageQuerySet {
    pub total: i32,
    pub items: Vec<Message>,
}

graphql_object!(MessageQuerySet: Context |&self| {
	field total() -> i32 {
		self.total
	}

	field hasMore() -> bool {
		false
	}

	field items() -> Option<Vec<Option<Message>>> {
		Some(self.items.iter().map(|item| Some(item.clone())).collect())
	}
});