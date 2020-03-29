use solana_client::client_error::ClientError;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    hash::hashv, instruction::Instruction, pubkey::Pubkey, signature::Signer,
    transaction::Transaction,
};
use solana_stake_program::{stake_instruction, stake_state::StakeAuthorize};
use std::error::Error;

pub(crate) struct TransferStakeKeys {
    pub stake_authority_keypair: Box<dyn Signer>,
    pub withdraw_authority_keypair: Box<dyn Signer>,
    pub fee_payer_keypair: Box<dyn Signer>,
    pub new_stake_authority_pubkey: Pubkey,
    pub new_withdraw_authority_pubkey: Pubkey,
}

pub const MAX_SEED_LEN: usize = 32;

#[derive(Debug)]
pub enum PubkeyError {
    MaxSeedLengthExceeded,
}

// Return the number of derived stake accounts with balances
pub fn count_stake_accounts(
    client: &RpcClient,
    base_pubkey: &Pubkey,
) -> Result<usize, ClientError> {
    let mut i = 0;
    while client.get_balance(&derive_stake_account_address(base_pubkey, i))? > 0 {
        i += 1;
    }
    Ok(i)
}

// TODO: Once solana-1.1 is released, use `Pubkey::create_with_seed`.
fn create_with_seed(base: &Pubkey, seed: &str, program_id: &Pubkey) -> Result<Pubkey, PubkeyError> {
    if seed.len() > MAX_SEED_LEN {
        return Err(PubkeyError::MaxSeedLengthExceeded);
    }

    Ok(Pubkey::new(
        hashv(&[base.as_ref(), seed.as_ref(), program_id.as_ref()]).as_ref(),
    ))
}

fn derive_stake_account_address(base_pubkey: &Pubkey, i: usize) -> Pubkey {
    create_with_seed(base_pubkey, &i.to_string(), &solana_stake_program::id()).unwrap()
}

// Return derived addresses
pub fn derive_stake_account_addresses(base_pubkey: &Pubkey, num_accounts: usize) -> Vec<Pubkey> {
    (0..num_accounts)
        .map(|i| derive_stake_account_address(base_pubkey, i))
        .collect()
}

fn create_authorize_instructions(
    stake_account_address: &Pubkey,
    keys: &TransferStakeKeys,
) -> Vec<Instruction> {
    let stake_authority_pubkey = keys.stake_authority_keypair.pubkey();
    let withdraw_authority_pubkey = keys.withdraw_authority_keypair.pubkey();
    let instruction0 = stake_instruction::authorize(
        &stake_account_address,
        &stake_authority_pubkey,
        &keys.new_stake_authority_pubkey,
        StakeAuthorize::Staker,
    );
    let instruction1 = stake_instruction::authorize(
        &stake_account_address,
        &withdraw_authority_pubkey,
        &keys.new_withdraw_authority_pubkey,
        StakeAuthorize::Withdrawer,
    );
    vec![instruction0, instruction1]
}

fn create_move_transaction(
    stake_account_address: &Pubkey,
    keys: &TransferStakeKeys,
    lamports: u64,
    i: usize,
) -> Transaction {
    let stake_authority_pubkey = keys.stake_authority_keypair.pubkey();
    let fee_payer_pubkey = keys.fee_payer_keypair.pubkey();

    let new_stake_account_address =
        derive_stake_account_address(&keys.new_stake_authority_pubkey, i);
    let mut instructions = stake_instruction::split_with_seed(
        &stake_account_address,
        &stake_authority_pubkey,
        lamports,
        &new_stake_account_address,
        &keys.new_stake_authority_pubkey,
        &i.to_string(),
    );

    let authorize_instructions = create_authorize_instructions(&new_stake_account_address, keys);

    instructions.extend(authorize_instructions.into_iter());
    Transaction::new_with_payer(instructions, Some(&fee_payer_pubkey))
}

pub(crate) fn authorize_stake_accounts(
    client: &RpcClient,
    keys: &TransferStakeKeys,
    num_accounts: usize,
) -> Result<(), Box<dyn Error>> {
    let stake_account_addresses =
        derive_stake_account_addresses(&keys.stake_authority_keypair.pubkey(), num_accounts);
    let fee_payer_pubkey = keys.fee_payer_keypair.pubkey();
    let transactions = stake_account_addresses
        .iter()
        .map(|stake_account_address| {
            let instructions = create_authorize_instructions(stake_account_address, keys);
            Ok(Transaction::new_with_payer(
                instructions,
                Some(&fee_payer_pubkey),
            ))
        })
        .collect::<Result<Vec<_>, ClientError>>()?;

    let signers = vec![
        &*keys.stake_authority_keypair,
        &*keys.withdraw_authority_keypair,
        &*keys.fee_payer_keypair,
    ];
    client.send_and_confirm_transactions(transactions, &signers)
}

pub(crate) fn move_stake_accounts(
    client: &RpcClient,
    keys: &TransferStakeKeys,
    num_accounts: usize,
) -> Result<(), Box<dyn Error>> {
    let stake_account_addresses =
        derive_stake_account_addresses(&keys.stake_authority_keypair.pubkey(), num_accounts);
    let transactions = stake_account_addresses
        .iter()
        .enumerate()
        .map(|(i, stake_account_address)| {
            let lamports = client.get_balance(&stake_account_address)?;
            let transaction = create_move_transaction(stake_account_address, keys, lamports, i);
            Ok(transaction)
        })
        .collect::<Result<Vec<_>, ClientError>>()?;

    let signers = vec![
        &*keys.stake_authority_keypair,
        &*keys.withdraw_authority_keypair,
        &*keys.fee_payer_keypair,
    ];
    client.send_and_confirm_transactions(transactions, &signers)
}
