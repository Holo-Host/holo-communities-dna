
use hdk::{
    self,
    utils,
    error::ZomeApiResult,
    AGENT_ADDRESS,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing, error::HolochainError,
        json::JsonString,
        cas::content::Address,
        entry::Entry,
    },
};

const COMMENTER_LINK_TAG: &str = "commenter";

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Post {
    pub title: String,
    pub details: String,
    pub post_type: String,
    pub creator: Address,
    pub announcement: bool,
    pub timestamp: String,
}

pub fn get_post(post_addr: Address) -> ZomeApiResult<Post> {
    utils::get_as_type(post_addr)
}

pub fn create_post(title: String, details: String, post_type: String, announcement: bool, timestamp: String) -> ZomeApiResult<Address> {
    hdk::commit_entry(
        &Entry::App (
            "post".into(),
            Post {
                title,
                details,
                post_type,
                creator: AGENT_ADDRESS.to_string().into(),
                announcement,
                timestamp
            }.into()
        )
    )
}


pub fn def() -> ValidatingEntryType {
    entry!(
        name: "post",
        description: "",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<Post>| {
            Ok(())
        },

        links: [
            to!(
                "%agent_id",
                tag: COMMENTER_LINK_TAG,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}
