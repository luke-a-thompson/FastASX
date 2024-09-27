A paraser for the ITCH message format implemented by the NASDAQ & ASX written in Rust.

## Performance:
* Uses a lock-free queue for reading and parsing ITCH messages in parallel.
* Maintains a separate order book for each stock.
    * Supports adding, executing, replacing, cancelling and deleting orders.
* Builds an intraday stock directory from start-of-day directory messages.
* Logging to stdout.

## Performance:
* Parses ~40m messages per second on a Ryzen 5600X.
* Updates orderbooks at ~2m messages per second on a Ryzen 5600X (28/09/24).

Work in progress.
