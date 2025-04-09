use crate::connector::SqlEngine;
use crate::models::profile::Profile;
use crate::models::project::Project;
use crate::rendering::{render, ValueType};
use std::collections::HashMap;
use std::error::Error;
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

    pub fn build(&mut self) -> Result<(), Box<dyn Error>> {
        let mut engine = SqlEngine::new("./cm.db");
        copy_website_to_dist("./html/website", "./html/dist")?;
        let mut context: Context = Vec::new();

        if let Some(home_context) = add_home_page(&mut engine) {
            context.extend_from_slice(&home_context);
        }
        if let Some(project_context) = add_project_page(&mut engine) {
            context.extend_from_slice(&project_context);
        }
        render(context)?;
        Ok(())
    }
}

fn copy_website_to_dist(src: &str, dest: &str) -> Result<(), Box<dyn Error>> {
    let src_path = Path::new(src);
    let dest_path = Path::new(dest);
    if !dest_path.exists() {
        fs::create_dir(dest_path)?;
    }
    if src_path.is_dir() {
        copy_dir(src_path, src, dest)?;
    }
    Ok(())
}

fn copy_dir(src: &Path, base: &str, dst: &str) -> Result<(), Box<dyn Error>> {
    let dir_list = fs::read_dir(src)?;
    for file in dir_list {
        if let Ok(file) = file {
            let file = file.path();
            if file.is_dir() {
                if let Some(new_file) = file.to_str() {
                    let new_path = new_file.replace(base, dst);
                    let _ = fs::create_dir(new_path);
                }
                copy_dir(&file, base, dst)?;
            } else {
                if let Some(new_file) = file.to_str() {
                    let new_path = new_file.replace(base, dst);
                    let _ = fs::copy(file, new_path.clone());
                }
            }
        }
    }
    Ok(())
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

fn add_project_page(engine: &mut SqlEngine) -> Option<Context> {
    let projects_data = Project::all(engine);

    let mut context: Context = vec![];
    let mut projects: Vec<ValueType> = Vec::new();
    for project in projects_data {
        let mut item: HashMap<String, ValueType> = HashMap::new();
        item.insert("{title}".to_string(), ValueType::Text(project.name));
        item.insert(
            "{description}".to_string(),
            ValueType::Text(project.description),
        );
        item.insert(
            "{picture}".to_string(),
            ValueType::Text(format!("images/{}", project.picture)),
        );
        item.insert("{github}".to_string(), ValueType::Text(project.github));
        item.insert("{skills}".to_string(), get_skills(project.skills));
        projects.push(ValueType::Hash(Box::new(item)));
    }
    context.push(("projects".to_string(), ValueType::List(projects)));
    Some(context)
}

fn get_skills(skills: Vec<String>) -> ValueType {
    let mut wrapped_skills = Vec::new();
    for skill in skills {
        wrapped_skills.push(ValueType::Text(skill))
    }
    ValueType::List(wrapped_skills)
}
