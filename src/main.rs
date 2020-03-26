mod args;
mod stake_accounts;

use crate::args::{parse_args, Command};
use crate::stake_accounts::{move_stake_account, TransferStakeKeys};
use solana_cli_config::Config;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use std::env;

fn main() {
    let command_config = parse_args(env::args_os());
    let config = Config::load(&command_config.config_file).unwrap();
    let json_rpc_url = command_config.url.unwrap_or(config.json_rpc_url);
    let client = RpcClient::new(json_rpc_url);

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
