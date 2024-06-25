/* For the P2P network, I'll use libp2p

This P2P implementation uses libp2p to create a basic node that can discover other nodes on the local network using mDNS and communicate with them using the Floodsub protocol. This forms the foundation for direct communication between lenders and borrowers in our system.

*/

use libp2p::{
    core::upgrade,
    floodsub::{Floodsub, FloodsubEvent, Topic},
    mdns::{Mdns, MdnsEvent},
    swarm::{NetworkBehaviourEventProcess, Swarm},
    NetworkBehaviour, PeerId,
};
use log::{error, info};
use std::error::Error;
use tokio::io::AsyncBufReadExt;

// Define the network behavior for our P2P node
#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
struct LendingBehaviour {
    floodsub: Floodsub,
    mdns: Mdns,
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for LendingBehaviour {
    fn inject_event(&mut self, event: FloodsubEvent) {
        if let FloodsubEvent::Message(message) = event {
            println!(
                "Received: '{:?}' from {:?}",
                String::from_utf8_lossy(&message.data),
                message.source
            );
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for LendingBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer, _) in list {
                    self.floodsub.add_node_to_partial_view(peer);
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer, _) in list {
                    if !self.mdns.has_node(&peer) {
                        self.floodsub.remove_node_from_partial_view(&peer);
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Create a random PeerId
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    // Create a transport
    let transport = libp2p::development_transport(local_key).await?;

    // Create a Floodsub topic
    let floodsub_topic = Topic::new("lending");

    // Create a Swarm to manage peers and events
    let mut swarm = {
        let mdns = Mdns::new(Default::default()).await?;
        let mut behaviour = LendingBehaviour {
            floodsub: Floodsub::new(local_peer_id),
            mdns,
        };

        behaviour.floodsub.subscribe(floodsub_topic.clone());
        Swarm::new(transport, behaviour, local_peer_id)
    };

    // Listen on all interfaces and whatever port the OS assigns
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Read full lines from stdin
    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();

    // Kick it off
    loop {
        tokio::select! {
            line = stdin.next_line() => {
                let line = line?.expect("stdin closed");
                swarm.behaviour_mut().floodsub.publish(floodsub_topic.clone(), line.as_bytes());
            }
            event = swarm.select_next_some() => {
                if let SwarmEvent::NewListenAddr { address, .. } = event {
                    println!("Listening on {:?}", address);
                }
            }
        }
    }
}