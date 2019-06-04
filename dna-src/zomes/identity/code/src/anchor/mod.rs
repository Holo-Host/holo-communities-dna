use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing, error::HolochainError, json::JsonString,
    },
};

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Anchor {
    pub name: String,
}

pub fn def() -> ValidatingEntryType {
    entry!(
        name: "anchor",
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
                link_type: "registered",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(// delete this later. This is just because test users are anchors and don't have a real agent_address
                "anchor",
                link_type: "registered",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "%agent_id",
                link_type: "belongs_to",

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(// delete this later. This is just because test users are anchors and don't have a real agent_address
                "anchor",
                link_type: "belongs_to",

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
