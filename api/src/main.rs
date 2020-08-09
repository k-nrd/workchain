use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use blockchain::Blockchain;
use pretty_env_logger;
use std::env;
use std::io;

#[get("/{name}")]
async fn index(req: HttpRequest, name: web::Path<String>) -> String {
    println!("REQ: {:?}", req);
    format!("Hello: {}!\r\n", name)
}

#[get("/api/blocks")]
async fn get_blocks(_req: HttpRequest, data: web::Data<Blockchain>) -> impl Responder {
    HttpResponse::Ok().json(data.as_ref().chain.clone())
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    pretty_env_logger::init();

    // we need to use actix to handle chain operations asynchronously
    let bc = Blockchain::new();

    HttpServer::new(move || {
        App::new()
            .data(bc.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(index)
            .service(get_blocks)
    })
    .bind("127.0.0.1:8080")?
    .workers(1)
    .run()
    .await
}
