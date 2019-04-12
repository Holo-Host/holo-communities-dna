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
         get_post: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<post::Post>|,
            handler: post::get_post
        }
        create_post: {
            inputs: |base: String, title: String, details: String, post_type: String, announcement: bool, timestamp: String|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: post::create_post
        }
        get_posts: {
            inputs: |base: String|,
            outputs: |result: ZomeApiResult<Vec<Address>>|,
            handler: post::get_posts
        }
    ]

    traits: {
        hc_public [
            get_post,
            create_post,
            get_posts
        ]
    }
}
