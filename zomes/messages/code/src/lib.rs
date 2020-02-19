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
        time::Iso8601,
    },
    holochain_json_api::{
        error::JsonError,
        json::JsonString,
    },
    holochain_persistence_api::cas::content::Address,
};

mod message;
mod thread;

define_zome! {
    entries: [
        message::def(),
        thread::def()
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
        create_thread: {
            inputs: |participant_addresses: Vec<Address>, timestamp: Iso8601|,
            outputs: |result: ZomeApiResult<thread::Thread>|,
            handler: thread::create
        }
        create_message: {
            inputs: |thread_address: Address, text: String, timestamp: Iso8601|,
            outputs: |result: ZomeApiResult<message::Message>|,
            handler: message::create
        }
        all_threads: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Vec<thread::Thread>>|,
            handler: thread::all
        }
        get_thread: {
            inputs: |thread_address: Address|,
            outputs: |result: ZomeApiResult<thread::Thread>|,
            handler: thread::get
        }
        all_messages_for_thread: {
            inputs: |thread_address: Address|,
            outputs: |result: ZomeApiResult<Vec<message::Message>>|,
            handler: message::all
        }
        set_last_read_time: {
            inputs: |thread_address: Address, last_read_time: Iso8601|,
            outputs: |result: ZomeApiResult<thread::Thread>|,
            handler: thread::set_last_read_time
        }
    ]
    traits: {
        hc_public [
            create_thread,
            create_message,
            all_threads,
            get_thread,
            all_messages_for_thread,
            set_last_read_time
        ]
    }
}
