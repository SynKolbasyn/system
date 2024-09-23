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
  io::{stdin, stdout, Stdin, Stdout, Write},
  fs::create_dir_all,
  path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use strum::{EnumIter, EnumMessage, IntoEnumIterator};
use ssh_key::{PrivateKey, rand_core::OsRng, Algorithm, LineEnding};
use homedir::my_home;

use crate::ui::menu::Menu;
use crate::user::User;


#[derive(EnumIter, EnumMessage)]
pub(crate) enum Login {
  #[strum(message = "Login", detailed_message = "Log in to an existing account")]
  Login,

  #[strum(message = "Register", detailed_message = "Register an account")]
  Register,
}


impl Default for Login {
  fn default() -> Self {
    Self::Login
  }
}


impl Menu for Login {
  fn show_menu(&self) -> Result<()> {
    for (i, e) in Self::iter().enumerate() {
      let action_name: String = e.get_message().context("The name of the action was not found")?.to_string();
      let action_description: String = e.get_detailed_message().context("The description of the action was not found")?.to_string();
      println!("[{}] [{action_name}] -> {action_description}", i + 1);
    }
    print!("~$ ");
    stdout().flush()?;
    Ok(())
  }


  fn process_action(&self, user: &User) -> Result<(Box<dyn Menu>, User)> {
    let mut action: String = String::new();
    stdin().read_line(&mut action)?;

    let (menu, user): (Box<dyn Menu>, User) = match action.to_lowercase().trim() {
      "1" => (Self::default_menu(), Self::load_user()?),
      "login" => (Self::default_menu(), Self::load_user()?),

      "2" => (Self::default_menu(), Self::create_user()?),
      "register" => (Self::default_menu(), Self::create_user()?),

      _ => {
        println!("Unknown action");
        (Self::default_menu(), User::default())
      },
    };

    Ok((
      menu,
      user,
    ))
  }
}


impl Login {
  fn create_user() -> Result<User> {
    let stdin: Stdin = stdin();
    let mut stdout: Stdout = stdout();

    let mut first_name: String = String::new();
    print!("Enter your first name: ");
    stdout.flush()?;
    stdin.read_line(&mut first_name)?;

    let mut last_name: String = String::new();
    print!("Enter your last name: ");
    stdout.flush()?;
    stdin.read_line(&mut last_name)?;

    let mut username: String = String::new();
    print!("Enter the username: ");
    stdout.flush()?;
    stdin.read_line(&mut username)?;

    let mut password: String = String::new();
    print!("Enter password: ");
    stdout.flush()?;
    stdin.read_line(&mut password)?;

    let mut key: PrivateKey = PrivateKey::random(&mut OsRng, Algorithm::Ed25519)?;
    key.set_comment(username.trim());
    key.encrypt(&mut OsRng, password.trim())?.write_openssh_file(&data_path("")?.join("key.pem"), LineEnding::LF)?;

    Ok(User::from(
      first_name.trim(),
      last_name.trim(),
      username.trim(),
    ))
  }


  fn load_user() -> Result<User> {
    Ok(User::default())
  }


  pub(crate) fn default_menu() -> Box<Self> {
    Box::new(Self::default())
  }
}


fn data_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
  let path: PathBuf = my_home()?.context("The user's home folder was not found")?.join(".system/").join(path);
  if !path.exists() {
    create_dir_all(&path)?;
  }
  Ok(path)
}
