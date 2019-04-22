use itertools::Itertools;

use crate::holochain_juniper::{call_cached, HID};
use juniper::{FieldResult, ID};
use serde_json::json;
use crate::Context;
use hdk::holochain_core_types::{
	error::HolochainError,
	json::JsonString,
	cas::content::Address,
};
use hdk::error::{ZomeApiResult, ZomeApiError};
use std::convert::TryFrom;
use super::person::Person;

use crate::schema::{
	comment::{
		Comment,
		CommentQuerySet,
	},
	community::Community,
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
    pub base: String,
}

impl Post {
	fn retrieve_entry(&self) -> ZomeApiResult<PostEntry> {
		let result = JsonString::from(call_cached("posts", "get_post", 
			json!({
				"address": self.id.to_string()
			}).into())?
		);
		let post_entry = PostEntry::try_from(result)?;
		Ok(post_entry)
	}

	fn get_comments(&self) -> ZomeApiResult<Vec<Comment>> {
		let result = call_cached("comments", "get_comments",
			json!({
				"base": self.id.to_string()
			}).into()
		)?;

	    Ok(
	    	result.as_array()
	    	.ok_or(ZomeApiError::Internal(
	    		format!("Could not parse get comments response: {}", result)
	    	))?
	    	.to_vec()
	    	.into_iter()
	    	.map(|id| Comment{
	    		id: id.as_str().unwrap().to_string().into(),
	    	}).collect()
	    )
	}

	fn get_commenters(&self) -> ZomeApiResult<Vec<Person>> {
		Ok(self.get_comments()?
			.iter()
			.map(|comment| {
				let id: String = comment.retrieve_entry().unwrap().creator;
	    		Person{id: id.into()}
			})
			.dedup()
			.collect()
		)
	}
}

graphql_object!(Post: Context |&self| {
	field id() -> ID {
		self.id.clone().into()
	}

	field creator() -> FieldResult<Person> {
  		let id: String = self.retrieve_entry()?.creator.to_string();
  		Ok(Person{id: id.into()})
  	}

	field type() -> FieldResult<String> {
		Ok(self.retrieve_entry()?.post_type)
	}

	field title() -> FieldResult<String> {
  		Ok(self.retrieve_entry()?.title)
  	}

	field details() -> FieldResult<String> {
  		Ok(self.retrieve_entry()?.details)
  	}

	field comments(first: Option<i32>, cursor: Option<ID>, order: Option<String>) -> FieldResult<CommentQuerySet> {
		let comments = self.get_comments()?;
	    Ok(CommentQuerySet{
	    	total: comments.len() as i32,
	    	items: comments
	    })
	}

	field createdAt() -> FieldResult<String> {
  		Ok(self.retrieve_entry()?.timestamp)
	}

	field updatedAt() -> FieldResult<String> {
  		Ok(self.retrieve_entry()?.timestamp)
	}

	field commenters(first: Option<i32>, cursor: Option<ID>, order: Option<String>) -> FieldResult<Option<Vec<Option<Person>>>> {
		Ok(Some(
			self.get_commenters()?
			.into_iter()
			.map(Some)
			.collect()
		))
	}

	field commentersTotal() -> FieldResult<i32> {
		Ok(self.get_commenters()?.len() as i32)
	}

	field communities(first: Option<i32>, cursor: Option<ID>, order: Option<String>) -> FieldResult<Option<Vec<Option<Community>>>> {
  		let community_slug = self.retrieve_entry()?.base;
  		let id: HID = call_cached("community", "get_community_address_by_slug", json!(
            {
                "slug": community_slug,
            }
        ).into())?.as_str().unwrap().into();

        hdk::debug(id.clone())?;

		Ok(Some(
			vec![Some(Community{id})]
		))
	}

});


#[derive(Constructor, Clone)]
pub struct PostQuerySet {
    pub total: i32,
    pub items: Vec<Post>,
}

graphql_object!(PostQuerySet: Context |&self| {
	field total() -> i32 {
		self.total
	}

	field hasMore() -> bool {
		false
	}

	field items() -> Option<Vec<Option<Post>>> {
		Some(self.items.iter().map(|item| Some(item.clone())).collect())
	}
});
