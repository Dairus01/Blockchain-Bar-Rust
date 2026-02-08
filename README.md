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

Uses `clap` v4 with derive macros. Currently defines argument structure but subcommands are incomplete (`Commands` enum exists but has no variants defined).

Arguments:
- `name`: Optional positional argument
- `-c, --config <FILE>`: Custom config file path
- `-d, --debug`: Debug flag (can be repeated)

## Dependencies

```toml
clap = { version = "4.5.47", features = ["derive"] }
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.143"
thiserror = "2.0.16"
```

## File Structure

```
database/logs/
  genesis.json    # Initial state definition
  tx.db           # Append-only transaction log (JSON lines)
```

## Tests

Integration tests in `tests/test.rs` (currently commented out). Tests genesis deserialization and state loading.

## Known Issues

- `main.rs` line 26 references `cli.count` which does not exist (likely meant `cli.debug`)
- `src/balances.rs` is empty
- `database/state.json` is empty
- `src/main.rs` line 22 has dangling `enum` declaration
- Path construction in `new_state_from_disk()` hardcodes `rust_blockchain/database/logs` which may not match actual directory structure
