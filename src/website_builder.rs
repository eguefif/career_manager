use crate::connector::SqlEngine;
use crate::models::profile::Profile;

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
        let profile = Profile::take_first(&mut engine);
        println!("name: {}", profile.display_name);
        println!("description: {}", profile.description);
        println!("picture: {}", profile.picture);
    }
}
