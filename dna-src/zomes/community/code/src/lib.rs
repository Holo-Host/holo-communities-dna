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
        get_community_address_by_slug: {
            inputs: |slug: String|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: community::get_community_address_by_slug
        }
        create_community: {
            inputs: |name: String, slug: String|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: community::create_community
        }
        get_communities: {
            inputs: | |,
            outputs: |result: ZomeApiResult<Vec<Address>>|,
            handler: community::get_communities
        }
    ]

    traits: {
        hc_public [
            get_community,
            create_community,
            get_communities,
            get_community_address_by_slug
        ]
    }
}
