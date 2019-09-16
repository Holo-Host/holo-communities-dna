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

mod post;

define_zome! {
    entries: [
        post::post_def(),
        post::base_def()
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
            outputs: |result: ZomeApiResult<post::PostWithAddress>|,
            handler: post::get
        }
        create: {
            inputs: |base: String, title: String, details: String, post_type: String, announcement: bool, timestamp: String|,
            outputs: |result: ZomeApiResult<post::PostWithAddress>|,
            handler: post::create
        }
        all_for_base: {
            inputs: |base: String|,
            outputs: |result: ZomeApiResult<Vec<post::PostWithAddress>>|,
            handler: post::all_for_base
        }
    ]

    traits: {
        hc_public [
            get,
            create,
            all_for_base
        ]
    }
}
