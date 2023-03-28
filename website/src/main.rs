#[macro_use] extern crate rocket;
use rocket::{response::Redirect, fs::FileServer};
use rocket_dyn_templates::Template;
use pulldown_cmark::{html, Parser};
use std::{fs, path::PathBuf};
use serde_json::json;

fn format_markdown(markdown: &str) -> String {
    let sections = markdown.split("---");
    sections.into_iter().map(|section| {
        let parser = Parser::new(&section);
        let mut section_html = String::new();
        html::push_html(&mut section_html, parser);
        format!("<section>{}</section>", section_html)
    }).collect()
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/home"))
}

#[get("/favicon.ico")]
fn favicon() -> Redirect {
    Redirect::to(uri!("/assets/favicon.ico"))
}

#[get("/<file..>")]
fn markdown(file: PathBuf) -> Option<Template> {
    let path = format!("static/{}.md", file.as_path().display());
    let markdown = fs::read_to_string(path).ok()?;
    let html = format_markdown(&markdown);
    let context = json!({
        "title": file.as_path().display().to_string(),
        "content": html,
    });
    Some(Template::render("template", &context))
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/assets", FileServer::from("static/assets").rank(-10))
        .mount("/", routes![index, favicon, markdown])
        .attach(Template::fairing())
        .launch()
        .await
        .unwrap();
}
