#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;
extern crate derive_more;

mod anchor;
mod people;

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

define_zome! {
    entries: [
        anchor::def(),
        people::def()
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
            inputs: |agent_id: Address|,
            outputs: |result: ZomeApiResult<people::PersonWithAddress>|,
            handler: people::get
        }
        get_me: {
            inputs: | |,
            outputs: |result: ZomeApiResult<people::PersonWithAddress>|,
            handler: people::get_me
        }
        is_registered: {
            inputs: | |,
            outputs: |result: ZomeApiResult<bool>|,
            handler: people::is_registered
        }
        register_user: {
            inputs: |name: String, avatar_url:String|,
            outputs: |result: ZomeApiResult<people::PersonWithAddress>|,
            handler: people::register_user
        }
        all: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Vec<people::PersonWithAddress>>|,
            handler: people::all
        }
    ]
    traits: {
        hc_public [
            get,
            get_me,
            is_registered,
            register_user,
            all
        ]
    }
}
