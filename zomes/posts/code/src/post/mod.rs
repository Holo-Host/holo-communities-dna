
use hdk::{
    self,
    utils,
    error::ZomeApiResult,
    AGENT_ADDRESS,
    entry_definition::ValidatingEntryType,
    prelude::{QueryArgsOptions, QueryResult},
    holochain_core_types::{
        dna::entry_types::Sharing,
        entry::Entry,
        link::LinkMatch,
    },
    holochain_json_api::{
        error::JsonError,
        json::{JsonString,RawString},
    },
    holochain_persistence_api::{cas::content::Address},
};
use hdk_helpers::DagList;
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Post {
    pub title: String,
    pub details: String,
    pub post_type: String,
    pub creator: Address,
    pub announcement: bool,
    pub timestamp: String,
    pub base: String,
    // fields for the dag list
    prev_authored: Address,
    prev_foreign: Address,
}

impl Post {
    pub fn with_address(&self, address: Address) -> PostWithAddress {
        PostWithAddress {
            address,
            title: self.title.clone(),
            details: self.details.clone(),
            post_type: self.post_type.clone(),
            creator: self.creator.clone(),
            announcement: self.announcement.clone(),
            timestamp: self.timestamp.clone(),
            base: self.base.clone()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct PostWithAddress {
    pub address: Address,
    pub title: String,
    pub details: String,
    pub post_type: String,
    pub creator: Address,
    pub announcement: bool,
    pub timestamp: String,
    pub base: String,
}

pub type Base = RawString;

const POST_ENTRY_TYPE: &str = "post";
const POST_BASE_ENTRY: &str = "post_base";
const POST_LINK_TYPE: &str = "posted_in";

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct GetPostsResult {
    posts: Vec<PostWithAddress>,
    more: bool,
}

pub fn get(address: Address) -> ZomeApiResult<PostWithAddress> {
    utils::get_as_type::<Post>(address.clone())
        .map(|post| {
            post.with_address(address)
        })
}

pub fn create(base: String, title: String, details: String, post_type: String, announcement: bool, timestamp: String) -> ZomeApiResult<PostWithAddress> {

    let base_entry = Entry::App(POST_BASE_ENTRY.into(), RawString::from(base.clone()).into());
    let base_address = hdk::commit_entry(&base_entry)?;

    let post: Post = Post {
        title,
        details,
        post_type,
        creator: AGENT_ADDRESS.to_string().into(),
        announcement,
        timestamp,
        base
    };

    let post_address = hdk::commit_entry(
        &Entry::App (
            POST_ENTRY_TYPE.into(),
            post.clone().into()
        )
    )?;

    // link the post to its originating thing
    hdk::link_entries(
        &base_address,
        &post_address,
        POST_LINK_TYPE,
        ""
    )?;

    Ok(post.with_address(post_address))
}

/**
 * @brief      Traverse the graph and recover all the posts (possibly up to a given limit)
 *
 * @param      base        The base/community for these posts. This is a string and can be considered equivalent to a database table name
 * 
 * @param      since       The starting point for the traversal. Can be the address of a community, or another post.
 *                         If it is a post it will only return those occurring later (allowing for pagination)
 *                         
 * @param      limit       Number of posts to return as a maximum. If this limit is hit will return true for the more boolean
 * 
 * @param      backsteps   Number of backward steps to take in the graph before beginning the traversal.
 *                         This is because it cannot be guaranteed that all posts will be retrieved with a forward only traversal.
 *                         
 *
 * @return     Returns a tuple of the returned entries/addresses and a bool which is true if there are more posts available
 */
pub fn all_for_base(base: String, _since: Option<Address>, _limit: Option<usize>, _backsteps: Option<usize>) -> ZomeApiResult<GetPostsResult> {
    let address = hdk::entry_address(&Entry::App(POST_BASE_ENTRY.into(), RawString::from(base).into()))?;
    Ok(GetPostsResult{
        posts: hdk::get_links(&address, LinkMatch::Exactly(POST_LINK_TYPE.into()), LinkMatch::Any)?
            .addresses()
            .iter()
            .map(|address| get(address.to_string().into()).unwrap())
            .collect(),
        more: false 
    })
}

pub struct PostDagList {}

impl DagList for PostDagList {
    fn author<E: Into<JsonString> + Clone>(
        &mut self,
        table: &str,
        content: E,
        prev_authored: Option<Address>,
        prev_foreign: Option<Address>,
    ) -> ZomeApiResult<Address> {
        let entry = Entry::App(
            "dag_entry_item".into(),
            Post {
                prev_authored: prev_authored.clone().unwrap(),
                prev_foreign: prev_foreign.clone().unwrap(),
                base: String::from(table),
                content: content.into(),
            }.into()
        );
        let entry_addr = hdk::commit_entry(&entry)?;
        if let Some(prev_authored) = prev_authored {
            hdk::link_entries(&prev_authored, &entry_addr, "dag/next", table).or_else(|_| {
                hdk::link_entries(&prev_authored, &entry_addr, "dag/author_root", table)
            })?;
        }
        if let Some(prev_foreign) = prev_foreign {
            hdk::link_entries(&prev_foreign, &entry_addr, "dag/next", table).or_else(|_| {
                hdk::link_entries(&prev_foreign, &entry_addr, "dag/foreign_root", table)
            })?;
        }
        Ok(entry_addr)
    }

    fn author_root_address(&self) -> Address {
        Address::from(hdk::AGENT_ADDRESS.to_string())
    }

    fn get_prev_authored(&self, address: &Address) ->  ZomeApiResult<Option<Address>> {
        if let Some(Entry::App(_, raw)) = hdk::get_entry(address)? {
            if let Ok(item) = Post::try_from(raw) {
                return Ok(Some(item.prev_authored))
            }
        }
        Ok(None)
    }

    fn get_prev_foreign(&self, address: &Address) -> ZomeApiResult<Option<Address>> {
        if let Some(Entry::App(_, raw)) = hdk::get_entry(address)? {
            if let Ok(item) = Post::try_from(raw) {
                return Ok(Some(item.prev_foreign))
            }
        }
        Ok(None)
    }

    fn most_recent_authored(&self, table: &str) -> ZomeApiResult<Option<Address>> {
        match hdk::query_result("dag_entry_item".into(), QueryArgsOptions{ entries: true, ..Default::default()})? {
            QueryResult::Entries(entries) => {
                Ok(entries.iter()
                .filter(|(_addr, entry)| {
                    match entry {
                        Entry::App(_, content) => {
                            let item = Post::try_from(content).unwrap();
                            item.base == table
                        }, 
                        _ => false
                    }
                })
                .map(|(addr, _entry)| addr.clone())
                .collect::<Vec<_>>().last().cloned())
            },
            _ => unreachable!()
        }
    }

    fn get_next(&self, table: &str, address: &Address) -> ZomeApiResult<Vec<Address>> {
        hdk::get_links(address, LinkMatch::Regex("dag/*"), LinkMatch::Exactly(table)).map(|results| {
            results.addresses()
        })
    }
}

pub fn post_def() -> ValidatingEntryType {
    entry!(
        name: POST_ENTRY_TYPE,
        description: "",
        sharing: Sharing::Public,

        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_validation_data: hdk::EntryValidationData<Post>| {
            Ok(())
        }
    )
}

pub fn base_def() -> ValidatingEntryType {
    entry!(
        name: POST_BASE_ENTRY,
        description: "Universally unique ID of something that is being posted in",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Base>| {
            Ok(())
        },
        links: [
            to!(
                POST_ENTRY_TYPE,
                link_type: POST_LINK_TYPE,
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
