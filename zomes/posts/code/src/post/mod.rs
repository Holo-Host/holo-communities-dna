
use hdk::{
    self,
    utils,
    error::ZomeApiResult,
    AGENT_ADDRESS,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing,
        entry::Entry,
        link::LinkMatch,
    },
    holochain_json_api::{
        error::JsonError,
        json::{JsonString,RawString},
    },
    holochain_persistence_api::{cas::content::Address},
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

const POST_ENTRY_TYPE: &str = "post";
const POST_BASE_ENTRY: &str = "post_base";
const POST_LINK_TYPE: &str = "posted_in";


pub fn get(address: Address) -> ZomeApiResult<PostWithAddress> {
    utils::get_as_type::<Post>(address.clone())
        .map(|post| {
            post.with_address(address)
        })
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

pub fn all_for_base(base: String) -> ZomeApiResult<Vec<PostWithAddress>> {
    let address = hdk::entry_address(&Entry::App(POST_BASE_ENTRY.into(), RawString::from(base).into()))?;
    Ok(hdk::get_links(&address, LinkMatch::Exactly(POST_LINK_TYPE.into()), LinkMatch::Any)?
        .addresses()
        .iter()
        .map(|address| get(address.to_string().into()).unwrap())
        .collect()
    )
}

pub fn post_def() -> ValidatingEntryType {
    entry!(
        name: POST_ENTRY_TYPE,
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
