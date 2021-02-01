use iota::{Api, Client};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
extern crate rgsl;

#[derive(Debug)]
struct MsgResult {
    msg: iota::MessageId,
    delta_t: Duration,
}

#[tokio::main]
async fn main() {
    let url = "http://api.hornet-1.testnet.chrysalis2.com";
    let msg = "iota_rspammer be spammin'";
    let index = "INDEX";
    let (tx, mut rx): (
        mpsc::UnboundedSender<MsgResult>,
        mpsc::UnboundedReceiver<MsgResult>,
    ) = mpsc::unbounded_channel();
    for n in 0..4 {
        let local_url = url.clone();
        let local_tx = tx.clone();
        let local_msg = msg.clone();
        let local_n = n.clone();
        let local_index = index.clone();
        tokio::spawn(async move {
            let iota = Client::builder() // Crate a client instance builder
                .with_node(local_url) // Insert the node here
                .unwrap()
                .with_local_pow(false)
                //.with_request_timeout(Duration::new(500, 0))
                .with_api_timeout(Api::PostMessageWithRemotePow, Duration::new(500, 0))
                .finish()
                .unwrap();
            println!("Created IOTA Client n [{}]", local_n);
            loop {
                let start = Instant::now();
                let msg = format!("{} from thread [{}]!", local_msg, local_n);
                let message = iota
                    .send()
                    .with_index(local_index)
                    .with_data(msg.as_bytes().to_vec())
                    .finish()
                    .await
                    .unwrap();
                let delta_t = start.elapsed();
                let msg_result = MsgResult {
                    msg: message.id().0,
                    delta_t: delta_t,
                };
                local_tx.send(msg_result).unwrap();
            }
        });
    }
    let mut delta_vec = vec![];
    let mut msg_vec = vec![];
    loop {
        let msg_result = rx.recv().await.unwrap();
        println!(
            "messageId: {}, delta_t: {}",
            msg_result.msg,
            msg_result.delta_t.as_millis()
        );
        msg_vec.push(msg_result.msg);
        delta_vec.push(msg_result.delta_t);
    }
}