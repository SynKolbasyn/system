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
use serde::{Serialize, Deserialize};

use crate::user::User;


#[derive(Serialize, Deserialize)]
pub(crate) struct UserData {
  first_name: String,
  last_name: String,
  user_name: String,
  money: f64,
  public_key: String,
}


impl Default for UserData {
  fn default() -> Self {
    Self::new(
      String::default(),
      String::default(),
      String::default(),
      f64::default(),
      String::default(),
    )
  }
}


impl UserData {
  fn new(first_name: String, last_name: String, user_name: String, money: f64, public_key: String) -> Self {
    Self {
      first_name,
      last_name,
      user_name,
      money,
      public_key,
    }
  }


  pub(crate) fn create_new<
    FN: Into<String>,
    LN: Into<String>,
    UN: Into<String>,
  >(
    first_name: FN,
    last_name: LN,
    user_name: UN,
    public_key: String,
  ) -> Self {
    Self::new(
      first_name.into(),
      last_name.into(),
      user_name.into(),
      0.0,
      public_key,
    )
  }


  pub(crate) fn from_user(user: &User) -> Result<Self> {
    Ok(Self::new(
      user.get_first_name(),
      user.get_last_name(),
      user.get_user_name(),
      user.get_money(),
      user.get_key().public_key().to_openssh()?,
    ))
  }


  pub(crate) fn get_first_name(&self) -> String {
    self.first_name.clone()
  }


  pub(crate) fn get_last_name(&self) -> String {
    self.last_name.clone()
  }


  pub(crate) fn get_user_name(&self) -> String {
    self.user_name.clone()
  }


  pub(crate) fn get_money(&self) -> f64 {
    self.money
  }


  pub(crate) fn get_public_key(&self) -> String {
    self.public_key.clone()
  }
}
