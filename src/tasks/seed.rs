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
    "CREATE TABLE project (id INTEGER PRIMARY KEY, name TEXT, description TEXT, picture TEXT, skills TEXT, github TEXT);
    CREATE TABLE profile(id INTEGER, display_name TEXT, picture TEXT, description TEXT);
    CREATE TABLE article(id INTEGER, title TEXT, description TEXT, date TEXT);
    "
    .to_string()
}

fn get_populate_tables_query() -> String {
    let profile = get_profile_query();
    let project1 = get_project1_query();
    let project2 = get_project2_query();
    let project3 = get_project3_query();
    let project4 = get_project4_query();

    format!(
        "{}{}{}{}{}",
        profile, project1, project2, project3, project4
    )
}

fn get_profile_query() -> String {
    let profile_description = "\
Lifelong learner, I made my first program when I was sixteen. It was a GCD calculator implementing an algorithm I had learned at school. In the first part of my adult life, I studied sociology and then became a teacher to share my passion for learning and nurture my student's curiosity. After meeting one of my student's father, I realized that talking about computers made me feel very good, and I decided to turn what was a passion into a profession. I now work as a full stack developper and learn everything I can about architecture.";
    format!(
        "INSERT INTO profile (display_name, picture, description, id)
        VALUES(\"Emmanuel Guefif\", \"emmanuel.jpeg\", \"{}\", 0);
            ",
        profile_description
    )
}

fn get_project1_query() -> String {
    let description = "This 42 project is all about Ray Tracing. I worked especially on the implementation of different figure: sphere, place, cylinder, triangle. We optimized rendering by implementing a sample accumulator. We also take advantage of multithreading to render ray by batch.";
    format!("INSERT INTO project (name, picture, description, skills, github)
        VALUES (\"Ray tracer\", \"spheres.png\", \"{}\", \"language C, multithreading, Ray Tracing\", \"https://github.com/PelletierM/miniRT\");", description)
}

fn get_project2_query() -> String {
    let description = "I made this project for my work at Demarque. This project bind a marc record library with an elixir package. When I used this project in the parser, I had to optimize memory usage.";
    format!(
        "INSERT INTO project (name, picture, description, skills, github)
        VALUES (\"Marc-record-ex\", \"marc21.jpg\", \"{}\", \"Rust, Rustler, memory\", \"https://github.com/demarque/marc-record-ex\");",
        description
    )
}

fn get_project3_query() -> String {
    let description = "This project was made when I was reading the book, 'Make a Monkey interpreter in Go. I did in Rust. I've learned a lot about parsing and recursion.";
    format!(
        "INSERT INTO project (name, picture, description, skills, github)
        VALUES (\"Monkey Interpreter\", \"monkey.gif\", \"{}\", \"Rust, parsing\", \"https://github.com/eguefif/monkey_interpreter\");",
        description
    )
}

fn get_project4_query() -> String {
    let description = "This GameBoy emulator is able make Tetris works. This was not an easy project. There is no official documentation but there is a lot of resource you have to gather yourself. It was the occasion to learn by looking at other people code and understand the logic.";
    format!(
        "INSERT INTO project (name, picture, description, skills, github)
        VALUES (\"GameBoy Emulator\", \"tetris.jpg\", \"{}\", \"Rust, CPU architecture, System Interrupt, Low-level Rendering\", \"https://github.com/eguefif/game_boy_emulator\");",
        description
    )
}
