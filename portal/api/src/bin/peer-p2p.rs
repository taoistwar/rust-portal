use std::time::Duration;

use futures::StreamExt;
use libp2p::{
    noise, ping,
    swarm::{behaviour, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId,
};
use rust_portal_api::new_keypair;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let local_key = new_keypair();

    let mut swarm: libp2p::Swarm<ping::Behaviour> =
        libp2p::SwarmBuilder::with_existing_identity(local_key)
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_behaviour(|_| {
                let config = ping::Config::new();
                ping::Behaviour::new(config)
            })?
            .with_swarm_config(|cfg| {
                cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX))
            })
            .build();

    let address = "/ip4/0.0.0.0/tcp/0".parse()?;
    swarm.listen_on(address)?;

    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {addr}")
    }

    loop {
        let event = swarm.select_next_some().await;
        match event {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
            SwarmEvent::Behaviour(event) => {
                println!("Behaviour: {event:?}");
            }
            SwarmEvent::IncomingConnection {
                connection_id,
                local_addr,
                send_back_addr,
            } => {
                println!("IncomingConnection: connection_id:{connection_id:?} local_addr:{local_addr:?} send_back_addr:{send_back_addr:?}");
            }
            SwarmEvent::ConnectionEstablished {
                peer_id,
                connection_id,
                endpoint,
                num_established,
                ref concurrent_dial_errors,
                established_in,
            } => {
                println!("ConnectionEstablished: peer_id:{peer_id:?}  connection_id:{connection_id:?}  endpoint:{endpoint:?}  num_established:{num_established:?}  concurrent_dial_errors:{concurrent_dial_errors:?}  established_in:{established_in:?}");
            }
            SwarmEvent::ConnectionClosed {
                peer_id,
                connection_id,
                endpoint,
                num_established,
                cause,
            } => {
                println!("ConnectionClosed: peer_id:{peer_id:?}  connection_id:{connection_id:?}  endpoint:{endpoint:?}  num_established:{num_established:?}  cause:{cause:?}");
            }
            x => {
                println!("Other: {x:?}");
            }
        }
    }
    Ok(())
}
