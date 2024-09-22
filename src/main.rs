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


mod user;
pub mod net;
mod blockchain;


use std::io::{stdin, stdout, Write};

use anyhow::Result;
use tokio::task::{self, JoinHandle};

use crate::user::User;
use crate::net::server::server_main;


#[tokio::main]
async fn main() {
  loop {
    match main_loop().await {
      Ok(_) => break,
      Err(e) => eprintln!("CRITICAL ERROR: {e}"),
    }
  }
}


async fn main_loop() -> Result<()> {
  if is_server_needed()? {
    task::spawn(async {
      server_main().await
    });
  }

  let user: User = User::get_account().await?;

  loop {
    
  }
}


fn is_server_needed() -> Result<bool> {
  print!("Do you want to use your computer as a server as well? [Y/n]: ");
  stdout().flush()?;

  let mut agreement = String::new();
  stdin().read_line(&mut agreement)?;

  Ok(match agreement.to_lowercase().trim() {
    "y" => true,
    "yes" => true,
    _ => false,
  })
}
