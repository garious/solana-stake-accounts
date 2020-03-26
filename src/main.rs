mod args;
mod stake_accounts;

use crate::args::{parse_args, Command};
use crate::stake_accounts::{move_stake_account, TransferStakeKeys};
use solana_cli_config::Config;
use solana_client::thin_client::create_client;
use solana_core::{cluster_info::VALIDATOR_PORT_RANGE, gossip_service::discover_cluster};
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use std::env;
use std::net::SocketAddr;
use url::Url;

fn compute_entrypoint_addr(json_rpc_url: &str) -> Vec<SocketAddr> {
    let mut url = json_rpc_url.parse::<Url>().unwrap();
    url.set_port(Some(8001)).unwrap();
    url.socket_addrs(|| None).unwrap()
}

// TODO: This is unreliable when pointed at http://api.mainnet-beta.solana.com
fn find_target(gossip_addrs: &[SocketAddr], num_nodes: usize) -> Option<(SocketAddr, SocketAddr)> {
    for gossip_addr in gossip_addrs {
        let (nodes, _) = discover_cluster(&gossip_addr, num_nodes).unwrap();
        for node in &nodes {
            if node.gossip == *gossip_addr {
                return Some(node.client_facing_addr());
            }
        }
    }
    None
}

fn main() {
    let command_config = parse_args(env::args_os());
    let config = Config::load(&command_config.config_file).unwrap();
    let json_rpc_url = command_config.url.unwrap_or(config.json_rpc_url);
    let gossip_addrs = compute_entrypoint_addr(&json_rpc_url);
    let target = find_target(&gossip_addrs, command_config.num_nodes).expect("should have target");
    let client = create_client(target, VALIDATOR_PORT_RANGE);

    match command_config.command {
        Command::Move(_) => {
            let keys = TransferStakeKeys {
                stake_authority_keypair: Keypair::new(),
                withdraw_authority_keypair: Keypair::new(),
                new_stake_authority_pubkey: Pubkey::default(),
                new_withdraw_authority_pubkey: Pubkey::default(),
            };
            move_stake_account(&client, &keys).unwrap();
        }
        _ => todo!(),
    }
}
