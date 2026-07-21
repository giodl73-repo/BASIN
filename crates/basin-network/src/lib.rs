use std::collections::{HashMap, HashSet};

use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Undirected;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Typed hydrologic basis for a conveyance rating (REQ-007): is the capacity
/// stated against firm yield (drought-of-record) or an average year?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HydroBasis {
    FirmYield,
    AverageYear,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Conveyance {
    pub id: String,
    pub capacity_af: f64,
    pub basis: HydroBasis,
}

#[derive(Debug, Error, PartialEq)]
pub enum NetworkError {
    #[error("node id already exists: {0}")]
    DuplicateNode(String),
    #[error("conveyance id already exists: {0}")]
    DuplicateConveyance(String),
    #[error("unknown node id: {0}")]
    UnknownNode(String),
    #[error("conveyance capacity_af must be positive for {conveyance_id}: {capacity_af}")]
    NonPositiveCapacity {
        conveyance_id: String,
        capacity_af: f64,
    },
}

#[derive(Debug, Default)]
pub struct Network {
    graph: Graph<Node, Conveyance, Undirected>,
    nodes_by_id: HashMap<String, NodeIndex>,
    conveyance_ids: HashSet<String>,
}

impl Network {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node: Node) -> Result<(), NetworkError> {
        if self.nodes_by_id.contains_key(&node.id) {
            return Err(NetworkError::DuplicateNode(node.id));
        }
        let id = node.id.clone();
        let index = self.graph.add_node(node);
        self.nodes_by_id.insert(id, index);
        Ok(())
    }

    pub fn add_conveyance(
        &mut self,
        from_node: &str,
        to_node: &str,
        conveyance: Conveyance,
    ) -> Result<(), NetworkError> {
        if conveyance.capacity_af <= 0.0 {
            return Err(NetworkError::NonPositiveCapacity {
                conveyance_id: conveyance.id,
                capacity_af: conveyance.capacity_af,
            });
        }
        if self.conveyance_ids.contains(&conveyance.id) {
            return Err(NetworkError::DuplicateConveyance(conveyance.id));
        }
        let from = self.node_index(from_node)?;
        let to = self.node_index(to_node)?;
        let conveyance_id = conveyance.id.clone();
        self.graph.add_edge(from, to, conveyance);
        self.conveyance_ids.insert(conveyance_id);
        Ok(())
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn conveyance_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// Degree of a node — the connectivity/centrality proxy for DIM-04.
    pub fn degree(&self, node_id: &str) -> Result<usize, NetworkError> {
        let index = self.node_index(node_id)?;
        Ok(self.graph.edges(index).count())
    }

    pub fn incident_capacity_af(&self, node_id: &str) -> Result<f64, NetworkError> {
        let index = self.node_index(node_id)?;
        Ok(self
            .graph
            .edges(index)
            .map(|edge| edge.weight().capacity_af)
            .sum())
    }

    /// Redundant incident capacity (AF) after losing the single largest incident
    /// conveyance — used by tier SLA conformance.
    pub fn redundant_capacity_af(&self, node_id: &str) -> Result<f64, NetworkError> {
        let index = self.node_index(node_id)?;
        let mut total = 0.0_f64;
        let mut largest = 0.0_f64;
        for edge in self.graph.edges(index) {
            let capacity = edge.weight().capacity_af;
            total += capacity;
            largest = largest.max(capacity);
        }
        Ok(total - largest)
    }

    pub fn is_connected(&self, a: &str, b: &str) -> Result<bool, NetworkError> {
        let start = self.node_index(a)?;
        let goal = self.node_index(b)?;
        Ok(self.reachable(start, goal, None))
    }

    /// True when a second node-disjoint path survives removal of any single
    /// intermediate node — resilience for water supply.
    pub fn has_diverse_path(&self, a: &str, b: &str) -> Result<bool, NetworkError> {
        let start = self.node_index(a)?;
        let goal = self.node_index(b)?;
        if start == goal || !self.reachable(start, goal, None) {
            return Ok(false);
        }
        for node in self.graph.node_indices() {
            if node == start || node == goal {
                continue;
            }
            if !self.reachable(start, goal, Some(node)) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn node_index(&self, node_id: &str) -> Result<NodeIndex, NetworkError> {
        self.nodes_by_id
            .get(node_id)
            .copied()
            .ok_or_else(|| NetworkError::UnknownNode(node_id.to_string()))
    }

    fn reachable(&self, start: NodeIndex, goal: NodeIndex, excluded: Option<NodeIndex>) -> bool {
        if Some(start) == excluded || Some(goal) == excluded {
            return false;
        }
        let mut visited = HashSet::new();
        let mut stack = vec![start];
        while let Some(node) = stack.pop() {
            if node == goal {
                return true;
            }
            if Some(node) == excluded || !visited.insert(node) {
                continue;
            }
            for edge in self.graph.edges(node) {
                let neighbor = if edge.target() == node {
                    edge.source()
                } else {
                    edge.target()
                };
                if Some(neighbor) != excluded && !visited.contains(&neighbor) {
                    stack.push(neighbor);
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(id: &str) -> Node {
        Node {
            id: id.to_string(),
            name: format!("{id} node"),
            kind: "reservoir".to_string(),
        }
    }

    fn conveyance(id: &str, capacity_af: f64, basis: HydroBasis) -> Conveyance {
        Conveyance {
            id: id.to_string(),
            capacity_af,
            basis,
        }
    }

    fn three_node_system() -> Network {
        let mut network = Network::new();
        network.add_node(node("a")).expect("node a accepted");
        network.add_node(node("b")).expect("node b accepted");
        network.add_node(node("c")).expect("node c accepted");
        network
            .add_conveyance("a", "b", conveyance("ab", 1000.0, HydroBasis::FirmYield))
            .expect("conveyance ab accepted");
        network
            .add_conveyance("b", "c", conveyance("bc", 750.0, HydroBasis::AverageYear))
            .expect("conveyance bc accepted");
        network
    }

    #[test]
    fn builds_graph_and_counts_nodes_and_conveyances() {
        let network = three_node_system();
        assert_eq!(network.node_count(), 3);
        assert_eq!(network.conveyance_count(), 2);
    }

    #[test]
    fn incident_capacity_sums_conveyance_ratings() {
        let network = three_node_system();
        assert_eq!(network.incident_capacity_af("b"), Ok(1750.0));
        assert_eq!(network.incident_capacity_af("a"), Ok(1000.0));
    }

    #[test]
    fn redundant_capacity_removes_largest_incident_conveyance() {
        let network = three_node_system();
        assert_eq!(network.redundant_capacity_af("b"), Ok(750.0));
        assert_eq!(network.redundant_capacity_af("a"), Ok(0.0));
    }

    #[test]
    fn degree_counts_incident_conveyances() {
        let network = three_node_system();
        assert_eq!(network.degree("b"), Ok(2));
        assert_eq!(network.degree("c"), Ok(1));
    }

    #[test]
    fn connectivity_distinguishes_reachable_and_gap() {
        let mut network = three_node_system();
        network.add_node(node("z")).expect("node z accepted");
        assert_eq!(network.is_connected("a", "c"), Ok(true));
        assert_eq!(network.is_connected("a", "z"), Ok(false));
    }

    #[test]
    fn system_loop_has_diverse_path() {
        let mut network = three_node_system();
        network
            .add_conveyance("a", "c", conveyance("ac", 500.0, HydroBasis::FirmYield))
            .expect("conveyance ac accepted");
        assert_eq!(network.has_diverse_path("a", "c"), Ok(true));
        assert_eq!(network.has_diverse_path("a", "b"), Ok(true));
    }

    #[test]
    fn single_path_system_has_no_diverse_path() {
        let network = three_node_system();
        assert_eq!(network.has_diverse_path("a", "c"), Ok(false));
    }

    #[test]
    fn conveyance_basis_is_preserved() {
        let network = three_node_system();
        let bases = network
            .graph
            .edge_weights()
            .map(|conveyance| conveyance.basis)
            .collect::<HashSet<_>>();
        assert!(bases.contains(&HydroBasis::FirmYield));
        assert!(bases.contains(&HydroBasis::AverageYear));
    }

    #[test]
    fn duplicate_node_is_rejected_with_typed_error() {
        let mut network = Network::new();
        network.add_node(node("a")).expect("first node accepted");
        assert_eq!(
            network.add_node(node("a")),
            Err(NetworkError::DuplicateNode("a".to_string()))
        );
    }

    #[test]
    fn duplicate_conveyance_is_rejected_with_typed_error() {
        let mut network = three_node_system();
        assert_eq!(
            network.add_conveyance("a", "c", conveyance("ab", 250.0, HydroBasis::FirmYield)),
            Err(NetworkError::DuplicateConveyance("ab".to_string()))
        );
    }

    #[test]
    fn non_positive_capacity_is_rejected_with_typed_error() {
        let mut network = Network::new();
        network.add_node(node("a")).expect("node a accepted");
        network.add_node(node("b")).expect("node b accepted");
        assert_eq!(
            network.add_conveyance("a", "b", conveyance("ab", 0.0, HydroBasis::FirmYield)),
            Err(NetworkError::NonPositiveCapacity {
                conveyance_id: "ab".to_string(),
                capacity_af: 0.0
            })
        );
    }

    #[test]
    fn unknown_node_is_rejected_with_typed_error() {
        let mut network = Network::new();
        network.add_node(node("a")).expect("node a accepted");
        assert_eq!(
            network.add_conveyance(
                "a",
                "missing",
                conveyance("am", 10.0, HydroBasis::FirmYield)
            ),
            Err(NetworkError::UnknownNode("missing".to_string()))
        );
        assert_eq!(
            network.degree("missing"),
            Err(NetworkError::UnknownNode("missing".to_string()))
        );
    }
}
