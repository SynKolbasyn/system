use chrono::{DateTime, Utc};


pub(crate) struct Data {
  id: u128,
  prev_block_hash: Vec<u8>,
  timestamp: DateTime<Utc>,
  data: Vec<u8>,
  miner: String,
  miner_amount: f64,
  proof_of_work: Vec<u8>,
}
