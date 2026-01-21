# `block-dn` Client

This is a client implementation for a subset of the [`block-dn`](https://github.com/guggero/block-dn) endpoints. `block-dn` is a server that allows for Bitcoin light clients to sync to the blockchain in a fast and privacy preserving way.

# Routes available

`/`: The root HTML of the server, suitable for web rendering.

`/status`: The status of the server, including sync height, genesis block, etc.

`headers/<start_block>`: Returns up to 100_000 block headers.

