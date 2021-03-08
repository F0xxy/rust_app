mod repository;
mod user;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use repository::{MemoryReposiory, Repository};
use std::{
    env::var,
    sync::{
        atomic::{AtomicU16, Ordering},
        Arc,
    },
};
struct RepositoryInjector(Box<dyn Repository>);
impl RepositoryInjector {
    fn new(repo: impl Repository) -> Self {
        Self(Box::new(repo))
    }
    fn new_share(repo: impl Repository) -> Arc<Self> {
        Arc::new(Self::new(repo))
    }
}
use uuid::Uuid;
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
        let thread_index = thread_counter.fetch_add(1, Ordering::SeqCst);
        println!("thread: {}", thread_index);
        let repo = RepositoryInjector::new_share(MemoryReposiory::default());

        App::new()
            .data(thread_index)
            .data(repo.clone())
            .service(web::resource("/user/{id}").route(web::get().to(get_user)))
            .route("/", web::get().to(greet))
            .route(
                "/health",
                web::get().to(|index: web::Data<u16>| {
                    HttpResponse::Ok()
                        .header("thread-id", index.to_string())
                        .finish()
                }),
            )
            .route("/rust", web::get().to(|| async { "hola Rust" }))
            .route("/{name}", web::get().to(greet))
    })
    .bind(&addr)
    .unwrap_or_else(|err| {
        panic!(
            "🔥🔥🔥 coudn't start server in porst: {}, error:{:?}",
            port, err
        )
    })
    .run()
    .await
}

async fn get_user(id: web::Path<Uuid>, repo: web::Data<Arc<RepositoryInjector>>) -> HttpResponse {
    match repo.0.get_user(&id) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}
