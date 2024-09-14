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


use anyhow::{Result, Context};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sha3::{Digest, Sha3_512};


#[derive(Serialize, Deserialize)]
pub(crate) struct Data {
  pub(crate) id: u128,
  prev_block_hash: Vec<u8>,
  create_time: DateTime<Utc>,
  data: Vec<u8>,
  miner: String,
  miner_amount: f64,
  proof_of_work: Vec<u8>,
}


impl Data {
  fn new(id: u128, prev_block_hash: Vec<u8>, create_time: DateTime<Utc>, data: Vec<u8>, miner: String, miner_amount: f64, proof_of_work: Vec<u8>) -> Self {
    Self {
      id,
      prev_block_hash,
      create_time,
      data,
      miner,
      miner_amount,
      proof_of_work,
    }
  }


  pub(crate) fn create<D: Serialize + for<'a> Deserialize<'a>>(data: D, miner_amount: f64) -> Result<Self> {
    Ok(Self::new(
      u128::MIN,
      Vec::new(),
      Utc::now(),
      serde_json::to_vec(&data)?,
      String::new(),
      miner_amount,
      Vec::new(),
    ))
  }


  pub(crate) fn mine(&mut self, prev_block_id: u128, prev_block_hash: Vec<u8>, miner: String) -> Result<Vec<u8>> {
    self.id = prev_block_id + 1;
    self.prev_block_hash = prev_block_hash;
    self.miner = miner;

    let mut proof_of_work: Vec<u8> = Vec::new();
    let mut idx: usize = 0;

    loop {
      proof_of_work.push(u8::MIN);
      for byte in u8::MIN..=u8::MAX {
        proof_of_work[idx] = byte;
        self.proof_of_work = proof_of_work.clone();

        let hash: Vec<u8> = self.hash()?;
        let first_byte_of_hash: &u8 = hash.first().context("The hash does not contain the first byte")?;

        if *first_byte_of_hash == u8::MIN {
          return Ok(hash);
        }
      }
      idx += 1;
    }
  }

  
  pub(crate) fn hash(&self) -> Result<Vec<u8>> {
    Ok(Sha3_512::digest(serde_json::to_vec(self)?).to_vec())
  }
}
