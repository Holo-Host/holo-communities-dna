use hdk::error::ZomeApiResult;
use juniper::{FieldResult, ID};
use serde_json::json;

use crate::Context;
use crate::holochain_juniper::{HID, call_cached};

use super::person::Person;
use super::message_thread::MessageThread;


#[derive(Constructor, Clone)]
pub struct Message {
    pub id: HID,
}

fn retrieve_message(m: &Message) -> ZomeApiResult<serde_json::Value> {
	let id: String = m.id.clone().into();
	call_cached("chat", "get_message", json!({"message_addr": id}).into())
}

graphql_object!(Message: Context |&self| {
	field id(&executor) -> ID {
		self.id.clone().into()
	}

	field text(&executor) -> FieldResult<String> {
		retrieve_message(self)?
			.get("text")
			.map(|s| String::from(s.as_str().unwrap()))
			.ok_or("Could not retrieve field of message".into())
	}

	field creator(&executor) -> FieldResult<Person> {
		let id: String = retrieve_message(self)?
			.get("creator")
			.map(|s| String::from(s.as_str().unwrap()))
			.ok_or("Could not retrieve field of message")?;
		Ok(Person{id: id.into()})
	}

	field messageThread(&executor) -> FieldResult<MessageThread> {
		let id: String = retrieve_message(self)?
			.get("message_thread")
			.map(|s| String::from(s.as_str().unwrap()))
			.ok_or("Could not retrieve field of message")?;
		Ok(MessageThread{id: "".to_string().into()})
	}

	field createdAt(&executor) -> String {
		"2019-01-14T07:52:22+0000".into()
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
	field total(&executor) -> i32 {
		self.total
	}

	field hasMore(&executor) -> bool {
		false
	}

	field items(&executor) -> Option<Vec<Option<Message>>> {
		Some(self.items.iter().map(|item| Some(item.clone())).collect())
	}
});