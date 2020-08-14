use crate::pubsub::{PubSub, Subscribe};
use actix::prelude::*;
use blockchain::{Block, Blockchain};
use std::io;

pub struct Node {
    pub blockchain: Blockchain,
    pub pubsub: Addr<PubSub>,
}

impl Actor for Node {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        self.pubsub.do_send(Subscribe("main"));
    }
}

impl Node {
    fn new(blockchain: Blockchain, pubsub: Addr<PubSub>) -> Self {
        Node { blockchain, pubsub }
    }

    pub fn from_pubsub(pubsub: Addr<PubSub>) -> Self {
        Node::new(Blockchain::new(), pubsub)
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
