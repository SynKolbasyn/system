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
use serde::Serialize;
use tokio::{task::JoinHandle, sync::watch::Sender};

use crate::blockchain::{block::Block, data::Data};
use crate::net::send_data::SendData;


pub(crate) struct API {
  net_handle: JoinHandle<Result<()>>,
  sender: Sender<SendData>,
}


impl API {
  pub(crate) fn new(net_handle: JoinHandle<Result<()>>, sender: Sender<SendData>) -> Self {
    Self {
      net_handle,
      sender,
    }
  }


  fn send<T: Into<String>, S: Serialize>(&self, topic: T, data: S) -> Result<()> {
    let send_data: SendData = SendData::create(topic, serde_json::to_vec(&data)?);
    self.sender.send(send_data)?;
    Ok(())
  }


  pub(crate) fn send_block(&self, block: &Block) -> Result<()> {
    self.send("block", block)?;
    Ok(())
  }


  pub(crate) fn send_block_data(&self, block_data: &Data) -> Result<()> {
    self.send("block_data", block_data)?;
    Ok(())
  }
}
