use iota::Client;
use std::time::{Duration, Instant};
use std::thread;
use std::sync::mpsc;
extern crate rgsl;

struct MsgResult {
    msg: iota::MessageId,
    delta_t: Duration,
}

fn main() {
    let url = "https://api.hornet01.alphanet.iota.cafe:443";
    let msg = "iota_rspammer be spammin'";
    let index = "INDEX";
    let n_threads = 4;

    let (tx, rx): (mpsc::SyncSender<MsgResult>, mpsc::Receiver<MsgResult>) = mpsc::sync_channel(n_threads);

    for n in 0..n_threads {
        let local_url = url.clone();
        let local_tx = tx.clone();
        let local_msg = msg.clone();
        let local_n = n.clone();
        let local_index = index.clone();

        thread::spawn( move || async move {
            let iota = Client::build() // Crate a client instance builder
                .with_node(local_url) // Insert the node here
                .unwrap()
                .with_node_sync_disabled()
                .with_local_pow(false)
                .with_request_timeout(Duration::new(500, 0))
                .finish()
                .unwrap();
            println!("Created IOTA Client n [{}]", local_n);
            loop {
                let start = Instant::now();
                let msg = format!("{} from thread [{}]!", local_msg, local_n);
                let message_id = iota.send().indexation(local_index).with_data(msg.as_bytes().to_vec()).finish().await.unwrap();
                let delta_t = start.elapsed();
                let msg_result = MsgResult{ msg: message_id, delta_t: delta_t };
                local_tx.send(msg_result).unwrap();
            }
        });
    }

    let mut delta_vec = vec![];
    let mut msg_vec = vec![];

    loop {
        let msg_result = rx.recv().unwrap();
        println!("messageId: {}, delta_t: {}", msg_result.msg, msg_result.delta_t.as_millis());
        msg_vec.push(msg_result.msg);
        delta_vec.push(msg_result.delta_t);
    }

}

// #[tokio::main]
// async fn main() {
//
//     let handle = thread::spawn(|| async {
//         let iota = Client::build() // Crate a client instance builder
//             .with_node("https://api.hornet01.alphanet.iota.cafe:443") // Insert the node here
//             .unwrap()
//             .with_node_sync_disabled()
//             .with_local_pow(false)
//             .with_request_timeout(Duration::new(500, 0))
//             .finish()
//             .unwrap();
//
//         let mut delta_vec = vec![];
//
//         loop {
//             let start = Instant::now();
//
//             println!("Sending message. Please wait for PoW.");
//             let message_id = iota.send().indexation("INDEX").with_data("Post-Chrysalis Spamming!".as_bytes().to_vec()).finish().await.unwrap();
//             let delta = start.elapsed();
//
//             delta_vec.push(delta.as_millis() as f64);
//
//             let delta_avg = rgsl::statistics::mean(&delta_vec, 1, delta_vec.len());
//             let delta_std_dev = rgsl::statistics::sd(&delta_vec, 1, delta_vec.len());
//
//             println!("{}", message_id);
//             println!("Time: {:?}, Average Time: {:?} ms, Std. Deviation Time: {:?} ms", delta, delta_avg, delta_std_dev);
//         }
//     });
//
//     handle.join().unwrap();
//
// }
