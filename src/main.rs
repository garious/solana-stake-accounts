mod stake_accounts;

use crate::stake_accounts::{move_stake_account, TransferStakeKeys};
use clap::{value_t, value_t_or_exit, App, Arg};
use solana_client::thin_client::create_client;
use solana_core::{cluster_info::VALIDATOR_PORT_RANGE, gossip_service::discover_cluster};
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use std::net::SocketAddr;
use std::process::exit;

fn main() {
    let matches = App::new("solana-stake-accounts")
        .about("about")
        .version("version")
        .arg(
            Arg::with_name("entrypoint")
                .long("entrypoint")
                .takes_value(true)
                .value_name("HOST:PORT")
                .help("Gossip entrypoint address. Usually <ip>:8001"),
        )
        .arg(
            Arg::with_name("num_nodes")
                .long("num_nodes")
                .takes_value(true)
                .value_name("NUM")
                .help("Number of gossip nodes to look for."),
        )
        .get_matches();

    let mut entrypoint_addr = SocketAddr::from(([127, 0, 0, 1], 8001));
    if let Some(addr) = matches.value_of("entrypoint") {
        entrypoint_addr = solana_net_utils::parse_host_port(addr).unwrap_or_else(|e| {
            eprintln!("failed to parse entrypoint address: {}", e);
            exit(1)
        });
    }
    let num_nodes = value_t!(matches, "num_nodes", usize).unwrap_or(1);
    let (nodes, _) = discover_cluster(&entrypoint_addr, num_nodes).unwrap();
    let mut target = None;
    for node in &nodes {
        if node.gossip == entrypoint_addr {
            target = Some(node.client_facing_addr());
            break;
        }
    }
    let target = target.expect("should have target");

    let client = create_client(target, VALIDATOR_PORT_RANGE);
    let keys = TransferStakeKeys {
        stake_authority_keypair: Keypair::new(),
        withdraw_authority_keypair: Keypair::new(),
        new_stake_authority_pubkey: Pubkey::default(),
        new_withdraw_authority_pubkey: Pubkey::default(),
    };
    move_stake_account(&client, &keys).unwrap();
}
