mod stake_accounts;

use crate::stake_accounts::{move_stake_account, TransferStakeKeys};
use solana_client::thin_client::create_client;
use solana_core::{cluster_info::VALIDATOR_PORT_RANGE, gossip_service::discover_cluster};
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

fn main() {
    //if let Some(addr) = matches.value_of("entrypoint") {
    //    entrypoint_addr = solana_net_utils::parse_host_port(addr).unwrap_or_else(|e| {
    //        eprintln!("failed to parse entrypoint address: {}", e);
    //        exit(1)
    //    });
    //}
    //let num_nodes = value_t!(matches, "num_nodes", usize).unwrap_or(1);
    let entrypoint_addr = solana_net_utils::parse_host_port("devnet.solana.com:8001").unwrap();
    let num_nodes = 1;

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
