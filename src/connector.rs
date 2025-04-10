use sqlite;
use sqlite::{State, Type};
use std::collections::HashMap;

pub type SqlResult = Vec<HashMap<String, SqlType>>;

pub enum SqlType {
    Text(String),
    Num(f64),
    Bool(bool),
    Binary(Vec<u8>),
    Null,
}

pub struct SqlEngine {
    conn: sqlite::Connection,
}

impl SqlEngine {
    pub fn new(path: &str) -> Self {
        Self {
            conn: sqlite::open(path).expect("Error: Impossible to open db"),
        }
    }

    pub fn execute(&mut self, query: &str) -> SqlResult {
        let mut retval: SqlResult = Vec::new();
        let mut statement = self.conn.prepare(query).unwrap();
        while let Ok(State::Row) = statement.next() {
            let mut row: HashMap<String, SqlType> = HashMap::new();
            for (i, col) in statement.column_names().iter().enumerate() {
                let col_type = statement.column_type(i).unwrap();
                let value = match col_type {
                    Type::String => SqlType::Text(statement.read::<String, usize>(i).unwrap()),
                    _ => panic!("Not handled"),
                };
                row.insert(col.to_string(), value);
            }
            retval.push(row);
        }
        retval
    }
}
