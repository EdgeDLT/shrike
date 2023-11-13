# Shrike

Shrike is a suite of tools designed for analyzing Neo blockchain data. The infrastructure comprises four main components:

- **Indexer** - Synchronizes a NeoGo node, retrieves blockchain data, and processes it into a relational database.
- **API** - Provides a REST API for serving useful queries on indexed data.
- **GUI** - A user-friendly web interface for interacting with the data provided by Shrike.
- **Lib** - A shared library containing methods and models used across multiple Shrike components.

Each component has its own README with instructions for use. Contributions in the form of pull requests and suggestions for improvements or additional features are welcomed.

## Getting Started

To get started with Shrike, visit the README for the component you want to use:

- [Indexer README](./indexer/README.md)
- [API README](./api/README.md)
- [GUI README](./gui/README.md)
- [Lib README](./lib/README.md)

## Acknowledgements

Thanks to the [NeoGo](https://github.com/nspcc-dev/neo-go) team for their excellent software and documentation. Also thanks to @liaojinghui for their work on [neo-rs](https://github.com/Liaojinghui/neo-rs/).