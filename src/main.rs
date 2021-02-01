use iota::{Api, Client};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use structopt::StructOpt;
use url::{Url};
extern crate rgsl;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Message Payload
    #[structopt(short = "m", long = "msg", default_value = "iota_rspammer be spammin'!")]
    msg: String,

    /// Message index
    #[structopt(short = "i", long = "index", default_value = "iota_rspammer")]
    index: String,

    /// Node URL
    #[structopt(short = "u", long = "url", parse(try_from_str = Url::parse), default_value = "http://api.hornet-1.testnet.chrysalis2.com")]
    url: Url,

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
    delta_t: Duration,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    let msg = opt.msg;
    let index = opt.index;
    let url = opt.url;
    let n_threads = opt.n_threads;
    let local_pow = opt.local_pow;

    println!("Starting iota_rspammer with the following parameters:");
    println!("message payload: {}", msg);
    println!("message index: {}", index);
    println!("node url: {}", url.as_str());
    println!("local PoW: {}\n", local_pow);

    let (tx, mut rx): (
        mpsc::UnboundedSender<MsgResult>,
        mpsc::UnboundedReceiver<MsgResult>,
    ) = mpsc::unbounded_channel();
    for n in 0..n_threads {
        let thread_url = url.clone();
        let thread_tx = tx.clone();
        let thread_msg = msg.clone();
        let thread_n = n.clone();
        let thread_index = index.clone();
        let thread_local_pow = local_pow.clone();

        tokio::spawn(async move {
            let iota = Client::builder() // Crate a client instance builder
                .with_node(thread_url.as_str()) // Insert the node here
                .unwrap()
                .with_local_pow(thread_local_pow)
                //.with_request_timeout(Duration::new(500, 0))
                .with_api_timeout(Api::PostMessageWithRemotePow, Duration::new(500, 0))
                .finish()
                .unwrap();
            println!("Created IOTA Client n [{}]", thread_n);
            loop {
                let start = Instant::now();
                let msg = format!("{} from thread [{}]!", thread_msg, thread_n);
                let message = iota
                    .send()
                    .with_index(&thread_index)
                    .with_data(msg.as_bytes().to_vec())
                    .finish()
                    .await
                    .unwrap();
                let delta_t = start.elapsed();
                let msg_result = MsgResult {
                    thread_n: n,
                    msg: message.id().0,
                    delta_t: delta_t,
                };
                thread_tx.send(msg_result).unwrap();
            }
        });
    }
    let mut delta_vec = vec![];
    let mut msg_vec = vec![];
    loop {
        let msg_result = rx.recv().await.unwrap();
        msg_vec.push(msg_result.msg);

        delta_vec.push(msg_result.delta_t.as_millis() as f64);
        let delta_avg = rgsl::statistics::mean(&delta_vec, 1, delta_vec.len());

        println!(
            "thread n: {}, messageId: {}, duration: {} ms, average mps: {}",
            msg_result.thread_n,
            msg_result.msg,
            msg_result.delta_t.as_millis(),
            1.0 / delta_avg*1000.0
        );
    }
}