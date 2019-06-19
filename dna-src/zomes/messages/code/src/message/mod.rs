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


#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Message {
    pub timestamp: String,
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
    pub timestamp: String,
    pub text: String,
    pub thread_address: Address,
    pub creator: Address
}

pub fn create(thread_address: Address, text: String, timestamp: String) -> ZomeApiResult<MessageWithAddress> {
    let message = Message { 
        text, 
        timestamp: timestamp, 
        thread_address: thread_address.to_owned(), 
        creator: AGENT_ADDRESS.to_string().into() 
    };
    let message_entry = Entry::App(
        "message".into(), 
        message.clone().into()
    );
    let message_addr = hdk::commit_entry(&message_entry)?;
    utils::link_entries_bidir(&message_addr, &thread_address, "message_thread", "messages", "", "")?;
    Ok(message.with_address(message_addr))
}


pub fn get(message_addr: Address) -> ZomeApiResult<MessageWithAddress> {
    utils::get_as_type::<Message>(message_addr.clone())
        .map(|message| {
            message.with_address(message_addr)
        })
}


pub fn def() -> ValidatingEntryType {
    entry!(
        name: "message",
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
                "thread",
                link_type: "message_thread",

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
