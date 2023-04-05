# Shrike

Shrike is a set of tools built for the purpose of Neo blockchain data analysis. The infrastructure comprises three components:

**[Indexer](https://github.com/EdgeDLT/shrike#indexer)** - Synchronizes a NeoGo node, retrieves blockchain data, and processes it into a relational DB.

**[API](https://github.com/EdgeDLT/shrike#api)** - Serves a set of useful queries for indexed data over a REST API. Used to power the GUI and hopefully other third-party applications in the future.

**[GUI](https://github.com/EdgeDLT/shrike#gui)** - A simple web interface for interacting with the data made available by Shrike. A hosted version of this application may be found [here](https://google.com).

**[Lib](https://github.com/EdgeDLT/shrike#lib)** - A shared library for methods and models that are used across multiple Shrike components.

You can find instructions on how to operate each of the components independently in the respective sections below.

Pull requests and suggestions are welcomed in any of the components. There are innumerable ways to improve the code and broaden the featureset.

## Indexer

A 🔥 *blazingly fast* 🔥 chain indexer for Neo N3.

Indexer is the first and primary component. It was built with personal projects in mind, and as a learning experience, so is very much a WIP. However, it should be just about safe for human consumption.

The Indexer oversees three functions:

* Synchronize a NeoGo instance.
* Fetch block, transaction, and application log data.
* Process and store the chain data into SQLite tables.

### Requirements

* The latest stable Rust version. I recommend using [Rustup](https://rustup.rs/).
* (Optional) An SQLite-compatible DB browser/query editor. For simplicity I enjoy [DB Browser](https://sqlitebrowser.org/), going more advanced you might prefer [DBeaver](https://dbeaver.io/).

Indexer is built with and therefore mostly tested on **Windows 10**. It should also work on other platforms, although I have only tested it on **Debian 11**. To build Indexer on Ubuntu, you will also need to install a C linker, OpenSSL, and possibly pkg-config:
`sudo apt install build-essential libssl-dev pkg-config`

### Quickstart

1. Clone or otherwise download the Indexer folder.
2. Open the root directory in a terminal and enter `cargo run --release` to build and run.
3. Follow the prompt to download NeoGo if you haven't already.
4. Do something else for a while.

A keep-alive mode has been added, which will keep the Indexer active and listening for blocks. Blocks are synced as soon as they are detected. This mode can be activated with the `-k` / `--keep-alive` flag, e.g. `cargo run --release -- -k`.

### Notes

<sup>All figures below are accurate on my machine as of block height ~2.7M.</sup>

#### Database structure

The database has two tables: `blocks` and `transactions`. They are modelled to closely match their typical NeoRPC forms, with some allowances made for SQL and the cramming of the relevant parts of their respective `application logs` into each.

I'm not against the idea of changing the tables, depending on feedback, if there's good reason for it. I also plan to add `contracts` and perhaps `balances` or `transfers`, depending on if I have a use case for them. Feel free to make a PR if you want to expedite that process.

#### NeoGo sync time

Indexer will wait for its NeoGo instance to sync before it will start fetching data. Syncing NeoGo currently takes a little over an hour. You can speed it up by adjusting the config to `SkipBlockVerification`, but this is not advised. Once you have caught up to the chain head once, sync time is generally negligible.

#### Indexing time

Indexer works quickly and quietly, you can use your machine as you usually would while it runs. Once syncing is complete, fully populating the block and transaction tables from scratch takes me less than 15 minutes.

#### Storage requirements

You'll need a healthy amount of storage space to use the Indexer, slightly more than is required to sync a node on its own. My chain folder is currently 26.6GB and the Shrike DB is 7.18GB. Extrapolate from there to determine how much headroom you need to account for future blockchain growth, depending on your use case.

#### Alternative networks

You can point Indexer at any Neo N3 network that is compatible with the current NeoGo version used by the program. This can be done by adjusting the protocol config file. References can be found [here](https://github.com/nspcc-dev/neo-go/tree/master/config). You may have to adjust the `NODE_PATH` in `rpc.rs` if you alter the RPC port.

### Acknowledgements

Thanks to the [NeoGo](https://github.com/nspcc-dev/neo-go) team for their excellent software and documentation. Also thanks to @liaojinghui, whose work on [neo-rs](https://github.com/Liaojinghui/neo-rs/) saved me a lot of headache with the cumbersome task of converting script hashes to public addresses.

## API

An Actix Web-based service that performs various queries against indexed data and serves the responses. Only relatively basic queries are implemented so far. There is currently no caching for queries that only need to be performed once per block, it will scale very poorly to multiple users until then.

### Quickstart

1. Clone or otherwise download the API folder.
2. Get a copy of the Shrike DB from the download page (TODO) or by running the Indexer. Adjust the file path in `main.rs` via the `DB_PATH` constant.
3. Use `cargo run` or `cargo run --release` to serve the API.
4. Make your requests! The default path for the API when run locally is as follows: `http://0.0.0.0:8080/v1/module/method/parameter`.

A hosted version will be provided in the future.

### API Reference

TODO

## GUI

A simple web application built using SolidJS (SolidStart) and PicoCSS. It was created to give a way for regular users to leverage Shrike, but power users will be better served by running custom queries against their own copy of the Shrike DB.

### Quickstart

1. Clone or otherwise download the GUI folder.
2. Run the API following the above instructions, or update the path in `/constants/index.js` to use the hosted version (coming soon).
3. Serve the GUI locally with `npm run dev` and open it in your browser at `http://127.0.0.1:5173/`.

## Lib

A shared library for Shrike components. It currently only contains Neo data conversion methods, but will be expanded with other functions and models over time when refactoring.
