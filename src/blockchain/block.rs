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


use std::iter::repeat;

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ssh_key::{PrivateKey, SshSig, HashAlg, LineEnding};
use sha3::{Digest, Sha3_512};
use itertools::Itertools;

use crate::blockchain::data::Data;


#[derive(Serialize, Deserialize)]
struct Block {
  id: u128,
  prev_block_hash: Vec<u8>,
  data: Data,
  timestamp: DateTime<Utc>,
  proof_of_work: Vec<u8>,
  miner: String,
  signature: String,
  hash: Vec<u8>,
}


impl Block {
  fn new(id: u128, prev_block_hash: Vec<u8>, data: Data, timestamp: DateTime<Utc>, proof_of_work: Vec<u8>, miner: String, signature: String, hash: Vec<u8>) -> Self {
    Self {
      id,
      prev_block_hash,
      data,
      timestamp,
      proof_of_work,
      miner,
      signature,
      hash,
    }
  }


  fn create(data: Data, prev_block: Block, miner: PrivateKey) -> Result<Self> {
    let mut block: Self = Self::new(
      prev_block.id,
      prev_block.hash,
      data,
      Utc::now(),
      Vec::new(),
      miner.public_key().to_openssh()?,
      String::new(),
      Vec::new(),
    );

    let mut proof_of_work_size: usize = 1;
    loop {
      for proof_of_work in repeat(u8::MIN..=u8::MAX).take(proof_of_work_size).multi_cartesian_product() {
        block.proof_of_work = proof_of_work;
        let hash: Vec<u8> = block.hash()?;
        if hash.starts_with(&[0; 5]) {
          block.hash = hash;
          block.signature = SshSig::sign(&miner, "", HashAlg::Sha512, &serde_json::to_vec(&block)?)?.to_pem(LineEnding::LF)?;
          return Ok(block);
        }
      }
      proof_of_work_size += 1;
    }
  }


  fn hash(&self) -> Result<Vec<u8>> {
    Ok(Sha3_512::digest(serde_json::to_vec(self)?).to_vec())
  }
}
