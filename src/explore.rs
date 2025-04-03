use sqlite;
use sqlite::{State, Type};

fn main() {
    let conn = sqlite::open("cm.db").unwrap();
    let query = "SELECT * FROM project;";
    let mut statement = conn.prepare(query).unwrap();
    while let Ok(State::Row) = statement.next() {
        for (i, col) in statement.column_names().iter().enumerate() {
            let col_type = statement.column_type(i).unwrap();
            match col_type {
                Type::String => {
                    let value = statement.read::<String, usize>(i).unwrap();
                    println!("key{col}, value(String){value}");
                }
                _ => {
                    println!("not handle");
                }
            }
        }
    }
}
