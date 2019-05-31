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
mod identity;

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
        identity::def()
    ]

    genesis: || { Ok(()) }

    functions: [
        get_identity: {
            inputs: |agent_id: Address|,
            outputs: |result: ZomeApiResult<identity::Identity>|,
            handler: identity::get_identity
        }
        register_user: {
            inputs: |name: String, avatar_url:String|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: identity::register_user
        }
        get_people: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Vec<Address>>|,
            handler: identity::get_people
        }
    ]
    traits: { 
        hc_public [
            get_identity,
            register_user,
            get_people
        ] 
    }
}