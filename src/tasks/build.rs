use career_manager::website_builder::WebsiteBuilder;

fn main() {
    let config =
        std::fs::read_to_string("config.txt").expect("Error: impossible to read config file");
    let mut cm = WebsiteBuilder::new(config);
    if let Err(e) = cm.build() {
        eprintln!("Error: building failed: {e}");
    }
}
