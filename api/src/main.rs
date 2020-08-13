mod actor;

use actix::prelude::{Actor, Addr};
use actix_web::middleware::{Compress, Logger};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actor::{GetBlocks, MineBlock, Node};
use dotenv;
use pretty_env_logger;
use std::io;
use tracing::trace;

#[get("/api/blocks")]
async fn get_blocks(node_addr: web::Data<Addr<Node>>) -> impl Responder {
    match node_addr.as_ref().send(GetBlocks).await {
        Ok(res) => HttpResponse::Ok().json(res.unwrap()),
        Err(err) => HttpResponse::from_error(err.into()),
    }
}

#[post("/api/mine")]
async fn mine_block(node_addr: web::Data<Addr<Node>>, body: web::Json<Vec<u8>>) -> impl Responder {
    trace!("{:#?}", body.clone());
    match node_addr.as_ref().send(MineBlock(body.clone())).await {
        Ok(res) => HttpResponse::Ok().json(res.unwrap()),
        Err(err) => HttpResponse::from_error(err.into()),
    }
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    // we need to use actix to handle chain operations asynchronously
    let addr = Node::default().start();

    HttpServer::new(move || {
        App::new()
            .data(addr.clone())
            .wrap(Compress::default())
            .wrap(Logger::default())
            .service(get_blocks)
            .service(mine_block)
    })
    .bind(format!(
        "127.0.0.1:{}",
        dotenv::var("DEFAULT_PORT").unwrap_or(3000.to_string())
    ))?
    .workers(1)
    .run()
    .await
}
