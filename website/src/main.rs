#[macro_use]
extern crate rocket;
use pulldown_cmark::{html, Parser};
use rocket::{fs::FileServer, response::Redirect};
use rocket_dyn_templates::Template;
use serde_json::json;
use std::{fs, path::PathBuf};

fn format_markdown(markdown: &str) -> String {
    let sections = markdown.split("---");
    sections
        .into_iter()
        .map(|section| {
            let parser = Parser::new(section);
            let mut section_html = String::new();
            html::push_html(&mut section_html, parser);
            format!("<section>{}</section>", section_html)
        })
        .collect()
}

fn make_title(path: &str) -> String {
    let words = path.split('/').last().unwrap_or_default().split('-');
    let result = words.collect::<Vec<&str>>().join(" ");
    match result.as_str() {
        "" => "page".to_owned(),
        _ => result.trim().to_owned()
    }
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
    let title = make_title(&file.as_path().display().to_string());
    let context = json!({
        "title": title,
        "content": html,
    });
    Some(Template::render("template", context))
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

#[cfg(test)]
mod test {
    use crate::make_title;

    #[test]
    fn test_make_title() {
        assert_eq!(make_title("/static/abc"), "abc");
        assert_eq!(make_title("/static/"), "page");
        assert_eq!(make_title("/static/abc-123"), "abc 123");
        assert_eq!(make_title("/static/abc-"), "abc");
        assert_eq!(make_title("/static/-123"), "123");
    }
}
