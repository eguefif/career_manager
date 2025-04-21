use crate::connector::SqlEngine;
use crate::models::article::Article;
use crate::rendering::{render_template, write_file, ValueType};
use std::error::Error;

use super::Context;

const BUNDLE_DEV_ARTICLE_TEMPLATE: &str = "./html/website/dev/templates/article.html";
const ARTICLES_DIST: &str = "./html/website/dist/articles/";

pub fn build_articles(engine: &mut SqlEngine) -> Result<(), Box<dyn Error>> {
    let articles = Article::all(engine, None);
    let template = std::fs::read_to_string(BUNDLE_DEV_ARTICLE_TEMPLATE)?;
    for article in articles {
        println!("building articles");
        let context = build_context(&article);
        let content = render_template(&template, context)?;
        let filename = make_filename(&article);
        println!("Content in ({}): \n{:?}", filename, content);
        write_file(&content, &filename)?;
    }
    Ok(())
}

fn build_context(article: &Article) -> Context {
    let content = transform_md_to_html(article.content.clone());
    let created_at = article.created_at.clone().unwrap();
    vec![
        ("title".to_string(), ValueType::Text(article.title.clone())),
        ("created_at".to_string(), ValueType::Text(created_at)),
        ("content".to_string(), ValueType::Text(content)),
    ]
}

fn transform_md_to_html(content: String) -> String {
    md_to_html::transform(&content)
}

fn make_filename(article: &Article) -> String {
    format!("{}/{}.html", ARTICLES_DIST, &article.id.unwrap())
}
