mod custom_filters;
#[cfg(test)]
mod tests;

use std::path::PathBuf;

use lazy_static::lazy_static;
use rocket::fs::NamedFile;
use rocket::response::content::RawHtml;
use rocket::response::Redirect;
use rocket::Build;
use rocket_dyn_templates::tera::Context;
use rocket_dyn_templates::{tera::Tera, Template};

#[macro_use]
extern crate rocket;

lazy_static! {
    static ref TERA: Tera = {
        let mut tera = match Tera::new("./templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.register_filter("markdown", custom_filters::markdown);
        tera
    };
}

#[get("/<file..>")]
fn serve_page(file: std::path::PathBuf) -> Option<RawHtml<String>> {
    let template_name = format!("{}.html", file.display());
    match TERA.render(&template_name, &Context::new()) {
        Ok(html) => Some(RawHtml(html)),
        Err(e) => {
            println!("Error retrieving '{}': {}", template_name, e);
            None
        }
    }
}

#[get("/static/<file..>")]
async fn serve_static(file: std::path::PathBuf) -> Option<NamedFile> {
    let file_path = format!("./static/{}", file.display());
    NamedFile::open(file_path).await.ok()
}

#[get("/")]
async fn serve_home() -> Option<RawHtml<String>> {
    serve_page(PathBuf::from("index"))
}

#[get("/favicon.ico")]
async fn serve_favicon() -> Option<NamedFile> {
    serve_static(PathBuf::from("/favicon.ico")).await
}

#[catch(404)]
fn catch_404() -> Redirect {
    Redirect::to("/404")
}

fn rocket() -> rocket::Rocket<Build> {
    rocket::build()
        .mount(
            "/",
            routes![serve_home, serve_favicon, serve_static, serve_page],
        )
        .register("/", catchers![catch_404])
        .attach(Template::fairing())
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket().launch().await?;
    Ok(())
}
