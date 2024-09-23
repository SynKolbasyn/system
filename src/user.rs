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


use std::path::PathBuf;


// #[derive(Debug, Clone)]
pub(crate) struct User {
  first_name: String,
  last_name: String,
  username: String,
  key_path: PathBuf,
}


impl Default for User {
  fn default() -> Self {
    Self::new(
      String::default(),
      String::default(),
      String::default(),
      PathBuf::default(),
    )
  }
}


impl User {
  fn new(first_name: String, last_name: String, username: String, key_path: PathBuf) -> Self {
    Self {
      first_name,
      last_name,
      username,
      key_path,
    }
  }
}
