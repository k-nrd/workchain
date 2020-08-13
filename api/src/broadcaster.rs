use actix::prelude::*;
use std::collections::{HashMap, HashSet};

pub struct Publisher {
    channels: HashMap<String, HashSet<usize>>;
}

impl Actor for Publisher {
    type Context = Context<Self>;
}

impl Default for Publisher {
    fn default() -> Self {
        let mut channels = HashMap::new();
        channels.insert("Main".to_owned(), HashSet::new());
        channels.insert("Test".to_owned(), HashSet::new());
        Publisher {
            channels 
        }
    }
}

