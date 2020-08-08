use crate::block::Block;
use crate::config::MINE_RATE;
use chrono::{DateTime, Duration, Utc};

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            chain: vec![Block::genesis()],
        }
    }

    pub fn add_block(&mut self, data: &Vec<u8>) {
        self.chain
            .push(Block::mine(self.chain.last().unwrap(), data));
    }

    fn is_valid_genesis(block: &Block) -> bool {
        block.hash == Block::genesis().hash
    }

    fn is_valid_hash_chain(chain: &Vec<Block>) -> bool {
        // should be false when:
        // 1. there are blocks with invalid pointers to previous hash
        // 2. there are blocks with invalid hashes
        // 3. there are blocks with difficulty jumps
        for (i, block) in chain.iter().enumerate().skip(1) {
            if &block.prev != &chain[i - 1].hash
                || !block.valid_hash()
                || (chain[i - 1].diff as isize - block.diff as isize).abs() > 1
            {
                return false;
            }
        }
        true
    }

    pub fn is_valid(chain: &Vec<Block>) -> bool {
        Blockchain::is_valid_genesis(chain.first().unwrap())
            && Blockchain::is_valid_hash_chain(chain)
    }

    pub fn replace(&mut self, chain: &Vec<Block>) {
        if self.chain.len() > chain.len() || !Blockchain::is_valid(chain) {
            return;
        } else {
            self.chain = chain.to_vec();
        }
    }
}

#[cfg(test)]
mod blockchain_tests {
    use super::*;

    #[test]
    fn blockchain_starts_with_genesis() {
        // created chain should have genesis
        let bc = Blockchain::new();
        // genesis hashes should be equal
        assert_eq!(bc.chain[0].hash, Block::genesis().hash)
    }

    #[test]
    fn blockchain_adds_block() {
        // create chain
        let mut bc = Blockchain::new();

        // create and add 2nd block
        let data = vec![0, 1, 2];
        bc.add_block(&data);

        // data of last block should be equal to data added
        assert_eq!(bc.chain.last().unwrap().data, data);
    }

    #[test]
    fn blockchain_invalidates_chains_with_fake_genesis() {
        // create chain
        let mut bc = Blockchain::new();

        // pop real genesis, push fake one
        bc.chain.pop();
        let fake_gen = Block::mine(&Block::genesis(), &vec![6, 6, 6]);
        bc.chain.push(fake_gen);

        assert_eq!(Blockchain::is_valid(&bc.chain), false);
    }

    #[test]
    fn blockchain_invalidates_chains_with_altered_prev_hash() {
        // create chain
        let mut bc = Blockchain::new();
        let mut fake_bc = Blockchain::new();

        // create and add valid 2nd block
        let a_data = vec![0, 1, 2];
        bc.add_block(&a_data);
        fake_bc.add_block(&a_data);

        // create and add valid 3rd block
        let b_data = vec![3, 4, 5];
        bc.add_block(&b_data);

        // change 3rd block previous hash, make it invalid
        fake_bc.chain.push(Block::new(
            "fake-last-hash".to_owned(),
            bc.chain.last().unwrap().hash.to_owned(),
            0,
            3,
            b_data,
        ));

        assert_eq!(Blockchain::is_valid(&fake_bc.chain), false);
    }

    #[test]
    fn blockchain_invalidates_chains_with_altered_data() {
        // create chain
        let mut bc = Blockchain::new();
        let mut fake_bc = Blockchain::new();

        // create and add valid 2nd block
        let a_data = vec![0, 1, 2];
        bc.add_block(&a_data);
        fake_bc.add_block(&a_data);

        // create and add valid 3rd block
        let b_data = vec![3, 4, 5];
        bc.add_block(&b_data);

        // alter 3rd block data, make it invalid
        fake_bc.chain.push(Block::new(
            fake_bc.chain.last().unwrap().hash.to_owned(),
            bc.chain.last().unwrap().hash.to_owned(),
            0,
            3,
            vec![6, 6, 6],
        ));

        assert_eq!(Blockchain::is_valid(&fake_bc.chain), false);
    }

    #[test]
    fn blockchain_validates_valid_chains() {
        // create chain
        let mut bc = Blockchain::new();

        // create and add valid 2nd block
        let a_data = vec![0, 1, 2];
        bc.add_block(&a_data);

        // create and add valid 3rd block
        let b_data = vec![3, 4, 5];
        bc.add_block(&b_data);

        assert_eq!(Blockchain::is_valid(&bc.chain), true);
    }

