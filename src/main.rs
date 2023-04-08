#[macro_use]
extern crate rocket;
use std::io;
use ::markdown::to_html;
use rocket::{fs::FileServer, response::Redirect, Build, Rocket};
use rocket_dyn_templates::Template;
use serde_json::json;
use std::{fs, path::PathBuf};

fn format_markdown(markdown: &str) -> String {
    to_html(markdown)
}

fn make_title(path: &str) -> String {
    let words = path.split('/').last().unwrap_or_default().split('-');
    let result = words.collect::<Vec<&str>>().join(" ");
    match result.as_str() {
        "" => "page".to_owned(),
        _ => result.trim().to_owned(),
    }
}

fn markdown_context(path: &str) -> Result<serde_json::Value, io::Error> {
    let markdown = fs::read_to_string(path)?;
    let html = format_markdown(&markdown);
    let title = make_title(path);
    Ok(json!({
        "title": title,
        "content": html,
    }))
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
    let context = markdown_context(&path).ok()?;
    Some(Template::render("template", context))
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/assets", FileServer::from("static/assets").rank(-10))
        .mount("/", routes![index, favicon, markdown])
        .attach(Template::fairing())
}

#[cfg(test)]
mod test {
    use super::rocket;
    use crate::make_title;
    use rocket::{http::Status, local::blocking::Client};

    #[test]
    fn test_index_redirect() {
        let client = Client::tracked(rocket()).unwrap();
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::SeeOther);
        assert_eq!(response.headers().get_one("Location"), Some("/home"));
    }

    #[test]
    fn test_favicon_redirect() {
        let client = Client::tracked(rocket()).unwrap();
        let response = client.get("/favicon.ico").dispatch();
        assert_eq!(response.status(), Status::SeeOther);
        assert_eq!(
            response.headers().get_one("Location"),
            Some("/assets/favicon.ico")
        );
    }

    #[test]
    fn test_markdown_template() {
        let client = Client::tracked(rocket()).unwrap();
        let response = client.get("/about").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response
            .into_string()
            .unwrap()
            .contains("<h1>About Me</h1>"));
    }

    #[test]
    fn test_invalid_markdown_path() {
        let client = Client::tracked(rocket()).unwrap();
        let response = client.get("/invalid/path").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_make_title() {
        assert_eq!(make_title("/static/abc"), "abc");
        assert_eq!(make_title("/static/"), "page");
        assert_eq!(make_title("/static/abc-123"), "abc 123");
        assert_eq!(make_title("/static/abc-"), "abc");
        assert_eq!(make_title("/static/-123"), "123");
    }
}
