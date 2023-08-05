#![feature(proc_macro_hygiene, decl_macro)]

use pulldown_cmark::{html, Parser};
use rocket::fs::NamedFile;
use rocket::response::content::RawHtml;

#[macro_use]
extern crate rocket;

// Function to convert Markdown to HTML
fn markdown_to_html(input: &str) -> String {
    let parser = Parser::new(input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

// Rocket route to serve Markdown files as HTML
#[get("/<file..>")]
fn serve_markdown(file: std::path::PathBuf) -> Option<RawHtml<String>> {
    let file_path = format!("./content/{}.md", file.display());
    if let Ok(content) = std::fs::read_to_string(file_path) {
        let html_content = markdown_to_html(&content);
        Some(RawHtml(html_content))
    } else {
        None
    }
}

// Rocket route to serve static files (e.g., CSS, images, etc.)
#[get("/static/<file..>")]
async fn serve_static(file: std::path::PathBuf) -> Option<NamedFile> {
    let file_path = format!("./static/{}", file.display());
    NamedFile::open(file_path).await.ok()
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/", routes![serve_markdown, serve_static])
        .launch()
        .await?;
    Ok(())
}
