use crate::schema::community::Community;
use crate::holochain_juniper::HID;
use juniper::{FieldResult, ID};
use crate::Context;
use crate::identity;

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
graphql_object!(Person: Context |&self| {
	field id() -> FieldResult<ID> {
		// be careful. This field is the Hylo ID not the holochain ID
		Ok(self.id.to_string().into())
	}

	field name() -> FieldResult<String> {
		Ok(identity::get_identity(self.id.to_string().into())?.name)
	}

	field avatarUrl() -> FieldResult<String> {
		Ok(identity::get_identity(self.id.to_string().into())?.avatar_url)
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
