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
        post_message_to_thread: {
            inputs: |thread_addr: Address, text: String, timestamp: String|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: message::post_message_to_thread
        } 
        get_message: {
            inputs: |message_addr: Address|,
            outputs: |result: ZomeApiResult<message::Message>|,
            handler: message::get_message
        }    
        // thread functions
        get_my_threads: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Vec<Address>>|,
            handler: thread::get_my_threads
        }  
        get_or_create_thread: {
            inputs: |participant_ids: Vec<String>|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: thread::get_or_create_thread
        }  
        get_thread_participants: {
            inputs: |thread_addr: Address|,
            outputs: |result: ZomeApiResult<Vec<Address>>|,
            handler: thread::get_thread_participants
        }          
        get_thread_messages: {
            inputs: |thread_addr: Address|,
            outputs: |result: ZomeApiResult<Vec<Address>>|,
            handler: thread::get_thread_messages
        }   
    ]
    traits: { 
        hc_public [
            post_message_to_thread,
            get_message,
            get_my_threads,
            get_or_create_thread,
            get_thread_participants,
            get_thread_messages
        ] 
    }
}
