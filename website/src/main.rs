#[macro_use] extern crate rocket;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use pulldown_cmark::{html, Parser};
use std::{fs, path::PathBuf};
use serde_json::json;

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/home"))
}

#[get("/<file..>")]
fn markdown(file: PathBuf) -> Option<Template> {
    let path = format!("static/{}.md", file.as_path().display());
    let markdown = fs::read_to_string(path).ok()?;
    let parser = Parser::new(&markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    let context = json!({
        "title": file.as_path().display().to_string(),
        "content": html_output,
    });
    Some(Template::render("template", &context))
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index, markdown])
        .attach(Template::fairing())
        .launch()
        .await
        .unwrap();
}
