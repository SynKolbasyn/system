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


use crate::net::{
  behaviour::{Behaviour, BehaviourEvent},
  server_list::ServerList,
};


use std::{
  time::Duration,
  net::{Ipv4Addr, Ipv6Addr},
  path::Path,
};

use anyhow::{Result, Context};
use ssh_key::{PrivateKey, Algorithm, rand_core::OsRng};
use libp2p::{
  futures::StreamExt,
  swarm::{Swarm, SwarmEvent, Config},
  SwarmBuilder,
  tcp,
  tls,
  noise,
  yamux,
  multiaddr::{Multiaddr, Protocol},
  gossipsub::{self, Topic, Sha256Topic, Message},
  identify,
  kad,
  identity::Keypair,
};


pub(crate) async fn server_main() -> Result<()> {
  loop {
    match main_loop().await {
      Ok(_) => break,
      Err(error) => eprintln!("CRITICAL SERVER ERROR: {error}"),
    }
  }
  Ok(())
}


async fn main_loop() -> Result<()> {
  let key: PrivateKey = PrivateKey::random(&mut OsRng, Algorithm::Ed25519)?;
  let key: Keypair = Keypair::ed25519_from_bytes(key.key_data().ed25519().context("Key type is not ed25519")?.private.to_bytes())?;

  let behaviour: Behaviour = Behaviour::from_key(key.clone())?;

  let mut swarm: Swarm<Behaviour> = SwarmBuilder::with_existing_identity(key.clone())
  .with_tokio()
  .with_tcp(
    tcp::Config::default(),
    (tls::Config::new, noise::Config::new),
    yamux::Config::default,
  )?
  .with_quic()
  .with_dns()?
  .with_behaviour(|_| -> Behaviour {
    behaviour
  })?
  .with_swarm_config(|config: Config| -> Config {
    config.with_idle_connection_timeout(Duration::from_secs(u64::MAX))
  })
  .build();

  let addr_v4_tcp = Multiaddr::empty()
  .with(Protocol::from(Ipv4Addr::UNSPECIFIED))
  .with(Protocol::Tcp(0));

  let addr_v6_tcp = Multiaddr::empty()
  .with(Protocol::from(Ipv6Addr::UNSPECIFIED))
  .with(Protocol::Tcp(0));
  
  let addr_v4_udp = Multiaddr::empty()
  .with(Protocol::from(Ipv4Addr::UNSPECIFIED))
  .with(Protocol::Udp(0))
  .with(Protocol::QuicV1);

  let addr_v6_udp = Multiaddr::empty()
  .with(Protocol::from(Ipv6Addr::UNSPECIFIED))
  .with(Protocol::Udp(0))
  .with(Protocol::QuicV1);
  
  swarm.listen_on(addr_v4_tcp)?;
  swarm.listen_on(addr_v6_tcp)?;
  swarm.listen_on(addr_v4_udp)?;
  swarm.listen_on(addr_v6_udp)?;

  let mut server_list: ServerList = ServerList::default();
  swarm.behaviour_mut().kademlia.set_mode(Some(kad::Mode::Server));

  let blockchain_topic: Topic<_> = Sha256Topic::new("blockchain");
  let blocks_for_verification_topic: Topic<_> = Sha256Topic::new("blocks_for_verification");
  let verified_blocks_topic: Topic<_> = Sha256Topic::new("verified_blocks");
  swarm.behaviour_mut().gossipsub.subscribe(&blockchain_topic)?;
  swarm.behaviour_mut().gossipsub.subscribe(&blocks_for_verification_topic)?;
  swarm.behaviour_mut().gossipsub.subscribe(&verified_blocks_topic)?;

  loop {
    tokio::select! {
      event = swarm.select_next_some() => match event {
        SwarmEvent::Behaviour(event) => {
          match event {
            BehaviourEvent::Identify(event) => match event {
              identify::Event::Received { peer_id, info: identify::Info { listen_addrs, .. }, .. } => {
                listen_addrs.iter().for_each(|addr: &Multiaddr| {
                  swarm.behaviour_mut().kademlia.add_address(&peer_id, addr.clone());
                });
              },
  
              identify::Event::Sent { .. } => (),
              identify::Event::Pushed { .. } => (),
              identify::Event::Error { .. } => (),
            },

            BehaviourEvent::Kademlia(event) => match event {
              kad::Event::InboundRequest { .. } => (),
              kad::Event::OutboundQueryProgressed { .. } => (),
              kad::Event::RoutingUpdated { .. } => (),
              kad::Event::UnroutablePeer { .. } => (),
              kad::Event::RoutablePeer { .. } => (),
              kad::Event::PendingRoutablePeer { .. } => (),
              kad::Event::ModeChanged { .. } => (),
            },

            BehaviourEvent::Gossipsub(event) => match event {
              gossipsub::Event::Message { message: Message { data, .. } , .. } => println!("{}", String::from_utf8(data)?),
              gossipsub::Event::Subscribed { .. } => (),
              gossipsub::Event::Unsubscribed { .. } => (),
              gossipsub::Event::GossipsubNotSupported { .. } => (),
            },
          }
        },

        SwarmEvent::ConnectionEstablished { .. } => (),
        SwarmEvent::ConnectionClosed { .. } => (),
        SwarmEvent::IncomingConnection { .. } => (),
        SwarmEvent::IncomingConnectionError { .. } => (),
        SwarmEvent::OutgoingConnectionError { .. } => (),

        SwarmEvent::NewListenAddr { listener_id, address } => {
          server_list.add_addr_and_save::<&Path>(None, swarm.local_peer_id().clone(), listener_id, address)?;
        },

        SwarmEvent::ExpiredListenAddr { .. } => (),
        SwarmEvent::ListenerClosed { .. } => (),
        SwarmEvent::ListenerError { .. } => (),
        SwarmEvent::Dialing { .. } => (),
        SwarmEvent::NewExternalAddrCandidate { .. } => (),
        SwarmEvent::ExternalAddrConfirmed { .. } => (),
        SwarmEvent::ExternalAddrExpired { .. } => (),
        SwarmEvent::NewExternalAddrOfPeer { .. } => (),

        _ => (),
      },

      // Ok(Some(line)) = stdin.next_line() => {
      //   swarm.behaviour_mut().gossipsub.publish(topic.clone(), line)?;
      // },
    }
  }
}
