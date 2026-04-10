#[cfg(test)]
mod integration_tests {
    use crate::{FiveHGraph, PathQuery, find_privacy_preserving_path, KeyPair};
    use reqwest::Client;

    #[tokio::test]
    async fn test_rust_to_python_proxy_integration() {
        // 1. Load real graph
        let graph = FiveHGraph::load_test_vector().unwrap();

        // 2. Find a path
        let path_result = find_privacy_preserving_path(&graph, PathQuery {
            from_did: "did:5h:alice".to_string(),
            to_did: "did:5h:margaret".to_string(),
            max_hops: 5,
        });
        assert!(path_result.reachable);

        // 3. Create a minimal ContactRequest
        let request = serde_json::json!({
            "request_id": uuid::Uuid::new_v7().to_string(),
            "requester_did": "did:5h:alice",
            "target_did": "did:5h:margaret",
            "hop_number": 1,
            "intent": { "summary": "Test professional inquiry", "anonymity_level": "identified", "intent_type": "professional_inquiry" },
            "preferred_outcome": "forward",
            "consent_receipts": [],
            "signature": "placeholder",
            "encryption": "aes-256-gcm"
        });

        // 4. Send to Python proxy (Branch 2)
        let client = Client::new();
        let response = client.post("http://localhost:8000/v1/proxy/forward")
            .json(&request)
            .send()
            .await
            .expect("Failed to reach Python proxy");

        assert_eq!(response.status(), 200);
        println!("✅ Rust → Python proxy integration test passed");

        // 5. Quick crypto round-trip
        let keys = KeyPair::generate();
        let msg = b"test message from Rust core";
        let sig = keys.sign(msg);
        assert!(keys.verify(msg, &sig));
        println!("✅ Ed25519 signing/verification test passed");
    }
}
