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
        validation::EntryValidationData,
    },
    holochain_json_api::{
        error::JsonError,
        json::{JsonString},
    },
    holochain_persistence_api::{cas::content::Address},
};

mod communities;

define_zome! {
    entries: [
        communities::base_def(),
        communities::community_def()
    ]

    init: || { Ok(()) }

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
