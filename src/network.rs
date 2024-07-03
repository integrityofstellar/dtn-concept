use crate::bundle::Bundle;
use crate::storage::PersistentStorage;
use serde_cbor;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream}; // Import Arc for atomic reference counting

pub async fn handle_connection(
    mut stream: TcpStream,
    address: String,
    storage: Arc<PersistentStorage>,
) {
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await.unwrap();
    let bundle: Bundle = serde_cbor::from_slice(&buffer[..n]).unwrap();
    println!("Received bundle at {}: {:?}", address, bundle);

    // Store the bundle in persistent storage
    storage.store_bundle(&bundle);

    // Check if this is the final destination
    if bundle.destination == address {
        println!("Bundle has reached its final destination at {}", address);
        // Save the bundle payload to a file using storage method
        storage.save_bundle(&bundle, "received_file.txt");
    } else {
        println!("Relaying bundle to {}", bundle.destination);
        // Forward the bundle to the next hop
        let next_hop = bundle.destination.clone();
        send_bundle(next_hop, bundle, Arc::clone(&storage)).await;
    }
}

pub async fn run_server(address: &str) {
    let listener = TcpListener::bind(address).await.unwrap();
    let storage = Arc::new(PersistentStorage::new("dtn_storage")); // Wrap storage in Arc
    println!("Server running on {}", address);
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let addr = address.to_string();
        let storage = Arc::clone(&storage); // Clone Arc for each connection
        tokio::spawn(async move {
            handle_connection(stream, addr, storage).await;
        });
    }
}

pub async fn send_bundle(destination: String, bundle: Bundle, storage: Arc<PersistentStorage>) {
    let mut stream = TcpStream::connect(&destination).await.unwrap();
    let data = serde_cbor::to_vec(&bundle).unwrap();
    stream.write_all(&data).await.unwrap();
    println!("Sent bundle to {}", destination);

    // Store the bundle in persistent storage after sending
    storage.store_bundle(&bundle);
}
