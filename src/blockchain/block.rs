//!   system. The program that gives access to the system.
//!   Copyright (C) 2024  Andrew Kozmin
//!   
//!   This program is free software: you can redistribute it and/or modify
//!   it under the terms of the GNU Affero General Public License as published
//!   by the Free Software Foundation, either version 3 of the License, or
//!   (at your option) any later version.
//!   
//!   This program is distributed in the hope that it will be useful,
//!   but WITHOUT ANY WARRANTY; without even the implied warranty of
//!   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//!   GNU Affero General Public License for more details.
//!   
//!   You should have received a copy of the GNU Affero General Public License
//!   along with this program.  If not, see <https://www.gnu.org/licenses/>.


use std::{
  fs::File,
  path::Path,
};

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use ssh_key::PrivateKey;

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


  pub(crate) fn create<D: Serialize + for<'a> Deserialize<'a>>(data: D, private_key: PrivateKey, miner_amount: f64) -> Result<Self> {
    Ok(Self::new(
      Data::create(data, private_key, miner_amount)?,
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
