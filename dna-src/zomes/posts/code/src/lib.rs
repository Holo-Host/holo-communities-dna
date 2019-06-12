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

mod post;

define_zome! {
    entries: [
        post::post_def(),
        post::base_def()
    ]

    genesis: || { Ok(()) }

    functions: [
         get: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<post::PostResult>|,
            handler: post::get
        }
        create: {
            inputs: |base: String, title: String, details: String, post_type: String, announcement: bool, timestamp: String|,
            outputs: |result: ZomeApiResult<post::PostResult>|,
            handler: post::create
        }
        all_for_base: {
            inputs: |base: String|,
            outputs: |result: ZomeApiResult<Vec<post::PostResult>>|,
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
