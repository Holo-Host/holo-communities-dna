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

mod community;

define_zome! {
    entries: [
        community::base_def(),
        community::community_def()
    ]

    genesis: || { Ok(()) }

    functions: [
         get_community: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<community::Community>|,
            handler: community::get_community
        }
        create_community: {
            inputs: |base: String, name: String, slug: String|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: community::create_community
        }
        get_communitys: {
            inputs: |base: String|,
            outputs: |result: ZomeApiResult<Vec<Address>>|,
            handler: community::get_communitys
        }
    ]

    traits: {
        hc_public [
            get_community,
            create_community,
            get_communitys
        ]
    }
}
