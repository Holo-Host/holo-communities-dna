#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

mod comments;

use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
        agent::AgentId,
        time::Iso8601,
        validation::EntryValidationData,
    },
    holochain_json_api::{
        error::JsonError,
        json::{JsonString},
    },
    holochain_persistence_api::{cas::content::Address},
};

define_zome! {
    entries: [
       comments::comment_def(),
       comments::base_def()
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
        create: {
            inputs: |base: String, text: String, timestamp: Iso8601|,
            outputs: |result: ZomeApiResult<comments::CommentWithAddress>|,
            handler: comments::create
        }
        get: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<comments::CommentWithAddress>|,
            handler: comments::get
        }
        all_for_base: {
            inputs: |base: String|,
            outputs: |result: ZomeApiResult<Vec<comments::CommentWithAddress>>|,
            handler: comments::all_for_base
        }
    ]

    traits: {
        hc_public [
            create,
            get,
            all_for_base
        ]
    }
}
