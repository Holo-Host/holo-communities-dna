use crate::schema::community::Community;
use crate::holochain_juniper::{call_cached, HID};
use juniper::{FieldResult, ID};
use crate::Context;
use serde_json::json;
use hdk::holochain_core_types::{
	error::HolochainError,
	json::JsonString,
};
use hdk::error::ZomeApiResult;
use std::convert::TryFrom;

/*
type Person {
  id: ID
  name: String
  avatarUrl: String
  ...
}
*/
#[derive(Constructor, Clone, PartialEq, Eq)]
pub struct Person {
    pub id: HID,
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct PersonEntry {
	pub name: String,
	pub avatar_url: String,
}

impl Person {
	fn retrieve_entry(&self) -> ZomeApiResult<PersonEntry> {
		let id: String = self.id.clone().into();
		let result = JsonString::from(call_cached("identity", "get_identity", json!({"agent_id": id}).into())?);
		let person_entry = PersonEntry::try_from(result)?;
		Ok(person_entry)
	}
}

graphql_object!(Person: Context |&self| {
	field id() -> FieldResult<ID> {
		// be careful. This field is the Hylo ID not the holochain ID
		Ok(self.id.to_string().into())
	}

	field name() -> FieldResult<String> {
  		Ok(self.retrieve_entry()?.name)
	}

	field avatarUrl() -> FieldResult<String> {
  		Ok(self.retrieve_entry()?.avatar_url)
	}

	field memberships(first: Option<i32>, cursor: Option<ID>, order: Option<String>) -> FieldResult<Vec<Membership>> {
		Ok(Vec::new())
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
pub struct PersonQuerySet {
    pub total: i32,
    pub items: Vec<Person>,
}
graphql_object!(PersonQuerySet: Context |&self| {
	field total() -> i32 {
		self.total
	}

	field hasMore() -> bool {
		false
	}

	field items() -> Option<Vec<Option<Person>>> {
		Some(self.items.iter().map(|item| Some(item.clone())).collect())
	}
});


#[derive(Constructor, Clone)]
pub struct Membership {
    pub id: HID,
}

graphql_object!(Membership: Context |&self| {
	field id() -> FieldResult<ID> {
		// be careful. This field is the Hylo ID not the holochain ID
		Ok(self.id.to_string().into())
	}

	field community() -> FieldResult<Community> {
		Ok(Community::new("".to_string().into()))
	}
});
