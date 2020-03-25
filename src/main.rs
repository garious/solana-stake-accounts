mod args;
mod stake_accounts;

use crate::args::parse_args;
use crate::stake_accounts::{move_stake_account, TransferStakeKeys};
use solana_client::thin_client::create_client;
use solana_core::{cluster_info::VALIDATOR_PORT_RANGE, gossip_service::discover_cluster};
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use std::env;

fn main() {
    let config = parse_args(env::args_os());
    let (nodes, _) = discover_cluster(&config.entrypoint_addr, config.num_nodes).unwrap();
    let mut target = None;
    for node in &nodes {
        if node.gossip == config.entrypoint_addr {
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
