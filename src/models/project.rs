use crate::connector::{SqlEngine, SqlType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub picture: String,
    pub skills: Vec<String>,
    pub github: String,
    pub id: Option<i64>,
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
            let github = if let SqlType::Text(value) = result.get("github").unwrap() {
                value.to_string()
            } else {
                "".to_string()
            };
            let id = if let SqlType::Text(value) = result.get("id").unwrap() {
                value.parse::<i64>().unwrap()
            } else {
                0
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
                github,
                id: Some(id),
            });
        }
        retval
    }

    pub fn save(&mut self, engine: &mut SqlEngine) -> String {
        self.sanitize();
        let skills = self.skills.iter().fold(String::new(), |acc, skill| {
            if skill.len() > 0 {
                format!("{}{},", acc, skill)
            } else {
                acc
            }
        });
        let query = format!(
            "
        INSERT INTO project (name, description, picture, github, skills)
                VALUES (\"{}\", \"{}\", \"{}\", \"{}\", \"{}\");",
            self.name, self.description, self.picture, self.github, skills,
        );
        println!("{:?}", engine.execute(&query));
        String::from("{\"success\": true}")
    }

    pub fn delete(&mut self, engine: &mut SqlEngine) {
        if let Some(id) = self.id {
            let query = format!("DELETE FROM project WHERE id = {}", id);
            engine.execute(&query);
        }
    }

    fn sanitize(&mut self) {
        self.name = self.name.replace("\'", "\'\'");
        self.description = self.description.replace("\'", "\'\'");
        self.picture = self.picture.replace("\'", "\'\'");
        self.picture = self.picture.replace("\'", "\'\'");
        self.picture = self.picture.replace("\'", "\'\'");
        self.skills = self
            .skills
            .iter()
            .map(|skill| skill.replace("\'", "\'\'"))
            .collect::<_>();
    }
}
