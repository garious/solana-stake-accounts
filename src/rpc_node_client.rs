use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    account::Account,
    client::SyncClient,
    clock::Slot,
    commitment_config::CommitmentConfig,
    fee_calculator::{FeeCalculator, FeeRateGovernor},
    hash::Hash,
    instruction::Instruction,
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signers::Signers,
    transaction::{self, Transaction},
    transport::Result,
};

pub struct RpcNodeClient<'a>(pub &'a RpcClient);

impl SyncClient for RpcNodeClient<'_> {
    fn send_message<T: Signers>(&self, signers: &T, message: Message) -> Result<Signature> {
        let mut transaction = Transaction::new_unsigned(message);
        let _signature = self
            .0
            .send_and_confirm_transaction(&mut transaction, signers)
            .unwrap();
        Ok(Signature::default())
    }

    fn send_instruction(&self, _keypair: &Keypair, _instruction: Instruction) -> Result<Signature> {
        todo!();
    }

    fn transfer(&self, _lamports: u64, _keypair: &Keypair, _pubkey: &Pubkey) -> Result<Signature> {
        todo!();
    }

    fn get_account_data(&self, _pubkey: &Pubkey) -> Result<Option<Vec<u8>>> {
        todo!();
    }

    fn get_account(&self, _pubkey: &Pubkey) -> Result<Option<Account>> {
        todo!();
    }

    fn get_account_with_commitment(
        &self,
        _pubkey: &Pubkey,
        _commitment_config: CommitmentConfig,
    ) -> Result<Option<Account>> {
        todo!();
    }

    fn get_balance(&self, pubkey: &Pubkey) -> Result<u64> {
        self.0.get_balance(pubkey).map_err(Into::into)
    }

    fn get_balance_with_commitment(
        &self,
        _pubkey: &Pubkey,
        _commitment_config: CommitmentConfig,
    ) -> Result<u64> {
        todo!();
    }

    fn get_recent_blockhash(&self) -> Result<(Hash, FeeCalculator)> {
        todo!();
    }

    fn get_recent_blockhash_with_commitment(
        &self,
        _commitment_config: CommitmentConfig,
    ) -> Result<(Hash, FeeCalculator)> {
        todo!();
    }

    fn get_fee_calculator_for_blockhash(&self, _blockhash: &Hash) -> Result<Option<FeeCalculator>> {
        todo!();
    }

    fn get_fee_rate_governor(&self) -> Result<FeeRateGovernor> {
        todo!();
    }

    fn get_signature_status(
        &self,
        _signature: &Signature,
    ) -> Result<Option<transaction::Result<()>>> {
        todo!();
    }

    fn get_signature_status_with_commitment(
        &self,
        _signature: &Signature,
        _commitment_config: CommitmentConfig,
    ) -> Result<Option<transaction::Result<()>>> {
        todo!();
    }

    fn get_slot(&self) -> Result<Slot> {
        todo!();
    }

    fn get_slot_with_commitment(&self, _commitment_config: CommitmentConfig) -> Result<u64> {
        todo!();
    }

    fn get_transaction_count(&self) -> Result<u64> {
        todo!();
    }

    fn get_transaction_count_with_commitment(
        &self,
        _commitment_config: CommitmentConfig,
    ) -> Result<u64> {
        todo!();
    }

    fn poll_for_signature_confirmation(
        &self,
        _signature: &Signature,
        _min_confirmed_blocks: usize,
    ) -> Result<usize> {
        todo!();
    }

    fn poll_for_signature(&self, _signature: &Signature) -> Result<()> {
        todo!();
    }

    fn get_new_blockhash(&self, _blockhash: &Hash) -> Result<(Hash, FeeCalculator)> {
        todo!();
    }
}
