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


use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use ssh_key::{PrivateKey, SshSig, HashAlg, LineEnding};


#[derive(Serialize, Deserialize)]
pub(crate) struct Data {
  timestamp: DateTime<Utc>,
  data: Vec<u8>,
  miner_amount: f64,
  public_key: String,
  signature: String,
}


impl Data {
  fn new(timestamp: DateTime<Utc>, data: Vec<u8>, miner_amount: f64, public_key: String, signature: String) -> Self {
    Self {
      timestamp,
      data,
      miner_amount,
      public_key,
      signature,
    }
  }


  fn create<D: Serialize>(data: D, miner_amount: f64, private_key: PrivateKey) -> Result<Self> {
    let data: Vec<u8> = serde_json::to_vec(&data)?;
    let data: Self = Self::new(
      Utc::now(),
      data,
      miner_amount,
      private_key.public_key().to_openssh()?,
      String::new(),
    );
    let signature: SshSig = SshSig::sign(&private_key, "", HashAlg::Sha512, &serde_json::to_vec(&data)?)?;
    Ok(Self {
      signature: signature.to_pem(LineEnding::LF)?,
      ..data
    })
  }
}
