mod custom_filters;

use std::path::PathBuf;

use lazy_static::lazy_static;
use rocket::fs::NamedFile;
use rocket::response::content::RawHtml;
use rocket::response::Redirect;
use rocket_dyn_templates::tera::Context;
use rocket_dyn_templates::{Template, tera::Tera};

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
        println!("{:?}", tera);
        tera
    };
}

#[get("/<file..>")]
fn serve_page(file: std::path::PathBuf) -> Option<RawHtml<String>> {
    let template_name = format!("{}.html", file.display().to_string());
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

#[catch(404)]
fn catch_404() -> Redirect {
    Redirect::to("/404")
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    println!("{:?}", TERA.get_template_names().collect::<Vec<&str>>());
    rocket::build()
        .mount("/", routes![serve_static, serve_page, serve_home])
        .register("/", catchers![catch_404])
        .attach(Template::fairing())
        .launch()
        .await?;
    Ok(())
}
