
use hdk::{
    self,
    utils,
    error::{ZomeApiError, ZomeApiResult},
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing, error::HolochainError,
        json::JsonString,
        json::RawString,
        cas::content::{Address},
        entry::Entry,
    },
};

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Community {
    pub name: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct CommunityResult {
    pub address: Address,
    pub name: String,
    pub slug: String
}

pub type Base = RawString;

const COMMUNITY_BASE_ENTRY: &str = "community_base";
const COMMUNITY_LINK_TYPE: &str = "member_of";

pub fn get(address: Address) -> ZomeApiResult<CommunityResult> {
    let community: Result<Community, _> = utils::get_as_type(address.clone());

    match community {
        Ok(community) => {
            Ok(CommunityResult {
                address,
                name: community.name,
                slug: community.slug})
        },
        Err(_err) => {
            Err(ZomeApiError::Internal("Community not found".into()))
        }
    }
}

pub fn get_by_slug(slug: String) -> ZomeApiResult<CommunityResult> {
    let slug_address = hdk::entry_address(&Entry::App(COMMUNITY_BASE_ENTRY.into(), RawString::from(slug).into()))?;
    let all_communities = hdk::get_links(&slug_address, Some(COMMUNITY_LINK_TYPE.into()), None)?.addresses().clone();
    let community_address = all_communities.to_owned().into_iter().next().ok_or(ZomeApiError::Internal("No communities for this slug".into())).unwrap();
    get(community_address)
}

pub fn create(name: String, slug: String) -> ZomeApiResult<CommunityResult> {

    let base_entry = Entry::App(COMMUNITY_BASE_ENTRY.into(), RawString::from(COMMUNITY_BASE_ENTRY).into());
    let base_address = hdk::commit_entry(&base_entry)?;

    let slug_entry = Entry::App(COMMUNITY_BASE_ENTRY.into(), RawString::from(slug.clone()).into());
    let slug_address = hdk::commit_entry(&slug_entry)?;

    let community_address = hdk::commit_entry(
        &Entry::App (
            "community".into(),
            Community {
                name: name.clone(),
                slug: slug.clone()
            }.into()
        )
    )?;

    hdk::link_entries(
        &base_address,
        &community_address,
        COMMUNITY_LINK_TYPE,
        ""
    )?;
    hdk::link_entries(
        &slug_address,
        &community_address,
        COMMUNITY_LINK_TYPE,
        ""
    )?;

    Ok(CommunityResult {
        address: community_address,
        name,
        slug
    })
}

pub fn all() -> ZomeApiResult<Vec<CommunityResult>> {
    let address = hdk::entry_address(&Entry::App(COMMUNITY_BASE_ENTRY.into(), RawString::from(COMMUNITY_BASE_ENTRY).into()))?;
    Ok(hdk::get_links(&address, Some(COMMUNITY_LINK_TYPE.into()), None)?
        .addresses()
        .iter()
        .map(|address| get(address.to_string().into()).unwrap())
        .collect()
    )
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
                link_type: COMMUNITY_LINK_TYPE,
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
