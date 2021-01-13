use iota::Client;
use std::time::{Duration, Instant};
extern crate rgsl;

#[tokio::main]
async fn main() {
    let iota = Client::build() // Crate a client instance builder
        .with_node("http://192.168.0.86:14267") // Insert the node here
        .unwrap()
        .with_node_sync_disabled()
        .with_local_pow(false)
        .with_request_timeout(Duration::new(500, 0))
        .finish()
        .unwrap();

    let mut delta_vec = vec![];

    loop {
        let start = Instant::now();

        println!("Sending message. Please wait for PoW.");
        let message_id = iota.send().indexation("INDEX").with_data("Post-Chrysalis Spamming!".as_bytes().to_vec()).finish().await.unwrap();
        let delta = start.elapsed();

        delta_vec.push(delta.as_millis() as f64);

        let delta_avg = rgsl::statistics::mean(&delta_vec, 1, delta_vec.len());
        let delta_std_dev = rgsl::statistics::sd(&delta_vec, 1, delta_vec.len());

        println!("{}", message_id);
        println!("Time: {:?}, Average Time: {:?} ms, Std. Deviation Time: {:?} ms", delta, delta_avg, delta_std_dev);
    }
}
