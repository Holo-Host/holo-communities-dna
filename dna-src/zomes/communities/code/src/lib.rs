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

mod communities;

define_zome! {
    entries: [
        communities::base_def(),
        communities::community_def()
    ]

    genesis: || { Ok(()) }

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
