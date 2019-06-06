use crate::anchor;
use hdk::{
    self,
    utils,
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    // error::ZomeApiResult,
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
pub struct Identity {
    pub name: String,
    pub avatar_url: String
}

const IDENTITY_LINK_TYPE: &str = "registered";

pub fn get_identity(agent_id: Address) -> ZomeApiResult<Identity> {
    utils::get_links_and_load_type::<Identity>(&agent_id, Some(IDENTITY_LINK_TYPE.to_string()), None)?
        .first()
        .map(|result| result.to_owned())
        .ok_or(ZomeApiError::Internal(
            "Agent has not been registered".into(),
        ))
}

pub fn get_me() -> ZomeApiResult<Address> {
    Ok(AGENT_ADDRESS.to_string().into())
}

pub fn is_registered() -> ZomeApiResult<bool> {
    Ok(get_identity(AGENT_ADDRESS.to_string().into()).is_ok())
}

pub fn register_user(name: String, avatar_url: String) -> ZomeApiResult<Address> {
    let identity_entry = Entry::App("identity".into(), Identity { name, avatar_url }.into());

    let ident_addr = hdk::commit_entry(&identity_entry)?;
    hdk::link_entries(&AGENT_ADDRESS, &ident_addr, IDENTITY_LINK_TYPE, "")?;

    let anchor_entry = Entry::App(
        "anchor".into(),
        anchor::Anchor {
            name: "people".into(),
        }
        .into(),
    );
    let anchor_addr = hdk::commit_entry(&anchor_entry)?;
    hdk::link_entries(&anchor_addr, &AGENT_ADDRESS, IDENTITY_LINK_TYPE, "")?;

    Ok(ident_addr.to_string().into())
}

pub fn get_people() -> ZomeApiResult<Vec<Address>> {
    let anchor_entry = Entry::App(
        "anchor".into(),
        anchor::Anchor {
            name: "people".into(),
        }
        .into(),
    );
    Ok(hdk::get_links(&anchor_entry.address(), Some(IDENTITY_LINK_TYPE.to_string()), None)?
        .addresses()
        .to_owned())
}

pub fn def() -> ValidatingEntryType {
    entry!(
        name: "identity",
        description: "Extra information attached to an agent address",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<Identity>| {
            Ok(())
        },

        links: [
            from!(
                "%agent_id",
                link_type: IDENTITY_LINK_TYPE,

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
