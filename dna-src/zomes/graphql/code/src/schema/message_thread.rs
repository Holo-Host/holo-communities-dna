use juniper::{FieldResult, ID};
use serde_json::{
  self,
  json,
};

use crate::holochain_juniper::call_cached;
use crate::holochain_juniper::HID;
use crate::Context;

use super::person::Person;
use super::message::{Message, MessageQuerySet};


#[derive(Constructor, Clone)]
pub struct MessageThread {
    pub id: HID,
}

graphql_object!(MessageThread: Context |&self| {
	field id() -> ID {
		self.id.clone().into()
	}

	field createdAt() -> String {
		"2019-01-14T07:52:22+0000".into()
	}

	field updatedAt() -> String {
		"2019-01-14T07:52:22+0000".into()
	}

  field participants(first: Option<i32>, cursor: Option<ID>, order: Option<String>) -> FieldResult<Vec<Option<Person>>> {
      let result = call_cached("chat", "get_thread_participants", json!({"thread_address": self.id.to_string()}).into())?;
      let person_ids: Vec<serde_json::Value> = result.as_array().unwrap().to_vec();

      Ok(person_ids.iter().map(|id| Some(Person{
      	id: id.as_str().unwrap().to_string().into(),
      })).collect())
  }

  field participantsTotal() -> FieldResult<i32> {
    let result = call_cached("chat", "get_thread_participants", json!({"thread_address": self.id.to_string()}).into())?;
    Ok(result.as_array().unwrap().to_vec().len() as i32)
  }

  field messages(first: Option<i32>, cursor: Option<ID>, order: Option<String>) -> FieldResult<MessageQuerySet> {
  	let result = call_cached("chat", "get_thread_messages", json!({"thread_address": self.id.to_string()}).into())?;
    let message_ids: Vec<serde_json::Value> = result.as_array().unwrap().to_vec();

    Ok(MessageQuerySet{
    	total: message_ids.len() as i32,
    	items: message_ids.into_iter().map(|id| Message{
    		id: id.as_str().unwrap().to_string().into(),
    	}).collect()
    })
  }

  field unreadCount() -> i32 {
  	0
  }

  field lastReadAt() -> String {
	"".into()
  }
});



/*
type MessageThreadQuerySet {
  total: Int
  hasMore: Boolean
  items: [Person]
}
*/
#[derive(Constructor, Clone)]
pub struct MessageThreadQuerySet {
    pub total: i32,
    pub items: Vec<MessageThread>,
}
graphql_object!(MessageThreadQuerySet: Context |&self| {
  field total() -> i32 {
    self.total
  }

  field hasMore() -> bool {
    false
  }

  field items() -> Option<Vec<Option<MessageThread>>> {
    Some(self.items.iter().map(|item| Some(item.clone())).collect())
  }
});