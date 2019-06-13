
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

use super::message::{
    get,
    MessageResult
};

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Thread {
    pub participants: Vec<String>,
}

pub fn get_threads() -> ZomeApiResult<Vec<Address>> {
    hdk::debug(AGENT_ADDRESS.to_string())?;
    Ok(hdk::get_links(&AGENT_ADDRESS, Some("message_threads".to_string()), None)?
        .addresses()
        .to_owned())
}

pub fn create_thread(participant_ids: Vec<String>) -> ZomeApiResult<Address> {
    let mut participant_agent_ids = participant_ids.clone();
    participant_agent_ids.push(AGENT_ADDRESS.to_string()); // add this agent to the list
    let thread_entry = Entry::App(
        "thread".into(),
        Thread {
            participants: participant_agent_ids.clone(),
        }
        .into(),
    );
    let entry_addr = hdk::commit_entry(&thread_entry)?;

    for participant_id in participant_agent_ids {
        hdk::link_entries(&participant_id.into(), &entry_addr, "message_threads", "")?;
    }

    Ok(entry_addr)
}

pub fn get_thread_participants(thread_address: Address) -> ZomeApiResult<Vec<Address>> {
    Ok(utils::get_as_type::<Thread>(thread_address)?
        .participants
        .iter()
        .map(|elem| elem.to_owned().into())
        .collect())
}

pub fn get_thread_messages(thread_address: Address) -> ZomeApiResult<Vec<MessageResult>> {
    Ok(hdk::get_links(&thread_address, Some("messages".to_string()), None)?
        .addresses()
        .iter()
        .map(|address| get(address.to_string().into()).unwrap())
        .collect()
    )
}

pub fn def() -> ValidatingEntryType {
    entry!(
        name: "thread",
        description: "A thread in which messages are posted",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<Thread>| {
            Ok(())
        },

        links: [
            from!(
                "%agent_id",
                link_type: "message_threads",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(// delete this later. This is just because test users are anchors and don't have a real agent_address
                "anchor",
                link_type: "message_threads",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "message",
                link_type: "messages",

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
