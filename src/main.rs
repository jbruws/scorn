use actix_web::*;
use serde::Deserialize;

struct AppState<'a> {
    handle_formatter: handlebars::Handlebars<'a>,
    template_post: String,
    template_index: String,
}

#[derive(Deserialize)]
pub struct PathInfo {
    postname_raw: String,
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
    post_list.push_str("</ul>");
    HttpResponse::Ok().body(
        data.handle_formatter
            .render_template(
                &data.template_index,
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
    let md_contents = std::fs::read_to_string(format!("./markdown/{}", postname))
        .unwrap_or_else(|_| panic!("Can't read {}", &postname));
    HttpResponse::Ok().body(
            data.handle_formatter.render_template(
                &data.template_post,
                &serde_json::json!({"post_title": info.postname_raw, "contents": md_contents, "footer": "<a href=\"..\">Back to index</a>".to_string()})
            ).unwrap()
        )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                handle_formatter: handlebars::Handlebars::new(),
                template_post: include_str!("../post_template.html").to_string(),
                template_index: include_str!("../index_template.html").to_string(),
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
