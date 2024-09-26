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
use ssh_key::{PrivateKey, rand_core::OsRng, Algorithm, LineEnding};

use crate::{
  blockchain::{Blockchain, data::user::UserData},
  utils::data_path
};


pub(crate) struct User {
  first_name: String,
  last_name: String,
  user_name: String,
  money: f64,
  key: PrivateKey,
  blockchain: Blockchain,
}


impl User {
  pub(crate) fn new(first_name: String, last_name: String, user_name: String, money: f64, key: PrivateKey, blockchain: Blockchain) -> Self {
    Self {
      first_name,
      last_name,
      user_name,
      money,
      key,
      blockchain,
    }
  }


  fn from_user_data(user_data: UserData, key: PrivateKey, blockchain: Blockchain) -> Self {
    Self::new(
      user_data.get_first_name(),
      user_data.get_last_name(),
      user_data.get_user_name(),
      user_data.get_money(),
      key,
      blockchain,
    )
  }


  pub(crate) fn create<
    FN: Into<String>,
    LN: Into<String>,
    UN: Into<String> + Clone,
    PW: Into<String>,
  >(
    first_name: FN,
    last_name: LN,
    user_name: UN,
    password: PW,
  ) -> Result<Self> {
    let mut key: PrivateKey = PrivateKey::random(&mut OsRng, Algorithm::Ed25519)?;
    key.set_comment(user_name.clone());
    key.encrypt(&mut OsRng, password.into())?.write_openssh_file(&data_path("")?.join("key.pem"), LineEnding::LF)?;

    let blockchain: Blockchain = Blockchain::from_key(&key)?;
    
    let user_data: UserData = UserData::create_new(
      first_name,
      last_name,
      user_name,
      key.public_key().to_openssh()?,
    );

    let user: Self = Self::from_user_data(user_data, key, blockchain);
    user.blockchain.add_user(&user)?;
    
    Ok(user)
  }


  pub(crate) fn from_password(password: String) -> Result<Self> {
    let key: PrivateKey = PrivateKey::read_openssh_file(&data_path("")?.join("key.pem"))?;
    let key: PrivateKey = key.decrypt(password)?;
    let blockchain: Blockchain = Blockchain::from_key(&key)?;
    let user_data: UserData = blockchain.get_user(key.public_key())?;
    Ok(Self::from_user_data(user_data, key, blockchain))
  }


  pub(crate) fn get_key(&self) -> PrivateKey {
    self.key.clone()
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
}



