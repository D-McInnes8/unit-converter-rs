use std::collections::VecDeque;
use std::fmt::Debug;

use log::{debug, warn};

pub type NodeIndex = usize;
pub type EdgeIndex = usize;

#[derive(Debug)]
pub struct NodeData<T> {
    value: T,
    edges: Vec<EdgeIndex>,
}

#[derive(Debug)]
pub struct EdgeData<T> {
    target: NodeIndex,
    weight: T,
}

pub struct Graph<N, E>
where
    N: PartialEq + Debug,
{
    pub id: String,
    nodes: Vec<NodeData<N>>,
    edges: Vec<EdgeData<E>>,
}

#[derive(Debug, PartialEq)]
pub enum GraphOperationError {
    NodeDoesNotExist,
    NodeAlreadyExistsForValue,
}

impl<N, E> Graph<N, E>
where
    N: PartialEq + Debug,
{
    pub fn default() -> Graph<N, E> {
        Graph {
            id: String::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn new(id: String) -> Graph<N, E> {
        Graph {
            id,
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, value: N) -> NodeIndex {
        for (i, node) in self.nodes.iter().enumerate() {
            if node.value == value {
                /*debug!(
                    "Attempted to insert a node with a value {:?} that already exists, returning existing ({})",
                    &value,
                    existing
                );*/
                return i;
            }
        }

        let index = self.nodes.len();
        debug!(
            "Inserting new node for value {:?} at index {}",
            &value, &index
        );
        self.nodes.push(NodeData {
            value,
            edges: Vec::new(),
        });
        index
    }

    pub fn add_edge(
        &mut self,
        source: NodeIndex,
        target: NodeIndex,
        weight: E,
    ) -> Result<(), GraphOperationError> {
        if source >= self.nodes.len() || target >= self.nodes.len() {
            warn!("Failed trying to create edge because at least one node doesn't exist (Source: {}, Target: {})", source, target);
            return Err(GraphOperationError::NodeDoesNotExist);
        }

        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];

        self.edges.push(EdgeData { target, weight });

        node_data.edges.push(edge_index);
        Ok(())
    }

    pub fn get_node_index(&self, node_value: N) -> Option<NodeIndex> {
        for (i, node) in self.nodes.iter().enumerate() {
            if node.value == node_value {
                return Some(i);
            }
        }
        None
    }

    pub fn get_edge_weight(&self, source: NodeIndex, target: NodeIndex) -> Option<&E> {
        debug!(
            "Getting edge weight between nodes {} [{:?}] and {} [{:?}]",
            source, self.nodes[source].value, target, self.nodes[target].value
        );
        if source >= self.nodes.len() || target >= self.nodes.len() {
            warn!("Attempting to get the edge weight for a node that is out of bounds");
            return None;
        }

        for edge in &self.nodes[source].edges {
            let edge_data = &self.edges[*edge];
            /*debug!(
                "{:?} -> {:?} = {:?}",
                self.nodes[source].value, self.nodes[edge_data.target].value, edge_data.weight
            );*/
            if edge_data.target == target {
                //debug!("Found edge weight, returning {:?}", &edge_data.weight);
                return Some(&edge_data.weight);
            }
        }

        warn!(
            "Unable to find edge weight between nodes {} and {}",
            source, target
        );
        None
    }

    pub fn shortest_path(&self, source: NodeIndex, target: NodeIndex) -> Vec<(&N, &E)> {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.nodes.len()];
        let mut dist = vec![usize::MAX; self.nodes.len()];
        let mut path = vec![None; self.nodes.len()];

        visited[source] = true;
        dist[source] = 0;
        queue.push_back(source);

        while let Some(node) = queue.pop_front() {
            for edge in &self.nodes[node].edges {
                let edge_data = &self.edges[*edge];
                if !visited[edge_data.target] {
                    visited[edge_data.target] = true;
                    dist[edge_data.target] = dist[node] + 1;
                    path[edge_data.target] = Some((node, edge_data.target, &edge_data.weight));

                    queue.push_back(edge_data.target);
                    if edge_data.target == target {
                        break;
                    }
                }
            }
        }

        let mut i = target;
        let mut result = vec![];
        while let Some((source_node, target_node, weight)) = path[i] {
            result.push((&self.nodes[target_node].value, weight));
            i = source_node;
        }
        result.reverse();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_nodes_no_edges() {
        let mut graph = Graph::<i32, i32>::default();

        let n0 = graph.add_node(1);
        let n1 = graph.add_node(2);

        assert_eq!(graph.get_edge_weight(n0, n1), None);
        assert_eq!(graph.get_edge_weight(n1, n0), None);
    }

    #[test]
    fn two_nodes_with_single_edge() {
        let mut graph = Graph::default();

        let n0 = graph.add_node(1);
        let n1 = graph.add_node(2);

        _ = graph.add_edge(n0, n1, 5);

        assert_eq!(graph.get_edge_weight(n0, n1), Some(&5));
        assert_eq!(graph.get_edge_weight(n1, n0), None);
    }

    #[test]
    fn multiple_nodes_multiple_edges() {
        let mut graph = Graph::default();

        let n0 = graph.add_node(1);
        let n1 = graph.add_node(2);
        let n2 = graph.add_node(3);
        let n3 = graph.add_node(4);

        _ = graph.add_edge(n0, n1, 6);
        _ = graph.add_edge(n1, n0, 5);
        _ = graph.add_edge(n2, n1, 2);
        _ = graph.add_edge(n3, n0, 10);

        assert_eq!(graph.get_edge_weight(n0, n1), Some(&6));
        assert_eq!(graph.get_edge_weight(n1, n0), Some(&5));
        assert_eq!(graph.get_edge_weight(n2, n1), Some(&2));
        assert_eq!(graph.get_edge_weight(n1, n2), None);
        assert_eq!(graph.get_edge_weight(n3, n0), Some(&10));
        assert_eq!(graph.get_edge_weight(n0, n3), None);
    }

    #[test]
    fn add_invalid_edges_no_nodes() {
        let mut graph = Graph::default();

        let n0 = graph.add_node(1);
        let n1 = graph.add_node(2);
        let n2 = n1 + 1;

        let e0 = graph.add_edge(n2, n1, 4);
        let e1 = graph.add_edge(n0, n2, 7);

        assert_eq!(e0, Err(GraphOperationError::NodeDoesNotExist));
        assert_eq!(e1, Err(GraphOperationError::NodeDoesNotExist));
        assert_eq!(graph.get_edge_weight(n0, n2), None);
    }

    #[test]
    fn add_duplicate_nodes() {
        let mut graph = Graph::default();

        let n0 = graph.add_node(1);
        let n1 = graph.add_node(2);
        _ = graph.add_edge(n0, n1, 5);

        let actual = graph.add_node(1);
        assert_eq!(actual, n0);
    }

    #[test]
    fn shortest_path_dijkstra() {
        let mut graph = Graph::default();

        let n0 = graph.add_node(1);
        let n1 = graph.add_node(2);
        let n2 = graph.add_node(3);
        let n3 = graph.add_node(4);
        let n4 = graph.add_node(5);

        _ = graph.add_edge(n0, n1, 25);
        _ = graph.add_edge(n1, n3, 30);
        _ = graph.add_edge(n1, n2, 25);
        _ = graph.add_edge(n2, n3, 30);
        _ = graph.add_edge(n3, n4, 40);

        let actual = graph.shortest_path(n0, n3);
        assert_eq!(actual, vec![(&2, &25), (&4, &30)]);
    }

    #[test]
    fn shortest_path_direct_path() {
        let mut graph = Graph::default();

        let n0 = graph.add_node(1);
        let n1 = graph.add_node(2);
        let n2 = graph.add_node(3);

        _ = graph.add_edge(n0, n2, 25);
        _ = graph.add_edge(n0, n1, 5);
        _ = graph.add_edge(n1, n2, 10);

        let actual = graph.shortest_path(n0, n2);
        assert_eq!(actual, vec![(&3, &25)]);
    }

    #[test]
    fn shortest_path_single_edge() {
        let mut graph = Graph::default();

        let n0 = graph.add_node(1);
        let n1 = graph.add_node(2);
        _ = graph.add_edge(n0, n1, 30);

        let actual = graph.shortest_path(n0, n1);
        assert_eq!(actual, vec![(&2, &30)])
    }
}
