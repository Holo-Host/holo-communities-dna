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


/*
type MessageThread {
  id: ID
  createdAt: String
  updatedAt: String
  participants(first: i32, cursor: ID, order: String): [Person]
  participantsTotal: Int
  messages(first: Int, cursor: ID, order: String): MessageQuerySet
  unreadCount: Int
  lastReadAt: String
}
*/
#[derive(Constructor, Clone)]
pub struct MessageThread {
    pub id: HID,
}

graphql_object!(MessageThread: Context |&self| {
	field id(&executor) -> ID {
		self.id.clone().into()
	}

	field createdAt(&executor) -> String {
		"2019-01-14T07:52:22+0000".into()
	}

	field updatedAt(&executor) -> String {
		"2019-01-14T07:52:22+0000".into()
	}

  field participants(&executor, first: Option<i32>, cursor: Option<ID>, order: Option<String>) -> FieldResult<Vec<Option<Person>>> {
      let result = call_cached("chat", "get_thread_participants", json!({"thread_addr": self.id.to_string()}).into())?;
      let person_ids: Vec<serde_json::Value> = result.as_array().unwrap().to_vec();

      Ok(person_ids.iter().map(|id| Some(Person{
      	id: id.as_str().unwrap().to_string().into(),
      })).collect())
  }

  field messages(&executor, first: Option<i32>, cursor: Option<ID>, order: Option<String>) -> FieldResult<MessageQuerySet> {
  	// let message_ids = thread::get_thread_messages(executor.context().cache.borrow_mut(), &self.id.to_string().into())?;
   //    Ok(MessageQuerySet{
   //    	total: message_ids.len() as i32,
   //    	items: message_ids.into_iter().map(|id| Message{
   //    		id: id.into(),
   //    	}).collect()
   //    })
    Ok(MessageQuerySet{
      total: 0,
      items: Vec::new()
    }) 
  }

  field unreadCount(&executor) -> i32 {
  	0
  }

  field lastReadAt(&executor) -> String {
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
  field total(&executor) -> i32 {
    self.total
  }

  field hasMore(&executor) -> bool {
    false
  }

  field items(&executor) -> Option<Vec<Option<MessageThread>>> {
    Some(self.items.iter().map(|item| Some(item.clone())).collect())
  }
});