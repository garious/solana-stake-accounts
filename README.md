# solana-stake-accounts

A command-line tool for managing Solana stake accounts. The functionality here is all
possible using the Solana CLI, but this tool creates accounts with particular
conventions, such that stake accounts are easy to find and generally more accessible.

## Usage

### Create a stake account

Create a derived stake account at the stake authority public key:

```bash
solana-stake-accounts new <SENDER_KEYPAIR> <BASE_KEYPAIR> <AMOUNT> \
    --stake-authority <KEYPAIR> --withdraw-authority <KEYPAIR>
```

Create derived stake accounts with a vesting schedule:

```bash
solana-stake-accounts new <SENDER_KEYPAIR> <BASE_KEYPAIR> <AMOUNT> \
    --stake-authority <KEYPAIR> --withdraw-authority <KEYPAIR> \
    --cliff <PERCENTAGE> --cliff-years <NUMBER> --unlock-years <NUMBER> \
    --unlocks <NUMBER> --custodian <PUBKEY>
```

### Get stake account balances

Sum the balance of all stake accounts:

```bash
solana-stake-accounts balance <BASE_PUBKEY>
```

Sum the balance of a specific number of accounts:

```bash
solana-stake-accounts balance <BASE_PUBKEY> --num-accounts <NUMBER>
```

### Get stake account public keys

List the public key of each stake account derived from the given public key:

```bash
solana-stake-accounts pubkeys <BASE_PUBKEY>
```

List a specific amount of derived public keys, whichout first checking if
the account exists:

```bash
solana-stake-accounts pubkeys <BASE_PUBKEY> --num-accounts <NUMBER>
```

### Show all stake accounts

Show all the stake accounts derived from the given public key:

```bash
solana-stake-accounts show <BASE_PUBKEY>
```

Show a specific number of derived stake accounts:

```bash
solana-stake-accounts show <BASE_PUBKEY> --num-accounts <NUMBER>
```

### Withdraw tokens

Withdraw tokens from the first derived, unlocked stake account:

```bash
solana-stake-accounts withdraw <BASE_PUBKEY> <RECIPIENT_ACCOUNT_ADDRESS> <AMOUNT> \
    --withdraw-authority <KEYPAIR>
```

Withdraw tokens from a particular stake account:

```bash
solana-stake-accounts withdraw <BASE_PUBKEY> <RECIPIENT_ACCOUNT_ADDRESS> <AMOUNT> \
    --withdraw-authority <KEYPAIR> --index <NUMBER>
```

### Move stake accounts

Move stake accounts account to a new location.

```bash
solana-stake-accounts rebase <BASE_PUBKEY> <NEW_BASE_KEYPAIR> --stake-authority <KEYPAIR>
```

Set new authorities:

```bash
solana-stake-accounts authorize <BASE_KEYPAIR> \
    --stake-authority <KEYPAIR> --withdraw-authority <KEYPAIR>
    --new-stake-authority <KEYPAIR> --new-withdraw-authority <PUBKEY>
```

Rebase stake accounts and authorize new authorities:

```bash
solana-stake-accounts move <BASE_KEYPAIR> \
    --stake-authority <KEYPAIR> --withdraw-authority <KEYPAIR> \
    --new-stake-authority <KEYPAIR> --new-withdraw-authority <PUBKEY>
```
