use crate::config::MINE_RATE;
use crate::utils::hex_to_binary;
use chrono::prelude::*;
use chrono::Duration;
use sha2::{Digest, Sha256};
use serde::{Serialize};

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Block {
    // Headers
    pub timestamp: DateTime<Utc>,
    pub prev: String,
    pub hash: String,
    pub nonce: usize,
    pub diff: usize,
    // Body
    pub data: Vec<u8>,
}

impl Block {
    pub fn new(prev: String, hash: String, nonce: usize, diff: usize, data: Vec<u8>) -> Self {
        Self {
            timestamp: Utc::now(),
            prev,
            hash,
            nonce,
            diff,
            data,
        }
    }

    pub fn genesis() -> Self {
        Self {
            timestamp: Utc::now(),
            prev: "gen-prev".to_owned(),
            hash: "gen-hash".to_owned(),
            nonce: 0,
            diff: 3,
            data: [0u8; 8].to_owned().to_vec(),
        }
    }

    pub fn mine(prev_block: &Block, data: &Vec<u8>) -> Self {
        let mut timestamp = Utc::now();
        let mut nonce = 0;
        let mut diff = prev_block.diff;
        let mut hash = Block::to_hash(&prev_block.hash, &timestamp, nonce, diff, data);

        while hex_to_binary(&hash)[0..diff] != "0".repeat(diff) {
            nonce += 1;
            timestamp = Utc::now();
            diff = Block::adjust_diff(prev_block, &timestamp);
            hash = Block::to_hash(&prev_block.hash, &timestamp, nonce, diff, data);
        }

        Self {
            timestamp,
            prev: prev_block.hash.to_owned(),
            hash,
            nonce,
            diff,
            data: data.to_owned(),
        }
    }

    pub fn to_hash(
        prev: &str,
        timestamp: &DateTime<Utc>,
        nonce: usize,
        diff: usize,
        data: &Vec<u8>,
    ) -> String {
        format!(
            "{:X}",
            Sha256::default()
                .chain(timestamp.timestamp().to_le_bytes())
                .chain(prev)
                .chain(nonce.to_le_bytes())
                .chain(diff.to_le_bytes())
                .chain(data)
                .finalize()
        )
    }

    pub fn valid_hash(&self) -> bool {
        self.hash
            == Block::to_hash(
                &self.prev,
                &self.timestamp,
                self.nonce,
                self.diff,
                &self.data,
            )
    }

    pub fn adjust_diff(block: &Block, timestamp: &DateTime<Utc>) -> usize {
        if &(block.timestamp + Duration::milliseconds(MINE_RATE)) > timestamp {
            block.diff + 1
        } else {
            block.diff - 1
        }
    }
}

#[cfg(test)]
mod block_tests {
    use super::*;

    #[test]
    fn block_identity_test() {
        let block = Block::new(
            "prevhash".to_owned(),
            "hash".to_owned(),
            1,
            1,
            [0u8; 32].to_owned().to_vec(),
        );
        assert_eq!(block.prev, "prevhash".to_owned());
        assert_eq!(block.hash, "hash".to_owned());
        assert_eq!(block.nonce, 1);
        assert_eq!(block.diff, 1);
        assert_eq!(block.data, [0u8; 32].to_owned().to_vec());
    }

    #[test]
    fn genesis_block_test() {
        let block = Block::genesis();
        assert_eq!(block.prev, "gen-prev".to_owned());
        assert_eq!(block.hash, "gen-hash".to_owned());
        assert_eq!(block.nonce, 0);
        assert_eq!(block.diff, 3);
        assert_eq!(block.data, [0u8; 32].to_owned().to_vec());
    }

    #[test]
    fn mine_block_test() {
        let prev = Block::genesis();
        let data = "mined data".to_owned().into_bytes();
        let mined = Block::mine(&prev, &data);
        let mined_hash = format!(
            "{:X}",
            Sha256::default()
                .chain(mined.timestamp.timestamp().to_le_bytes())
                .chain(&prev.hash)
                .chain(mined.nonce.to_le_bytes())
                .chain(mined.diff.to_le_bytes())
                .chain(&data)
                .finalize(),
        );
        assert_eq!(mined.data, data);
        assert_eq!(mined.prev, prev.hash);
        assert_eq!(mined.hash, mined_hash);
        assert_eq!(hex_to_binary(&mined.hash)[0..mined.diff], "0".repeat(mined.diff))
    }

    #[test]
    fn hash_identity_test() {
        let hex = format!("{:X}", Sha256::default().chain("foo").finalize());
        assert_eq!(
            hex,
            "2C26B46B68FFC68FF99B453C1D30413413422D706483BFA0F98A5E886266E7AE"
        );
        assert_eq!(
            hex_to_binary(&hex), 
            "0010110000100110101101000110101101101000111111111100011010001111111110011001101101000101001111000001110100110000010000010011010000010011010000100010110101110000011001001000001110111111101000001111100110001010010111101000100001100010011001101110011110101110"
            )
    }

    #[test]
    fn increase_diff_for_next_block() {
        let previous = Block::mine(&Block::genesis(), &vec![0, 0, 0]);

        // previous timestamp + MINE_RATE is higher than timestamp + MINE_RATE + 100
        // thus, the following block was quickly mined
        // difficulty should be increased
        assert_eq!(
            Block::adjust_diff(
                &previous,
                &(previous.timestamp + Duration::milliseconds(MINE_RATE)
                    - Duration::milliseconds(100))
            ),
            previous.diff + 1
        );
    }

    #[test]
    fn decrease_diff_for_next_block() {
        let previous = Block::mine(&Block::genesis(), &vec![0, 0, 0]);

        // previous timestamp + MINE_RATE is lower than timestamp + MINE_RATE - 100
        // thus, the following block was slowly mined
        // difficulty should be decreased
        assert_eq!(
            Block::adjust_diff(
                &previous,
                &(previous.timestamp
                    + Duration::milliseconds(MINE_RATE)
                    + Duration::milliseconds(100))
            ),
            previous.diff - 1
        );
    }

    #[test]
    fn block_adjusts_diff() {
        let previous = Block::mine(&Block::genesis(), &vec![0, 0, 0]);
        let next = Block::mine(&previous, &vec![0, 0, 0]);
        let next_possible_diffs = vec![previous.diff + 1, previous.diff - 1];

        assert!(next_possible_diffs.contains(&next.diff));
    }
}
