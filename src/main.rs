mod args;
mod stake_accounts;

use crate::args::{parse_args, Command};
use crate::stake_accounts::{move_stake_account, TransferStakeKeys};
use clap::ArgMatches;
use solana_clap_utils::keypair::{pubkey_from_path, signer_from_path};
use solana_cli_config::Config;
use solana_client::rpc_client::RpcClient;
use solana_remote_wallet::remote_wallet::maybe_wallet_manager;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let command_config = parse_args(env::args_os());
    let config = Config::load(&command_config.config_file).unwrap();
    let json_rpc_url = command_config.url.unwrap_or(config.json_rpc_url);
    let client = RpcClient::new(json_rpc_url);

    let matches = ArgMatches::default();
    let wallet_manager = maybe_wallet_manager()?;
    let wallet_manager = wallet_manager.as_ref();
    match command_config.command {
        Command::Move(move_config) => {
            let authorize_config = &move_config.authorize_config;
            let stake_authority_keypair = signer_from_path(
                &matches,
                authorize_config.stake_authority.as_ref().unwrap(),
                "stake authority",
                wallet_manager,
            )
            .unwrap();
            let withdraw_authority_keypair = signer_from_path(
                &matches,
                authorize_config.withdraw_authority.as_ref().unwrap(),
                "withdraw authority",
                wallet_manager,
            )
            .unwrap();
            let new_stake_authority_pubkey = pubkey_from_path(
                &matches,
                authorize_config.new_stake_authority.as_ref().unwrap(),
                "new stake authority",
                wallet_manager,
            )
            .unwrap();
            let new_withdraw_authority_pubkey = pubkey_from_path(
                &matches,
                authorize_config.new_withdraw_authority.as_ref().unwrap(),
                "new withdraw authority",
                wallet_manager,
            )
            .unwrap();
            let keys = TransferStakeKeys {
                stake_authority_keypair,
                withdraw_authority_keypair,
                new_stake_authority_pubkey,
                new_withdraw_authority_pubkey,
            };
            move_stake_account(&client, &keys).unwrap();
        }
        _ => todo!(),
    }
    Ok(())
}
