
use hdk::{
    self,
    utils,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::{
        dna::entry_types::Sharing,
        entry::Entry,
        link::LinkMatch,
    },
    AGENT_ADDRESS,
    holochain_json_api::{
        error::JsonError,
        json::{JsonString},
    },
    holochain_persistence_api::{cas::content::Address},

};

use super::message::{
    MESSAGE_ENTRY_TYPE,
    get,
    MessageWithAddress
};

pub const THREAD_ENTRY_TYPE: &str = "thread";
pub const MESSAGE_LINK_TYPE: &str = "message_link_thread";
pub const AGENT_MESSAGE_THREAD_LINK_TYPE: &str = "agent_message_thread";

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Thread {
    pub participants: Vec<String>,
}

pub fn get_threads() -> ZomeApiResult<Vec<Address>> {
    hdk::debug(AGENT_ADDRESS.to_string())?;
    Ok(hdk::get_links(&AGENT_ADDRESS, LinkMatch::Exactly(AGENT_MESSAGE_THREAD_LINK_TYPE.into()), LinkMatch::Any)?
        .addresses()
        .to_owned())
}

pub fn create_thread(participant_ids: Vec<String>) -> ZomeApiResult<Address> {
    let mut participant_agent_ids = participant_ids.clone();
    participant_agent_ids.push(AGENT_ADDRESS.to_string()); // add this agent to the list
    let thread_entry = Entry::App(
        THREAD_ENTRY_TYPE.into(),
        Thread {
            participants: participant_agent_ids.clone(),
        }
        .into(),
    );
    let entry_addr = hdk::commit_entry(&thread_entry)?;

    for participant_id in participant_agent_ids {
        hdk::link_entries(&participant_id.into(), &entry_addr, AGENT_MESSAGE_THREAD_LINK_TYPE, "")?;
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

pub fn get_thread_messages(thread_address: Address) -> ZomeApiResult<Vec<MessageWithAddress>> {
    Ok(hdk::get_links(&thread_address, LinkMatch::Exactly(MESSAGE_LINK_TYPE.into()), LinkMatch::Any)?
        .addresses()
        .iter()
        .map(|address| get(address.to_string().into()).unwrap())
        .collect()
    )
}

pub fn def() -> ValidatingEntryType {
    entry!(
        name: THREAD_ENTRY_TYPE,
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
                link_type: AGENT_MESSAGE_THREAD_LINK_TYPE,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                MESSAGE_ENTRY_TYPE,
                link_type: MESSAGE_LINK_TYPE,

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
