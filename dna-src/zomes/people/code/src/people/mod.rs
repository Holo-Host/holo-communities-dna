use crate::anchor;
use hdk::{
    self,
    utils,
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::{
        cas::content::{Address, AddressableContent},
        dna::entry_types::Sharing,
        entry::Entry,
        error::HolochainError,
        json::JsonString,
    },
    AGENT_ADDRESS,
};

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Person {
    pub name: String,
    pub avatar_url: String
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct PersonResult {
    pub address: Address,
    pub name: String,
    pub avatar_url: String
}

const PEOPLE_LINK_TYPE: &str = "registered";

pub fn get(agent_id: Address) -> ZomeApiResult<PersonResult> {
    let person = utils::get_links_and_load_type::<Person>(&agent_id, Some(PEOPLE_LINK_TYPE.to_string()), None)?
        .first()
        .map(|result| result.to_owned());

    match person {
        Some(person) => {
            Ok(PersonResult {
                address: agent_id,
                name: person.name,
                avatar_url: person.avatar_url})
        },
        None => {
            Err(ZomeApiError::Internal("Agent has not been registered".into()))
        }
    }
}

pub fn get_me() -> ZomeApiResult<PersonResult> {
    get(AGENT_ADDRESS.to_string().into())
}

pub fn is_registered() -> ZomeApiResult<bool> {
    Ok(get(AGENT_ADDRESS.to_string().into()).is_ok())
}

pub fn register_user(name: String, avatar_url: String) -> ZomeApiResult<PersonResult> {
    let person_entry = Entry::App(
        "person".into(), 
        Person { 
            name: name.clone(), 
            avatar_url: avatar_url.clone()
        }.into()
    );

    let person_addr = hdk::commit_entry(&person_entry)?;
    hdk::link_entries(&AGENT_ADDRESS, &person_addr, PEOPLE_LINK_TYPE, "")?;

    let anchor_entry = Entry::App(
        "anchor".into(),
        anchor::Anchor {
            name: "people".into(),
        }
        .into(),
    );
    let anchor_addr = hdk::commit_entry(&anchor_entry)?;
    hdk::link_entries(&anchor_addr, &AGENT_ADDRESS, PEOPLE_LINK_TYPE, "")?;

    Ok(PersonResult {
        address: AGENT_ADDRESS.to_string().into(),
        name: name.to_string(), 
        avatar_url: avatar_url.to_string()
    })
}

pub fn all() -> ZomeApiResult<Vec<PersonResult>> {
    let anchor_entry = Entry::App(
        "anchor".into(),
        anchor::Anchor {
            name: "people".into(),
        }
        .into(),
    );
    Ok(hdk::get_links(&anchor_entry.address(), Some(PEOPLE_LINK_TYPE.to_string()), None)?
        .addresses()
        .iter()
        .map(|address| get(address.to_string().into()).unwrap())
        .collect()
    )
}

pub fn def() -> ValidatingEntryType {
    entry!(
        name: "person",
        description: "Extra information attached to an agent address",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<Person>| {
            Ok(())
        },

        links: [
            from!(
                "%agent_id",
                link_type: PEOPLE_LINK_TYPE,

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
