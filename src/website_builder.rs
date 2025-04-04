use crate::connector::SqlEngine;
use crate::models::profile::Profile;
use crate::models::project::Project;
use std::fs;
use std::io::Write;
use std::path::Path;

const BUNDLE_DIST: &str = "./html/dist/bundle.js";
const HOME_TL: &str = "./html/templates/home_page.tl.html";
const PROJECTS_TL: &str = "./html/templates/projects.tl.html";
const PROJECT_TL: &str = "./html/templates/project.tl.html";
const SKILL_TL: &str = "./html/templates/skill.tl.html";

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
        copy_website_to_dist("./html/website", "./html/dist");
        copy_website_to_dist("./html/website", "./html/interface");

        add_home_page(&mut engine);
        add_project_page(&mut engine);
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

fn add_home_page(engine: &mut SqlEngine) {
    let profile = Profile::take_first(engine);
    if let Some(profile) = profile {
        let template =
            std::fs::read_to_string(HOME_TL).expect("Impossible to read template homepage");
        let context: Vec<(&str, String)> = vec![
            ("display_name", profile.display_name),
            ("description", profile.description),
            ("picture", format!("./images/{}", profile.picture)),
        ];
        render_template(template, context);
    }
}

fn render_template(mut template: String, context: Vec<(&str, String)>) {
    for (token, value) in context {
        template = template.replace(format!("{{{token}}}").as_str(), &value);
    }
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(BUNDLE_DIST)
        .expect("Cannot open bundle file");
    file.write(template.as_bytes())
        .expect("impossible to write in bundle");
}

fn add_project_page(engine: &mut SqlEngine) {
    let projects_data = Project::all(engine);

    let mut templated_projects = String::new();
    let template =
        std::fs::read_to_string(PROJECTS_TL).expect("Impossible to read template projects");

    let empty_template =
        std::fs::read_to_string(PROJECT_TL).expect("Impossible to read template project");

    for project in projects_data {
        let mut templated_project = String::from(&empty_template);
        let skills = get_skills(project.skills);
        let context: Vec<(&str, String)> = vec![
            ("{title}", project.name),
            ("{description}", project.description),
            ("{picture}", format!("images/{}", project.picture)),
            ("{github}", project.github),
            ("{skills}", skills),
        ];
        for (token, value) in context {
            templated_project = templated_project.replace(token, &value);
        }
        templated_projects.push_str(&templated_project);
    }
    let context: Vec<(&str, String)> = vec![("projects", templated_projects)];
    render_template(template, context);
}

fn get_skills(skills: Vec<String>) -> String {
    let template = std::fs::read_to_string(SKILL_TL).unwrap();
    let mut retval = String::new();
    for skill in skills {
        let templated_skill = template.replace("{SKILL}", &skill);
        retval.push_str(templated_skill.trim());
    }
    retval
}
