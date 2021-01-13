use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    // your custom commands
    // multiple arguments are allowed
    // note that rename_all = "camelCase": you need to use "myCustomCommand" on JS
    // MyCustomCommand { argument: String },
    ChooseSkinElement { 
        elem_type: String,
        callback: String,
        error: String,
    },
}

#[derive(Serialize)]
pub struct ChooseSkinElementResponse {
    images: HashMap<String, String>,
}

impl ChooseSkinElementResponse { 
    pub fn new(images: HashMap<String, String>) -> Self {
        Self { images }
    }
}
