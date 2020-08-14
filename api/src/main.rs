mod node;
mod pubsub;

use actix::prelude::{Actor, Addr};
use actix_web::middleware::{Compress, Logger};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv;
use node::{GetBlocks, MineBlock, Node};
use pretty_env_logger;
use pubsub::PubSub;
use serde::Deserialize;
use std::io;
use tracing::trace;

#[derive(Deserialize, Clone, Debug)]
struct ReqData {
    data: String,
}

#[get("/api/blocks")]
async fn get_blocks(node_addr: web::Data<Addr<Node>>) -> impl Responder {
    match node_addr.as_ref().send(GetBlocks).await {
        Ok(res) => HttpResponse::Ok().json(res.unwrap()),
        Err(err) => HttpResponse::from_error(err.into()),
    }
}

#[post("/api/mine")]
async fn mine_block(node_addr: web::Data<Addr<Node>>, body: web::Json<ReqData>) -> impl Responder {
    trace!("{:#?}", body.clone());
    match node_addr
        .as_ref()
        .send(MineBlock(body.data.as_bytes().to_vec()))
        .await
    {
        Ok(res) => HttpResponse::Ok().json(res.unwrap()),
        Err(err) => HttpResponse::from_error(err.into()),
    }
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    // Create an actor to handle PubSub with Redis, get its address.
    let pubsub = PubSub::from_addr("localhost:6379").start();

    // Create our Node Actor and get its address.
    let node = Node::from_pubsub(pubsub).start();

    HttpServer::new(move || {
        App::new()
            .data(node.clone())
            .wrap(Compress::default())
            .wrap(Logger::default())
            .service(get_blocks)
            .service(mine_block)
    })
    .bind(format!(
        "0.0.0.0:{}",
        dotenv::var("DEFAULT_PORT").unwrap_or(3000.to_string())
    ))?
    .workers(1)
    .run()
    .await
}
