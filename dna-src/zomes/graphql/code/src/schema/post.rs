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
use super::person::Person;

use crate::schema::comment::{
	Comment,
	CommentQuerySet,
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
		let result = JsonString::from(call_cached("posts", "get_post", json!({"address": id}).into())?);
		let post_entry = PostEntry::try_from(result)?;
		Ok(post_entry)
	}
}

graphql_object!(Post: Context |&self| {
	field id(&executor) -> ID {
		self.id.clone().into()
	}

	field creator(&executor) -> FieldResult<Person> {
  		let id: String = self.retrieve_entry()?.creator.to_string();
  		Ok(Person{id: id.into()})
  	}

	field type(&executor) -> FieldResult<String> {
		Ok(self.retrieve_entry()?.post_type)
	}

	field title(&executor) -> FieldResult<String> {
  		Ok(self.retrieve_entry()?.title)
  	}

	field details(&executor) -> FieldResult<String> {
  		Ok(self.retrieve_entry()?.details)
  	}

	field comments(&executor, first: Option<i32>, cursor: Option<ID>, order: Option<String>) -> FieldResult<CommentQuerySet> {

		let result = call_cached("comments", "get_comments",
			json!({
				"base": self.id.to_string()
			}).into()
		)?;

		// hdk::debug(result.clone())?;

	    let comment_ids: Vec<serde_json::Value> = result.as_array()
	    	.ok_or(FieldError::new(
	    		format!("Could not parse get comments response: {}", result),
	    		graphql_value!({ "internal_error": "Could not parse" })
	    		))?
	    	.to_vec();

	    Ok(CommentQuerySet{
	    	total: comment_ids.len() as i32,
	    	items: comment_ids.into_iter().map(|id| Comment{
	    		id: id.as_str().unwrap().to_string().into(),
	    	}).collect()
	    })
	}

	field createdAt(&executor) -> String {
		"2019-01-14T07:52:22+0000".into()
	}

	field updatedAt(&executor) -> String {
		"2019-01-14T07:52:22+0000".into()
	}
});


#[derive(Constructor, Clone)]
pub struct PostQuerySet {
    pub total: i32,
    pub items: Vec<Post>,
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
