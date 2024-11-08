#[allow(warnings)]
mod bindings;

use crate::bindings::exports::cd40::helm::template::Guest;

struct Component;

impl Guest for Component {
    fn execute(_chart_zip: Vec<u8>) -> Vec<String> {
        vec!["hello", "world"].iter().map(|s| s.to_string()).collect()
    }
}

bindings::export!(Component with_types_in bindings);
