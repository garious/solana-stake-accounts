mod args;
mod stake_accounts;

use crate::args::{parse_args, AuthorizeCommandConfig, Command};
use crate::stake_accounts::{
    authorize_stake_accounts, derive_stake_account_addresses, move_stake_accounts,
    TransferStakeKeys,
};
use clap::ArgMatches;
use solana_clap_utils::keypair::{pubkey_from_path, signer_from_path};
use solana_cli_config::Config;
use solana_client::rpc_client::RpcClient;
use solana_remote_wallet::remote_wallet::{maybe_wallet_manager, RemoteWalletManager};
use solana_sdk::native_token::lamports_to_sol;
use std::env;
use std::error::Error;
use std::sync::Arc;

fn create_transfer_stake_keys(
    authorize_config: &AuthorizeCommandConfig,
    wallet_manager: Option<&Arc<RemoteWalletManager>>,
) -> Result<TransferStakeKeys, Box<dyn Error>> {
    let matches = ArgMatches::default();
    let stake_authority_keypair = signer_from_path(
        &matches,
        authorize_config.stake_authority.as_ref().unwrap(),
        "stake authority",
        wallet_manager,
    )?;
    let withdraw_authority_keypair = signer_from_path(
        &matches,
        authorize_config.withdraw_authority.as_ref().unwrap(),
        "withdraw authority",
        wallet_manager,
    )?;
    let new_stake_authority_pubkey = pubkey_from_path(
        &matches,
        authorize_config.new_stake_authority.as_ref().unwrap(),
        "new stake authority",
        wallet_manager,
    )?;
    let new_withdraw_authority_pubkey = pubkey_from_path(
        &matches,
        authorize_config.new_withdraw_authority.as_ref().unwrap(),
        "new withdraw authority",
        wallet_manager,
    )?;
    let fee_payer_keypair = signer_from_path(
        &matches,
        authorize_config.fee_payer.as_ref().unwrap(),
        "fee-payer",
        wallet_manager,
    )?;
    let keys = TransferStakeKeys {
        stake_authority_keypair,
        withdraw_authority_keypair,
        fee_payer_keypair,
        new_stake_authority_pubkey,
        new_withdraw_authority_pubkey,
    };
    Ok(keys)
}

fn main() -> Result<(), Box<dyn Error>> {
    let command_config = parse_args(env::args_os());
    let config = Config::load(&command_config.config_file)?;
    let json_rpc_url = command_config.url.unwrap_or(config.json_rpc_url);
    let client = RpcClient::new(json_rpc_url);

    let matches = ArgMatches::default();
    let wallet_manager = maybe_wallet_manager()?;
    let wallet_manager = wallet_manager.as_ref();
    match command_config.command {
        Command::Pubkeys(query_config) => {
            let base_pubkey = pubkey_from_path(
                &matches,
                &query_config.base_pubkey,
                "base pubkey",
                wallet_manager,
            )?;
            let pubkeys =
                derive_stake_account_addresses(&client, &base_pubkey, query_config.num_accounts);
            for pubkey in pubkeys {
                println!("{:?}", pubkey);
            }
        }
        Command::Balance(query_config) => {
            let base_pubkey = pubkey_from_path(
                &matches,
                &query_config.base_pubkey,
                "base pubkey",
                wallet_manager,
            )?;
            let pubkeys =
                derive_stake_account_addresses(&client, &base_pubkey, query_config.num_accounts);
            let sum: u64 = pubkeys
                .iter()
                .map(|pubkey| client.get_balance(&pubkey).unwrap())
                .sum();
            println!("{} SOL", lamports_to_sol(sum));
        }
        Command::Authorize(authorize_config) => {
            let keys = create_transfer_stake_keys(&authorize_config, wallet_manager)?;
            authorize_stake_accounts(&client, &keys, authorize_config.num_accounts)?;
        }
        Command::Move(move_config) => {
            let authorize_config = &move_config.authorize_config;
            let keys = create_transfer_stake_keys(&authorize_config, wallet_manager)?;
            move_stake_accounts(&client, &keys, authorize_config.num_accounts)?;
        }
        _ => todo!(),
    }
    Ok(())
}
