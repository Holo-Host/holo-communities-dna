use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::{
        dna::entry_types::Sharing,
        entry::Entry,
        time::Iso8601,
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
    THREAD_ENTRY_TYPE,
    set_last_read_message
};

pub const MESSAGE_ENTRY_TYPE: &str = "message";

pub const MESSAGE_MESSAGE_THREAD_LINK_TYPE: &str = "message_threads";

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Message {
    pub timestamp: Iso8601,
    pub text: String,
    pub thread_address: Address,
    pub creator: Address,
}

impl Message {
    pub fn with_address(&self, address: Address) -> MessageWithAddress {
        MessageWithAddress {
            address,
            thread_address: self.thread_address.clone(),
            text: self.text.clone(),
            timestamp: self.timestamp.clone(),
            creator: self.creator.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct MessageWithAddress {
    address: Address,
    pub timestamp: Iso8601,
    pub text: String,
    pub thread_address: Address,
    pub creator: Address,
}

pub fn create(
    thread_address: Address,
    text: String,
    timestamp: Iso8601,
) -> ZomeApiResult<MessageWithAddress> {
    let message = Message {
        text,
        timestamp: timestamp.clone().into(),
        thread_address: thread_address.to_owned(),
        creator: AGENT_ADDRESS.to_string().into(),
    };
    let message_entry = Entry::App(MESSAGE_ENTRY_TYPE.into(), message.clone().into());
    let message_address = hdk::commit_entry(&message_entry)?;

    utils::link_entries_bidir(
        &message_address,
        &thread_address,
        MESSAGE_MESSAGE_THREAD_LINK_TYPE,
        MESSAGE_LINK_TYPE,
        "",
        "",
    )?;

    // Set this as the latest unread message for the thread
    //
    // Using hdk::call...
    // 
    // #[derive(Serialize, Deserialize, Debug, DefaultJson)]
    // struct MessageAddressInput {
    //     message_address: String,
    // };
    // hdk::call(
    //     hdk::THIS_INSTANCE,
    //     "thread",
    //     Address::from(hdk::PUBLIC_TOKEN.to_string()),
    //     "set_last_read_message",
    //     MessageAddressInput {
    //         message_address: message_address.to_owned().into()
    //     }.into()
    // )?;
    set_last_read_message(thread_address.into(), message_address.clone().into())?;

    Ok(message.with_address(message_address.clone()))
}

pub fn get(message_address: Address) -> ZomeApiResult<MessageWithAddress> {
    utils::get_as_type::<Message>(message_address.clone())
        .map(|message| message.with_address(message_address))
}

pub fn def() -> ValidatingEntryType {
    entry!(
        name: MESSAGE_ENTRY_TYPE,
        description: "A generic message entry",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<Message>| {
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
