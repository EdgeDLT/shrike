# Shrike

Shrike is a set of tools built for the purpose of Neo blockchain data analysis. The infrastructure comprises of three components:

**Indexer** - Synchronizes a NeoGo node, retrieves blockchain data, and processes it into a relational DB.

**API** - Serves a set of useful queries for indexed data over a REST API. Used to power the GUI and hopefully other third-party applications in the future.

**GUI** - A simple web interface for interacting with the data made available by Shrike. A hosted version of this application may be found [here](https://google.com).

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
* The NeoGo **v0.101.0** binary for your platform. Get that [here](https://github.com/nspcc-dev/neo-go/releases/tag/v0.101.0). Indexer has **not been tested** on any platform except Windows 10.
* (Optional) An SQLite-compatible DB browser/query editor. For simplicity I enjoy [DB Browser](https://sqlitebrowser.org/), going more advanced you might prefer [DBeaver](https://dbeaver.io/).

### Quickstart

1. Clone or otherwise download the Indexer folder.
2. Drop your NeoGo binary in the root directory (where `Cargo.toml` lives). On Windows, rename the binary to `neogo.exe`. On other platforms, you'll likely need to edit `main.rs` to use the correct path in `spawn::NeoGo::new()`.
3. Open the root directory in a terminal and enter `cargo run --release` to build and run.
4. Do something else for a while.

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

TODO

## GUI

TODO
