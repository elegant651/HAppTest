#[macro_use]
extern crate hdk;

define_zome! {
    entries: [
    ]

    init: || {
        Ok(())
    }

    validate_agent: |validation_data : EntryValidationData::<AgentId>| {
        Ok(())
    }

    functions: [
    ]

    traits: {
    }
}