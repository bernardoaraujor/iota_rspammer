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
- [ ] Non-Valued Message Variable Payload
- [ ] Non-Valued Message Statistics
- [ ] Valued Message (via `wallet.rs` + [faucet.testnet.chrysalis2.com](https://faucet.testnet.chrysalis2.com/))
- [ ] Valued Message Variable Payload
- [ ] Valued Message Statistics
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
    -m, --msg <msg>                Message Payload [default: iota_rspammer be spammin'!]
    -n, --n_threads <n-threads>    Number of Spammer Threads [default: 1]
    -t, --timeout <timeout>        Set Timeout (seconds) [default: 500]
    -u, --url <url>                Node URL [default: http://api.hornet-1.testnet.chrysalis2.com]

```

```
$ cargo run -- -n 3 -m msg_payload -i msg_index -u http://api.hornet-1.testnet.chrysalis2.com/ -l
Starting iota_rspammer with the following parameters:
message payload: msg_payload
message index: msg_index
node url: http://api.hornet-1.testnet.chrysalis2.com/
local PoW: true

Created IOTA Client n [0]
Created IOTA Client n [2]
Created IOTA Client n [1]
thread n: 2, messageId: da36752454db418e0e6405c662226a4aa6a2f6aa3668cf2ebc6ee55519bedc30, duration: 8048 ms, average mps: 0.12425447316103379
thread n: 1, messageId: 4e289fa5e1f75e2b49df47763e34c2d7aa0a3d30bb90864aad5e0cb90dcacb15, duration: 9071 ms, average mps: 0.11682925404521291
thread n: 2, messageId: 33a666fd42910f6f8be5c2803aca4e3fc7012815bb543e2a6bac0a756c95ea2e, duration: 3993 ms, average mps: 0.14209928003031452
thread n: 2, messageId: da3e9b8d840237ac2aee19b580b637d94e975ab004b6ba912717163e1047d360, duration: 3174 ms, average mps: 0.1647039446594746
thread n: 0, messageId: 2be344290c1eaa85aa961812e9aeae4663eb4ad02be789ec68a73438e3c25e00, duration: 28529 ms, average mps: 0.09467007478935908

```