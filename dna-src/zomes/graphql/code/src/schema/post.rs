use crate::holochain_juniper::{call_cached, HID};
use juniper::{FieldResult, FieldError, ID};
use serde_json::json;
use crate::Context;

use crate::schema::comment::{
	Comment,
	CommentQuerySet,
};

#[derive(Constructor, Clone)]
pub struct Post {
    pub id: HID,
}

graphql_object!(Post: Context |&self| {
	field id(&executor) -> ID {
		self.id.clone().into()
	}

	field comments(&executor, first: Option<i32>, cursor: Option<ID>, order: Option<String>) -> FieldResult<CommentQuerySet> {
		
		let result = call_cached("comments", "get_comments", 
			json!({
				"base": self.id.to_string()
			}).into()
		)?;

		hdk::debug(result.clone())?;

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
});
