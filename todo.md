## To Do List

### Backend:

* Cache results of once-per-block calls (e.g. total GAS burned) for speed and scaling to many users
* Write glue code to keep API synced
	- Option 1: Write a keep-alive script that spawns and kills the indexer every few seconds
	- Option 2: Build a keep-alive mode directly into the indexer
* Query sanitizing (integer & char limits, hex check, etc.)
* Make get block call support hash as argument
* Add more useful methods
	- e.g. 1: Method to get all transfers by address
	- e.g. 2: Method to get all transactions by block index/hash

* Make a separate table (view?) for transfers/contracts/balances and associated queries
* Add DB download utility for easy data sharing

### Frontend:

* JSON to CSV converter for transfers by address
* Add more methods and stats aimed at regular users
* Write the about section and repo link
* Meta descriptions for Lighthouse
* Base58/64 decoding?
* Prettify JSON values
* Better errors
* DB download page
* Wallet support?
