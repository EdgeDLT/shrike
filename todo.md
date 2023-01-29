## To Do List

### Backend:

* Add more useful methods
	- e.g. 1: Method to get all transfers by address
	- e.g. 2: Method to get all transactions by block index/hash
* Make a separate table (view?) for transfers/contracts/balances and associated queries
* Add DB download utility for easy data sharing
* Contract validation/blacklisting (prevent stat manipulation via fake events)
* Add graceful shutdown for SIGTERM
* Create shared lib

### Frontend:

* JSON to CSV converter for transfers by address
* Add more methods and stats aimed at regular users
* Base58/64 decoding?
* Prettify JSON values
* Replace DB download link with a better download utility
* Wallet support?
