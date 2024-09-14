use std::{
  fs::File,
  path::Path,
};

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::blockchain::data::Data;


#[derive(Serialize, Deserialize)]
pub(crate) struct Block {
  pub(crate) data: Data,
  pub(crate) hash: Vec<u8>,
  confirm_time: DateTime<Utc>,
}


impl Block {
  fn new(data: Data, hash: Vec<u8>, confirm_time: DateTime<Utc>) -> Self {
    Self {
      data,
      hash,
      confirm_time,
    }
  }


  pub(crate) fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
    let file: File = File::options().truncate(false).read(true).open(path)?;
    Ok(serde_json::from_reader(file)?)
  }


  pub(crate) fn create<D: Serialize + for<'a> Deserialize<'a>>(data: D, miner_amount: f64) -> Result<Self> {
    Ok(Self::new(
      Data::create(data, miner_amount)?,
      Vec::new(),
      DateTime::default(),
    ))
  }


  pub(crate) fn confirm(&mut self, prev_block_id: u128, prev_block_hash: Vec<u8>, miner: String) -> Result<()> {
    self.hash = self.data.mine(prev_block_id, prev_block_hash, miner)?;
    self.confirm_time = Utc::now();
    Ok(())
  }
}
