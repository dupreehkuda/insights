use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use askama::Template;
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    message: String,
}

#[get("/json-endpoint")]
async fn json_endpoint() -> impl Responder {
    let message = Message {
        message: "Hello, JSON!".to_string(),
    };

    HttpResponse::Ok().json(message)
}

#[derive(Template)]
#[template(path = "new_insight.html")]
struct NewInsightTemplate {
    message: String,
}

#[get("/html-page")]
async fn html_page() -> impl Responder {
    let template = NewInsightTemplate {
        message: "Hello, HTML with Templating!".to_string(),
    };

    let body = template.render().unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(json_endpoint)
            .service(html_page)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}