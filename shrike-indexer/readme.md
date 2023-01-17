## Shrike Indexer

A 🔥 *blazingly fast* 🔥 indexer for the Neo N3 blockchain.

Shrike is a set of data analysis tools for the Neo blockchain. Indexer is the first and primary component. It was built with personal projects in mind, and as a learning experience, so is very much a WIP. However, it should be just about safe for human consumption.

Indexer oversees three functions:

* Synchronize a NeoGo instance.
* Fetch block, transaction, and application log data.
* Process and store the chain data into SQLite tables.

Please feel free to make suggestions or PRs if you spot ways to improve the code or broaden the featureset. My only requirement is that if you use dark magic to achieve such a goal, you must teach me your ways before the enhancement will be accepted.

Last thing: this project makes liberal use of `unwrap`. If this *rustles* your jimmies, please avert your gaze. I'll add proper error handling if I get really really bored or if something stops working consistently.

Current version: v0.1.0 (alpha)

### Requirements

* The latest stable Rust version. I recommend using [Rustup](https://rustup.rs/).
* The NeoGo **v0.100.0** binary for your platform. Get that [here](https://github.com/nspcc-dev/neo-go/releases/tag/v0.100.0). Indexer has **not been tested** on any platform except Windows 10.
* (Optional) An SQLite-compatible DB browser/query editor. For simplicity I enjoy [DB Browser](https://sqlitebrowser.org/), going more advanced you might prefer [DBeaver](https://dbeaver.io/).

### Quickstart

1. Clone or otherwise download the repo.
2. Drop your NeoGo binary in the root directory (where `Cargo.toml` lives). On Windows, rename the binary to `neogo.exe`. On other platforms, you'll likely need to edit `main.rs` to use the correct path in `spawn::NeoGo::new()`.
3. Open the root directory in a terminal and enter `cargo run --release` to build and run.
4. Do something else for a while.

### Notes

<sup>All figures below are accurate on my machine as of block height ~2.7M.</sup>

* Database structure

The database has two tables: `blocks` and `transactions`. They are modelled to closely match their typical NeoRPC forms, with some allowances made for SQL and the cramming of the relevant parts of their respective `application logs` into each.

I'm not against the idea of changing the tables, depending on feedback, if there's good reason for it. I also plan to add `contracts` and perhaps `balances` or `transfers`, depending on if I have a use case for them. Feel free to make a PR if you want to expedite that process.

* NeoGo sync time

Indexer will wait for its NeoGo instance to sync before it will start fetching data. Syncing NeoGo currently takes approximately an hour. You can speed it up by adjusting the config to `SkipBlockVerification`, but this is not advised. Once you have caught up to the chain head once, sync time is generally negligible.

* Indexing time

Indexer works quickly and quietly, you can use your machine as you usually would while it runs. Once syncing is complete, fully populating the block and transaction tables from scratch takes me less than 15 minutes.

* Storage requirements

You'll need a healthy amount of storage space to use the Indexer, slightly more than is required to sync a node on its own. My chain folder is currently 26.6GB and the Shrike DB is 7.18GB. Extrapolate from there to determine how much headroom you need to account for future blockchain growth, depending on your use case.

* Alternative networks

You can point Indexer at any Neo N3 network that is compatible with the current NeoGo version used by the program. This can be done by adjusting the protocol config file. References can be found [here](https://github.com/nspcc-dev/neo-go/tree/master/config). You may have to adjust the `NODE_PATH` in `rpc.rs` if you alter the RPC port.

### Acknowledgements

Thanks to the [NeoGo](https://github.com/nspcc-dev/neo-go) team for their excellent software and documentation. Also thanks to @liaojinghui, whose work on [neo-rs](https://github.com/Liaojinghui/neo-rs/) saved me a lot of headache with the cumbersome task of converting script hashes to public addresses.
