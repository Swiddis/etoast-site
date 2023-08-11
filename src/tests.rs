use super::rocket;
use rocket::http::Status;
use rocket::local::blocking::{Client, LocalResponse};
use walkdir::WalkDir;

fn test_page<'a>(uri: &'a str, client: &'a Client) -> LocalResponse<'a> {
    let response = client.get(uri).dispatch();

    assert_eq!(response.status(), Status::Ok);
    response
}

#[test]
fn test_index() {
    let client = Client::untracked(rocket()).expect("valid rocket instance");
    test_page("/", &client);
}

#[test]
fn test_favicon() {
    let client = Client::untracked(rocket()).expect("valid rocket instance");
    test_page("/favicon.ico", &client);
}

#[test]
fn test_statics() {
    let client = Client::untracked(rocket()).expect("valid rocket instance");

    for entry in WalkDir::new("static")
        .into_iter()
        .filter_entry(|e| !e.path().is_dir())
    {
        let path = entry.unwrap().path().display().to_string();
        println!("{}", path);
        test_page(&path, &client);
    }
}

#[test]
fn test_html_templates() {
    let client = Client::untracked(rocket()).expect("valid rocket instance");

    for entry in WalkDir::new("templates")
        .into_iter()
        .filter_entry(|e| e.path().ends_with(".html"))
    {
        let path = entry.unwrap().path().display().to_string();
        println!("{}", path);
        test_page(path.strip_prefix("/templates").unwrap(), &client);
    }
}
