use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::{
        dna::entry_types::Sharing,
        entry::Entry,
        time::Iso8601,
        link::LinkMatch,
    },
    holochain_json_api::{
        error::JsonError,
        json::JsonString,
    },
    holochain_persistence_api::cas::content::Address,
    utils,
    AGENT_ADDRESS,
};
use super::thread::{
    MESSAGE_LINK_TYPE,
    THREAD_ENTRY_TYPE
};

// Core types

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct MessageEntry {
    pub timestamp: Iso8601,
    pub text: String,
    pub thread_address: Address,
    pub creator: Address,
}
impl MessageEntry {
    pub fn to_message(&self, address: Address) -> Message {
        Message {
            address,
            thread_address: self.thread_address.clone(),
            text: self.text.clone(),
            timestamp: self.timestamp.clone(),
            creator: self.creator.clone(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Message {
    pub address: Address,
    pub timestamp: Iso8601,
    pub text: String,
    pub thread_address: Address,
    pub creator: Address,
}
pub const MESSAGE_ENTRY_TYPE: &str = "message";
pub const MESSAGE_MESSAGE_THREAD_LINK_TYPE: &str = "message_threads";

// API

pub fn create(
    thread_address: Address,
    text: String,
    timestamp: Iso8601,
) -> ZomeApiResult<Message> {
    let message_entry = MessageEntry {
        text,
        timestamp: timestamp.clone().into(),
        thread_address: thread_address.to_owned(),
        creator: AGENT_ADDRESS.to_string().into(),
    };
    let message_address = hdk::commit_entry(
        &Entry::App(MESSAGE_ENTRY_TYPE.into(), message_entry.clone().into())
    )?;

    utils::link_entries_bidir(
        &message_address,
        &thread_address,
        MESSAGE_MESSAGE_THREAD_LINK_TYPE,
        MESSAGE_LINK_TYPE,
        "",
        "",
    )?;

    Ok(message_entry.to_message(message_address.clone()))
}

pub fn all(thread_address: Address) -> ZomeApiResult<Vec<Message>> {
    Ok(hdk::get_links(
        &thread_address,
        LinkMatch::Exactly(MESSAGE_LINK_TYPE.into()),
        LinkMatch::Any,
    )?
    .addresses()
    .iter()
    .map(|address| get(address.clone()).unwrap())
    .collect::<Vec<Message>>())
}

fn get(message_address: Address) -> ZomeApiResult<Message> {
    let message_entry = utils::get_as_type::<MessageEntry>(message_address.clone())?;

    Ok(
        Message {
            address: message_address,
            thread_address: message_entry.thread_address,
            creator: AGENT_ADDRESS.to_string().into(),
            text: message_entry.text,
            timestamp: message_entry.timestamp,
        }
    )
}

// PRIVATE

pub fn def() -> ValidatingEntryType {
    entry!(
        name: MESSAGE_ENTRY_TYPE,
        description: "A generic message entry",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<MessageEntry>| {
            Ok(())
        },

        links: [
            to!(
                THREAD_ENTRY_TYPE,
                link_type: MESSAGE_MESSAGE_THREAD_LINK_TYPE,

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
