use actix::prelude::*;
use std::io;

const DEFAULT_CHANNELS: [&'static str; 2] = ["main", "test"];

pub struct PubSub {
    pub client: redis::Client,
    pub channels: Vec<&'static str>,
}

impl Actor for PubSub {
    type Context = Context<Self>;
}

impl PubSub {
    fn new(client: redis::Client, channels: Vec<&'static str>) -> Self {
        PubSub { client, channels }
    }

    pub fn from_addr(addr: &str) -> Self {
        PubSub::new(
            redis::Client::open(format!("redis://{}", addr)).unwrap(),
            DEFAULT_CHANNELS.to_vec(),
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), io::Error>")]
pub struct Subscribe(pub &'static str);

impl Handler<Subscribe> for PubSub {
    type Result = Result<(), io::Error>;

    fn handle(&mut self, msg: Subscribe, _ctx: &mut Context<Self>) -> Self::Result {
        let mut con = self.client.get_connection().unwrap();
        let mut sub = con.as_pubsub();
        sub.subscribe(msg.0).unwrap();

        loop {
            let sub_msg = sub.get_message().unwrap();
            let payload: String = sub_msg.get_payload().unwrap();
            println!("channel '{}': {}", sub_msg.get_channel_name(), payload);
        }
    }
}
