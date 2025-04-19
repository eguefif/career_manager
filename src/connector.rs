use sqlite;
use sqlite::{State, Type};
use std::collections::HashMap;
use std::fmt;

use crate::log_error;

pub type SqlResult = Vec<HashMap<String, SqlType>>;

#[derive(Debug)]
pub enum SqlType {
    Text(String),
    Int(i64),
    Bool(bool),
    Binary(Vec<u8>),
    Null,
}

impl fmt::Display for SqlType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SqlType::Text(value) => write!(f, "{}", value),
            SqlType::Int(value) => write!(f, "{}", value),
            SqlType::Bool(value) => write!(f, "{}", value),
            SqlType::Binary(value) => write!(f, "{}", String::from_utf8_lossy(value)),
            SqlType::Null => write!(f, "NULL"),
        }
    }
}

pub struct SqlEngine {
    conn: sqlite::ConnectionThreadSafe,
}

impl SqlEngine {
    pub fn new(path: &str) -> Self {
        Self {
            conn: sqlite::Connection::open_thread_safe(path).expect("Error: Impossible to open db"),
        }
    }

    pub fn execute(&mut self, query: &str) -> SqlResult {
        println!("\x1b[94mDB query: {query}\x1b[0m\n");
        let mut retval: SqlResult = Vec::new();
        let mut statement = self.conn.prepare(query).unwrap();
        while let Ok(State::Row) = statement.next() {
            let mut row: HashMap<String, SqlType> = HashMap::new();
            for (i, col) in statement.column_names().iter().enumerate() {
                let col_type = statement.column_type(i).unwrap();
                let value = match col_type {
                    Type::String => SqlType::Text(statement.read::<String, usize>(i).unwrap()),
                    Type::Integer => SqlType::Int(statement.read::<i64, usize>(i).unwrap()),
                    Type::Null => SqlType::Null,
                    Type::Binary => SqlType::Binary(statement.read::<Vec<u8>, usize>(i).unwrap()),
                    _ => {
                        log_error(&format!("Error with SQL response: {:?}", col_type));
                        panic!("In execute Connector");
                    }
                };
                row.insert(col.to_string(), value);
            }
            retval.push(row);
        }
        retval
    }

    pub fn execute_insert(
        &mut self,
        table: &str,
        cols: &[&str],
        values: &[SqlType],
    ) -> Result<(), String> {
        let query = build_insert_query(table, cols);
        self.execute_query(query, values)
    }
    pub fn execute_update(
        &mut self,
        table: &str,
        cols: &[&str],
        values: &mut Vec<SqlType>,
        id: String,
    ) -> Result<(), String> {
        let query = build_update_query(table, cols);
        values.push(SqlType::Text(id));
        self.execute_query(query, values)
    }

    fn execute_query(&mut self, query: String, values: &[SqlType]) -> Result<(), String> {
        println!("\x1b[94mDB query: {query}\x1b[0m\n");
        println!("\x1b[94mDB values: {:?}\x1b[0m\n", values);
        match self.conn.prepare(query) {
            Ok(mut stmt) => {
                for (i, value) in values.iter().enumerate() {
                    match value {
                        SqlType::Text(value) => stmt.bind((i + 1, value.as_str())).unwrap(),
                        SqlType::Int(value) => stmt.bind::<(usize, i64)>((i + 1, *value)).unwrap(),
                        SqlType::Bool(value) => {
                            if *value {
                                stmt.bind::<(usize, i64)>((i + 1, 1)).unwrap()
                            } else {
                                stmt.bind::<(usize, i64)>((i + 1, 0)).unwrap()
                            }
                        }
                        SqlType::Binary(value) => {
                            stmt.bind::<(usize, &[u8])>((i + 1, value)).unwrap()
                        }
                        SqlType::Null => todo!(),
                    }
                }
                loop {
                    if let Ok(State::Done) = stmt.next() {
                        return Ok(());
                    }
                }
            }
            Err(e) => {
                return Err(format!("Impossible to prepare connection: {}", e));
            }
        }
    }

    pub fn execute_delete_id(&mut self, table: &str, value: SqlType) -> Result<(), String> {
        let query = format!("DELETE FROM {} WHERE id=?", table);
        self.execute_query(query, &[value])
    }
}

fn build_insert_query(table: &str, cols: &[&str]) -> String {
    let mut query = String::new();
    query.push_str(&format!("INSERT INTO {} (", table));
    let mut cols_str = String::new();
    let mut values_str = String::new();
    for (i, col) in cols.iter().enumerate() {
        if i > 0 {
            cols_str.push(',');
            values_str.push(',');
        }
        cols_str.push_str(col);
        values_str.push('?');
    }
    query.push_str(&cols_str);
    query.push(')');
    query.push_str("VALUES (");
    query.push_str(&values_str);
    query.push_str(");");
    query
}

fn build_update_query(table: &str, cols: &[&str]) -> String {
    let mut query = String::new();
    query.push_str(&format!("UPDATE {} SET ", table));
    for (i, col) in cols.iter().enumerate() {
        query.push_str(&format!("{}=?", col));
        if i < cols.len() - 1 {
            query.push(',');
        }
    }
    query.push_str(" WHERE id = ?");
    query.push(';');
    query
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_build_insert_query() {
        let result = build_insert_query("article", &["title", "content", "created_at"]);

        assert_eq!(
            result,
            "INSERT INTO article (title,content,created_at)VALUES (?,?,?);"
        );
    }

    #[test]
    fn it_should_build_update_query() {
        let result = build_update_query("article", &["title", "content", "created_at"]);

        assert_eq!(
            result,
            "UPDATE article SET title=?,content=?,created_at=? WHERE id = ?;"
        );
    }
}
