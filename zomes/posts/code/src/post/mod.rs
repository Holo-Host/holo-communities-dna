use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::{
        dna::entry_types::Sharing,
        entry::Entry,
        link::LinkMatch,
    },
    holochain_json_api::{
        error::JsonError,
        json::{
            JsonString,
            RawString,
        },
    },
    holochain_persistence_api::cas::content::{Address},
    utils,
    AGENT_ADDRESS,
};

// Core types

pub type Base = RawString;
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
impl PostEntry {
    pub fn with_address(&self, address: Address) -> Post {
        Post {
            address,
            title: self.title.clone(),
            details: self.details.clone(),
            post_type: self.post_type.clone(),
            creator: self.creator.clone(),
            announcement: self.announcement.clone(),
            timestamp: self.timestamp.clone(),
            base: self.base.clone(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Post {
    pub address: Address,
    pub title: String,
    pub details: String,
    pub post_type: String,
    pub creator: Address,
    pub announcement: bool,
    pub timestamp: String,
    pub base: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct GetPostsResult {
    posts: Vec<Post>,
    more: bool,
}
pub const POST_ENTRY_TYPE: &str = "post";
pub const POST_BASE_ENTRY: &str = "post_base";
pub const POST_LINK_TYPE: &str = "posted_in";

// API

pub fn get(address: Address) -> ZomeApiResult<Post> {
    utils::get_as_type::<PostEntry>(address.clone()).map(|post| post.with_address(address))
}

pub fn create(
    base: String,
    title: String,
    details: String,
    post_type: String,
    announcement: bool,
    timestamp: String,
) -> ZomeApiResult<Post> {
    let base_entry = Entry::App(POST_BASE_ENTRY.into(), RawString::from(base.clone()).into());
    let base_address = hdk::commit_entry(&base_entry)?;

    let post: PostEntry = PostEntry {
        title,
        details,
        post_type,
        creator: AGENT_ADDRESS.to_string().into(),
        announcement,
        timestamp,
        base
    };

    let post_address = hdk::commit_entry(
        &Entry::App (
            POST_ENTRY_TYPE.into(),
            post.clone().into()
        )
    )?;

    // link the post to its originating thing	
    hdk::link_entries(	
        &base_address,	
        &post_address,	
        POST_LINK_TYPE,	
        ""	
    )?;

    Ok(post.with_address(post_address))
}

pub fn all_for_base(base: String) -> ZomeApiResult<GetPostsResult> {
    let address = hdk::entry_address(&Entry::App(POST_BASE_ENTRY.into(), RawString::from(base).into()))?;
    // TODO: Returning { posts, more } response format in anticipation of pagination
    let posts = hdk::get_links(&address, LinkMatch::Exactly(POST_LINK_TYPE.into()), LinkMatch::Any)?
      .addresses()
      .iter()
      .rev()
      .map(|address| get(address.to_string().into()).unwrap())
      .collect();
    
    Ok(GetPostsResult { posts, more: false })
    // Ok(hdk::get_links(&address, LinkMatch::Exactly(POST_LINK_TYPE.into()), LinkMatch::Any)?
    //   .addresses()
    //   .iter()
    //   .map(|address| get(address.to_string().into()).unwrap())
    //   .collect()
    // )
}

pub fn post_def() -> ValidatingEntryType {
    entry!(
        name: POST_ENTRY_TYPE,
        description: "",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<PostEntry>| {
            Ok(())
        }
    )
}

pub fn base_def() -> ValidatingEntryType {
    entry!(
        name: POST_BASE_ENTRY,
        description: "Universally unique ID of something that is being posted in",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Base>| {
            Ok(())
        },
        links: [
            to!(
                POST_ENTRY_TYPE,
                link_type: POST_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}
