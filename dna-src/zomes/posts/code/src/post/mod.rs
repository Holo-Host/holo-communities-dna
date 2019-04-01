
use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing, error::HolochainError,
        json::JsonString,
    },
};


#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Post {
    pub participants: Vec<String>,
}


pub fn def() -> ValidatingEntryType {
    entry!(
        name: "post",
        description: "",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<Post>| {
            Ok(())
        },

        links: [
            from!(
                "%agent_id",
                tag: "posts",

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
