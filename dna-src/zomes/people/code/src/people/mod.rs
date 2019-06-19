use crate::anchor::{
    Anchor,
    ANCHOR_ENTRY_TYPE,
    ANCHOR_PERSON_LINK_TYPE
};
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

pub const PERSON_ENTRY_TYPE: &str = "person";

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Person {
    pub name: String,
    pub avatar_url: String
}

impl Person {
    pub fn with_address(&self, address: Address) -> PersonWithAddress {
        PersonWithAddress {
            address,
            name: self.name.clone(),
            avatar_url: self.avatar_url.clone()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct PersonWithAddress {
    pub address: Address,
    pub name: String,
    pub avatar_url: String
}

pub fn get(agent_id: Address) -> ZomeApiResult<PersonWithAddress> {
    let person = utils::get_links_and_load_type::<Person>(&agent_id, Some(ANCHOR_PERSON_LINK_TYPE.to_string()), None)?
        .first()
        .map(|result| result.to_owned());

    match person {
        Some(person) => {
            Ok(person.with_address(agent_id))
        },
        None => {
            Err(ZomeApiError::Internal("Agent has not been registered".into()))
        }
    }
}

pub fn get_me() -> ZomeApiResult<PersonWithAddress> {
    get(AGENT_ADDRESS.to_string().into())
}

pub fn is_registered() -> ZomeApiResult<bool> {
    Ok(get(AGENT_ADDRESS.to_string().into()).is_ok())
}

pub fn register_user(name: String, avatar_url: String) -> ZomeApiResult<PersonWithAddress> {
    let person = Person { 
        name: name.clone(), 
        avatar_url: avatar_url.clone()
    };

    let person_entry = Entry::App(
        PERSON_ENTRY_TYPE.into(), 
        person.clone().into()
    );

    let person_addr = hdk::commit_entry(&person_entry)?;
    hdk::link_entries(&AGENT_ADDRESS, &person_addr, ANCHOR_PERSON_LINK_TYPE, "")?;

    let anchor_entry = Entry::App(
        ANCHOR_ENTRY_TYPE.into(),
        Anchor {
            name: "people".into(),
        }
        .into(),
    );
    let anchor_addr = hdk::commit_entry(&anchor_entry)?;
    hdk::link_entries(&anchor_addr, &AGENT_ADDRESS, ANCHOR_PERSON_LINK_TYPE, "")?;

    Ok(person.with_address(AGENT_ADDRESS.to_string().into()))
}

pub fn all() -> ZomeApiResult<Vec<PersonWithAddress>> {
    let anchor_entry = Entry::App(
        ANCHOR_ENTRY_TYPE.into(),
        Anchor {
            name: "people".into(),
        }
        .into(),
    );
    Ok(hdk::get_links(&anchor_entry.address(), Some(ANCHOR_PERSON_LINK_TYPE.to_string()), None)?
        .addresses()
        .iter()
        .map(|address| get(address.to_string().into()).unwrap())
        .collect()
    )
}

pub fn def() -> ValidatingEntryType {
    entry!(
        name: PERSON_ENTRY_TYPE,
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
                link_type: ANCHOR_PERSON_LINK_TYPE,

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
