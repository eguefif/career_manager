fn main() {
    let connection = sqlite::open("cm.db").expect("Error: cannot open db");
    let query = "SELECT * from project;";
    for row in connection
        .prepare(query)
        .unwrap()
        .into_iter()
        .map(|row| row.unwrap())
    {
        println!("Name: {:?}", row.read::<&str, _>("name"));
        println!("Description: {:?}", row.read::<&str, _>("description"));
        println!("Skills: {:?}", row.read::<&str, _>("skills"));
        println!("Skills: {:?}", row.read::<&str, _>("picture"));
        println!();
    }
}
