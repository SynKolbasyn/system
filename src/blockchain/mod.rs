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
  fs::read_dir
};

use anyhow::{Result, Context};
use homedir::my_home;

use crate::blockchain::block::Block;


pub(crate) fn get_last_block() -> Result<Block> {
  let blockchain_path: PathBuf = my_home()?.context("Could not get the path of the blockchain folder")?.join("blockchain/");
  let block_id: usize = read_dir(&blockchain_path)?.count();
  Ok(Block::from_path(blockchain_path.join(format!("{block_id}.json")))?)
}
