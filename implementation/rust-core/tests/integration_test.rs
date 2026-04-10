#[cfg(test)]
mod integration {
    use five_h_core::{FiveHGraph, PathQuery, find_privacy_preserving_path, KeyPair};
    use reqwest::Client;

    #[tokio::test]
    async fn full_protocol_integration_test() {
        println!("🧪 Running full 5H Protocol integration test suite...");

        // 1. Load real graph
        let graph = FiveHGraph::load_test_vector().unwrap();

        // 2. Privacy-preserving pathfinding
        let path = find_privacy_preserving_path(&graph, PathQuery {
            from_did: "did:5h:alice".to_string(),
            to_did: "did:5h:margaret".to_string(),
            max_hops: 5,
        });
        assert!(path.reachable);

        // 3. Crypto
        let keys = KeyPair::generate();
        let msg = b"test";
        let sig = keys.sign(msg);
        assert!(keys.verify(msg, &sig));

        // 4. Federation anchoring
        let anchor = five_h_core::federation_anchor::Anchor::anchor_graph(&graph);
        assert!(!anchor.merkle_root.is_empty());

        println!("🎉 All integration tests passed!");
    }
}