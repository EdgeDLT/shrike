# Indexer

The Indexer is a key component of Shrike. It uses a local node to process Neo N3 blocks and transactions into a more easily queried format.


```
PS > cargo run --release
    Finished release [optimized] target(s) in 1.85s
     Running `..\shrike\target\release\indexer.exe`
[INFO  indexer::db::database] Using database at ..\AppData\Local\Shrike\data\shrike.db3.
[INFO  indexer] Welcome to Shrike!
[INFO  indexer] Checking for NeoGo..
[INFO  indexer::db::database] WAL mode already active.
[INFO  indexer] Last stored block index: 4615157
[INFO  indexer] Starting node sync..

    _   ____________        __________
   / | / / ____/ __ \      / ____/ __ \
  /  |/ / __/ / / / /_____/ / __/ / / /
 / /|  / /___/ /_/ /_____/ /_/ / /_/ /
/_/ |_/_____/\____/      \____/\____/

/NEO-GO:0.105.0/

Current height: 4677439
[INFO  indexer] Sync completed in 95482 ms.
[INFO  indexer::spawn::indexer] Chain height is 4677745.
[INFO  indexer::spawn::indexer] Started indexing.
[INFO  indexer::spawn::indexer] Start height is 4615158. 62587 blocks to process.
[INFO  indexer::spawn::indexer] Updating tables:
Indexed 62587 block(s).
[INFO  indexer::spawn::indexer] Indexing completed in 41335 ms.
[INFO  indexer::spawn::indexer] New stored height is 4677744.
[WARN  indexer::spawn::sync] Shutdown signal received.
[WARN  indexer::spawn::sync] Node killed.
```

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