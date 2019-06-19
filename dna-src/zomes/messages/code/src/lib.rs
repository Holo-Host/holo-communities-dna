#![feature(try_from)]

#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
        json::JsonString,
        cas::content::Address,
        error::HolochainError,
    }
};

mod message;
mod thread;

define_zome! {
    entries: [
        message::def(),
        thread::def()
    ]

    genesis: || { Ok(()) }

    functions: [
        // message functions
        create: {
            inputs: |thread_address: Address, text: String, timestamp: String|,
            outputs: |result: ZomeApiResult<message::MessageWithAddress>|,
            handler: message::create
        } 
        get: {
            inputs: |message_addr: Address|,
            outputs: |result: ZomeApiResult<message::MessageWithAddress>|,
            handler: message::get
        }    
        // thread functions
        get_threads: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Vec<Address>>|,
            handler: thread::get_threads
        }  
        create_thread: {
            inputs: |participant_ids: Vec<String>|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: thread::create_thread
        }  
        get_participants: {
            inputs: |thread_address: Address|,
            outputs: |result: ZomeApiResult<Vec<Address>>|,
            handler: thread::get_thread_participants
        }          
        get_thread_messages: {
            inputs: |thread_address: Address|,
            outputs: |result: ZomeApiResult<Vec<message::MessageWithAddress>>|,
            handler: thread::get_thread_messages
        }   
    ]
    traits: { 
        hc_public [
            create,
            get,
            get_threads,
            create_thread,
            get_participants,
            get_thread_messages
        ] 
    }
}
