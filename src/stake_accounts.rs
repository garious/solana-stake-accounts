use solana_client::client_error::ClientError;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{hash::hashv, pubkey::Pubkey, signature::Signer};

pub(crate) struct TransferStakeKeys {
    pub stake_authority_keypair: Box<dyn Signer>, // Stake authority and Nonce Account
    pub withdraw_authority_keypair: Box<dyn Signer>, // Withdraw authority
    pub new_stake_authority_pubkey: Pubkey,
    pub new_withdraw_authority_pubkey: Pubkey,
}

pub const MAX_SEED_LEN: usize = 32;

#[derive(Debug)]
pub enum PubkeyError {
    MaxSeedLengthExceeded,
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

// Return addresses so long as they have a balance.
pub fn derive_stake_account_addresses(
    client: &RpcClient,
    base_pubkey: &Pubkey,
    num_accounts: Option<usize>,
) -> Vec<Pubkey> {
    let mut pubkeys = vec![];
    let mut i = 0;
    loop {
        let pubkey = derive_stake_account_address(base_pubkey, i);
        if let Some(num_accounts) = num_accounts {
            if i >= num_accounts {
                break;
            }
        } else if client.get_balance(&pubkey).unwrap() == 0 {
            break;
        }
        pubkeys.push(pubkey);
        i += 1;
    }
    pubkeys
}

fn split_stake_account(
    _client: &RpcClient,
    _stake_account_address: &Pubkey,
    _new_stake_account_address: &Pubkey,
    _stake_authority_keypair: &dyn Signer,
    _lamports: u64,
) -> Result<(), ClientError> {
    println!("Split stake account");
    Ok(())
}

fn set_authorities(
    _client: &RpcClient,
    _stake_account_address: &Pubkey,
    _keys: &TransferStakeKeys,
) -> Result<(), ClientError> {
    println!("Set authorities");
    Ok(())
}

pub(crate) fn move_stake_account(
    client: &RpcClient,
    keys: &TransferStakeKeys,
    num_accounts: Option<usize>,
) -> Result<(), ClientError> {
    let stake_account_addresses = derive_stake_account_addresses(
        client,
        &keys.stake_authority_keypair.pubkey(),
        num_accounts,
    );
    for (i, stake_account_address) in stake_account_addresses.iter().enumerate() {
        let new_stake_account_address =
            derive_stake_account_address(&keys.new_stake_authority_pubkey, i);
        let lamports = client.get_balance(&stake_account_address)?;
        split_stake_account(
            client,
            &stake_account_address,
            &new_stake_account_address,
            &*keys.stake_authority_keypair,
            lamports,
        )?;
        set_authorities(client, &new_stake_account_address, &keys)?;
    }
    Ok(())
}
