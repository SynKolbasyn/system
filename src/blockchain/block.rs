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
  iter::repeat,
  path::{Path, PathBuf},
  fs::File,
};

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ssh_key::{HashAlg, LineEnding, PrivateKey, PublicKey, SshSig};
use sha3::{Digest, Sha3_512};
use itertools::Itertools;

use crate::{
  blockchain::data::{Data, r#type::Type},
  utils::data_path,
};


const COMPLEXITY: [u8; 2] = [0; 2];


#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Block {
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


  pub(crate) fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
    let file: File = File::options().truncate(false).read(true).open(path)?;
    Ok(serde_json::from_reader(file)?)
  }


  pub(crate) fn check(&mut self, prev_block: Option<Self>) -> Result<bool> {
    match prev_block {
      Some(block) => if self.prev_block_hash != block.hash {
        return Ok(false);
      },
      None => (),
    }

    let signature: String = self.signature.clone();
    self.signature = String::default();

    let serialized_block: Vec<u8> = serde_json::to_vec(&self)?;

    self.signature = signature;

    let public_key: PublicKey = PublicKey::from_openssh(&self.miner)?;
    let signature: SshSig = SshSig::from_pem(&self.signature)?;
    
    Ok(public_key.verify("system", &serialized_block, &signature).is_ok())
  }


  pub(crate) fn create_first(data: Data, miner: PrivateKey)  -> Result<Self>{
    let mut block: Self = Self::new(
      0,
      Vec::default(),
      data,
      DateTime::default(),
      Vec::default(),
      miner.public_key().to_openssh()?,
      String::default(),
      Vec::default(),
    );

    let mut proof_of_work_size: usize = 1;
    loop {
      for proof_of_work in repeat(u8::MIN..=u8::MAX).take(proof_of_work_size).multi_cartesian_product() {
        block.proof_of_work = proof_of_work;
        let hash: Vec<u8> = block.hash()?;
        if hash.starts_with(&COMPLEXITY) {
          block.timestamp = Utc::now();
          block.hash = hash;
          block.signature = miner.sign("system", HashAlg::Sha512, &serde_json::to_vec(&block)?)?.to_pem(LineEnding::LF)?;
          return Ok(block);
        }
      }
      proof_of_work_size += 1;
    }
  }


  pub(crate) fn create(data: Data, prev_block: Block, miner: PrivateKey) -> Result<Self> {
    let mut block: Self = Self::new(
      prev_block.id + 1,
      prev_block.hash,
      data,
      DateTime::default(),
      Vec::default(),
      miner.public_key().to_openssh()?,
      String::default(),
      Vec::default(),
    );

    let mut proof_of_work_size: usize = 1;
    loop {
      for proof_of_work in repeat(u8::MIN..=u8::MAX).take(proof_of_work_size).multi_cartesian_product() {
        block.proof_of_work = proof_of_work;
        let hash: Vec<u8> = block.hash()?;
        if hash.starts_with(&COMPLEXITY) {
          block.timestamp = Utc::now();
          block.hash = hash;
          block.signature = miner.sign("system", HashAlg::Sha512, &serde_json::to_vec(&block)?)?.to_pem(LineEnding::LF)?;
          return Ok(block);
        }
      }
      proof_of_work_size += 1;
    }
  }


  fn hash(&self) -> Result<Vec<u8>> {
    Ok(Sha3_512::digest(serde_json::to_vec(self)?).to_vec())
  }


  pub(crate) fn get_file_name(&self) -> Result<PathBuf> {
    Ok(data_path("blockchain/")?.join(format!("{}.json", self.id)))
  }


  pub(crate) fn get_data(&self) -> Vec<u8> {
    self.data.get_data()
  }


  pub(crate) fn get_data_type(&self) -> Type {
    self.data.get_type()
  }
}
