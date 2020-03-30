# solana-stake-accounts

A command-line tool for managing Solana stake accounts. The functionality here is all
possible using the Solana CLI, but this tool creates accounts with particular
conventions, such that stake accounts are easy to find and generally more accessible.

## Usage

### Create a stake account

Create and fund a derived stake account at the stake authority public key:

```bash
solana-stake-accounts new <SENDER_KEYPAIR> <BASE_KEYPAIR> <AMOUNT> \
    --stake-authority <PUBKEY> --withdraw-authority <PUBKEY>
```

### Count accounts

Count the number of derived accounts:

```bash
solana-stake-accounts count <BASE_PUBKEY>
```

### Get stake account balances

Sum the balance of dervied stake accounts:

```bash
solana-stake-accounts balance <BASE_PUBKEY> --num-accounts <NUMBER>
```

### Get stake account public keys

List the public key of each stake account derived from the given public key:

```bash
solana-stake-accounts pubkeys <BASE_PUBKEY> --num-accounts <NUMBER>
```

### Set new authorities

Set new authorities on each derived stake account:

```bash
solana-stake-accounts authorize <BASE_PUBKEY> \
    --stake-authority <KEYPAIR> --withdraw-authority <KEYPAIR> \
    --new-stake-authority <KEYPAIR> --new-withdraw-authority <PUBKEY> \
    --num-accounts <NUMBER>
```

### Relocate stake accounts

Relocate stake accounts:

```bash
solana-stake-accounts rebase <BASE_PUBKEY> <NEW_BASE_KEYPAIR> \
    --stake-authority <KEYPAIR> --num-accounts <NUMBER>
```

To atomically rebase and authorize each stake account, use the 'move'
command:

```bash
solana-stake-accounts move <BASE_PUBKEY> <NEW_BASE_KEYPAIR> \
    --stake-authority <KEYPAIR> --withdraw-authority <KEYPAIR> \
    --new-stake-authority <KEYPAIR> --new-withdraw-authority <PUBKEY> \
    --num-accounts <NUMBER>
```
