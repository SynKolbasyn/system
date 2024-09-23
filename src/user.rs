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


// #[derive(Debug, Clone)]
pub(crate) struct User {
  first_name: String,
  last_name: String,
  username: String,
}


impl Default for User {
  fn default() -> Self {
    Self::new(
      String::default(),
      String::default(),
      String::default(),
    )
  }
}


impl User {
  pub(crate) fn new(first_name: String, last_name: String, username: String) -> Self {
    Self {
      first_name,
      last_name,
      username,
    }
  }


  pub(crate) fn from<FN: ToString, LN: ToString, UN: ToString>(first_name: FN, last_name: LN, username: UN) -> Self {
    Self::new(
      first_name.to_string(),
      last_name.to_string(),
      username.to_string(),
    )
  }
}
