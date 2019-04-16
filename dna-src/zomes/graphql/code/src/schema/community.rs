use crate::holochain_juniper::{call_cached, HID};
use juniper::{FieldResult, FieldError, ID};
use serde_json::json;
use crate::Context;
use hdk::holochain_core_types::{
	error::HolochainError,
	json::JsonString,
	cas::content::Address,
};
use hdk::error::ZomeApiResult;
use std::convert::TryFrom;

#[derive(Constructor, Clone)]
pub struct Community {
    pub id: HID,
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct CommunityEntry {
	pub name: String,
    pub slug: String,
}

impl Community {
	fn retrieve_entry(&self) -> ZomeApiResult<CommunityEntry> {
		let id: String = self.id.clone().into();
		let result = JsonString::from(call_cached("community", "get_community", json!({"address": id}).into())?);
		let community_entry = CommunityEntry::try_from(result)?;
		Ok(community_entry)
	}
}

graphql_object!(Community: Context |&self| {
	field id(&executor) -> ID {
		self.id.clone().into()
	}

	field name(&executor) -> FieldResult<String> {
  		Ok(self.retrieve_entry()?.name)
  	}

	field slug(&executor) -> FieldResult<String> {
  		Ok(self.retrieve_entry()?.slug)
  	}
});
