use crate::connector::{SqlEngine, SqlType};

pub const BASE_PICTURE_PATH: &str = "/images/";

pub struct Profile {
    pub display_name: String,
    pub description: String,
    pub picture: String,
}

impl Profile {
    pub fn take_first(engine: &mut SqlEngine) -> Option<Self> {
        let result = engine.execute("SELECT * FROM profile LIMIT 1");
        if result.len() != 1 {
            return None;
        }
        let display_name = if let SqlType::Text(value) = result[0].get("display_name").unwrap() {
            value.to_string()
        } else {
            "".to_string()
        };
        let description = if let SqlType::Text(value) = result[0].get("description").unwrap() {
            value.to_string()
        } else {
            "".to_string()
        };
        let picture = if let SqlType::Text(value) = result[0].get("picture").unwrap() {
            value.to_string()
        } else {
            "".to_string()
        };
        Some(Self {
            display_name,
            description,
            picture,
        })
    }

    pub fn to_json(&self) -> Vec<u8> {
        let picture = format!("{}{}", BASE_PICTURE_PATH, self.picture);
        let string = format!(
            "{{\"displayName\": \"{}\", \"description\": \"{}\", \"picture\": \"{}\"}}",
            self.display_name, self.description, picture
        );

        string.as_bytes().to_vec()
    }
}
