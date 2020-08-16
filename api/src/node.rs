use {
    actix::prelude::*,
    blockchain::{Block, Blockchain},
    redis_async::{error, resp, resp::FromResp},
    std::io,
};

pub struct Node {
    pub blockchain: Blockchain,
}

impl Actor for Node {
    type Context = Context<Self>;
}

impl Node {
    pub fn new(blockchain: Blockchain) -> Self {
        Node { blockchain }
    }
}

#[derive(Message)]
#[rtype(result = "Result<Vec<Block>, io::Error>")]
pub struct GetBlocks;

#[derive(Message)]
#[rtype(result = "Result<Block, io::Error>")]
pub struct MineBlock(pub Vec<u8>);

impl Handler<GetBlocks> for Node {
    type Result = Result<Vec<Block>, io::Error>;

    fn handle(&mut self, _msg: GetBlocks, _ctx: &mut Context<Self>) -> Self::Result {
        Ok(self.blockchain.chain.clone())
    }
}

impl Handler<MineBlock> for Node {
    type Result = Result<Block, io::Error>;

    fn handle(&mut self, msg: MineBlock, _ctx: &mut Context<Self>) -> Self::Result {
        // we should parse data
        // then add block
        // then broadcast chain
        // then return added block
        self.blockchain.add_block(&msg.0);
        Ok(self.blockchain.chain.last().unwrap().clone())
    }
}

impl StreamHandler<Result<resp::RespValue, error::Error>> for Node {
    fn handle(&mut self, msg: Result<resp::RespValue, error::Error>, _ctx: &mut Context<Self>) {
        println!("Stream handler is running!");
        if let Ok(message) = msg {
            println!("Got message: {}", String::from_resp(message).unwrap())
        }
    }
}
