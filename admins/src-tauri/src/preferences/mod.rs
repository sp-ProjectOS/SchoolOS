use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preferences {
    dark_mode: bool,
}

impl Preferences {
    pub fn new() -> Self {
        let preferences = Preferences { dark_mode: true };
        preferences
    }
}

pub fn get_local_preferences() -> Preferences {
    let preferences = Preferences::new();
    preferences
}
