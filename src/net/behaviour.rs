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


use std::time::Duration;

use anyhow::Result;
use libp2p::{
  gossipsub::{self, MessageAuthenticity, ValidationMode},
  identify,
  identity::{Keypair, PublicKey},
  kad::{self, store::MemoryStore, PROTOCOL_NAME},
  swarm::NetworkBehaviour,
  PeerId,
};


#[derive(NetworkBehaviour)]
pub(crate) struct Behaviour {
  pub(crate) gossipsub: gossipsub::Behaviour,
  pub(crate) identify: identify::Behaviour,
  pub(crate) kademlia: kad::Behaviour<MemoryStore>,
}


impl Behaviour {
  fn new(
    gossipsub: gossipsub::Behaviour,
    identify: identify::Behaviour,
    kademlia: kad::Behaviour<MemoryStore>,
  ) -> Self {
    Self {
      gossipsub,
      identify,
      kademlia,
    }
  }


  pub(crate) fn from_key(key: Keypair) -> Result<Self> {
    let publick_key: PublicKey = key.public();
    let peer_id: PeerId = publick_key.to_peer_id();

    let gossipsub_behaviour: gossipsub::Behaviour = {
      let gossipsub_config: gossipsub::Config = gossipsub::ConfigBuilder::default()
      .validation_mode(ValidationMode::Strict)
      .build()?;
      let privacy: MessageAuthenticity = MessageAuthenticity::Signed(key.clone());
      gossipsub::Behaviour::new(privacy, gossipsub_config).unwrap()
    };

    let identify_behaviour: identify::Behaviour = {
      let identify_config: identify::Config = identify::Config::new(
        String::from("polkadot/1.0.0"),
        publick_key,
      )
      .with_interval(Duration::from_secs(1));

      identify::Behaviour::new(identify_config)
    };

    let kademlia_behaviour: kad::Behaviour<MemoryStore> = {
      let mut kademlia_config: kad::Config = kad::Config::new(PROTOCOL_NAME);
      kademlia_config.set_query_timeout(Duration::from_secs(u64::MAX));
      kademlia_config.set_publication_interval(Some(Duration::from_secs(1)));
      kademlia_config.set_replication_interval(Some(Duration::from_secs(1)));
      let store: MemoryStore = MemoryStore::new(peer_id);
      kad::Behaviour::with_config(peer_id, store, kademlia_config)
    };

    Ok(Self::new(
      gossipsub_behaviour,
      identify_behaviour,
      kademlia_behaviour,
    ))
  }
}
