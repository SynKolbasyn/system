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


mod ui;
mod user;
mod blockchain;
mod net;
mod utils;


use std::io::{stdin, stdout, Write};

use anyhow::Result;
use tokio::task;

use crate::{ui::UI, net::server::server_main};



#[tokio::main]
async fn main() {
  loop {
    match main_loop() {
      Ok(_) => break,
      Err(error) => eprintln!("CRITICAL ERROR: {error}"),
    }
  }
}


fn main_loop() -> Result<()> {
  if is_server_needed()? {
    task::spawn(async {
      server_main().await?;
      anyhow::Ok(())
    });
  }

  let mut ui: UI = UI::create()?;

  loop {
    ui.show_menu()?;
    ui.process_action()?;
  }
}


fn is_server_needed() -> Result<bool> {
  let mut answer: String = String::new();
  print!("Do you want to run the server in addition to the main program? [Y/n]: ");
  stdout().flush()?;
  stdin().read_line(&mut answer)?;
  Ok(match answer.to_lowercase().trim() {
    "y" => true,
    "yes" => true,
    _ => false,
  })
}
