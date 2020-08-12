use actix::prelude::*;
use blockchain::{Block, Blockchain};

pub struct Node(pub Blockchain);

impl Actor for Node {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<Vec<Block>, std::io::Error>")]
pub struct GetBlocks;

#[derive(Message)]
#[rtype(result = "Result<Block, std::io::Error>")]
pub struct MineBlock(pub Vec<u8>);

impl Handler<GetBlocks> for Node {
    type Result = Result<Vec<Block>, std::io::Error>;

    fn handle(&mut self, _msg: GetBlocks, _ctx: &mut Context<Self>) -> Self::Result {
        Ok(self.0.chain.clone())
    }
}

impl Handler<MineBlock> for Node {
    type Result = Result<Block, std::io::Error>;

    fn handle(&mut self, msg: MineBlock, _ctx: &mut Context<Self>) -> Self::Result {
        // we should parse the Vec<u8> encoded data here, if needed
        self.0.add_block(&msg.0);
        Ok(self.0.chain.last().unwrap().clone())
    }
}
