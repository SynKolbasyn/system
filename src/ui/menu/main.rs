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


use std::io::{stdin, stdout, Write};

use anyhow::{Context, Result};
use strum::{EnumIter, EnumMessage, IntoEnumIterator};

use crate::{
  ui::menu::Menu,
  user::User,
};


#[derive(EnumIter, EnumMessage)]
pub(crate) enum Main {
  #[strum(message = "Transfer", detailed_message = "Transfer money to the user")]
  Transfer,
}


impl Default for Main {
  fn default() -> Self {
    Self::Transfer
  }
}


impl Menu for Main {
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


  fn process_action(&self, user: &mut User) -> Result<Box<dyn Menu>> {
    let mut action: String = String::new();
    stdin().read_line(&mut action)?;

    let menu: Box<dyn Menu> = match action.to_lowercase().trim() {
      "1" => {
        Self::transfer_money(user)?;
        Self::default_menu()
      },
      "transfer" => {
        Self::transfer_money(user)?;
        Self::default_menu()
      },

      _ => {
        println!("Unknown action");
        Self::default_menu()
      },
    };

    Ok(menu)
  }
}


impl Main {
  fn transfer_money(user: &mut User) -> Result<()> {
    Ok(())
  }


  pub(crate) fn default_menu() -> Box<Self> {
    Box::new(Self::default())
  }
}
