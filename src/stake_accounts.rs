use solana_client::client_error::ClientError;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{hash::hashv, pubkey::Pubkey, signature::Signer};

pub(crate) struct TransferStakeKeys<S: Signer> {
    pub stake_authority_keypair: S,    // Stake authority and Nonce Account
    pub withdraw_authority_keypair: S, // Withdraw authority
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
fn derive_stake_account_addresses(_client: &RpcClient, base_pubkey: &Pubkey) -> Vec<Pubkey> {
    println!("Derive stake account addresses");
    let mut pubkeys = vec![];
    let mut i = 0;
    while i < 1 {
        let pubkey = derive_stake_account_address(base_pubkey, i);
        pubkeys.push(pubkey);
        i += 1;
    }
    pubkeys
}

fn split_stake_account<S: Signer>(
    _client: &RpcClient,
    _stake_account_address: &Pubkey,
    _new_stake_account_address: &Pubkey,
    _stake_authority_keypair: &S,
    _lamports: u64,
) -> Result<(), ClientError> {
    println!("Split stake account");
    Ok(())
}

fn set_authorities<S: Signer>(
    _client: &RpcClient,
    _stake_account_address: &Pubkey,
    _keys: &TransferStakeKeys<S>,
) -> Result<(), ClientError> {
    println!("Set authorities");
    Ok(())
}

pub(crate) fn move_stake_account<S: Signer>(
    client: &RpcClient,
    keys: &TransferStakeKeys<S>,
) -> Result<(), ClientError> {
    let stake_account_addresses =
        derive_stake_account_addresses(client, &keys.stake_authority_keypair.pubkey());
    for (i, stake_account_address) in stake_account_addresses.iter().enumerate() {
        let new_stake_account_address =
            derive_stake_account_address(&keys.new_stake_authority_pubkey, i);
        let lamports = client.get_balance(&stake_account_address)?;
        split_stake_account(
            client,
            &stake_account_address,
            &new_stake_account_address,
            &keys.stake_authority_keypair,
            lamports,
        )?;
        set_authorities(client, &new_stake_account_address, &keys)?;
    }
    Ok(())
}
