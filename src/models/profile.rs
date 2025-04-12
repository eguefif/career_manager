use crate::connector::{SqlEngine, SqlType};
use serde::{Deserialize, Serialize};

pub const BASE_PICTURE_PATH: &str = "/images/";

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub display_name: String,
    pub description: String,
    pub picture: String,
    pub id: String,
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
        let id = if let SqlType::Text(value) = result[0].get("id").unwrap() {
            value.to_string()
        } else {
            "".to_string()
        };
        Some(Self {
            display_name,
            description,
            picture,
            id,
        })
    }

    pub fn update(&mut self, new_profile: Profile) {
        self.display_name = new_profile.display_name;
        self.description = new_profile.description;
        self.picture = new_profile.picture;

        self.sanitize();
    }

    fn sanitize(&mut self) {
        self.description = self.description.replace("\'", "\'\'");
        println!("Description: \n{}", self.description);
    }

    pub fn save(&mut self, engine: &mut SqlEngine) -> String {
        let query = format!(
            "UPDATE profile
SET display_name='{}', picture='{}', description='{}'
WHERE id = '{}';",
            self.display_name, self.picture, self.description, self.id
        );
        println!("{}", query);
        engine.execute(query.as_str());

        "{\"success\": true}".to_string()
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
