mod stake_accounts;

use crate::stake_accounts::{move_stake_account, TransferStakeKeys};
use clap::{value_t, App, Arg, ArgMatches, SubCommand};
use solana_clap_utils::{
    input_validators::{is_valid_pubkey, is_valid_signer},
    ArgConstant,
};
use solana_client::thin_client::create_client;
use solana_core::{cluster_info::VALIDATOR_PORT_RANGE, gossip_service::discover_cluster};
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use std::env;
use std::ffi::OsString;
use std::net::SocketAddr;
use std::process::exit;

pub const WITHDRAW_AUTHORITY_ARG: ArgConstant<'static> = ArgConstant {
    name: "withdraw_authority",
    long: "withdraw-authority",
    help: "Authorized withdrawer",
};

fn fee_payer_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("fee_payer")
        .long("fee-payer")
        .takes_value(true)
        .value_name("KEYPAIR")
        .validator(is_valid_signer)
        .help("Fee payer")
}

fn stake_authority_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("stake_authority")
        .long("stake-authority")
        .takes_value(true)
        .value_name("KEYPAIR")
        .validator(is_valid_signer)
        .help("Stake authority")
}

fn withdraw_authority_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("withdraw_authority")
        .long("withdraw-authority")
        .takes_value(true)
        .value_name("KEYPAIR")
        .validator(is_valid_signer)
        .help("Withdraw authority")
}

fn new_stake_authority_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("new_stake_authority")
        .long("new-stake-authority")
        .takes_value(true)
        .value_name("PUBKEY")
        .validator(is_valid_pubkey)
        .help("New stake authority")
}

fn new_withdraw_authority_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("new_withdraw_authority")
        .long("new-withdraw-authority")
        .takes_value(true)
        .value_name("PUBKEY")
        .validator(is_valid_pubkey)
        .help("New withdraw authority")
}

fn num_accounts_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("num_accounts")
        .long("num-accounts")
        .takes_value(true)
        .value_name("NUMBER")
        .help("Number of derived stake accounts")
}

