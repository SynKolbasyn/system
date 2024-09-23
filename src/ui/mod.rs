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


use anyhow::Result;

use crate::ui::menu::{Menu, Login};
use crate::user::User;


pub(crate) struct UI {
  menu: Box<dyn Menu>,
  user: User,
}


impl Default for UI {
  fn default() -> Self {
    Self::new(
      Login::default_menu(),
      User::default(),
    )
  }
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
    (self.menu, self.user) = self.menu.process_action(&self.user)?;
    Ok(())
  }
}
