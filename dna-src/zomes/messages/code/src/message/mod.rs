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
    pub thread_id: Address,
    pub creator: Address,
}

pub fn post_message_to_thread(thread_addr: Address, text: String, timestamp: String) -> ZomeApiResult<Address> {
    let message_entry = Entry::App(
        "message".into(), 
        Message { 
            text, 
            timestamp: timestamp, 
            thread_id: thread_addr.to_owned(), 
            creator: AGENT_ADDRESS.to_string().into() 
        }.into()
    );
    let message_addr = hdk::commit_entry(&message_entry)?;
    utils::link_entries_bidir(&message_addr, &thread_addr, "message_thread", "messages", "", "")?;
    Ok(message_addr)
}


pub fn get_message(message_addr: Address) -> ZomeApiResult<Message> {
    utils::get_as_type::<Message>(message_addr)
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
