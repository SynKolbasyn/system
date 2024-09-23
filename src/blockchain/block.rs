use chrono::{DateTime, Utc};

use crate::blockchain::data::Data;


struct Block {
  data: Data,
  timestamp: DateTime<Utc>,
  hash: Vec<u8>,
}
