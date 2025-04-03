use crate::connector::SqlEngine;
use crate::models::profile::Profile;
use crate::models::project::Project;
use std::io::Write;

const BUNDLE_FILE_PATH: &str = "./html/website/bundle.js";
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
        let _ = std::fs::copy(BUNDLE_FILE_PATH, BUNDLE_DIST);

        add_home_page(&mut engine);
        add_project_page(&mut engine);
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
