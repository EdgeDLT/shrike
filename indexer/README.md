# Indexer

The Indexer is a key component of Shrike, responsible for synchronizing a NeoGo instance, fetching blockchain data, and processing it into SQLite tables.

## Features

- Synchronizes a NeoGo instance.
- Fetches block, transaction, and application log data.
- Processes and stores chain data in SQLite tables.

## Getting Started

### Prerequisites

- Latest stable Rust version. We recommend using [Rustup](https://rustup.rs/).
- SQLite-compatible database browser or query editor for data analysis. We recommend [DB Browser](https://sqlitebrowser.org/) for simplicity or [DBeaver](https://dbeaver.io/) for advanced use cases.

### Quickstart

1. Visit the [releases](#) page and download the appropriate executable for your platform (e.g., `indexer.exe` for `Windows`).
2. Download the [NeoGo config file](https://github.com/EdgeDLT/shrike/blob/main/indexer/config/protocol.mainnet.yml) and place it in a `config` directory located next to the executable.
3. Run the executable and follow the prompt to download NeoGo if you haven't already.
4. Wait for sync & index completion.

### Build Instructions

1. Clone or download the Indexer folder.
2. Open the root directory in a terminal and run `cargo run --release` to build and execute the Indexer.
3. Follow the prompt to download NeoGo if you haven't already.
4. Allow some time for the process to complete.

## Database

The Shrike database consists of two tables: `blocks` and `transactions`. They are modeled to closely resemble their typical NeoRPC forms, with some adjustments for SQL and incorporating relevant parts of their respective `application logs`.

### Database Location

- On Windows: `C:\\Users\<username>\AppData\Local\Shrike\data\shrike.db3`
- On Linux: `/home/<username>/.local/share/Shrike/shrike.db3`
- On MacOS: `/Users/<username>/Library/Application Support/Shrike/shrike.db3`

### Storage Requirements

Using the Indexer requires a significant amount of storage space, slightly more than syncing a node alone. As of block height 4408282, As of now, the chain folder is 39.1GB, and the Shrike DB is 12.2GB. Estimate the required headroom to account for future blockchain growth based on your use case.

## Contributing

As the project is in early development, schema changes may occur from time to time when there is strong justification for it. Additional tables (such as `contracts` or `balances`) are also possible, feel free to submit a PR if you want to expedite such a process.