fn parse_args<'a, I, T>(args: I) -> ArgMatches<'a>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    App::new("solana-stake-accounts")
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
                .long("num-nodes")
                .takes_value(true)
                .value_name("NUMBER")
                .help("Number of gossip nodes to look for"),
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("Create derived stake accounts")
                .arg(fee_payer_arg())
                .arg(
                    Arg::with_name("sender_keypair")
                        .required(true)
                        .index(1)
                        .takes_value(true)
                        .value_name("SENDER_KEYPAIR")
                        .validator(is_valid_signer)
                        .help("Keypair to fund accounts"),
                )
                .arg(
                    Arg::with_name("base_keypar")
                        .required(true)
                        .index(2)
                        .takes_value(true)
                        .value_name("BASE_KEYPAIR")
                        .validator(is_valid_signer)
                        .help("Keypair which stake account addresses are derived from"),
                )
                .arg(
                    Arg::with_name("amount")
                        .required(true)
                        .index(3)
                        .takes_value(true)
                        .value_name("AMOUNT")
                        .help("Amount to move into the new stake accounts, in SOL"),
                )
                .arg(
                    Arg::with_name("stake_authority")
                        .long("stake-authority")
                        .takes_value(true)
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .help("Stake authority"),
                )
                .arg(
                    Arg::with_name("withdraw_authority")
                        .long("withdraw-authority")
                        .takes_value(true)
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .help("Withdraw authority"),
                )
                .arg(
                    Arg::with_name("cliff_fraction")
                        .long("cliff-fraction")
                        .takes_value(true)
                        .value_name("PERCENTAGE")
                        .help("Percentage of stake to unlock in the first derived stake account"),
                )
                .arg(
                    Arg::with_name("cliff_years")
                        .long("cliff-years")
                        .takes_value(true)
                        .value_name("NUMBER")
                        .help("Years until first unlock"),
                )
                .arg(
                    Arg::with_name("unlock_years")
                        .long("unlock-years")
                        .takes_value(true)
                        .value_name("NUMBER")
                        .help("Years between unlocks after cliff"),
                )
                .arg(
                    Arg::with_name("unlocks")
                        .long("unlocks")
                        .takes_value(true)
                        .value_name("NUMBER")
                        .help(
                            "Number of unlocks after cliff; one derived stake account per unlock",
                        ),
                )
                .arg(
                    Arg::with_name("custodian")
                        .long("custodian")
                        .takes_value(true)
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .help("Authority to set lockups"),
                ),
        )
        .subcommand(
            SubCommand::with_name("balance")
                .about("Sum balances of all derived stake accounts")
                .arg(
                    Arg::with_name("base_pubkey")
                        .required(true)
                        .index(1)
                        .takes_value(true)
                        .value_name("BASE_PUBKEY")
                        .validator(is_valid_pubkey)
                        .help("Public key which stake account addresses are derived from"),
                )
                .arg(num_accounts_arg()),
        )
        .subcommand(
            SubCommand::with_name("pubkeys")
                .about("Show public keys of all derived stake accounts")
                .arg(
                    Arg::with_name("base_pubkey")
                        .required(true)
                        .index(1)
                        .takes_value(true)
                        .value_name("BASE_PUBKEY")
                        .validator(is_valid_pubkey)
                        .help("Public key which stake account addresses are derived from"),
                )
                .arg(num_accounts_arg()),
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("Show all derived stake accounts")
                .arg(
                    Arg::with_name("base_pubkey")
                        .required(true)
                        .index(1)
                        .takes_value(true)
                        .value_name("BASE_PUBKEY")
                        .validator(is_valid_pubkey)
                        .help("Public key which stake account addresses are derived from"),
                )
                .arg(num_accounts_arg()),
        )
        .subcommand(
            SubCommand::with_name("withdraw")
                .about("Withdraw SOL from a derived stake account")
                .arg(fee_payer_arg())
                .arg(
                    Arg::with_name("base_pubkey")
                        .required(true)
                        .index(1)
                        .takes_value(true)
                        .value_name("BASE_PUBKEY")
                        .validator(is_valid_pubkey)
                        .help("Public key which stake account addresses are derived from"),
                )
                .arg(
                    Arg::with_name("recipient_account_address")
                        .required(true)
                        .index(2)
                        .takes_value(true)
                        .value_name("RECIPIENT_ACCOUNT_ADDRESS")
                        .validator(is_valid_pubkey)
                        .help("Recipient account address"),
                )
                .arg(
                    Arg::with_name("amount")
                        .required(true)
                        .index(3)
                        .takes_value(true)
                        .value_name("AMOUNT")
                        .help("Amount to withdraw, in SOL"),
                )
                .arg(
                    Arg::with_name("index")
                        .long("index")
                        .required(true)
                        .takes_value(true)
                        .value_name("NUMBER")
                        .help("Index of derived stake account to withdraw from"),
                )
                .arg(stake_authority_arg()),
        )
        .subcommand(
            SubCommand::with_name("rebase")
                .about("Move derived stake accounts to a new location")
                .arg(fee_payer_arg())
                .arg(
                    Arg::with_name("base_pubkey")
                        .required(true)
                        .index(1)
                        .takes_value(true)
                        .value_name("BASE_PUBKEY")
                        .validator(is_valid_pubkey)
                        .help("Public key which stake account addresses are derived from"),
                )
                .arg(
                    Arg::with_name("new_base_keypair")
                        .required(true)
                        .index(2)
                        .takes_value(true)
                        .value_name("NEW_BASE_KEYPAIR")
                        .validator(is_valid_signer)
                        .help("New keypair which stake account addresses are derived from"),
                )
                .arg(stake_authority_arg())
                .arg(num_accounts_arg()),
        )
        .subcommand(
            SubCommand::with_name("authorize")
                .about("Set new authorities in all derived stake accounts")
                .arg(fee_payer_arg())
                .arg(
                    Arg::with_name("base_pubkey")
                        .required(true)
                        .index(1)
                        .takes_value(true)
                        .value_name("BASE_PUBKEY")
                        .validator(is_valid_pubkey)
                        .help("Public key which stake account addresses are derived from"),
                )
                .arg(stake_authority_arg())
                .arg(withdraw_authority_arg())
                .arg(new_stake_authority_arg())
                .arg(new_withdraw_authority_arg())
                .arg(num_accounts_arg()),
        )
        .subcommand(
            SubCommand::with_name("move")
                .about("Rebase and set new authorities in all derived stake accounts")
                .arg(fee_payer_arg())
                .arg(
                    Arg::with_name("base_pubkey")
                        .required(true)
                        .index(1)
                        .takes_value(true)
                        .value_name("BASE_PUBKEY")
                        .validator(is_valid_pubkey)
                        .help("Public key which stake account addresses are derived from"),
                )
                .arg(stake_authority_arg())
                .arg(withdraw_authority_arg())
                .arg(new_stake_authority_arg())
                .arg(new_withdraw_authority_arg())
                .arg(num_accounts_arg()),
        )
        .get_matches_from(args)
}

fn main() {
    let matches = parse_args(env::args_os());
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
