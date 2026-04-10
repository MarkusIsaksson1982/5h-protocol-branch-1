use petgraph::graph::DiGraph;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub r#type: String, // "type" is a Rust keyword → renamed in JSON
    pub verification_level: u8,
    pub visibility: String,
    pub ai_proxy_allowed: Option<bool>,
    // Additional fields can be added later; this subset is enough for the test vector
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub r#type: String,
    pub mutual_consent: bool,
}

#[derive(Debug)]
pub struct FiveHGraph {
    pub graph: DiGraph<Node, Edge>,
    pub node_index: HashMap<String, petgraph::graph::NodeIndex>,
}

impl FiveHGraph {
    /// Loads the official 15-node test vector from the spec directory
    pub fn load_test_vector() -> Result<Self, String> {
        let path = "/app/../spec/test-vectors/15-node-graph.json";
        let json =
            fs::read_to_string(path).map_err(|e| format!("Failed to read test vector: {}", e))?;

        let data: serde_json::Value =
            serde_json::from_str(&json).map_err(|e| format!("JSON parse error: {}", e))?;

        let nodes: Vec<Node> = serde_json::from_value(data["nodes"].clone())
            .map_err(|e| format!("Node deserialization failed: {}", e))?;

        let edges: Vec<Edge> = serde_json::from_value(data["edges"].clone())
            .map_err(|e| format!("Edge deserialization failed: {}", e))?;

        let mut graph = DiGraph::new();
        let mut node_index = HashMap::new();

        for node in nodes {
            let idx = graph.add_node(node.clone());
            node_index.insert(node.id.clone(), idx);
        }

        for edge in edges {
            if let (Some(&from_idx), Some(&to_idx)) =
                (node_index.get(&edge.from), node_index.get(&edge.to))
            {
                if edge.mutual_consent {
                    graph.add_edge(from_idx, to_idx, edge);
                }
            }
        }

        println!(
            "✅ Successfully loaded 15-node test vector ({} nodes, {} edges)",
            graph.node_count(),
            graph.edge_count()
        );

        Ok(FiveHGraph { graph, node_index })
    }
}
