
use hdk::{
    self,
    utils,
    error::{ZomeApiError, ZomeApiResult},
    AGENT_ADDRESS,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing, error::HolochainError,
        json::JsonString,
        json::RawString,
        cas::content::Address,
        entry::Entry,
    },
};

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Post {
    pub title: String,
    pub details: String,
    pub post_type: String,
    pub creator: Address,
    pub announcement: bool,
    pub timestamp: String,
    pub base: String,
}

impl Post {
    pub fn with_address(&self, address: Address) -> PostWithAddress {
        PostWithAddress {
            address,
            title: self.title.clone(),
            details: self.details.clone(),
            post_type: self.post_type.clone(),
            creator: self.creator.clone(),
            announcement: self.announcement.clone(),
            timestamp: self.timestamp.clone(),
            base: self.base.clone()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct PostWithAddress {
    pub address: Address,
    pub title: String,
    pub details: String,
    pub post_type: String,
    pub creator: Address,
    pub announcement: bool,
    pub timestamp: String,
    pub base: String,
}

pub type Base = RawString;

const POST_BASE_ENTRY: &str = "post_base";
const POST_LINK_TYPE: &str = "posted_in";

pub fn get(address: Address) -> ZomeApiResult<PostWithAddress> {
    let post: Result<Post, _> = utils::get_as_type(address.clone());

        match post {
        Ok(post) => {
            Ok(post.with_address(address))
        },
        Err(_err) => {
            Err(ZomeApiError::Internal("Post not found".into()))
        }
    }
}

pub fn create(base: String, title: String, details: String, post_type: String, announcement: bool, timestamp: String) -> ZomeApiResult<PostWithAddress> {

    let base_entry = Entry::App(POST_BASE_ENTRY.into(), RawString::from(base.clone()).into());
    let base_address = hdk::commit_entry(&base_entry)?;

    let post: Post = Post {
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
            "post".into(),
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

pub fn all_for_base(base: String) -> ZomeApiResult<Vec<PostWithAddress>> {
    let address = hdk::entry_address(&Entry::App(POST_BASE_ENTRY.into(), RawString::from(base).into()))?;
    Ok(hdk::get_links(&address, Some(POST_LINK_TYPE.into()), None)?
        .addresses()
        .iter()
        .map(|address| get(address.to_string().into()).unwrap())
        .collect()
    )
}

pub fn post_def() -> ValidatingEntryType {
    entry!(
        name: "post",
        description: "",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<Post>| {
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
                "post",
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
