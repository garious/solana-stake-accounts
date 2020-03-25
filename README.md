# solana-stake-accounts

A command-line tool for managing Solana stake accounts. The functionality here is all
possible using the Solana CLI, but this tool creates accounts with particular
conventions, such that stake accounts are easy to find and generally more accessible.

## Usage

### Create a stake account

Create a derived stake account at the stake authority public key:

```bash
solana-stake-accounts new <SENDER_KEYPAIR> <STAKE_AUTHORITY_KEYPAIR> <AMOUNT> --withdraw-authority=<KEYPAIR>
```

Create a derived stake account and a fee-payer account at the stake authority public key:

```bash
solana-stake-accounts new <SENDER_KEYPAIR> <STAKE_AUTHORITY_KEYPAIR> <AMOUNT> --withdraw-authority=<KEYPAIR> --add-fee-payer
```

Create derived stake accounts with a vesting schedule:

```bash
solana-stake-accounts new <SENDER_KEYPAIR> <STAKE_AUTHORITY_KEYPAIR> <AMOUNT> --withdraw-authority=<KEYPAIR> --cliff=<PERCENTAGE> --cliff-years=<NUMBER> --unlock-years=<NUMBER> --unlocks=<NUMBER>
```

### Get stake account balances

Sum the balance of all stake accounts:

```bash
solana-stake-accounts balance <STAKE_AUTHORITY_PUBKEY>
```

Sum the balance of all stake accounts, including a fee-payer account:

```bash
solana-stake-accounts balance <STAKE_AUTHORITY_PUBKEY> --include-fee-payer
```

### Withdraw tokens

Withdraw tokens from the first derived, unlocked stake account:

```bash
solana-stake-accounts withdraw <STAKE_AUTHORITY_KEYPAIR> <RECIPIENT_ACCOUNT_ADDRESS> <AMOUNT> --withdraw-authority=<KEYPAIR>
```

### Move stake accounts

Move stake accounts account to a new location. Also move the fee-payer account at `<STAKE_AUTHORITY_KEYPAIR>`, if it exists:

```bash
solana-stake-accounts rebase <STAKE_AUTHORITY_KEYPAIR> <NEW_BASE_KEYPAIR>
```

Set new authorities:

```bash
solana-stake-accounts authorize <STAKE_AUTHORITY_KEYPAIR> --withdraw-authority=<KEYPAIR> --new-stake-authority=<KEYPAIR> --new-withdraw-authority=<PUBKEY>
```

Rebase stake accounts and authorize new authorities:

```bash
solana-stake-accounts move <STAKE_AUTHORITY_KEYPAIR> --withdraw-authority=<KEYPAIR> --new-stake-authority=<KEYPAIR> --new-withdraw-authority=<PUBKEY>
```
