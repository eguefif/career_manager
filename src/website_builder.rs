use crate::connector::SqlEngine;
use crate::models::profile::Profile;
use std::io::Write;

const BUNDLE_FILE_PATH: &str = "./html/website/bundle.js";
const HOME_TL: &str = "./html/templates/home_page.tl.html";

#[allow(dead_code)]
pub struct WebsiteBuilder {
    dest: String,
}

impl WebsiteBuilder {
    pub fn new(dest: String) -> Self {
        Self { dest }
    }

    pub fn build(&mut self) {
        let mut engine = SqlEngine::new("./cm.db");

        add_home_page(&mut engine);
    }
}

fn add_home_page(engine: &mut SqlEngine) {
    let profile = Profile::take_first(engine);
    if let Some(profile) = profile {
        let template =
            std::fs::read_to_string(HOME_TL).expect("Impossible to read template homepage");
        let context: Vec<(&str, String)> = vec![
            ("display_name", profile.display_name),
            ("description", profile.description),
            ("picture", profile.picture),
        ];
        render_template(template, context);
    }
}

fn render_template(mut template: String, context: Vec<(&str, String)>) {
    for (token, value) in context {
        template = template.replace(token, &value);
    }
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(BUNDLE_FILE_PATH)
        .expect("Cannot open bundle file");
    file.write(template.as_bytes())
        .expect("impossible to write in bundle");
}
