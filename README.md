# IOTA RSpammer

Rust-based IOTA 1.5 Client Spammer (Post Chrysalis pt.2)

Uses asynchronous message passing between spammer threads, each with its own IOTA Client.

Allows custom Message Payload and Index, custom number of threads and optional local PoW.

Useful for experimentation with spamming where you don't want to necessarily use a Spammer Node Plugin.
Provides insight into underlying statistical patterns of the protocol from the perspective of an IOTA client.

## Progress
- [x] CLI args
- [x] Async MultiThread Messaging
- [x] Non-Valued Message
- [x] Non-Valued Message Variable Payload Size
- [ ] Valued Message (via `wallet.rs` + [faucet.testnet.chrysalis2.com](https://faucet.testnet.chrysalis2.com/))
- [ ] Valued Message Variable Payload Size
- [ ] Format output to files

## Build
```
$ sudo apt-get install libgsl0-dev
$ git clone https://github.com/bernardoaraujor/iota_rspammer.git
$ cd iota_rspammer
$ cargo build
```

## Run
```
$ cargo run -- -h
iota_rspammer 0.1.0

USAGE:
    iota_rspammer [FLAGS] [OPTIONS]

FLAGS:
    -h, --help         Prints help information
    -l, --local_pow    Enable local_pow
    -V, --version      Prints version information

OPTIONS:
    -i, --index <index>            Message index [default: iota_rspammer]
    -m, --msg_size <msg_size>      Message Payload Size (bytes) [default: 10]
    -n, --n_threads <n-threads>    Number of Spammer Threads [default: 1]
    -t, --timeout <timeout>        Set Timeout (seconds) [default: 500]
    -u, --url <url>                Node URL [default: http://api.hornet-1.testnet.chrysalis2.com]

```

```
$ cargo run -- -n 3 -m 32 -i rspammer_index -u http://api.hornet-1.testnet.chrysalis2.com/ -l
Starting iota_rspammer with the following parameters:
message payload size: 32 bytes
message index: iota_rspammer
node url: http://api.hornet-1.testnet.chrysalis2.com/
local PoW: false

Created IOTA Client 1.
Created IOTA Client 0.
thread n: 1, messageId: 112201254220a5322877ea31c4bc91d5fc3563737ac1d42930bfdb0751e3e636, confirmation time: 5557 ms, global average mps: 0.17995321216483715
thread n: 0, messageId: 623720ef043c0c8f653dff6cfcd1eb129c9c6c1a5b336608b02b0c6bc36ccb5f, confirmation time: 8799 ms, global average mps: 0.13931457230426303

```