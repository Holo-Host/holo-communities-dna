use crate::anchor::{Anchor, ANCHOR_ENTRY_TYPE, ANCHOR_PERSON_LINK_TYPE};
use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    error::{
        ZomeApiError,
        ZomeApiResult,
    },
    holochain_core_types::{
        dna::entry_types::Sharing,
        entry::Entry,
        link::LinkMatch,
    },
    holochain_json_api::{
        error::JsonError,
        json::JsonString,
    },
    holochain_persistence_api::cas::content::{
        Address,
        AddressableContent,
    },
    utils,
    AGENT_ADDRESS,
};
use hdk_helpers::commit_if_not_in_chain;

// Core types

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct PersonEntry {
    pub name: String,
    pub avatar_url: String,
}
impl PersonEntry {
    pub fn with_address(&self, address: Address) -> Person {
        Person {
            address,
            name: self.name.clone(),
            avatar_url: self.avatar_url.clone(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Person {
    pub address: Address,
    pub name: String,
    pub avatar_url: String,
}
pub const PERSON_ENTRY_TYPE: &str = "person";
pub const PERSON_AGENT_LINK_TYPE: &str = "person_to_agent_link";

// API

pub fn get(agent_id: Address) -> ZomeApiResult<Person> {
    let person = utils::get_links_and_load_type::<PersonEntry>(
        &agent_id,
        LinkMatch::Exactly(PERSON_AGENT_LINK_TYPE.into()),
        LinkMatch::Any,
    )?
    .first()
    .map(|result| result.to_owned());

    match person {
        Some(person) => Ok(person.with_address(agent_id)),
        None => Err(ZomeApiError::Internal(
            "Agent has not been registered".into(),
        )),
    }
}

pub fn get_me() -> ZomeApiResult<Person> {
    get(AGENT_ADDRESS.to_string().into())
}

pub fn is_registered() -> ZomeApiResult<bool> {
    Ok(get(AGENT_ADDRESS.to_string().into()).is_ok())
}

pub fn register_user(name: String, avatar_url: String) -> ZomeApiResult<Person> {
    let person = PersonEntry {
        name: name.clone(),
        avatar_url: avatar_url.clone(),
    };

    let person_entry = Entry::App(PERSON_ENTRY_TYPE.into(), person.clone().into());

    let person_address = hdk::commit_entry(&person_entry)?;
    hdk::link_entries(&AGENT_ADDRESS, &person_address, PERSON_AGENT_LINK_TYPE, "")?;

    let anchor_entry = Entry::App(
        ANCHOR_ENTRY_TYPE.into(),
        Anchor {
            name: "people".into(),
        }
        .into(),
    );
    let anchor_address = commit_if_not_in_chain(&anchor_entry)?;
    hdk::link_entries(&anchor_address, &AGENT_ADDRESS, ANCHOR_PERSON_LINK_TYPE, "")?;

    Ok(person.with_address(AGENT_ADDRESS.to_string().into()))
}

pub fn all() -> ZomeApiResult<Vec<Person>> {
    let anchor_entry = Entry::App(
        ANCHOR_ENTRY_TYPE.into(),
        Anchor {
            name: "people".into(),
        }
        .into(),
    );
    Ok(hdk::get_links(
        &anchor_entry.address(),
        LinkMatch::Exactly(ANCHOR_PERSON_LINK_TYPE.into()),
        LinkMatch::Any,
    )?
    .addresses()
    .iter()
    .map(|address| get(address.to_string().into()).unwrap())
    .collect())
}

pub fn def() -> ValidatingEntryType {
    entry!(
        name: PERSON_ENTRY_TYPE,
        description: "Extra information attached to an agent address",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<PersonEntry>| {
            Ok(())
        },

        links: [
            from!(
                "%agent_id",
                link_type: PERSON_AGENT_LINK_TYPE,

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
