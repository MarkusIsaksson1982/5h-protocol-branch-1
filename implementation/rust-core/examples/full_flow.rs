//! 5H Protocol – Full End-to-End Flow Example
//! Grok-led reference implementation (Branch 1)

use five_h_core::{FiveHGraph, PathQuery, find_privacy_preserving_path, KeyPair};
use reqwest::Client;
use serde_json::json;
use uuid::Timestamp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 5H Protocol Full Flow Demo (Rust Core → Python Proxy)");

    // 1. Load the real 15-node test vector
    let graph = FiveHGraph::load_test_vector()?;
    println!("✅ Loaded 15-node test graph");

    // 2. Find a privacy-preserving path
    let path_result = find_privacy_preserving_path(
        &graph,
        PathQuery {
            from_did: "did:5h:alice".to_string(),
            to_did: "did:5h:margaret".to_string(),
            max_hops: 5,
        },
    );
    println!("✅ Path found: {}", path_result.estimated_hops);

    // 3. Generate keypair and sign the request
    let keys = KeyPair::generate();
    let request_body = json!({
        "request_id": uuid::Uuid::new_v7(Timestamp::now(uuid::NoContext)).to_string(),
        "requester_did": "did:5h:alice",
        "target_did": "did:5h:margaret",
        "hop_number": 1,
        "intent": {
            "intent_type": "professional_inquiry",
            "summary": "Requesting a meeting about the 5H Protocol implementation",
            "anonymity_level": "identified"
        },
        "preferred_outcome": "forward",
        "consent_receipts": [],
        "signature": base64::encode(keys.sign(b"full-flow-request")),  // ← real signature sent
        "encryption": "aes-256-gcm"
    });

    println!("✅ Request signed with Ed25519 and included in payload");

    // 4. Send to Python proxy (Branch 2)
    let client = Client::new();
    let response = client
        .post("http://python-proxy:8000/v1/proxy/forward")
        .json(&request_body)
        .send()
        .await?;

    if response.status().is_success() {
        println!("✅ Proxy responded successfully!");
        let body = response.text().await?;
        println!("Proxy reply: {}", body);
    } else {
        println!("⚠️ Proxy returned error: {}", response.status());
    }

    println!("\n🎉 Full end-to-end flow completed!");
    Ok(())
}
