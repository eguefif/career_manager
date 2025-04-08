use crate::connector::SqlEngine;
use crate::models::profile::Profile;
use crate::rendering::{render, ValueType};
use std::fs;
use std::path::Path;

pub type Context = Vec<(String, ValueType)>;

/// A builder that build my personal website
#[allow(dead_code)]
pub struct WebsiteBuilder {
    dest: String,
}

impl WebsiteBuilder {
    /// Create a WebsiteBuilder
    /// It takes a destinations foler
    ///
    /// Example:
    /// ```rust
    /// let mut cm = WebsiteBuilder::new(MY_DEST_PATH);
    /// cm.build();
    /// ```
    pub fn new(dest: String) -> Self {
        Self { dest }
    }

    pub fn build(&mut self) {
        let mut engine = SqlEngine::new("./cm.db");
        copy_website_to_dist("./html/website", "./html/dist");
        let mut context: Context = Vec::new();

        if let Some(home_context) = add_home_page(&mut engine) {
            context.extend_from_slice(&home_context);
        }
        //if let Some(project_context) = add_project_page(&mut engine) {
        //    context.extend_from_slice(&project_context);
        //}
        render("./bundle.js", context);
    }
}

fn copy_website_to_dist(src: &str, dest: &str) {
    let src_path = Path::new(src);
    let dest_path = Path::new(dest);
    if !dest_path.exists() {
        fs::create_dir(dest_path).expect("Error: impossible to create html/dist folder");
    }
    if src_path.is_dir() {
        copy_dir(src_path, src, dest);
    }
}

fn copy_dir(src: &Path, base: &str, dst: &str) {
    let dir_list = fs::read_dir(src).unwrap();
    for file in dir_list {
        if let Ok(file) = file {
            let file = file.path();
            if file.is_dir() {
                if let Some(new_file) = file.to_str() {
                    let new_path = new_file.replace(base, dst);
                    let _ = fs::create_dir(new_path);
                }
                copy_dir(&file, base, dst);
            } else {
                if let Some(new_file) = file.to_str() {
                    let new_path = new_file.replace(base, dst);
                    let _ = fs::copy(file, new_path.clone());
                }
            }
        }
    }
}

fn add_home_page(engine: &mut SqlEngine) -> Option<Context> {
    let profile = Profile::take_first(engine);
    if let Some(profile) = profile {
        let context = vec![
            (
                "display_name".to_string(),
                ValueType::Text(profile.display_name),
            ),
            (
                "description".to_string(),
                ValueType::Text(profile.description),
            ),
            (
                "picture".to_string(),
                ValueType::Text(format!("./images/{}", profile.picture)),
            ),
        ];
        return Some(context);
    }
    None
}

//fn add_project_page(engine: &mut SqlEngine) {
//    let projects_data = Project::all(engine);
//
//    let mut context: Context = vec![];
//    for project in projects_data {
//        let skills = get_skills(project.skills);
//        let project: Vec<(String, String)> = vec![
//            ("{title}", project.name),
//            ("{description}", project.description),
//            ("{picture}", format!("images/{}", project.picture)),
//            ("{github}", project.github),
//            ("{skills}", skills),
//        ];
//    }
//}
//
//fn get_skills(skills: Vec<String>) -> String {
//    let template = std::fs::read_to_string(SKILL_TL).unwrap();
//    let mut retval = String::new();
//    for skill in skills {
//        let templated_skill = template.replace("{SKILL}", &skill);
//        retval.push_str(templated_skill.trim());
//    }
//    retval
//}
