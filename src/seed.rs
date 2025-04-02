use sqlite;
use std::fs;

fn main() -> std::io::Result<()> {
    fs::remove_file("cm.db")?;
    let connection = sqlite::open("cm.db").expect("Error: cannot open db");
    let mut queries: Vec<String> = Vec::new();
    queries.push(get_create_tables_query());
    queries.push(get_populate_tables_query());
    for query in queries {
        match connection.execute(query.clone()) {
            Ok(_) => println!("Executed: {query}"),
            Err(e) => println!("Error executing: {query}\nError: {e}"),
        }
    }
    Ok(())
}

fn get_create_tables_query() -> String {
    "CREATE TABLE project (name TEXT, description TEXT, picture TEXT, skills TEXT);
    CREATE TABLE profile(display_name TEXT, picture TEXT, description TEXT);
    "
    .to_string()
}

fn get_populate_tables_query() -> String {
    let profile = get_profile_query();
    let project1 = get_project1_query();
    let project2 = get_project2_query();

    format!("{}{}{}", profile, project1, project2)
}

fn get_profile_query() -> String {
    let profile_description = "\
Lifelong learner, I made my first program when I was sixteen. It was a GCD calculator implementing an algorithm I had learned at school. In the first part of my adult life, I studied sociology and then became a teacher to share my passion for learning and nurture my student's curiosity. After meeting one of my student's father, I realized that talking about computers made me feel very good, and I decided to turn what was a passion into a profession. I now work as a full stack developper and learn everything I can about architecture.";
    format!(
        "INSERT INTO profile (display_name, picture, description)
        VALUES(\"Emmanuel Guefif\", \"emmanuel.jpeg\", \"{}\");
            ",
        profile_description
    )
}

fn get_project1_query() -> String {
    let description = "Single Page Application pong with a backend. ";
    format!("INSERT INTO project (name, picture, description, skills)
        VALUES (\"ft_transcendance\", \"project1.jpeg\", \"{}\", \"django, javascript, python, async/await\");", description)
}

fn get_project2_query() -> String {
    let description = "Weather app with search location";
    format!(
        "INSERT INTO project (name, picture, description, skills)
        VALUES (\"Weather app\", \"project1.jpeg\", \"{}\", \"flutter, api\");",
        description
    )
}
