# Shrike

Shrike is a suite of tools designed for analyzing Neo blockchain data. The infrastructure comprises four main components:

**[Indexer](https://github.com/EdgeDLT/shrike#indexer)** - Synchronizes a NeoGo node, retrieves blockchain data, and processes it into a relational database.

**[API](https://github.com/EdgeDLT/shrike#api)** - Provides a REST API for serving useful queries on indexed data. Powers the GUI and aims to support third-party applications in the future.

**[GUI](https://github.com/EdgeDLT/shrike#gui)** - A user-friendly web interface for interacting with the data provided by Shrike. A hosted version of this application is planned for future release.

**[Lib](https://github.com/EdgeDLT/shrike#lib)** - A shared library containing methods and models used across multiple Shrike components.

Detailed instructions for operating each component independently can be found in the respective sections below. Pull requests and suggestions for improvements or additional features are welcomed.

## Indexer

A 🔥 *blazingly fast* 🔥 chain indexer for Neo N3.

The Indexer serves as the primary component of Shrike. Initially built for personal projects and learning purposes, it remains a work in progress but seems to be safe for human consumption.

The Indexer handles three main tasks:

* Synchronize a NeoGo instance.
* Fetch block, transaction, and application log data.
* Process and store chain data in SQLite tables.

### Running Indexer

For optimal use, pair the Indexer with a SQLite-compatible database browser or query editor. This allows for convenient analysis of data. For simplicity, consider using [DB Browser](https://sqlitebrowser.org/). For more advanced use cases (such as SSL database connections), [DBeaver](https://dbeaver.io/) is a suitable alternative.

#### Quickstart

* Visit the [releases]() page and download the appropriate executable for your platform (e.g., `indexer.exe` for `Windows`).
* Download the [NeoGo config file](https://github.com/EdgeDLT/shrike/blob/main/indexer/config/protocol.mainnet.yml) and place it in a `config` directory located next to the executable.
* Run the executable and follow the prompt to download NeoGo if you haven't already.
* Wait for sync & index completion.

### Build Instructions

#### Requirements

* The latest stable Rust version. I recommend using [Rustup](https://rustup.rs/).

The Indexer is primarily built and tested on **Windows 10** but should also work on Linux. It is also tested it on **Debian 11**. To build the Indexer on Linux, you may need to install a C linker, OpenSSL, and possibly pkg-config:
`sudo apt install build-essential libssl-dev pkg-config`

### Quickstart

1. Clone or download the Indexer folder.
2. Open the root directory in a terminal and run `cargo run --release` to build and execute the Indexer.
3. Follow the prompt to download NeoGo if you haven't already.
4. Allow some time for the process to complete.

A keep-alive mode is available to keep the Indexer active and listening for blocks. Blocks are synced as soon as they are detected. Activate this mode with the `-k` / `--keep-alive` flag, e.g., `cargo run --release -- -k`.

### Notes

<sup>All figures below are accurate on my machine as of block height ~2.7M.</sup>

#### Database location

Shrike will no longer store data in the Indexer directory. This change was made to facilitate binary releases of the Indexer and API, which require deterministic, platform-specific file paths for the database to function properly.

On Windows, the Shrike database may be found in the user `AppData` directory:

```
C:\\Users\username\AppData\Local\Shrike\data\shrike.db3
```

On Linux, the project directory and data is now placed under `.local`:

```
/home/<Username>/.local/share/Shrike/shrike.db3
```

**Note**: At this time, NeoGo and its `chains` & `config` folders will continue to co-locate with Indexer. Subject to change.

#### Database structure

The database consists of two tables: `blocks` and `transactions`. They are modeled to closely resemble their typical NeoRPC forms, with some adjustments for SQL and incorporating relevant parts of their respective `application logs`.

As the project is in early development, schema changes may occur from time to time when there is strong justification for it. Additional tables (such as `contracts` or `balances`) are also possible, feel free to submit a PR if you want to expedite such a process.

#### NeoGo sync time

The Indexer waits for its NeoGo instance to sync before fetching data. Syncing NeoGo currently takes slightly over an hour. You can accelerate this by adjusting the config to `SkipBlockVerification`, but patience is recommended instead. Once you catch up to the chain head once, sync time is generally negligible.

#### Indexing time

The Indexer operates efficiently, allowing you to use your machine as usual while it runs. After syncing, populating the block and transaction tables from scratch takes less than 15 minutes.

#### Storage requirements

Using the Indexer requires a significant amount of storage space, slightly more than syncing a node alone. As of now, my chain folder is 26.6GB, and the Shrike DB is 7.18GB. Estimate the required headroom to account for future blockchain growth based on your use case.

#### Alternative networks

The Indexer can target any Neo N3 network compatible with the current NeoGo version used by the program. Adjust the protocol config file to do so. References can be found [here](https://github.com/nspcc-dev/neo-go/tree/master/config). You may need to modify the `NODE_PATH` in `rpc.rs` if you change the RPC port.

### Acknowledgements

Thanks to the [NeoGo](https://github.com/nspcc-dev/neo-go) team for their excellent software and documentation. Also thanks to @liaojinghui for their work on [neo-rs](https://github.com/Liaojinghui/neo-rs/), which provided numerous useful code examples and inspiration, particularly for public address/script hash conversion.

## API

The API is an Actix Web-based service that executes various queries against indexed data and serves the responses. Currently, only basic queries are implemented. There is no caching for queries that need to be performed only once per block, excluding the `stats` call, which may result in poor scaling for multiple users, depending on the query. This can be improved in a case-specific manner by adding a custom SQLite index to speed up the query.

### Quickstart

1. Clone or download the API folder.
2. Obtain a copy of the Shrike DB from the download page (TODO) or by running the Indexer. Update the file path in `main.rs` through the `DB_PATH` constant.
3. Use `cargo run` or `cargo run --release` to serve the API.
4. Make your requests! The default path for the API when run locally is: `http://0.0.0.0:8080/v1/module/method/parameter`.

A hosted version will be available in the future.

### API Reference

TODO

## GUI

The GUI is a simple web application built using SolidJS (SolidStart) and PicoCSS. It was designed to allow regular users to access Shrike, but power users will benefit more from running custom queries against their own copy of the Shrike DB.

### Quickstart

1. Clone or download the GUI folder.
2. Run the API following the instructions above, or update the path in `/constants/index.js` to use the hosted version (coming soon).
3. Serve the GUI locally using `npm run dev` and open it in your browser at `http://127.0.0.1:5173/`.

## Lib

The Lib is a shared library for Shrike components. It currently includes only Neo data conversion methods and database path handling, but will be expanded with other functions and models as needed when refactoring.
