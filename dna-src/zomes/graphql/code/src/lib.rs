#![feature(try_from)]

#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate derive_more;
#[macro_use] extern crate cached;

use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::{
        json::{JsonString, RawString},
        error::HolochainError,
    }
};

mod schema;
mod holochain_juniper;

use crate::schema::{Mutation, Query, Schema};


define_zome! {
    entries: []

    genesis: || { Ok(()) }

    functions: [
        graphql: {
            inputs: |query: String, variables: juniper::Variables|,
            outputs: |result: ZomeApiResult<RawString>|,
            handler: handle_query
        }
    ]
    traits: { hc_public [graphql] }
}

pub struct Context {}
impl juniper::Context for Context {}

pub fn handle_query(query: String, variables: juniper::Variables) -> ZomeApiResult<RawString> {
    // execute query using juniper on this zomes schema

    hdk::debug(format!("{:?}", variables))?;

    let ctx = Context{};

    let (res, errors) =
        juniper::execute(&query, None, &Schema::new(Query, Mutation), &variables, &ctx)
            .map_err(|e| ZomeApiError::Internal(format!("{:?}", e)))?;

    match errors.len() {
        0 => {
            let result_string =
                serde_json::to_string(&res)
                .map_err(|e| ZomeApiError::Internal(e.to_string()))?;

            Ok(RawString::from(result_string))
        },
        _ => {
            hdk::debug(format!("{:?}", errors))?;
            Err(ZomeApiError::Internal(format!("{:?}", errors)))
        }
    }
}
