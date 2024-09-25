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


pub(crate) mod block;
pub(crate) mod data;


use std::fs::read_dir;

use anyhow::{bail, Result};
use ssh_key::{PrivateKey, PublicKey};

use crate::user::User;
use crate::net::{Net, api::API};
use crate::blockchain::{
  block::Block,
  data::{Data, user::UserData, r#type::Type},
};
use crate::utils::data_path;


pub(crate) struct Blockchain {
  net: API,
}


impl Blockchain {
  fn new(net: API) -> Self {
    Self {
      net,
    }
  }


  pub(crate) fn from_key(key: &PrivateKey) -> Result<Self> {
    let net: API = Net::from_key(&key)?;
    Ok(Self::new(net))
  }


  fn create_block(&self) -> Result<()> {
    Ok(())
  }


  fn check_block(&self, block: &Block) -> Result<bool> {
    Ok(true)
  }


  pub(crate) fn add_user(&self, user: &User) -> Result<()> {
    let data: Data = Data::create(Type::User, UserData::from_user(user)?, 0.0, user.get_key())?;
    self.net.send_block_data(data)?;
    Ok(())
  }


  pub(crate) fn get_user(&self, public_key: &PublicKey) -> Result<UserData> {
    let public_key: String = public_key.to_openssh()?;
    // let user_data: UserData = UserData::default();
    for block_path in read_dir(data_path("blockchain/")?)? {
      let block: Block = Block::from_path(block_path?.path())?;
      if !self.check_block(&block)? {
        todo!("ADD BLOCK VERIFICATION");
      }

      match block.get_data_type() {
        Type::User => {
          let user_data: UserData = serde_json::from_slice::<UserData>(&block.get_data())?;
          if user_data.get_public_key() == public_key {
            return Ok(user_data);
          }
        },
        Type::Transfer => (),
      }
    }

    bail!("User data not found in blockchain");
  }
}
