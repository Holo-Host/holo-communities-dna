use hdk::{
    self,
    utils,
    entry_definition::ValidatingEntryType,
    error::{ZomeApiResult, ZomeApiError},
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
    pub thread_id: Address,
    pub creator: Address,
}

impl Message {
    pub fn result(&self, address: Address) -> MessageResult {
        MessageResult {
            address,
            thread_id: self.thread_id.clone(),
            text: self.text.clone(),
            timestamp: self.timestamp.clone(),
            creator: self.creator.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct MessageResult {
    address: Address,
    pub timestamp: String,
    pub text: String,
    pub thread_id: Address,
    pub creator: Address
}

pub fn create(thread_addr: Address, text: String, timestamp: String) -> ZomeApiResult<MessageResult> {
    let message = Message { 
        text, 
        timestamp: timestamp, 
        thread_id: thread_addr.to_owned(), 
        creator: AGENT_ADDRESS.to_string().into() 
    };
    let message_entry = Entry::App(
        "message".into(), 
        message.clone().into()
    );
    let message_addr = hdk::commit_entry(&message_entry)?;
    utils::link_entries_bidir(&message_addr, &thread_addr, "message_thread", "messages", "", "")?;
    Ok(message.result(message_addr))
}


pub fn get(message_addr: Address) -> ZomeApiResult<MessageResult> {
    let message: Result<Message, _> = utils::get_as_type(message_addr.clone());

    match message {
        Ok(message) => {
            Ok(message.result(message_addr))
        },
        Err(_err) => {
            Err(ZomeApiError::Internal("Message not found".into()))
        }
    }
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
