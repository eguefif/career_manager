use crate::connector::{SqlEngine, SqlType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    pub title: String,
    pub content: String,
    pub date: String,
}

impl Article {
    pub fn find(engine: &mut SqlEngine, id: i64) -> Option<Self> {
        let results = Self::all(engine, Some(id));
        if results.len() == 0 {
            return None;
        }
        let project: &Article = &results[0];
        Some(project.clone())
    }

    pub fn all(engine: &mut SqlEngine, id: Option<i64>) -> Vec<Self> {
        let mut retval = Vec::new();
        let results = if let Some(id) = id {
            engine.execute(&format!("SELECT * FROM article WHERE id = {}", id))
        } else {
            engine.execute("SELECT * FROM article")
        };
        for result in results {
            let title = if let SqlType::Text(value) = result.get("title").unwrap() {
                value.to_string()
            } else {
                "".to_string()
            };
            let content = if let SqlType::Text(value) = result.get("content").unwrap() {
                value.to_string()
            } else {
                "".to_string()
            };
            let date = if let SqlType::Text(value) = result.get("date").unwrap() {
                value.to_string()
            } else {
                "".to_string()
            };
            retval.push(Self {
                title,
                content,
                date,
            });
        }
        retval
    }
}
