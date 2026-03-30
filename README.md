# rust_blockchain

A minimal Rust-based blockchain state manager with transaction mempool and persistence.

## Architecture

### Core Types

- **`UserAccount`** (`src/user.rs`): 32-byte address with balance and transaction history
- **`Tx`** (`src/tx.rs`): Transaction containing from/to accounts, value (u64), and data string
- **`State`** (`src/state.rs`): In-memory state with balances HashMap, transaction mempool vector, and optional file handle

### Modules

| Module | Purpose |
|--------|---------|
| `state` | State initialization from disk, transaction mempool management, persistence |
| `genesis` | Genesis block loading from JSON |
| `utils` | Transaction application logic, reward detection |
| `error` | `MempoolError` enum with `thiserror` definitions |
| `user` | Account structure definitions |
| `tx` | Transaction structure definitions |

## State Lifecycle

1. `new_state_from_disk()` reads `database/logs/genesis.json` and replays `database/logs/tx.db` line-by-line
2. Transactions are deserialized (JSON) and applied to update balances
3. New transactions enter via `State::add()` which validates through `apply()`
4. `State::persist()` writes mempool transactions to disk (one JSON object per line) and clears mempool

## Transaction Rules

- **Rewards**: Transactions with `data == "reward"` mint value to recipient without deducting from sender
- **Transfers**: Standard transactions deduct from sender, add to recipient (no explicit sender balance check implemented - panics if insufficient)

## CLI

Uses `clap` v4 with derive macros. Exposes two subcommands via the `Commands` enum.

Arguments:
- `name`: Optional positional argument
- `-c, --config <FILE>`: Custom config file path
- `-d, --debug`: Debug verbosity flag (repeatable, e.g. `-ddd`)

Subcommands:
- `balances` — Print current account balances from state
- `tx` — Add a new transaction to the mempool

## Balances

`src/balances.rs` exposes `State::get_balance(account) -> u64`, returning the current balance for a given `UserAccount` (or `0` if not found).

## Genesis Seeding

On startup, `new_state_from_disk()` loads `database/logs/genesis.json`, converts each named account into a `UserAccount` via `UserAccount::from_name()`, and seeds the in-memory balance map before replaying the transaction log.

## Dependencies

```toml
clap = { version = "4.5.47", features = ["derive"] }
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.143"
thiserror = "2.0.16"
```

## File Structure

```
database/
  logs/
    genesis.json    # Initial account balances
    tx.db           # Append-only transaction log (JSON lines)
  state.json        # Latest persisted state snapshot
src/
  main.rs           # CLI entrypoint (clap)
  lib.rs            # Module declarations
  state.rs          # State struct, mempool, persistence
  genesis.rs        # Genesis JSON deserialisation
  balances.rs       # get_balance helper
  utils.rs          # apply() and is_reward() logic
  tx.rs             # Tx struct
  user.rs           # UserAccount struct + from_name constructor
  error.rs          # MempoolError enum
tests/
  test.rs           # Integration tests
```

## Tests

Integration tests in `tests/test.rs`. Covers genesis deserialisation and state loading from disk.
