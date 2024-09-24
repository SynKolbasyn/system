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
mod data;


use std::sync::{Arc, RwLock};

use anyhow::Result;

use crate::net::Net;
use crate::blockchain::block::Block;


struct Blockchain {
  net: Arc<RwLock<Net>>,
}


impl Blockchain {
  fn new(net: Arc<RwLock<Net>>) -> Self {
    Self {
      net: net,
    }
  }


  fn create_block(&self) -> Result<()> {
    Ok(())
  }


  fn check_block(&self, block: Block) -> Result<bool> {

    Ok(true)
  }
}
