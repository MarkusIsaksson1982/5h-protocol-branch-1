//! 5H Protocol – Full End-to-End Flow Example (v0.3.1)
//! Grok-led reference implementation with semantic signature verification

use five_h_core::{FiveHGraph, PathQuery, find_privacy_preserving_path, KeyPair};
use reqwest::Client;
use serde_json::json;
use uuid::Timestamp;
use sha2::{Sha256, Digest};
use base64::{engine::general_purpose, Engine as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 5H Protocol Full Flow Demo v0.3.1 (Semantic Signature Verification)");

    // 1. Load the real 15-node test vector
    let graph = FiveHGraph::load_test_vector()?;
    println!("✅ Loaded 15-node test graph");

    // 2. Find a privacy-preserving path (Alice → Margaret)
    let path_result = find_privacy_preserving_path(
        &graph,
        PathQuery {
            from_did: "did:5h:alice".to_string(),
            to_did: "did:5h:margaret".to_string(),
            max_hops: 5,
        },
    );
    println!("✅ Path found: {} hops", path_result.estimated_hops);

    // 3. Generate keypair (used for both requester_public_key_b64 and signing)
    let keys = KeyPair::generate();
    let requester_public_key_b64 = general_purpose::URL_SAFE_NO_PAD.encode(keys.public_key_bytes());

    // 4. Build the request WITHOUT the signature field first
    let mut request_without_sig = json!({
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
        "ttl_hops": 5,
        "encryption": "aes-256-gcm",
        // NEW (change 1): embed public key for semantic verification
        "requester_public_key_b64": requester_public_key_b64,
    });

    // 5. Compute RFC 8785 canonical JSON (sorted keys, no whitespace) + SHA-256
    // (This matches spec/execution/canonical-serialization.md)
    let canonical_json = serde_json::to_string(&request_without_sig)
        .unwrap();  // already sorted by serde_json with sort_keys in practice; exact RFC8785 logic in production lib
    let mut hasher = Sha256::new();
    hasher.update(canonical_json.as_bytes());
    let hash = hasher.finalize();

    // 6. Sign the hash (change 3)
    let signature_bytes = keys.sign(&hash);
    let signature_b64 = general_purpose::URL_SAFE_NO_PAD.encode(signature_bytes);

    // 7. Add the signature to the final payload
    request_without_sig["signature"] = json!(signature_b64);

    println!("✅ Signed ContactRequest (semantic, over SHA-256 canonical hash)");

    // 8. Send to Branch 2 proxy
    let proxy_url = std::env::var("PROXY_URL")
        .unwrap_or_else(|_| "http://python-proxy:8000/v1/proxy/forward".to_string());

    let client = Client::new();
    let res = client
        .post(&proxy_url)
        .json(&request_without_sig)
        .send()
        .await?;

    let status = res.status();
    let body = res.text().await?;
    println!("✅ Proxy response (status {}): {}", status, body);

    Ok(())
}