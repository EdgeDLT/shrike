# API

The API is an Actix Web-based service that executes various queries against the indexed data and serves the responses. It is designed to provide a simple and efficient way to interact with the Shrike database.

## Features

- Executes queries against the Shrike database.
- Serves responses in a simple and efficient manner.
- Supports basic queries with potential for expansion.

## Getting Started

### Prerequisites

- Latest stable Rust version. We recommend using [Rustup](https://rustup.rs/).
- A copy of the Shrike database, which can be obtained by running the Indexer.

### Quickstart

1. Clone or download the API folder.
2. Update the `DB_PATH` constant in `main.rs` with the path to your Shrike database.
3. Run `cargo run --release` to serve the API.
4. Make your requests! The default path for the API when run locally is: `http://0.0.0.0:8080/v1/module/method/parameter`.

A hosted version of the API will be available in the future.

## API Reference

The API currently supports basic queries. More detailed documentation on the available endpoints and their usage will be provided in the future.

## Contributing

Contributions to the API are welcomed. If you have suggestions for additional queries or improvements to the existing ones, feel free to open an issue or submit a pull request.