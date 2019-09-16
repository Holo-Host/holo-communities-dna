use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing,
    },
    holochain_json_api::{
        error::JsonError,
        json::{JsonString},
    },
};
pub const ANCHOR_ENTRY_TYPE: &str = "anchor";
pub const ANCHOR_PERSON_LINK_TYPE: &str = "registered";

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Anchor {
    pub name: String,
}

pub fn def() -> ValidatingEntryType {
    entry!(
        name: ANCHOR_ENTRY_TYPE,
        description: "",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<Anchor>| {
            Ok(())
        },

        links: [
            to!(
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
