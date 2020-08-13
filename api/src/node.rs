use actix::prelude::*;
use blockchain::{Block, Blockchain};
use std::collections::HashSet;
use std::io;

const DEFAULT_CHANNELS: [&'static str; 2] = ["main", "test"];

pub struct Node {
    pub blockchain: Blockchain,
    pub redis: redis::Client,
    pub channels: HashSet<&'static str>,
}

impl Actor for Node {
    type Context = Context<Self>;
}

impl Node {
    fn new(blockchain: Blockchain, redis: redis::Client, channels: HashSet<&'static str>) -> Self {
        Node {
            blockchain,
            redis,
            channels,
        }
    }

    pub fn from_client(redis: redis::Client) -> Self {
        Node::new(
            Blockchain::new(),
            redis,
            DEFAULT_CHANNELS.to_vec().into_iter().collect(),
        )
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
