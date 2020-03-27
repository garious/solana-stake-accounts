use clap::{value_t, value_t_or_exit, App, Arg, ArgMatches, SubCommand};
use solana_clap_utils::input_validators::{is_amount, is_valid_pubkey, is_valid_signer};
use solana_cli_config::CONFIG_FILE;
use solana_sdk::native_token::sol_to_lamports;
use std::ffi::OsString;
use std::process::exit;

pub(crate) struct DepositCommandConfig {
    pub fee_payer: Option<String>,
    pub sender_keypair: String,
    pub lamports: u64,
    pub base_pubkey: String,
    pub cliff_fraction: Option<f64>,
    pub cliff_years: Option<f64>,
    pub unlock_years: Option<f64>,
    pub unlocks: Option<u64>,
}

pub(crate) struct NewCommandConfig {
    pub base_keypair: String,
    pub stake_authority: Option<String>,
    pub withdraw_authority: Option<String>,
    pub custodian: Option<String>,
    pub deposit_config: DepositCommandConfig,
}

pub(crate) struct QueryCommandConfig {
    pub base_pubkey: String,
    pub num_accounts: Option<usize>,
}

pub(crate) struct WithdrawCommandConfig {
    pub base_pubkey: String,
    pub recipient_account_address: String,
    pub lamports: u64,
    pub index: Option<usize>,
    pub withdraw_authority: Option<String>,
}

pub(crate) struct RebaseCommandConfig {
    pub fee_payer: Option<String>,
    pub base_pubkey: String,
    pub new_base_keypair: String,
    pub stake_authority: Option<String>,
    pub num_accounts: Option<usize>,
}

pub(crate) struct AuthorizeCommandConfig {
    pub fee_payer: Option<String>,
    pub base_pubkey: String,
    pub stake_authority: Option<String>,
    pub withdraw_authority: Option<String>,
    pub new_stake_authority: Option<String>,
    pub new_withdraw_authority: Option<String>,
    pub num_accounts: Option<usize>,
}

pub(crate) struct MoveCommandConfig {
    pub rebase_config: RebaseCommandConfig,
    pub authorize_config: AuthorizeCommandConfig,
}

pub(crate) enum Command {
    New(NewCommandConfig),
    Deposit(DepositCommandConfig),
    Balance(QueryCommandConfig),
    Pubkeys(QueryCommandConfig),
    Show(QueryCommandConfig),
    Withdraw(WithdrawCommandConfig),
    Rebase(RebaseCommandConfig),
    Authorize(AuthorizeCommandConfig),
    Move(MoveCommandConfig),
}

pub(crate) struct CommandConfig {
    pub config_file: String,
    pub url: Option<String>,
    pub command: Command,
}

fn fee_payer_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("fee_payer")
        .long("fee-payer")
        .required(true)
        .takes_value(true)
        .value_name("KEYPAIR")
        .validator(is_valid_signer)
        .help("Fee payer")
}

fn sender_keypair_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("sender_keypair")
        .required(true)
        .takes_value(true)
        .value_name("SENDER_KEYPAIR")
        .validator(is_valid_signer)
        .help("Keypair to fund accounts")
}

fn base_pubkey_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("base_pubkey")
        .required(true)
        .takes_value(true)
        .value_name("BASE_PUBKEY")
        .validator(is_valid_pubkey)
        .help("Public key which stake account addresses are derived from")
}

fn new_base_keypair_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("new_base_keypair")
        .required(true)
        .takes_value(true)
        .value_name("NEW_BASE_KEYPAIR")
        .validator(is_valid_signer)
        .help("New keypair which stake account addresses are derived from")
}

fn stake_authority_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("stake_authority")
        .long("stake-authority")
        .required(true)
        .takes_value(true)
        .value_name("KEYPAIR")
        .validator(is_valid_signer)
        .help("Stake authority")
}

fn withdraw_authority_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("withdraw_authority")
        .long("withdraw-authority")
        .required(true)
        .takes_value(true)
        .value_name("KEYPAIR")
        .validator(is_valid_signer)
        .help("Withdraw authority")
}

fn new_stake_authority_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("new_stake_authority")
        .long("new-stake-authority")
        .required(true)
        .takes_value(true)
        .value_name("PUBKEY")
        .validator(is_valid_pubkey)
        .help("New stake authority")
}

fn new_withdraw_authority_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("new_withdraw_authority")
        .long("new-withdraw-authority")
        .required(true)
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

fn cliff_fraction_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("cliff_fraction")
        .long("cliff-fraction")
        .takes_value(true)
        .value_name("PERCENTAGE")
        .help("Percentage of stake to unlock in the first derived stake account")
}

fn cliff_years_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("cliff_years")
        .long("cliff-years")
        .takes_value(true)
        .value_name("NUMBER")
        .help("Years until first unlock")
}

