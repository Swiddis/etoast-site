mod custom_filters;

use axum::{
    http::{StatusCode, Uri},
    response::Html,
    Router,
};
use lazy_static::lazy_static;
use tera::{Context, ErrorKind, Tera};
use tower_http::services::{ServeDir, ServeFile};

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

fn uri_to_template_name(uri: Uri) -> String {
    let path = uri.path();
    match path {
        "" | "/" => "index.html".to_owned(),
        _ => format!("{}.html", path.strip_prefix("/").unwrap_or(path)).to_owned()
    }
}

async fn serve_page(uri: Uri) -> (StatusCode, Html<String>) {
    let template_name = uri_to_template_name(uri);
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
                    }),
                }
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, {
                println!("500: {}", err);
                Html("<h1>Error 500</h1><p>Tell Toast</p>".to_owned())
            }),
        },
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route_service("/favicon.ico", ServeFile::new("static/favicon.ico"))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(serve_page);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
