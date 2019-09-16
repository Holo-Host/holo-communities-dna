/**
 * Comment entry type definition
 *
 * @package: Holochain comments
 * @author:  pospi <pospi@spadgos.com>
 * @since:   2019-03-26
 */

use hdk::{
    AGENT_ADDRESS,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    utils,
    holochain_core_types::{
        entry::Entry,
        time::Iso8601,
        dna::entry_types::Sharing,
        link::LinkMatch,
    },
    holochain_json_api::{
        error::JsonError,
        json::{JsonString, RawString},
    },
    holochain_persistence_api::{cas::content::Address},
};

// tag for links from base to comment

pub type Base = String;

pub const COMMENT_ENTRY_TYPE: &str = "comment";
pub const BASE_ENTRY_TYPE: &str = "base";
pub const COMMENT_LINK_TYPE: &str = "commented_on";

// comment type and result format

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Comment {
    base: String,
    creator: Address,
    text: String,
    timestamp: Iso8601,
}

// Converts a comment (without address) into a comment result for returning from the api call
impl Comment {
    pub fn with_address(&self, address: Address) -> CommentWithAddress {
        CommentWithAddress {
            address,
            base: self.base.clone(),
            text: self.text.clone(),
            timestamp: self.timestamp.clone(),
            creator: self.creator.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct CommentWithAddress {
    address: Address,
    base: String,
    creator: Address,
    text: String,
    timestamp: Iso8601
}

// API methods

pub fn create(base: String, text: String, timestamp: Iso8601) -> ZomeApiResult<CommentWithAddress> {
    // create and store the comment
    let comment = Comment {
        base: base.clone(),
        text: text.clone(),
        timestamp: timestamp.clone(),
        creator: AGENT_ADDRESS.to_string().into()
    };
    let entry = Entry::App(
        COMMENT_ENTRY_TYPE.into(),
        comment.clone().into()
    );
    let address = hdk::commit_entry(&entry)?;

    // store an entry for the ID of the base object the comment was made on
    let base_entry = Entry::App(BASE_ENTRY_TYPE.into(), RawString::from(base.clone()).into());
    let base_address = hdk::commit_entry(&base_entry)?;

    // link the comment to its originating thing
    hdk::link_entries(
        &base_address,
        &address,
        COMMENT_LINK_TYPE,
        ""
    )?;

    Ok(comment.with_address(address))
}

pub fn get(address: Address) -> ZomeApiResult<CommentWithAddress> {
    utils::get_as_type::<Comment>(address.clone())
        .map(|comment| {
            comment.with_address(address)
        })
}

pub fn all_for_base(base: String) -> ZomeApiResult<Vec<CommentWithAddress>> {
    let address = hdk::entry_address(&Entry::App(BASE_ENTRY_TYPE.into(), RawString::from(base).into()))?;
    Ok(hdk::get_links(&address, LinkMatch::Exactly(COMMENT_LINK_TYPE.into()), LinkMatch::Any)?
        .addresses()
        .iter()
        .map(|address| get(address.to_string().into()).unwrap())
        .collect()
    )
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

pub fn base_def() -> ValidatingEntryType {
    entry!(
        name: BASE_ENTRY_TYPE,
        description: "Universally unique ID of something that is being commented on",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Base>| {
            Ok(())
        },
        links: [
            to!(
                COMMENT_ENTRY_TYPE,
                link_type: COMMENT_LINK_TYPE,
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
