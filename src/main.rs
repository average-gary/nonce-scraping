use bitcoincore_rpc::{Auth, Client, RpcApi};
use serde::Serialize;
use std::fs::File;

fn main() {
    let rpc = Client::new(
        "http://localhost:8332",
        Auth::UserPass("mempool".to_string(), "mempool".to_string()),
    )
    .unwrap();
    let stop_height = 854682;
    let best_block_hash = rpc.get_best_block_hash().unwrap();
    let mut current_block = rpc.get_block_info(&best_block_hash).unwrap();
    let mut nonces_stuff = vec![NonceData {
        timestamp: current_block.time,
        nonce: current_block.nonce,
        height: current_block.height,
    }];
    let mut height = current_block.height;
    while height != stop_height {
        current_block = rpc
            .get_block_info(&current_block.previousblockhash.unwrap())
            .unwrap();
        height = current_block.height;
        println!("Working on block: {} with nonce {}", height, current_block.nonce);
        nonces_stuff.push(NonceData {
            timestamp: current_block.time,
            nonce: current_block.nonce,
            height: current_block.height,
        });
    }
    let file_name = format!("{}-nonces_data.csv", stop_height);
    let file = File::create(file_name).expect("Unable to create file");
    let mut wtr = csv::Writer::from_writer(file);
    for nonce in &nonces_stuff {
        if let Err(err) = wtr.serialize(nonce) {
            println!("Error serializing data: {}", err);
        }
    }

    // Call this method to ensure all bytes are written out before the file is closed.
    wtr.flush().expect("Unable to flush writer");
}

#[derive(Debug, Serialize)]
struct NonceData {
    timestamp: usize,
    nonce: u32,
    height: usize,
}
