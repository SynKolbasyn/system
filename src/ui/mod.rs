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


mod menu;


use std::io::{stdin, stdout, Stdin, Stdout, Write};

use anyhow::Result;

use crate::utils::data_path;
use crate::ui::menu::{Menu, main::Main};
use crate::user::User;


pub(crate) struct UI {
  menu: Box<dyn Menu>,
  user: User,
}


impl UI {
  fn new<M: Menu + 'static>(menu: Box<M>, user: User) -> Self {
    Self {
      menu,
      user,
    }
  }


  pub(crate) fn show_menu(&self) -> Result<()> {
    self.menu.show_menu()?;
    Ok(())
  }


  pub(crate) fn process_action(&mut self) -> Result<()> {
    self.menu = self.menu.process_action(&mut self.user)?;
    Ok(())
  }


  pub(crate) fn create() -> Result<Self> {
    let user: User = if !data_path("")?.join("key.pem").exists() {
      Self::create_user()?
    } else {
      let mut password: String = String::new();
      print!("Enter password: ");
      stdout().flush()?;
      stdin().read_line(&mut password)?;
      User::from_password(password.trim().to_string())?
    };

    Ok(Self::new(
      Main::default_menu(),
      user,
    ))
  }


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

    let mut user_name: String = String::new();
    print!("Enter your user name: ");
    stdout.flush()?;
    stdin.read_line(&mut user_name)?;

    let mut password: String = String::new();
    print!("Enter password: ");
    stdout.flush()?;
    stdin.read_line(&mut password)?;

    let user: User = User::create(
      first_name.trim(),
      last_name.trim(),
      user_name.trim(),
      password.trim(),
    )?;

    Ok(user)
  }
}
