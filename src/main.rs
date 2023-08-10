mod custom_filters;

use axum::{extract, response::{Html, IntoResponse, Redirect}, routing::get, Router, http::StatusCode};
use lazy_static::lazy_static;
use tera::{Context, ErrorKind, Tera};
use tower_http::services::ServeDir;

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

async fn serve_page(extract::Path(path): extract::Path<String>) -> (StatusCode, Html<String>) {
    let template_name = format!("{}.html", path);
    match TERA.render(&template_name, &Context::new()) {
        Ok(html) => (StatusCode::OK, Html(html)),
        Err(err) => match err.kind {
            ErrorKind::TemplateNotFound(_) => {
                let not_found = TERA.render("404.html", &Context::new());
                match not_found {
                    Ok(content) => (StatusCode::NOT_FOUND, Html(content)),
                    Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, {
                        println!("500: {}", err);
                        Html("<h1>Error 500</h1><p>Tell Toast</p>".to_owned())
                    })
                }
            },
            _ => (StatusCode::INTERNAL_SERVER_ERROR, {
                println!("500: {}", err);
                Html("<h1>Error 500</h1><p>Tell Toast</p>".to_owned())
            })
        },
    }
}

async fn serve_home() -> impl IntoResponse {
    serve_page(extract::Path("index".to_string())).await
}

async fn serve_favicon() -> impl IntoResponse {
    Redirect::to("/static/favicon.ico")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(serve_home))
        .route("/favicon.ico", get(serve_favicon))
        .nest_service("/static", ServeDir::new("static"))
        .route("/*path", get(serve_page));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
