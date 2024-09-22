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


use std::{
  path::PathBuf,
  fs::{read_dir, create_dir_all},
};

use anyhow::{Result, Context};
use homedir::my_home;
use ssh_key::PrivateKey;

use crate::blockchain::block::Block;
use crate::user::User;


pub(crate) struct Blockchain {
  path: PathBuf,
}


impl Blockchain {
  fn new(path: PathBuf) -> Self {
    Self {
      path,
    }
  }


  pub(crate) fn from_default_path() -> Result<Self> {
    Ok(Self::new(my_home()?.context("Could not get the path of the blockchain folder")?.join(".system/blockchain/")))
  }


  pub(crate) fn get_last_block(&self) -> Result<Block> {
    let block_id: usize = read_dir(&self.path)?.count();
    Ok(Block::from_path(self.get_path()?.join(format!("{block_id}.json")))?)
  }
  
  
  pub(crate) fn get_user(&self) -> Result<User> {
    let mut user = User::default();
  
    for block in read_dir(self.get_path()?)? {
      println!("{:?}", block);
    }
  
    Ok(user)
  }
  
  
  pub(crate) fn create_user(&self, user: User, private_key: PrivateKey) -> Result<User> {
    let block: Block = Block::create(user.clone(), private_key, f64::default())?;
    block.confirm(prev_block_id, prev_block_hash, user.);
    Ok(user)
  }


  fn get_path(&self) -> Result<PathBuf> {
    if !self.path.exists() {
      create_dir_all(&self.path)?;
    }
    Ok(self.path.clone())
  }
}
