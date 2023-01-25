## To Do List

### Backend:

* Improve cache with reset on new block
* Query sanitizing (integer & char limits, hex check, etc.)
* Add more useful methods
	- e.g. 1: Method to get all transfers by address
	- e.g. 2: Method to get all transactions by block index/hash

* Make a separate table (view?) for transfers/contracts/balances and associated queries
* Add DB download utility for easy data sharing
* Contract validation/blacklisting (prevent stat manipulation via fake events)
* Add graceful shutdown for SIGTERM

### Frontend:

* JSON to CSV converter for transfers by address
* Add more methods and stats aimed at regular users
* Base58/64 decoding?
* Prettify JSON values
* Better errors
* Replace DB download link with a better download utility
* Wallet support?
