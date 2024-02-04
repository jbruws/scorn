use actix_web::*;
use indexmap::IndexMap;
use serde::Deserialize;

mod routes;

pub struct AppState<'a> {
    handle_formatter: handlebars::Handlebars<'a>,
}

#[derive(Deserialize)]
pub struct PathInfo {
    postname_raw: String,
}

pub fn load_file(filepath: &str) -> String {
    std::fs::read_to_string(format!("./{}", filepath))
        .unwrap_or_else(|_| panic!("Can't read {}", &filepath))
}

pub fn md_to_html(raw_text: String) -> String {
    let rules: IndexMap<String, String> =
        serde_yaml::from_str(&load_file("./formatting.yaml")).unwrap();
    let mut result = raw_text.clone();
    for (template, expr) in rules.iter() {
        result = regex::Regex::new(expr)
            .unwrap()
            .replace_all(&result, template)
            .to_string();
    }
    result
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                handle_formatter: handlebars::Handlebars::new(),
            }))
            .service(actix_files::Files::new("/web_data", "./web_data"))
            .service(routes::blogpost)
            .service(routes::index)
    })
    .bind(("0.0.0.0", 7117))
    .expect("Something went wrong.")
    .run()
    .await
}
