use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::{
        dna::entry_types::Sharing,
        entry::Entry,
        link::LinkMatch,
        // time::Iso8601,
    },
    holochain_json_api::{
        error::JsonError,
        json::JsonString,
    },
    holochain_persistence_api::cas::content::Address,
    holochain_wasm_utils::api_serialization::{
        get_links::{
            GetLinksResult,
            LinksResult
        },
    },
    utils,
    AGENT_ADDRESS,
};
use std::{
    convert::{
        TryFrom,
    }
};
use super::message::{
    MESSAGE_ENTRY_TYPE
};

// Core types

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct ThreadEntry {
    pub participants: Vec<Address>,
}
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Thread {
    pub address: Address,
    pub participant_addresses: Vec<Address>,
    pub last_read_time: String,
}
pub const THREAD_ENTRY_TYPE: &str = "thread";
pub const MESSAGE_LINK_TYPE: &str = "message_link_thread";
pub const AGENT_MESSAGE_THREAD_LINK_TYPE: &str = "agent_message_thread";

// API

pub fn create(
    participant_addresses: Vec<Address>,
    // TODO: becomes Iso8601 once core regex tagging issue fixed
    timestamp: String
) -> ZomeApiResult<Thread> {
    let mut participant_addresses_including_current = participant_addresses.clone();

    participant_addresses_including_current.push(AGENT_ADDRESS.to_string().into()); // add this agent to the list
    
    let thread_entry = Entry::App(
        THREAD_ENTRY_TYPE.into(),
        ThreadEntry {
            participants: participant_addresses_including_current.clone()
        }.into(),
    );
    let thread_address = hdk::commit_entry(&thread_entry)?;

    for participant_id in participant_addresses_including_current.clone() {
        create_or_update_agent_thread_link(
            participant_id.into(),
            thread_address.clone(),
            timestamp.clone()
        )?;
    }

    Ok(
        Thread {
            address: thread_address.clone().into(),
            participant_addresses: participant_addresses_including_current.clone().into(),
            last_read_time: timestamp.clone()
        }
    )
}

pub fn set_last_read_time(
    thread_address: Address,
    // TODO: becomes Iso8601 once core regex tagging issue fixed
    last_read_time: String
) -> ZomeApiResult<Thread> {
    create_or_update_agent_thread_link(
        AGENT_ADDRESS.to_string().into(),
        thread_address.clone(),
        last_read_time.clone()
    )?;

    Ok(Thread {
        address: thread_address.clone(),
        participant_addresses: get_thread_participants(thread_address)?,
        last_read_time: last_read_time.clone()
    })
}

pub fn all() -> ZomeApiResult<Vec<Thread>> {
    Ok(all_thread_links_for_agent()?
        .links()
        .iter()
        .rev()
        .map(|agent_thread_link| build_thread_from_thread_link(agent_thread_link.to_owned()).unwrap())
        .collect::<Vec<Thread>>()
        .to_owned()
    )
}

pub fn get(thread_address: Address) -> ZomeApiResult<Thread> {
    let thread_link = get_thread_link(thread_address).unwrap().unwrap();

    build_thread_from_thread_link(thread_link)
}

// HELPERS

fn get_thread_entry(thread_address: Address) -> ZomeApiResult<ThreadEntry> {
    utils::get_as_type::<ThreadEntry>(thread_address.clone())
}

fn all_thread_links_for_agent() -> ZomeApiResult<GetLinksResult> {
    hdk::get_links(
        &AGENT_ADDRESS,
        LinkMatch::Exactly(AGENT_MESSAGE_THREAD_LINK_TYPE.clone().into()),
        LinkMatch::Any,
    )
}

fn get_thread_link(thread_address: Address) -> ZomeApiResult<Option<LinksResult>> {    
    Ok(all_thread_links_for_agent()?
        .links()
        .into_iter()
        .rev()
        .find(|thread_link| thread_link.address == thread_address))
}

fn create_or_update_agent_thread_link(
    agent_address: Address,
    thread_address: Address,
    // TODO: becomes Iso8601 once core regex tagging issue fixed
    last_read_time: String
) -> ZomeApiResult<Address> {
    if let Some(current_link) = get_thread_link(thread_address.clone())? {
        hdk::remove_link(
            &agent_address,
            &thread_address,
            AGENT_MESSAGE_THREAD_LINK_TYPE,
            &current_link.tag
        )?;
    }

    hdk::link_entries(
        &agent_address,
        &thread_address,
        AGENT_MESSAGE_THREAD_LINK_TYPE,
        &last_read_time.to_string()
    )
}

fn get_thread_participants(thread_address: Address) -> ZomeApiResult<Vec<Address>> {
    Ok(get_thread_entry(thread_address)?
        .participants
        .iter()
        .map(|participant_address| participant_address.to_owned().into())
        .collect::<Vec<Address>>() // <Vec<Person>>
        .to_owned())
}

fn build_thread_from_thread_link(agent_thread_link: LinksResult) -> ZomeApiResult<Thread> {
    let participant_addresses = self::get_thread_participants(agent_thread_link.address.clone())?;

    Ok(Thread {
        address: agent_thread_link.address,
        participant_addresses: participant_addresses,
        // TODO: becomes Iso8601 once core regex tagging issue fixed
        // last_read_time: Iso8601::try_from(agent_thread_link.tag.to_owned()).unwrap()
        last_read_time: String::try_from(agent_thread_link.tag.to_owned()).unwrap()
    })
}

// DEF

pub fn def() -> ValidatingEntryType {
    entry!(
        name: THREAD_ENTRY_TYPE,
        description: "A thread in which messages are posted",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<ThreadEntry>| {
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
