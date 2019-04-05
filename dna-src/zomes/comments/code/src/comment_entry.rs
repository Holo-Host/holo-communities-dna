/**
 * Comment entry type definition
 *
 * @package: Holochain comments
 * @author:  pospi <pospi@spadgos.com>
 * @since:   2019-03-26
 */

use holochain_core_types_derive::{ DefaultJson };

use hdk::{
    AGENT_ADDRESS,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    utils::get_as_type,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    entry::Entry,
    time::Iso8601,
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
};

use super::base_entry::{
    BASE_ENTRY_TYPE,
    COMMENT_LINK_TAG,
};

// comment type and entry format

pub const COMMENT_ENTRY_TYPE: &str = "comment";

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct CommentData {
    pub base: String,
    pub content: String,
    pub timestamp: Iso8601,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Comment {
    base: String,
    author: Address,
    content: String,
    timestamp: Iso8601,
}

/// Converts an input comment (without author) into a comment entry for saving to the DHT
impl CommentData {
    pub fn with_author(&self, author: Address) -> Comment {
        Comment{
            base: self.base.clone(),
            content: self.content.clone(),
            timestamp: self.timestamp.clone(),
            author,
        }
    }
}

// API methods

pub fn handle_create_comment(input_entry: CommentData) -> ZomeApiResult<Address> {
    // create and store the comment
    let entry = Entry::App(
        COMMENT_ENTRY_TYPE.into(),
        input_entry.with_author(
            AGENT_ADDRESS.to_string().into()
        ).into()
    );
    let address = hdk::commit_entry(&entry)?;

    // store an entry for the ID of the base object the comment was made on
    let base_entry = Entry::App(BASE_ENTRY_TYPE.into(), input_entry.base.into());
    let base_address = hdk::commit_entry(&base_entry)?;

    // link the comment to its originating thing
    hdk::link_entries(
        &base_address,
        &address,
        COMMENT_LINK_TAG,
    )?;

    // return address
    Ok(address)
}

pub fn handle_get_comment(address: Address) -> ZomeApiResult<Comment> {
    get_as_type(address)
}

// Entry definition

pub fn comment_def() -> ValidatingEntryType {
    entry!(
        name: COMMENT_ENTRY_TYPE,
        description: "A comment made against some other resource from elsewhere",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Comment>| {
            Ok(())
        }
    )
}