    #[test]
    fn blockchain_is_not_replaced_by_shorter_chain() {
        // create chains
        let mut bc = Blockchain::new();
        let shorter = Blockchain::new();

        // create and add valid 2nd block
        let a_data = vec![0, 1, 2];
        bc.add_block(&a_data);

        // store original chain, attempt to replace by shorter
        let original = &bc.chain.to_owned();
        bc.replace(&shorter.chain);

        // should not replace
        assert_eq!(&bc.chain, original);
    }

    #[test]
    fn blockchain_is_not_replaced_by_chain_with_diff_jump() {
        // create chain
        let mut bc = Blockchain::new();
        let mut invalid = Blockchain::new();

        // create and add valid 2nd block
        let a_data = vec![0, 1, 2];
        bc.add_block(&a_data);
        invalid.add_block(&a_data);

        // create and add invalid 3rd block to longer chain
        let b_data = vec![3, 4, 5];
        // we're more interested in lower diffs, but should also invalidate higher diff jumps
        let invalid_diff = bc.chain.last().unwrap().diff - 3;
        let last_hash = &bc.chain.last().unwrap().hash;
        invalid.chain.push(Block::new(
            last_hash.to_owned(),
            Block::to_hash(last_hash, &Utc::now(), 0, invalid_diff, &b_data),
            0,
            invalid_diff,
            b_data,
        ));

        // store original chain, attempt to replace by longer but invalid
        let original = &bc.chain.to_owned();
        bc.replace(&invalid.chain);

        // should not replace
        assert_eq!(&bc.chain, original);
    }

    #[test]
    fn blockchain_is_not_replaced_by_invalid_chain() {
        // create chain
        let mut bc = Blockchain::new();
        let mut invalid = Blockchain::new();

        // create and add valid 2nd block
        let a_data = vec![0, 1, 2];
        bc.add_block(&a_data);
        invalid.add_block(&a_data);

        // create and add invalid 3rd block to longer chain
        let b_data = vec![3, 4, 5];
        invalid.chain.push(Block::new(
            "fake-last-hash".to_owned(),
            Block::to_hash(&bc.chain.last().unwrap().hash, &Utc::now(), 0, 3, &b_data),
            0,
            3,
            b_data,
        ));

        // store original chain, attempt to replace by longer but invalid
        let original = &bc.chain.to_owned();
        bc.replace(&invalid.chain);

        // should not replace
        assert_eq!(&bc.chain, original);
    }

    #[test]
    fn blockchain_is_replaced_by_longer_chain() {
        // create chain
        let mut bc = Blockchain::new();
        let mut longer = Blockchain::new();

        // create and add valid 2nd block
        let a_data = vec![0, 1, 2];
        bc.add_block(&a_data);
        longer.add_block(&a_data);

        // create and add valid 3rd block to longer chain
        let b_data = vec![3, 4, 5];
        longer.add_block(&b_data);

        // attempt to replace shorter by longer
        bc.replace(&longer.chain);

        // should replace
        assert_eq!(&bc.chain, &longer.chain);
    }

    #[test]
    fn blockchain_average_mining_time_around_mine_rate() {
        // create chain
        let mut bc = Blockchain::new();
        let mut times = vec![];
        let mut last;
        let mut delta;
        let mut total: i64;
        let mut mean = 0;

        for i in 0..100usize {
            // store last timestamp
            last = bc.chain.last().unwrap().timestamp;
            // add block to bc
            bc.add_block(&i.to_le_bytes().to_vec());
            // get difference between last timestamp and new one
            delta = (bc.chain.last().unwrap().timestamp - last).num_milliseconds();
            times.push(delta);

            total = times.iter().sum();
            mean = total / times.len() as i64;

            // println!("Iterations left: {}", 100 - i);
            // println!(
            //     "Time to mine block: {}ms. Difficulty: {}. Average time: {}ms.",
            //     delta,
            //     bc.chain.last().unwrap().diff,
            //     mean
            // );
        }

        assert!(mean <= MINE_RATE + 100);
        assert!(mean >= MINE_RATE - 100);
    }
}
