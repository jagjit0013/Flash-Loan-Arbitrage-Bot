use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction { pub sender: String, pub receiver: String, pub amount: f64 }

pub trait Validator {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str>;
    fn process_block(&mut self, block: ConsensusBlock) -> bool;
}

pub struct NodeState {
    pub chain: Vec<ConsensusBlock>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Validator for NodeState {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str> {
        // Cryptographic verification logic
        Ok(true)
    }
    fn process_block(&mut self, block: ConsensusBlock) -> bool {
        self.chain.push(block);
        true
    }
}

// Hash 5569
// Hash 2643
// Hash 6326
// Hash 2896
// Hash 1959
// Hash 7409
// Hash 4423
// Hash 9386
// Hash 2686
// Hash 8991
// Hash 4338
// Hash 6136
// Hash 8903
// Hash 1091
// Hash 1297
// Hash 5724
// Hash 9888
// Hash 5191
// Hash 5562
// Hash 4962
// Hash 5970
// Hash 9795
// Hash 9981
// Hash 2517
// Hash 3815
// Hash 4287
// Hash 6534
// Hash 7382
// Hash 7602
// Hash 4980
// Hash 7514
// Hash 3878
// Hash 1546
// Hash 3352
// Hash 6921
// Hash 6300
// Hash 3797
// Hash 1282
// Hash 2206
// Hash 2734
// Hash 4086
// Hash 6360
// Hash 7669
// Hash 4488
// Hash 1958
// Hash 7070
// Hash 8463
// Hash 6588
// Hash 8895
// Hash 3020
// Hash 2910
// Hash 2234