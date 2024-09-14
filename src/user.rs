use std::{
  fs::create_dir_all,
  io::{stdin, stdout, Write},
  path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use homedir::my_home;
use ssh_key::{rand_core::OsRng, Algorithm, PrivateKey, LineEnding};
use tokio::task::JoinHandle;
use libp2p::identity::Keypair;

use crate::net::Net;
use crate::blockchain::{get_last_block, block::Block};


pub(crate) struct User {
  first_name: String,
  last_name: String,
  username: String,
}


impl User {
  pub(crate) fn new(first_name: String, last_name: String, username: String) -> Self {
    Self {
      first_name,
      last_name,
      username,
    }
  }


  pub(crate) fn from_key_path(key_path: &Path, password: String) -> Result<Self> {
    let key: PrivateKey = PrivateKey::read_openssh_file(key_path)?.decrypt(password)?;
    let keypair: Keypair = Keypair::ed25519_from_bytes(key.key_data().ed25519().context("Key type is not ed25519")?.private.to_bytes())?;
    let net_handle: JoinHandle<Result<()>> = Net::from_keypair(keypair)?.start();

    Ok(Self::new("".to_string(), "".to_string(), "".to_string()))
  }

  
  pub(crate) fn mine_block(&self, mut block: Block) -> Result<()> {
    let last_block: Block = get_last_block()?;
    block.confirm(last_block.data.id, last_block.hash, self.username.clone())?;
    Ok(())
  }


  pub(crate) fn create(key_path: &Path, password: String, first_name: String, last_name: String, username: String) -> Result<Self> {
    let mut key: PrivateKey = PrivateKey::random(&mut OsRng, Algorithm::Ed25519)?;
    key.set_comment(username.clone());
    key.encrypt(&mut OsRng, password)?.write_openssh_file(key_path, LineEnding::LF)?;
    Ok(Self::new(first_name, last_name, username))
  }


  pub(crate) fn get_account() -> Result<User> {
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

          match User::from_key_path(key_path, password.trim().to_string()) {
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
