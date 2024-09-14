pub(crate) mod block;
pub(crate) mod data;


use std::{
  path::PathBuf,
  fs::read_dir
};

use anyhow::{Result, Context};
use homedir::my_home;

use crate::blockchain::block::Block;


pub(crate) fn get_last_block() -> Result<Block> {
  let blockchain_path: PathBuf = my_home()?.context("Could not get the path of the blockchain folder")?.join("blockchain/");
  let block_id: usize = read_dir(&blockchain_path)?.count();
  Ok(Block::from_path(blockchain_path.join(format!("{block_id}.json")))?)
}
