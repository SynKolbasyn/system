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


pub(crate) mod server;
mod behaviour;
mod server_list;
mod send_data;
pub(crate) mod api;


use std::{
  net::{Ipv4Addr, Ipv6Addr},
  time::Duration,
  env::args,
};

use anyhow::{Result, Context};
use api::API;
use ssh_key::PrivateKey;
use tokio::{
  task::{self, JoinHandle},
  sync::watch::{Sender, Receiver, channel},
};
use libp2p::{
  futures::StreamExt,
  gossipsub::{Topic, Sha256Topic, Message},
  identity::Keypair,
  multiaddr::Protocol,
  noise,
  swarm::{Config, Swarm, SwarmEvent},
  tcp,
  tls,
  yamux,
  Multiaddr,
  SwarmBuilder,
  kad,
  gossipsub,
  identify,
};

use crate::net::{
  behaviour::{Behaviour, BehaviourEvent},
  server_list::ServerList,
  send_data::SendData,
};


pub(crate) struct Net {
  swarm: Swarm<Behaviour>,
  command_receiver: Receiver<SendData>,
}


impl Net {
  fn new(swarm: Swarm<Behaviour>, command_receiver: Receiver<SendData>) -> Self {
    Self {
      swarm,
      command_receiver,
    }
  }


  pub(crate) fn start(mut self) -> JoinHandle<Result<()>> {
    task::spawn(async move {
      loop {
        tokio::select! {
          event = self.swarm.select_next_some() => match event {
            SwarmEvent::Behaviour(event) => {
              match event {
                BehaviourEvent::Identify(event) => match event {
                  identify::Event::Received { peer_id, info: identify::Info { listen_addrs, .. }, .. } => {
                    listen_addrs.iter().for_each(|addr: &Multiaddr| {
                      self.swarm.behaviour_mut().kademlia.add_address(&peer_id, addr.clone());
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
            SwarmEvent::NewListenAddr { .. } => (),
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

          _ = self.command_receiver.changed() => {
            let data = self.command_receiver.borrow_and_update();
            println!("{:#?}", data.topic);
          },
        }
      }
    })
  }


  pub(crate) fn from_key(key: &PrivateKey) -> Result<API> {
    let key_bytes: [u8; 32] = key.key_data().ed25519().context("The key was not generated using the ed25519 algorithm")?.private.to_bytes();
    let key: Keypair = Keypair::ed25519_from_bytes(key_bytes)?;

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

    if let Ok(server_list) = ServerList::from_default_path() {
      for (peer_id, addresses) in server_list.addresses.iter() {
        addresses.values().for_each(|addr: &Multiaddr| {
          swarm.behaviour_mut().kademlia.add_address(peer_id, addr.clone());
        });
      }
    }
    else {
      let server_id: String = args().nth(1).context("Server ID isn't provided")?;
      let server_ip: String = args().nth(2).context("Server IP isn't provided")?;
      swarm.behaviour_mut().kademlia.add_address(&server_id.parse()?, server_ip.parse()?);
    }
    swarm.behaviour_mut().kademlia.bootstrap()?;

    let blockchain_topic: Topic<_> = Sha256Topic::new("blockchain");
    let blocks_for_verification_topic: Topic<_> = Sha256Topic::new("blocks_for_verification");
    let verified_blocks_topic: Topic<_> = Sha256Topic::new("verified_blocks");
    swarm.behaviour_mut().gossipsub.subscribe(&blockchain_topic)?;
    swarm.behaviour_mut().gossipsub.subscribe(&blocks_for_verification_topic)?;
    swarm.behaviour_mut().gossipsub.subscribe(&verified_blocks_topic)?;

    let (sender, receiver): (Sender<SendData>, Receiver<SendData>) = channel(SendData::default());
    let net: Self = Self::new(swarm, receiver);
    Ok(API::new(net.start(), sender))
  }
}
