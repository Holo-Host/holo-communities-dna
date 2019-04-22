use core::hash::Hash;
use hdk::error::{ZomeApiResult, ZomeApiError};
use hdk::holochain_core_types::error::HolochainError;
use std::fmt;

use juniper::ID;
use hdk::holochain_core_types:: {
    cas::content::Address,
    json::JsonString,
};

/*
 * a wrapper for the hdk::call function that routes it to other zomes in this DNA
 * and deals with deserializing the ZomeApiResult from string.Default
 * This will get much better soon with the upgrades to the hdk::call
*/

cached!{
    CACHE;
    fn call_cached(
        zome_name: &'static str,
        fn_name: &'static str,
        fn_args: JsonString
    ) -> ZomeApiResult<serde_json::Value> = {
        let result_json_string = hdk::call(hdk::THIS_INSTANCE, zome_name, Address::from(hdk::PUBLIC_TOKEN.to_string()), fn_name, fn_args)?;
        let v: serde_json::Value = serde_json::from_str(&result_json_string.to_string())
            .map_err(|_e| {
                HolochainError::ErrorGeneric("Could not parse response from call".to_string())
            })?;
        if let Some(result) = v.get("Ok") {
            return Ok(result.to_owned())
        }
        return Err(ZomeApiError::Internal(format!("Parsed response did not contain Ok variant, {:?}", v).to_string()))
    }
}



#[derive(Clone, PartialEq, Eq)]
pub struct HID(ID);

impl fmt::Display for HID {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt.write_str(&self.0.to_string())?;
		Ok(())
	}
}

// conversions to and from String
impl From<String> for HID {
    fn from(str: String) -> Self {
        HID(str.into())
    }
}

impl Into<String> for HID {
    fn into(self) -> String {
        self.0.to_string()
    }
}

impl From<&str> for HID {
    fn from(str: &str) -> Self {
        HID(String::from(str).into())
    }
}

// conversions to and from JsonString
impl From<JsonString> for HID {
    fn from(str: JsonString) -> Self {
        HID(str.to_string().into())
    }
}

impl Into<JsonString> for HID {
    fn into(self) -> JsonString {
        JsonString::from_json(&self.0.to_string())
    }
}

// conversions to and from juniper HID
impl From<ID> for HID {
    fn from(jid: ID) -> Self {
        HID(jid)
    }
}

impl Into<ID> for HID {
    fn into(self) -> ID {
        self.0
    }
}


// conversions to and from holochain Address
impl From<Address> for HID {
    fn from(a: Address) -> Self {
        HID(a.to_string().into())
    }
}

impl Into<Address> for HID {
    fn into(self) -> Address {
        self.0.to_string().into()
    }
}
