
use hdk::{
    self,
    utils,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::{
        cas::content::Address, dna::entry_types::Sharing, entry::Entry, error::HolochainError,
        json::JsonString,
    },
    AGENT_ADDRESS,
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
        native_type: Post,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_thread: Post, _ctx: hdk::ValidationData| {
            Ok(())
        },

        links: [
            to!(
                "%agent_id",
                tag: COMMENTER_LINK_TAG,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_base: Address, _target: Address, _ctx: hdk::ValidationData| {
                    Ok(())
                }
            )
        ]
    )
}
