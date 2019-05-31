use crate::schema::post::Post;
use crate::holochain_juniper::{call_cached, HID};
use juniper::{FieldResult, ID};
use crate::schema::post::PostQuerySet;
use serde_json::json;
use crate::Context;
use hdk::holochain_core_types::{
	error::HolochainError,
	json::JsonString,
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
	field id() -> ID {
		self.id.clone().into()
	}

	field name() -> FieldResult<String> {
		Ok(self.retrieve_entry()?.name)
	}

	field slug() -> FieldResult<String> {
		Ok(self.retrieve_entry()?.slug)
	}

  	/// All parameters for this are ignored and it will return all the posts regardless
  	field posts(
	    first: Option<i32>,
	    order: Option<String>,
	    sort_by: Option<String>,
	    offset: Option<i32>,
	    search: Option<String>,
	    filter: Option<String>,
	    topic: Option<ID>
	  ) -> FieldResult<PostQuerySet> {

	  	let slug = self.retrieve_entry()?.slug;
  		let result = call_cached("posts", "get_posts", json!({"base": slug}).into())?;
  		let post_ids: Vec<serde_json::Value> = result.as_array().unwrap().to_vec();

	    Ok(PostQuerySet{
	    	total: post_ids.len() as i32,
	    	items: post_ids.into_iter().map(|id| Post{
	    		id: id.as_str().unwrap().to_string().into(),
	    	}).collect()
	    })
  	}

});
