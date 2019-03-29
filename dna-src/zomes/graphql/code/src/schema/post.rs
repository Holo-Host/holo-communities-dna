use std::convert::TryFrom;
use crate::holochain_juniper::{call_cached, HID};
use juniper::{FieldResult, ID};
use serde_json::json;
use hdk::{
	error::ZomeApiResult,
	holochain_core_types::{
		json::JsonString,
		cas::content::Address,
		error::HolochainError,
	},
};

use crate::Context;
use crate::schema::person::{
	Person,
	PersonQuerySet
};

#[derive(Constructor, Clone)]
pub struct Post {
    pub id: HID,
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct PostEntry {
    pub title: String,
    pub details: String,
    pub post_type: String,
    pub creator: Address,
    pub announcement: bool,
    pub timestamp: String,
}

impl Post {
	fn retrieve_entry(&self) -> ZomeApiResult<PostEntry> {
		let id: String = self.id.clone().into();
		let result = JsonString::from(call_cached("chat", "get_message", json!({"message_addr": id}).into())?);
		let message_entry = PostEntry::try_from(result)?;
		Ok(message_entry)
	}
}

graphql_object!(Post: Context |&self| {
	field id(&executor) -> ID {
		self.id.clone().into()
	}

	field announcement(&executor) -> FieldResult<bool> {
		Ok(self.retrieve_entry()?.announcement)
	}

	field title(&executor) -> FieldResult<String> {
		Ok(self.retrieve_entry()?.title)
	}

	field details(&executor) -> FieldResult<String> {
		Ok(self.retrieve_entry()?.details)
	}

	field type(&executor) -> FieldResult<String> {
		Ok(self.retrieve_entry()?.post_type)
	}

	field creator(&executor) -> FieldResult<Person> {
		let id = self.retrieve_entry()?.creator;
		Ok(Person{id: id.into()})
	}

	field createdAd(&executor) -> FieldResult<String> {
		Ok(String::from(""))
	}

	field updatedAt(&executor) -> FieldResult<String> {
		Ok(self.retrieve_entry()?.timestamp)
	}

	field commenters(&executor) -> FieldResult<Vec<Person>> {
		Ok(Vec::new())
	}

	field commentersTotal(&executor) -> FieldResult<i32> {
		Ok(0)
	}

	field linkPreview(&executor) -> FieldResult<String> {
		Ok(String::from(""))

	}

	field votesTotal(&executor) -> FieldResult<String> {
		Ok(String::from(""))

	}

	field myVote(&executor) -> FieldResult<String> {
		Ok(String::from(""))

	}

	field communities(&executor) -> FieldResult<Vec<String>> {
		Ok(Vec::new())
	}

	field attachments(&executor) -> FieldResult<String> {
		Ok(String::from(""))

	}

	field postMemberships(&executor) -> FieldResult<String> {
		Ok(String::from(""))

	}

	field topics(&executor) -> FieldResult<String> {
		Ok(String::from(""))
	}

	field members(&executor) -> FieldResult<PersonQuerySet> {
		Ok(PersonQuerySet{
			items: Vec::new(),
			total: 0
		})
	}
});


/*
type PersonQuerySet {
  total: Int
  hasMore: Boolean
  items: [Person]
}
*/
#[derive(Constructor, Clone)]
struct PostQuerySet {
	total: i32,
	items: Vec<Post>
}
graphql_object!(PostQuerySet: Context |&self| {
	field total(&executor) -> i32 {
		self.total
	}

	field hasMore(&executor) -> bool {
		false
	}

	field items(&executor) -> Option<Vec<Option<Post>>> {
		Some(self.items.iter().map(|item| Some(item.clone())).collect())
	}
});
