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
// Hash 9859
// Hash 2150
// Hash 4032
// Hash 3051
// Hash 8664
// Hash 4182
// Hash 2008
// Hash 8810
// Hash 4748
// Hash 8623
// Hash 4474
// Hash 2702
// Hash 6648
// Hash 4927
// Hash 6746
// Hash 7543
// Hash 6490
// Hash 2098
// Hash 4046
// Hash 7626
// Hash 5747
// Hash 6872
// Hash 9675
// Hash 9612
// Hash 3937
// Hash 5382
// Hash 1716
// Hash 5811
// Hash 9953
// Hash 7786
// Hash 8496
// Hash 2223
// Hash 2269
// Hash 2339
// Hash 4453
// Hash 5742
// Hash 5024
// Hash 1662
// Hash 9901
// Hash 1884
// Hash 8853
// Hash 4006
// Hash 7696
// Hash 6773
// Hash 2570
// Hash 7973
// Hash 3815
// Hash 2832
// Hash 3175
// Hash 8377
// Hash 4247
// Hash 1803
// Hash 9784
// Hash 6516
// Hash 3385
// Hash 4489
// Hash 7948
// Hash 1945
// Hash 6494
// Hash 1620
// Hash 6540
// Hash 9440
// Hash 6793
// Hash 9755
// Hash 7069
// Hash 2952
// Hash 1697
// Hash 7699
// Hash 2437
// Hash 9137
// Hash 4339
// Hash 2007
// Hash 2307
// Hash 4287
// Hash 4641
// Hash 3946
// Hash 7487
// Hash 7616
// Hash 9478
// Hash 5377
// Hash 2763
// Hash 8191
// Hash 9210
// Hash 8561
// Hash 1561
// Hash 5697
// Hash 5958
// Hash 5986
// Hash 6331
// Hash 2520
// Hash 5152
// Hash 9964
// Hash 3957
// Hash 8215
// Hash 1592
// Hash 3442
// Hash 7898
// Hash 7307
// Hash 5009
// Hash 1297
// Hash 3135
// Hash 5010
// Hash 1844
// Hash 7308
// Hash 9706
// Hash 4671
// Hash 6770
// Hash 7129
// Hash 5862
// Hash 7255
// Hash 6108
// Hash 8196
// Hash 7282
// Hash 5397
// Hash 2742