fn unlock_years_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("unlock_years")
        .long("unlock-years")
        .takes_value(true)
        .value_name("NUMBER")
        .help("Years between unlocks after cliff")
}

fn unlocks_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("unlocks")
        .long("unlocks")
        .takes_value(true)
        .value_name("NUMBER")
        .help("Number of unlocks after cliff; one derived stake account per unlock")
}
pub(crate) fn get_matches<'a, I, T>(args: I) -> ArgMatches<'a>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let default_config_file = CONFIG_FILE.as_ref().unwrap();
    App::new("solana-stake-accounts")
        .about("about")
        .version("version")
        .arg(
            Arg::with_name("config_file")
                .long("config")
                .takes_value(true)
                .value_name("FILEPATH")
                .default_value(default_config_file)
                .help("Config file"),
        )
        .arg(
            Arg::with_name("url")
                .long("url")
                .global(true)
                .takes_value(true)
                .value_name("http://<HOST>:<PORT>")
                .help("RPC entrypoint address. i.e. http://devnet.solana.com"),
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("Create derived stake accounts")
                .arg(fee_payer_arg())
                .arg(sender_keypair_arg().index(1))
                .arg(
                    Arg::with_name("base_keypair")
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
                        .validator(is_amount)
                        .help("Amount to move into the new stake accounts, in SOL"),
                )
                .arg(
                    Arg::with_name("stake_authority")
                        .long("stake-authority")
                        .required(true)
                        .takes_value(true)
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .help("Stake authority"),
                )
                .arg(
                    Arg::with_name("withdraw_authority")
                        .long("withdraw-authority")
                        .required(true)
                        .takes_value(true)
                        .value_name("PUBKEY")
                        .validator(is_valid_pubkey)
                        .help("Withdraw authority"),
                )
                .arg(cliff_fraction_arg())
                .arg(cliff_years_arg())
                .arg(unlock_years_arg())
                .arg(unlocks_arg())
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
            SubCommand::with_name("deposit")
                .about("Add funds to existing stake accounts")
                .arg(fee_payer_arg())
                .arg(sender_keypair_arg().index(1))
                .arg(base_pubkey_arg().index(2))
                .arg(
                    Arg::with_name("amount")
                        .required(true)
                        .index(3)
                        .takes_value(true)
                        .value_name("AMOUNT")
                        .validator(is_amount)
                        .help("Amount to move into the new stake accounts, in SOL"),
                )
                .arg(cliff_fraction_arg())
                .arg(cliff_years_arg())
                .arg(unlock_years_arg())
                .arg(unlocks_arg()),
        )
        .subcommand(
            SubCommand::with_name("balance")
                .about("Sum balances of all derived stake accounts")
                .arg(base_pubkey_arg().index(1))
                .arg(num_accounts_arg()),
        )
        .subcommand(
            SubCommand::with_name("pubkeys")
                .about("Show public keys of all derived stake accounts")
                .arg(base_pubkey_arg().index(1))
                .arg(num_accounts_arg()),
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("Show all derived stake accounts")
                .arg(base_pubkey_arg().index(1))
                .arg(num_accounts_arg()),
        )
        .subcommand(
            SubCommand::with_name("withdraw")
                .about("Withdraw SOL from a derived stake account")
                .arg(fee_payer_arg())
                .arg(base_pubkey_arg().index(1))
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
                        .validator(is_amount)
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
                .arg(withdraw_authority_arg()),
        )
        .subcommand(
            SubCommand::with_name("rebase")
                .about("Move derived stake accounts to a new location")
                .arg(fee_payer_arg())
                .arg(base_pubkey_arg().index(1))
                .arg(new_base_keypair_arg().index(2))
                .arg(stake_authority_arg())
                .arg(num_accounts_arg()),
        )
        .subcommand(
            SubCommand::with_name("authorize")
                .about("Set new authorities in all derived stake accounts")
                .arg(fee_payer_arg())
                .arg(base_pubkey_arg().index(1))
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
                .arg(base_pubkey_arg().index(1))
                .arg(new_base_keypair_arg().index(2))
                .arg(stake_authority_arg())
                .arg(withdraw_authority_arg())
                .arg(new_stake_authority_arg())
                .arg(new_withdraw_authority_arg())
                .arg(num_accounts_arg()),
        )
        .get_matches_from(args)
}

fn parse_deposit_args(matches: &ArgMatches<'_>) -> DepositCommandConfig {
    let lamports = sol_to_lamports(value_t_or_exit!(matches, "amount", f64));
    let fee_payer = value_t!(matches, "fee_payer", String).ok();
    let sender_keypair = value_t_or_exit!(matches, "sender_keypair", String);
    let base_pubkey = value_t_or_exit!(matches, "base_pubkey", String);
    DepositCommandConfig {
        fee_payer,
        sender_keypair,
        lamports,
        base_pubkey,
        cliff_fraction: None,
        cliff_years: None,
        unlock_years: None,
        unlocks: None,
    }
}

