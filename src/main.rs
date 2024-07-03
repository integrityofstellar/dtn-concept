mod bundle;
mod network;
mod node;
mod storage;

use bundle::Bundle;
use network::{run_server, send_bundle};
use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use storage::PersistentStorage;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "Usage: {} <Node Address> <Action> [<File Path> <Next Hop Address>]",
            args[0]
        );
        eprintln!("Action: 'send <file_path> <destination>' or 'receive'");
        std::process::exit(1);
    }

    let node_address = &args[1];
    let action = &args[2];

    match action.as_str() {
        "receive" => {
            run_server(node_address).await;
        }
        "send" => {
            if args.len() != 5 {
                eprintln!(
                    "Usage for send: {} <Node Address> send <file_path> <destination>",
                    args[0]
                );
                std::process::exit(1);
            }

            let file_path = &args[3];
            let destination = args[4].clone(); // Clone the destination string
            let mut file = File::open(file_path).expect("Unable to open file");
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)
                .expect("Unable to read file");

            let bundle = Bundle {
                id: "1".to_string(),
                payload: contents,
                timestamp: 0,
                destination: destination.clone(), // Clone the destination string
            };

            // Create persistent storage instance
            let storage = Arc::new(PersistentStorage::new("dtn_storage"));

            // Send the bundle with persistent storage
            send_bundle(destination, bundle, Arc::clone(&storage)).await;
        }
        _ => {
            eprintln!("Unknown action. Use 'send <file_path> <destination>' or 'receive'.");
            std::process::exit(1);
        }
    }
}
