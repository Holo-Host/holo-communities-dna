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
        json::{
            JsonString,
            RawString,
        },
    },
    holochain_persistence_api::cas::content::{
        Address,
        AddressableContent,
    },
    utils,
};
use hdk_helpers::commit_if_not_in_chain;
use super::DEFAULT_COMMUNITIES;

// Core types

pub type Base = RawString;
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct CommunityEntry {
    pub name: String,
    pub slug: String,
}
impl CommunityEntry {
    pub fn with_address(&self, address: Address) -> Community {
        Community {
            address,
            name: self.name.clone(),
            slug: self.slug.clone(),
        }
    }
}
impl From<&(&str, &str)> for CommunityEntry {
    fn from(tuple: &(&str, &str)) -> Self {
        CommunityEntry {
            name: tuple.0.to_string(),
            slug: tuple.1.to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Community {
    pub address: Address,
    pub name: String,
    pub slug: String,
}
impl From<CommunityEntry> for Community {
    fn from(community_entry: CommunityEntry) -> Self {
        let address = Entry::App(COMMUNITY_ENTRY_TYPE.into(), community.clone().into()).address();
        Community {
            address,
            name: community_entry.name,
            slug: community_entry.slug,
        }
    }
}
pub const COMMUNITY_ENTRY_TYPE: &str = "community";
pub const COMMUNITY_BASE_ENTRY: &str = "community_base";
pub const COMMUNITY_LINK_TYPE: &str = "member_of";

// API

pub fn get(address: Address) -> ZomeApiResult<Community> {
    utils::get_as_type::<CommunityEntry>(address.clone())
        .map(|community| community.with_address(address))
}

pub fn get_by_slug(slug: String) -> ZomeApiResult<Community> {
    // first check the default communities and return early if one of those is found
    if let Some(t) = DEFAULT_COMMUNITIES
        .iter()
        .find(|(_, test_slug)| *test_slug == slug)
    {
        return Ok(Community::from(CommunityEntry::from(t)));
    }
    // otherwise go to the DHT
    let slug_address = hdk::entry_address(&Entry::App(
        COMMUNITY_BASE_ENTRY.into(),
        RawString::from(slug).into(),
    ))?;
    let all_communities = hdk::get_links(
        &slug_address,
        LinkMatch::Exactly(COMMUNITY_LINK_TYPE.into()),
        LinkMatch::Any,
    )?
    .addresses()
    .clone();
    let community_address = all_communities
        .to_owned()
        .into_iter()
        .next()
        .ok_or(ZomeApiError::Internal(
            "No communities for this slug".into(),
        ))
        .unwrap();
    get(community_address)
}

pub fn create(name: String, slug: String) -> ZomeApiResult<Community> {
    let base_entry = Entry::App(
        COMMUNITY_BASE_ENTRY.into(),
        RawString::from(COMMUNITY_BASE_ENTRY).into(),
    );
    let base_address = commit_if_not_in_chain(&base_entry)?;

    let slug_entry = Entry::App(
        COMMUNITY_BASE_ENTRY.into(),
        RawString::from(slug.clone()).into(),
    );
    let slug_address = hdk::commit_entry(&slug_entry)?;

    let community = CommunityEntry {
        name: name.clone(),
        slug: slug.clone(),
    };

    let community_address = hdk::commit_entry(&Entry::App(
        COMMUNITY_ENTRY_TYPE.into(),
        community.clone().into(),
    ))?;

    hdk::link_entries(&base_address, &community_address, COMMUNITY_LINK_TYPE, "")?;
    hdk::link_entries(&slug_address, &community_address, COMMUNITY_LINK_TYPE, "")?;

    Ok(community.with_address(community_address))
}

pub fn all() -> ZomeApiResult<Vec<Community>> {
    let address = hdk::entry_address(&Entry::App(
        COMMUNITY_BASE_ENTRY.into(),
        RawString::from(COMMUNITY_BASE_ENTRY).into(),
    ))?;
    Ok(hdk::get_links(
        &address,
        LinkMatch::Exactly(COMMUNITY_LINK_TYPE.into()),
        LinkMatch::Any,
    )?
    .addresses()
    .iter()
    .map(|address| get(address.to_string().into()).unwrap())
    .chain(
        DEFAULT_COMMUNITIES
            .iter()
            .map(|t| Community::from(CommunityEntry::from(t))),
    )
    .collect())
}

pub fn community_def() -> ValidatingEntryType {
    entry!(
        name: COMMUNITY_ENTRY_TYPE,
        description: "",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<CommunityEntry>| {
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
                COMMUNITY_ENTRY_TYPE,
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
