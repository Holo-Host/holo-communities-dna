#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod comment_entry;
mod base_entry;

use hdk::{
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    error::HolochainError,
    json::JsonString,
};

use comment_entry::{
    CommentData,
    Comment,
    comment_def,
    handle_create_comment,
    handle_get_comment,
};
use base_entry::{
    base_def,
    handle_get_comments,
};

define_zome! {
    entries: [
       comment_def(),
       base_def()
    ]

    genesis: || { Ok(()) }

    functions: [
        create_comment: {
            inputs: |comment: CommentData|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_comment
        }
        get_comment: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<Comment>|,
            handler: handle_get_comment
        }
        get_comments: {
            inputs: |base: String|,
            outputs: |result: ZomeApiResult<Vec<Comment>>|,
            handler: handle_get_comments
        }
    ]

    traits: {
        hc_public [
            create_comment,
            get_comment,
            get_comments
        ]
    }
}
