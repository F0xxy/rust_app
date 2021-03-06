mod user;
use std::{
    env::var,
    sync::{
        atomic::{AtomicU16, Ordering},
        Arc,
    },
};

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("Mundo");
    format!("Hola {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = var("PORT").unwrap_or("8080".to_string());
    let addr = format!("127.0.0.1:{}", port);
    let thread_counter = Arc::new(AtomicU16::new(1));
    HttpServer::new(move || {
        thread_counter.fetch_add(1, Ordering::SeqCst);
        let thread_index = thread_counter.load(Ordering::SeqCst);
        App::new()
            .route("/", web::get().to(greet))
            .route(
                "/health",
                web::get().to(move || {
                    HttpResponse::Ok()
                        .header("thread-id", thread_index.to_string())
                        .finish()
                }),
            )
            .route("/rust", web::get().to(|| async { "hola Rust" }))
            .route("/{name}", web::get().to(greet))
    })
    .bind(&addr)?
    .run()
    .await
}
