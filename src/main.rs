
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;

use log::{
//    debug,
//    error,
    info,
//    trace,
//    warn,
};
use log4rs;

#[get("/")]
async fn hello() -> impl Responder {
    info!("Hello!!");
    info!(module_path!());
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");

    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default().log_target("requests")) // output access log
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
