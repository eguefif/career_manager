use crate::connector::{SqlEngine, SqlType};
use crate::log_error;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    pub title: String,
    pub content: String,
    pub created_at: Option<String>,
    pub id: Option<i64>,
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
            let created_at = if let SqlType::Text(value) = result.get("created_at").unwrap() {
                value.to_string()
            } else {
                "".to_string()
            };
            let id = if let SqlType::Int(value) = result.get("id").unwrap() {
                *value
            } else {
                0
            };
            retval.push(Self {
                title,
                content,
                created_at: Some(created_at),
                id: Some(id),
            });
        }
        retval
    }

    pub fn save(&mut self, engine: &mut SqlEngine) -> String {
        let now: DateTime<Utc> = Utc::now();
        let created_at = format!("{}", now.format("%A, %d %m %Y %H:%M:%S GMT"));
        let mut params = self.make_params();
        params.push(SqlType::Text(created_at.clone()));
        match engine.execute_insert("article", &["title", "content", "created_at"], &params) {
            Ok(_) => String::from("{\"success\": true}"),
            Err(e) => {
                log_error(&format!("Error while preparing query: {}", e));
                String::from("{\"success\": false}")
            }
        }
    }

    fn make_params(&mut self) -> Vec<SqlType> {
        let mut retval = vec![];

        retval.push(SqlType::Text(self.title.clone()));
        retval.push(SqlType::Text(self.content.clone()));

        retval
    }
}
