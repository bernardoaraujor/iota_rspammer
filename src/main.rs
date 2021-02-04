use iota::{Api, Client};
use rand::Rng;
use std::time::{Duration, Instant};
use structopt::StructOpt;
use tokio::sync::mpsc;
use url::{Url};

fn parse_msg_size(size_str: String) -> usize {
    let size = match size_str.parse::<i32>() {
        Ok(n) => n,
        Err(_) => panic!("msg_size parameter must be integer, got {}.", size_str),
    };
    if size < 0 {
        panic!("msg_size parameter must be non-negative integer.");
    }
    size as usize
}

fn get_random_msg(size: usize) -> Vec<u8>{
    let mut rng = rand::thread_rng();
    let mut random_bytes = vec![];
    for _ in 0..size {
        random_bytes.push(rng.gen::<u8>());
    }

    random_bytes
}

#[derive(Debug, StructOpt)]
struct Opt {
    /// Message Size (bytes)
    #[structopt(short = "m", long = "msg_size", default_value = "10")]
    msg: String,

    /// Message index
    #[structopt(short = "i", long = "index", default_value = "iota_rspammer")]
    index: String,

    /// Node URL
    #[structopt(short = "u", long = "url", parse(try_from_str = Url::parse), default_value = "http://api.hornet-1.testnet.chrysalis2.com")]
    url: Url,

    /// Netword ID
    #[structopt(short = "d", long = "network_id", default_value = "alphanet1")]
    network_id: String,

    /// Number of Spammer Threads
    #[structopt(short = "n", long = "n_threads", default_value = "1")]
    n_threads: u32,

    /// Enable local_pow
    #[structopt(short = "l", long = "local_pow")]
    local_pow: bool,

    /// Set Timeout (seconds)
    #[structopt(short = "t", long = "timeout", default_value = "500")]
    timeout: u32,
}

#[derive(Debug)]
struct MsgResult {
    thread_n: u32,
    msg: iota::MessageId,
    confirmation_t: Duration,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    let msg_size_str = opt.msg;
    let index = opt.index;
    let url = opt.url;
    let network_id = opt.network_id;
    let n_threads = opt.n_threads;
    let local_pow = opt.local_pow;

    let msg_size = parse_msg_size(msg_size_str);

    println!("Starting iota_rspammer with the following parameters:");
    println!("message payload size: {} bytes", msg_size);
    println!("message index: {}", index);
    println!("node url: {}", url.as_str());
    println!("network ID: {}", network_id);
    println!("local PoW: {}\n", local_pow);

    let (tx, mut rx): (
        mpsc::UnboundedSender<MsgResult>,
        mpsc::UnboundedReceiver<MsgResult>,
    ) = mpsc::unbounded_channel();
    for n in 0..n_threads {
        let thread_url = url.clone();
        let thread_network_id = network_id.clone();
        let thread_tx = tx.clone();
        let thread_n = n.clone();
        let thread_index = index.clone();
        let thread_local_pow = local_pow.clone();

        tokio::spawn(async move {
            let iota = Client::builder() // Crate a client instance builder
                .with_node(thread_url.as_str()) // Insert the node here
                .unwrap()
                .with_local_pow(thread_local_pow)
                .with_network(thread_network_id.as_str())
                //.with_request_timeout(Duration::new(500, 0))
                .with_api_timeout(Api::PostMessageWithRemotePow, Duration::new(500, 0))
                .finish()
                .await
                .unwrap();

            println!("Created IOTA Client {}.", thread_n);
            loop {
                match iota.get_health().await.unwrap() {
                    true => (),
                    false => panic!("unhealthy node!"),
                };

                let start = Instant::now();
                let msg = get_random_msg(msg_size);
                let message = iota
                    .send()
                    .with_index(&thread_index)
                    .with_data(msg.to_vec())
                    .finish()
                    .await
                    .unwrap();
                let confirmation_t = start.elapsed();
                let msg_result = MsgResult {
                    thread_n: n,
                    msg: message.id().0,
                    confirmation_t: confirmation_t,
                };
                thread_tx.send(msg_result).unwrap();
            }
        });
    }
    let mut delta_vec = vec![];
    let mut msg_vec = vec![];
    loop {
        let start = Instant::now();
        let msg_result = rx.recv().await.unwrap();
        let delta_t = start.elapsed();

        msg_vec.push(msg_result.msg);
        delta_vec.push(delta_t.as_millis() as f64);

        let delta_avg = mean(&delta_vec);

        println!(
            "thread n: {}, messageId: {}, confirmation time: {} ms, global average mps: {}",
            msg_result.thread_n,
            msg_result.msg,
            msg_result.confirmation_t.as_millis(),
            1.0 / delta_avg * 1000.0
        );
    }
}

fn mean(list: &[f64]) -> f64 {
    let sum: f64 = Iterator::sum(list.iter());
    sum / (list.len() as f64)
}