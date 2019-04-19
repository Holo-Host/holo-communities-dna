
use hdk::{
    self,
    utils,
    error::ZomeApiResult,
    AGENT_ADDRESS,
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing, error::HolochainError,
        json::JsonString,
        json::RawString,
        cas::content::Address,
        entry::Entry,
    },
};

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Community {
    pub name: String,
    pub slug: String,
}

pub type Base = RawString;

const COMMUNITY_BASE_ENTRY: &str = "community_base";
const COMMUNITY_LINK_TAG: &str = "member_of";

pub fn get_community(address: Address) -> ZomeApiResult<Community> {
    utils::get_as_type(address)
}

pub fn create_community(name: String, slug: String) -> ZomeApiResult<Address> {

    let base_entry = Entry::App(COMMUNITY_BASE_ENTRY.into(), RawString::from("communities_base").into());
    let base_address = hdk::commit_entry(&base_entry)?;

    let community_address = hdk::commit_entry(
        &Entry::App (
            "community".into(),
            Community {
                name,
                slug
            }.into()
        )
    )?;

    // link the community to its originating thing
    hdk::link_entries(
        &base_address,
        &community_address,
        COMMUNITY_LINK_TAG,
    )?;

    Ok(community_address)
}

pub fn get_communitys() -> ZomeApiResult<Vec<Address>> {
    let address = hdk::entry_address(&Entry::App(COMMUNITY_BASE_ENTRY.into(), RawString::from("communities_base").into()))?;
    Ok(hdk::get_links(&address, COMMUNITY_LINK_TAG)?.addresses().to_vec())
}

pub fn community_def() -> ValidatingEntryType {
    entry!(
        name: "community",
        description: "",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<Community>| {
            Ok(())
        }
    )
}

pub fn base_def() -> ValidatingEntryType {
    entry!(
        name: COMMUNITY_BASE_ENTRY,
        description: "Universally unique ID of something that has communities in",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Base>| {
            Ok(())
        },
        links: [
            to!(
                "community",
                tag: COMMUNITY_LINK_TAG,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}
