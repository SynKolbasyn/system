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
  path::{Path, PathBuf},
  fs::create_dir_all,
};

use anyhow::{Result, Context};
use homedir::my_home;


pub(crate) fn data_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
  let path: PathBuf = my_home()?.context("The user's home folder was not found")?.join(".system/").join(path);
  if !path.exists() {
    create_dir_all(&path)?;
  }
  Ok(path)
}
