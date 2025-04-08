use career_manager::website_builder::WebsiteBuilder;

fn main() {
    let config =
        std::fs::read_to_string("config.txt").expect("Error: impossible to read config file");
    let mut cm = WebsiteBuilder::new(config);
    cm.build();
}
