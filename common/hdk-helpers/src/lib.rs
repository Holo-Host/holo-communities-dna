
use hdk::prelude::*;

pub fn commit_if_not_in_chain(entry: &Entry) -> ZomeApiResult<Address> {
	// use query to check the chain. When there is an HDK function doing this directly use it instead
	let existing_entries = hdk::query(entry.entry_type().into(), 0, 0)?;
	if existing_entries.contains(&entry.address()) {
		// do nothing and be happy
		Ok(entry.address())
	} else {
		// do the commit as usual
		hdk::commit_entry(entry)
	}
}
