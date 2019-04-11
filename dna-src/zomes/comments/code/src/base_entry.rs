/**
 * Base entry type definition
 *
 * A 'base' is some external or internal unique identifier that a user is commenting on.
 * This may be represented as a Holochain entry ID, web URL, or anything else unique.
 *
 * @package: Holochain comments
 * @author:  pospi <pospi@spadgos.com>
 * @since:   2019-03-27
 */

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
    cas::content::Address,
    json::RawString,
};

use super::comment_entry::{
    COMMENT_ENTRY_TYPE,
};

// record type for base entries

pub const BASE_ENTRY_TYPE: &str = "base";

pub type Base = String;

// tag for links from base to comment

pub const COMMENT_LINK_TAG: &str = "commented_on";

// API methods

pub fn handle_get_comments(base: String) -> ZomeApiResult<Vec<Address>> {
    let address = hdk::entry_address(&Entry::App(BASE_ENTRY_TYPE.into(), RawString::from(base).into()))?;
    Ok(hdk::get_links(&address, COMMENT_LINK_TAG)?.addresses().to_vec())
}

// Entry definition

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
                tag: COMMENT_LINK_TAG,
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
