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
        post::def()
    ]

    genesis: || { Ok(()) }

    functions: [
        
    ]
    traits: { 
        hc_public [
        ] 
    }
}
