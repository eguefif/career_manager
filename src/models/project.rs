use crate::connector::{SqlEngine, SqlType};

pub struct Project {
    pub name: String,
    pub description: String,
    pub picture: String,
    pub skills: Vec<String>,
}

impl Project {
    pub fn all(engine: &mut SqlEngine) -> Vec<Self> {
        let mut retval = Vec::new();
        let results = engine.execute("SELECT * FROM project");
        for result in results {
            let name = if let SqlType::Text(value) = result.get("name").unwrap() {
                value.to_string()
            } else {
                "".to_string()
            };
            let description = if let SqlType::Text(value) = result.get("description").unwrap() {
                value.to_string()
            } else {
                "".to_string()
            };
            let picture = if let SqlType::Text(value) = result.get("picture").unwrap() {
                value.to_string()
            } else {
                "".to_string()
            };
            let skills = if let SqlType::Text(value) = result.get("skills").unwrap() {
                let mut skills = Vec::new();
                let splits = value.split(",");
                for split in splits {
                    skills.push(split.trim().to_string());
                }
                skills
            } else {
                Vec::new()
            };
            retval.push(Self {
                name,
                description,
                picture,
                skills,
            });
        }
        retval
    }
}
