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


pub(crate) mod r#type;
pub(crate) mod user;


use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use ssh_key::{PrivateKey, HashAlg, LineEnding};

use crate::blockchain::data::r#type::Type;


#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Data {
  timestamp: DateTime<Utc>,
  r#type: Type,
  data: Vec<u8>,
  miner_amount: f64,
  public_key: String,
  signature: String,
}


impl Data {
  fn new(timestamp: DateTime<Utc>, r#type: Type, data: Vec<u8>, miner_amount: f64, public_key: String, signature: String) -> Self {
    Self {
      timestamp,
      r#type,
      data,
      miner_amount,
      public_key,
      signature,
    }
  }


  pub(crate) fn create<S: Serialize>(r#type: Type, data: S, miner_amount: f64, key: PrivateKey) -> Result<Self> {
    let data: Vec<u8> = serde_json::to_vec(&data)?;
    let data: Self = Self::new(
      Utc::now(),
      r#type,
      data,
      miner_amount,
      key.public_key().to_openssh()?,
      String::new(),
    );
    let signature: String = key.sign("system", HashAlg::Sha512, &serde_json::to_vec(&data)?)?.to_pem(LineEnding::LF)?;
    Ok(Self {
      signature,
      ..data
    })
  }


  pub(crate) fn get_data(&self) -> Vec<u8> {
    self.data.clone()
  }


  pub(crate) fn get_type(&self) -> Type {
    self.r#type.clone()
  }
}
