
use hdk::{
    self,
    utils,
    error::ZomeApiResult,
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

pub type Base = RawString;

const POST_BASE_ENTRY: &str = "post_base";
const POST_LINK_TAG: &str = "posted_in";

pub fn get_post(address: Address) -> ZomeApiResult<Post> {
    utils::get_as_type(address)
}

pub fn create_post(base: String, title: String, details: String, post_type: String, announcement: bool, timestamp: String) -> ZomeApiResult<Address> {

    let base_entry = Entry::App(POST_BASE_ENTRY.into(), RawString::from(base.clone()).into());
    let base_address = hdk::commit_entry(&base_entry)?;

    let post_address = hdk::commit_entry(
        &Entry::App (
            "post".into(),
            Post {
                title,
                details,
                post_type,
                creator: AGENT_ADDRESS.to_string().into(),
                announcement,
                timestamp,
                base
            }.into()
        )
    )?;

    // link the post to its originating thing
    hdk::link_entries(
        &base_address,
        &post_address,
        POST_LINK_TAG,
    )?;

    Ok(post_address)
}

pub fn get_posts(base: String) -> ZomeApiResult<Vec<Address>> {
    let address = hdk::entry_address(&Entry::App(POST_BASE_ENTRY.into(), RawString::from(base).into()))?;
    Ok(hdk::get_links(&address, POST_LINK_TAG)?.addresses().to_vec())
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
                tag: POST_LINK_TAG,
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
