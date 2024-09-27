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


use libp2p::gossipsub::Sha256Topic;


#[derive(Clone, Hash, PartialEq, Eq)]
pub(crate) struct SendData {
  topic: String,
  pub(crate) data: Vec<u8>,
}


impl Default for SendData {
  fn default() -> Self {
    Self::new(
      String::default(),
      Vec::default(),
    )
  }
}


impl SendData {
  fn new(topic: String, data: Vec<u8>) -> Self {
    Self {
      topic,
      data,
    }
  }


  pub(crate) fn create<T: Into<String>>(topic: T, data: Vec<u8>) -> Self {
    Self::new(
      topic.into(),
      data,
    )
  }


  pub(crate) fn topic(&self) -> Sha256Topic {
    Sha256Topic::new(self.topic.clone())
  }


  pub(crate) fn data(&self) -> Vec<u8> {
    self.data.clone()
  }
}
