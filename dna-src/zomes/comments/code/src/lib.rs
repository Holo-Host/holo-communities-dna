#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod comments;

use hdk::{
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    error::HolochainError,
    json::JsonString,
    time::Iso8601,
};

define_zome! {
    entries: [
       comments::comment_def(),
       comments::base_def()
    ]

    genesis: || { Ok(()) }

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
