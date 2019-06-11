#![feature(try_from)]

#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;
extern crate derive_more;

mod anchor;
mod people;

use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
        json::JsonString,
        cas::content::Address,
        error::HolochainError
    }
};

define_zome! {
    entries: [
        anchor::def(),
        people::def()
    ]

    genesis: || { Ok(()) }

    functions: [
        get: {
            inputs: |agent_id: Address|,
            outputs: |result: ZomeApiResult<people::PersonResult>|,
            handler: people::get
        }
        get_me: {
            inputs: | |,
            outputs: |result: ZomeApiResult<people::PersonResult>|,
            handler: people::get_me
        }
        is_registered: {
            inputs: | |,
            outputs: |result: ZomeApiResult<bool>|,
            handler: people::is_registered
        }
        register_user: {
            inputs: |name: String, avatar_url:String|,
            outputs: |result: ZomeApiResult<people::PersonResult>|,
            handler: people::register_user
        }
        all: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Vec<people::PersonResult>>|,
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