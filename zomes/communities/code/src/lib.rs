#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
        agent::AgentId,
        entry::Entry,
        validation::EntryValidationData,
    },
    holochain_json_api::{
        error::JsonError,
        json::{JsonString},
    },
    holochain_persistence_api::{cas::content::Address},
};

mod communities;
use communities::{Community, COMMUNITY_ENTRY_TYPE};

// I'll put this here for now but really it should live in the .dna.json properties
const DEFAULT_COMMUNITIES: &[(&str, &str)] = &[
    ("Hylo Holochain", "hylo-holochain"),
    ("HoloPort Owners", "holoport-owners"),
    ("HoloPort Support", "holoport-support"),
];

define_zome! {
    entries: [
        communities::base_def(),
        communities::community_def()
    ]

    init: || {{
        // create the default communities that every DNA has.
        // Don't use the create function because it is important we don't link them to anything
        for tuple in DEFAULT_COMMUNITIES {
            hdk::commit_entry(
                &Entry::App (
                    COMMUNITY_ENTRY_TYPE.into(),
                    Community::from(tuple).into(),
                )
            )?;
        }
        Ok(()) 
    }}

    validate_agent: |validation_data : EntryValidationData::<AgentId>| {{
         if let EntryValidationData::Create{entry, ..} = validation_data {
             let agent = entry as AgentId;
             if agent.nick == "reject_agent::app" {
                 Err("This agent will always be rejected".into())
             } else {
                 Ok(())
             }
         } else {
             Err("Cannot update or delete an agent at this time".into())
         }
     }}

    functions: [
        get: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<communities::CommunityWithAddress>|,
            handler: communities::get
        }
        get_by_slug: {
            inputs: |slug: String|,
            outputs: |result: ZomeApiResult<communities::CommunityWithAddress>|,
            handler: communities::get_by_slug
        }
        create: {
            inputs: |name: String, slug: String|,
            outputs: |result: ZomeApiResult<communities::CommunityWithAddress>|,
            handler: communities::create
        }
        all: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Vec<communities::CommunityWithAddress>>|,
            handler: communities::all
        }
    ]

    traits: {
        hc_public [
            get,
            create,
            all,
            get_by_slug
        ]
    }
}
