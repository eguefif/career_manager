use crate::connector::{SqlEngine, SqlType};
use crate::log_error;
use chrono::{DateTime, NaiveDateTime, Utc};
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
            engine.execute("SELECT * FROM article ORDER BY created_at")
        };
        for result in results {
            let title = if let SqlType::Text(value) = result.get("title").unwrap() {
                value.to_string()
            } else {
                "".to_string()
            };
            let content = if let SqlType::Binary(value) = result.get("content").unwrap() {
                String::from_utf8_lossy(value).to_string()
            } else {
                "".to_string()
            };
            let created_at = if let SqlType::Text(value) = result.get("created_at").unwrap() {
                let date = NaiveDateTime::parse_from_str(value, "%D %T %f");
                match date {
                    Ok(date) => date.format("%Y %B %d").to_string(),
                    Err(_) => "".to_string(),
                }
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
        let params = self.make_params();
        match engine.execute_insert("article", &["title", "content", "created_at"], &params) {
            Ok(_) => String::from("{\"success\": true}"),
            Err(e) => {
                log_error(&format!("Error while preparing query: {}", e));
                String::from("{\"success\": false}")
            }
        }
    }

    fn make_params(&mut self) -> Vec<SqlType> {
        let now: DateTime<Utc> = Utc::now();
        let mut retval = vec![];
        let created_at = format!("{}", now.format("%D %T %f"));

        retval.push(SqlType::Text(self.title.clone()));
        retval.push(SqlType::Binary(self.content.as_bytes().to_vec()));
        retval.push(SqlType::Text(created_at.clone()));
        self.created_at = Some(created_at);

        retval
    }

    pub fn delete(&mut self, engine: &mut SqlEngine) -> String {
        let id = self.id.unwrap();
        match engine.execute_delete_id("article", SqlType::Int(id)) {
            Ok(_) => String::from("{\"success\": true}"),
            Err(e) => {
                log_error(&format!("Error while preparing query: {}", e));
                String::from("{\"success\": false}")
            }
        }
    }

    pub fn update(&mut self, engine: &mut SqlEngine, article: Article) -> String {
        self.title = article.title;
        self.content = article.content;
        let mut params = self.make_params();
        let id = format!("{}", self.id.unwrap());
        match engine.execute_update(
            "article",
            &["title", "content", "created_at"],
            &mut params,
            id,
        ) {
            Ok(_) => String::from("{\"success\": true}"),
            Err(e) => {
                log_error(&format!("Error while preparing query: {}", e));
                String::from("{\"success\": false}")
            }
        }
    }
}