fn parse_new_args(matches: &ArgMatches<'_>) -> NewCommandConfig {
    let base_keypair = value_t_or_exit!(matches, "base_keypair", String);
    let stake_authority = value_t!(matches, "stake_authority", String).ok();
    let withdraw_authority = value_t!(matches, "withdraw_authority", String).ok();
    let custodian = value_t!(matches, "custodian", String).ok();
    let deposit_config = parse_deposit_args(matches);
    NewCommandConfig {
        base_keypair,
        stake_authority,
        withdraw_authority,
        custodian,
        deposit_config,
    }
}

fn parse_query_args(matches: &ArgMatches<'_>) -> QueryCommandConfig {
    let base_pubkey = value_t_or_exit!(matches, "base_pubkey", String);
    let num_accounts = value_t!(matches, "num_accounts", usize).ok();
    QueryCommandConfig {
        base_pubkey,
        num_accounts,
    }
}

fn parse_withdraw_args(matches: &ArgMatches<'_>) -> WithdrawCommandConfig {
    let base_pubkey = value_t_or_exit!(matches, "base_pubkey", String);
    let recipient_account_address = value_t_or_exit!(matches, "recipient_account_address", String);
    let withdraw_authority = value_t!(matches, "withdraw_authority", String).ok();
    let lamports = sol_to_lamports(value_t_or_exit!(matches, "amount", f64));
    let index = value_t!(matches, "index", usize).ok();
    WithdrawCommandConfig {
        base_pubkey,
        recipient_account_address,
        lamports,
        index,
        withdraw_authority,
    }
}

fn parse_rebase_args(matches: &ArgMatches<'_>) -> RebaseCommandConfig {
    let fee_payer = value_t!(matches, "fee_payer", String).ok();
    let base_pubkey = value_t_or_exit!(matches, "base_pubkey", String);
    let new_base_keypair = value_t_or_exit!(matches, "new_base_keypair", String);
    let stake_authority = value_t!(matches, "stake_authority", String).ok();
    let num_accounts = value_t!(matches, "num_accounts", usize).ok();
    RebaseCommandConfig {
        fee_payer,
        base_pubkey,
        new_base_keypair,
        stake_authority,
        num_accounts,
    }
}

fn parse_authorize_args(matches: &ArgMatches<'_>) -> AuthorizeCommandConfig {
    let fee_payer = value_t!(matches, "fee_payer", String).ok();
    let base_pubkey = value_t_or_exit!(matches, "base_pubkey", String);
    let stake_authority = value_t!(matches, "stake_authority", String).ok();
    let withdraw_authority = value_t!(matches, "withdraw_authority", String).ok();
    let new_stake_authority = value_t!(matches, "new_stake_authority", String).ok();
    let new_withdraw_authority = value_t!(matches, "new_withdraw_authority", String).ok();
    let num_accounts = value_t!(matches, "num_accounts", usize).ok();
    AuthorizeCommandConfig {
        fee_payer,
        base_pubkey,
        stake_authority,
        withdraw_authority,
        new_stake_authority,
        new_withdraw_authority,
        num_accounts,
    }
}

fn parse_move_args(matches: &ArgMatches<'_>) -> MoveCommandConfig {
    let rebase_config = parse_rebase_args(matches);
    let authorize_config = parse_authorize_args(matches);
    MoveCommandConfig {
        rebase_config,
        authorize_config,
    }
}

pub(crate) fn parse_args<'a, I, T>(args: I) -> CommandConfig
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let matches = get_matches(args);
    let config_file = matches.value_of("config_file").unwrap().to_string();
    let url = matches.value_of("url").map(|x| x.to_string());

    let command = match matches.subcommand() {
        ("new", Some(matches)) => Command::New(parse_new_args(matches)),
        ("deposit", Some(matches)) => Command::Deposit(parse_deposit_args(matches)),
        ("balance", Some(matches)) => Command::Balance(parse_query_args(matches)),
        ("pubkeys", Some(matches)) => Command::Pubkeys(parse_query_args(matches)),
        ("show", Some(matches)) => Command::Show(parse_query_args(matches)),
        ("withdraw", Some(matches)) => Command::Withdraw(parse_withdraw_args(matches)),
        ("rebase", Some(matches)) => Command::Rebase(parse_rebase_args(matches)),
        ("authorize", Some(matches)) => Command::Authorize(parse_authorize_args(matches)),
        ("move", Some(matches)) => Command::Move(parse_move_args(matches)),
        _ => {
            eprintln!("{}", matches.usage());
            exit(1);
        }
    };
    CommandConfig {
        config_file,
        url,
        command,
    }
}
