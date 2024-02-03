use actix_web::*;
use indexmap::IndexMap;
use serde::Deserialize;

struct AppState<'a> {
    handle_formatter: handlebars::Handlebars<'a>,
}

#[derive(Deserialize)]
pub struct PathInfo {
    postname_raw: String,
}

fn load_file(filepath: &str) -> String {
    std::fs::read_to_string(format!("./{}", filepath))
        .unwrap_or_else(|_| panic!("Can't read {}", &filepath))
}

fn md_to_html(raw_text: String) -> String {
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

#[get("/")]
async fn index(data: web::Data<AppState<'_>>) -> impl Responder {
    let md_posts_raw: Vec<_> = std::fs::read_dir("./markdown").unwrap().collect();

    let mut post_list = "<ul>".to_string();
    for i in md_posts_raw {
        if let Ok(v) = i {
            let filename = v.path().file_name().unwrap().to_str().unwrap().to_string();
            post_list.push_str(&format!(
                "\t<li><a href=\"/{f}\">{f}</a></li>\n",
                f = filename[..filename.len() - 3].to_string()
            ));
        }
    }
    let template_index = load_file("./templates/index_template.html");
    post_list.push_str("</ul>");
    HttpResponse::Ok().body(
        data.handle_formatter
            .render_template(
                &template_index,
                &serde_json::json!({"post_list": post_list}),
            )
            .unwrap(),
    )
}

#[get("/{postname_raw}")]
async fn blogpost(data: web::Data<AppState<'_>>, info: web::Path<PathInfo>) -> impl Responder {
    let md_posts_raw: Vec<_> = std::fs::read_dir("./markdown").unwrap().collect();

    let mut md_posts: Vec<String> = Vec::new();
    for i in md_posts_raw {
        if let Ok(v) = i {
            md_posts.push(v.path().to_str().unwrap().to_string());
        }
    }

    let postname = format!("{}.md", info.postname_raw);
    if !md_posts.contains(&format!("./markdown/{}", postname)) {
        return HttpResponse::Ok().body("Not Found".to_string());
    }
    let md_contents = load_file(format!("./markdown/{}", postname).as_str());
    let template_post = load_file("./templates/post_template.html");
    HttpResponse::Ok().body(
            data.handle_formatter.render_template(
                &template_post,
                &serde_json::json!({"post_title": info.postname_raw, "contents": md_to_html(md_contents.to_string()), "footer": "<a href=\"..\">Back to index</a>".to_string()})
            ).unwrap()
        )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                handle_formatter: handlebars::Handlebars::new(),
            }))
            .service(actix_files::Files::new("/web_data", "./web_data"))
            .service(blogpost)
            .service(index)
    })
    .bind(("0.0.0.0", 7117))
    .expect("Something went wrong.")
    .run()
    .await
}
