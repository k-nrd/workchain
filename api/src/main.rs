mod node;

use {
    actix::prelude::*,
    actix_web::middleware::{Compress, Logger},
    actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder},
    blockchain::Blockchain,
    dotenv,
    node::{GetBlocks, MineBlock, Node},
    pretty_env_logger,
    serde::Deserialize,
    std::io,
};

#[derive(Deserialize, Clone, Debug)]
struct ReqData {
    data: String,
}

#[get("/api/blocks")]
async fn get_blocks(node_addr: web::Data<Addr<Node>>) -> impl Responder {
    match node_addr.as_ref().send(GetBlocks).await {
        Ok(res) => HttpResponse::Ok().json(res.unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/api/mine")]
async fn mine_block(node_addr: web::Data<Addr<Node>>, body: web::Json<ReqData>) -> impl Responder {
    match node_addr
        .as_ref()
        .send(MineBlock(body.data.as_bytes().to_vec()))
        .await
    {
        Ok(res) => HttpResponse::Ok().json(res.unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    // Create node actor.
    let node_addr = Node::new(Blockchain::new()).start();

    HttpServer::new(move || {
        App::new()
            .data(node_addr.clone())
            .wrap(Compress::default())
            .wrap(Logger::default())
            .service(get_blocks)
            .service(mine_block)
    })
    .bind(format!(
        "0.0.0.0:{}",
        dotenv::var("API_PORT").unwrap_or(3000.to_string())
    ))?
    .run()
    .await
}
