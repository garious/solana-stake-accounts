use solana_sdk::{
    client::Client, hash::hashv, pubkey::Pubkey, signature::Signer, transport::TransportError,
};

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
fn derive_stake_account_addresses<C: Client>(_client: &C, base_pubkey: &Pubkey) -> Vec<Pubkey> {
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

fn split_stake_account<C: Client, S: Signer>(
    _client: &C,
    _stake_account_address: &Pubkey,
    _new_stake_account_address: &Pubkey,
    _stake_authority_keypair: &S,
    _lamports: u64,
) -> Result<(), TransportError> {
    println!("Split stake account");
    Ok(())
}

fn set_authorities<C: Client, S: Signer>(
    _client: &C,
    _stake_account_address: &Pubkey,
    _keys: &TransferStakeKeys<S>,
) -> Result<(), TransportError> {
    println!("Set authorities");
    Ok(())
}

fn move_nonce_account<C: Client, S: Signer>(
    _client: &C,
    _nonce_keypair: &S,
) -> Result<(), TransportError> {
    println!("Move nonce account");
    Ok(())
}

pub(crate) fn move_stake_account<C: Client, S: Signer>(
    client: &C,
    keys: &TransferStakeKeys<S>,
) -> Result<(), TransportError> {
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
    move_nonce_account(client, &keys.stake_authority_keypair)?;
    Ok(())
}