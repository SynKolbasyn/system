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


use std::{
  fs::create_dir_all,
  io::{stdin, stdout, Write},
  path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use homedir::my_home;
use serde::{Deserialize, Serialize};
use ssh_key::{rand_core::OsRng, Algorithm, PrivateKey, LineEnding};
use tokio::task::JoinHandle;
use libp2p::identity::Keypair;

use crate::net::Net;
use crate::blockchain::{self, get_last_block, block::Block};


#[derive(Serialize, Deserialize, Clone)]
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


  pub(crate) async fn from_key_path(key_path: &Path, password: String) -> Result<Self> {
    let key: PrivateKey = PrivateKey::read_openssh_file(key_path)?.decrypt(password)?;
    let keypair: Keypair = Keypair::ed25519_from_bytes(key.key_data().ed25519().context("Key type is not ed25519")?.private.to_bytes())?;
    let net_handle: JoinHandle<Result<()>> = Net::from_keypair(keypair)?.start();

    Ok(blockchain::get_user()?)
  }

  
  pub(crate) fn mine_block(&self, mut block: Block) -> Result<()> {
    let last_block: Block = get_last_block()?;
    block.confirm(last_block.data.id, last_block.hash, self.username.clone())?;
    Ok(())
  }


  pub(crate) fn create(key_path: &Path, password: String, first_name: String, last_name: String, username: String) -> Result<Self> {
    let mut key: PrivateKey = PrivateKey::random(&mut OsRng, Algorithm::Ed25519)?;
    key.set_comment(username.clone());

    let keypair: Keypair = Keypair::ed25519_from_bytes(key.key_data().ed25519().context("Key type is not ed25519")?.private.to_bytes())?;
    let net_handle: JoinHandle<Result<()>> = Net::from_keypair(keypair)?.start();

    key.encrypt(&mut OsRng, password)?.write_openssh_file(key_path, LineEnding::LF)?;

    

    Ok(Self::new(first_name, last_name, username))
  }


  pub(crate) async fn get_account() -> Result<User> {
    let default_path: PathBuf = my_home()?.context("Could not get the path of the home folder")?.join(".system/private_key.pem");
    
    'main_loop: loop {
      print!("Enter the path to the key (by default '{}'): ", default_path.to_string_lossy().to_string());
      stdout().flush()?;
  
      let mut path: String = String::new();
      stdin().read_line(&mut path)?;
  
      let key_path: &Path = if path.trim().is_empty() {
        default_path.as_path()
      }
      else {
        Path::new(&path)
      };
  
      if key_path.exists() {
        loop {
          print!("Enter password: ");
          stdout().flush()?;
  
          let mut password: String = String::new();
          stdin().read_line(&mut password)?;

          match User::from_key_path(key_path, password.trim().to_string()).await {
            Ok(user) => return Ok(user),
            Err(e) => eprintln!("Failed to load the user due to an error: {e}"),
          }
  
          print!("Do you want to try load again? [Y/n]: ");
          stdout().flush()?;
  
          let mut confirmation: String = String::new();
          stdin().read_line(&mut confirmation)?;
  
          match confirmation.to_lowercase().trim() {
            "y" => (),
            "yes" => (),
            _ => continue 'main_loop,
          }
        }
      }
  
      print!("The key was not found on the path '{}'. Do you want to create a new one? [Y/n]: ", key_path.to_string_lossy().to_string());
      stdout().flush()?;
  
      let mut confirmation: String = String::new();
      stdin().read_line(&mut confirmation)?;
  
      match confirmation.to_lowercase().trim() {
        "y" => { Self::create_account(default_path.as_path())?; },
        "yes" => { Self::create_account(default_path.as_path())?; },
        _ => (),
      }
    }
  }
  
  
  fn create_account(default_path: &Path) -> Result<User> {
    let mut first_name: String = String::new();
    let mut last_name: String = String::new();
    let mut username: String = String::new();
    let mut key_path: String = String::new();
    let mut password: String = String::new();
  
    print!("Enter your first name: ");
    stdout().flush()?;
    stdin().read_line(&mut first_name)?;
  
    print!("Enter your last name: ");
    stdout().flush()?;
    stdin().read_line(&mut last_name)?;
  
    print!("Enter your username: ");
    stdout().flush()?;
    stdin().read_line(&mut username)?;
  
    print!("Where do you want to save the secret key? (default - '{}'): ", default_path.to_string_lossy().to_string());
    stdout().flush()?;
    stdin().read_line(&mut key_path)?;

    print!("Enter password: ");
    stdout().flush()?;
    stdin().read_line(&mut password)?;
  
    let key_path: &Path = if key_path.trim().is_empty() {
      default_path
    }
    else {
      Path::new(&key_path)
    };
    create_dir_all(key_path.parent().unwrap_or(Path::new("/")))?;
    
  
    Ok(User::create(
      key_path,
      password.trim().to_string(),
      first_name.trim().to_string(),
      last_name.trim().to_string(),
      username.trim().to_string(),
    )?)
  }
}
