#![feature(proc_macro_hygiene, decl_macro)]

use pulldown_cmark::{html, Parser};
use rocket::fs::NamedFile;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use rocket_dyn_templates::serde::json::json;

#[macro_use]
extern crate rocket;

fn markdown_to_html(input: &str) -> String {
    let parser = Parser::new(input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[get("/<file..>")]
fn serve_markdown(file: std::path::PathBuf) -> Option<Template> {
    let file_path = format!("./content/{}.md", file.display());
    if let Ok(content) = std::fs::read_to_string(file_path) {
        let html_content = markdown_to_html(&content);
        Some(Template::render("main", json!({
            "title": file.display().to_string(),
            "content": html_content
        })))
    } else {
        None
    }
}

#[get("/static/<file..>")]
async fn serve_static(file: std::path::PathBuf) -> Option<NamedFile> {
    let file_path = format!("./static/{}", file.display());
    NamedFile::open(file_path).await.ok()
}

#[get("/")]
async fn serve_home() -> Redirect {
    Redirect::to("/home")
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/", routes![serve_markdown, serve_static, serve_home])
        .attach(Template::fairing())
        .launch()
        .await?;
    Ok(())
}
