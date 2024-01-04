use std::collections::BTreeMap;
use std::fmt::{Debug, Display};

pub type NodeIndex = usize;
pub type EdgeIndex = usize;

pub struct NodeData<T> {
    value: T,
    edges: Vec<EdgeIndex>,
}

pub struct EdgeData<T> {
    target: NodeIndex,
    weight: T,
}

pub struct Graph<N, E>
where
    N: PartialEq + Debug,
{
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
    N: Clone + PartialEq + Debug,
    E: Copy,
{
    pub fn new() -> Graph<N, E> {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, value: N) -> NodeIndex {
        let mut existing: NodeIndex = 0;
        for node in &self.nodes {
            if node.value == value {
                //return Err(GraphOperationError::NodeAlreadyExistsForValue);
                return existing;
            }
            existing += 1;
        }

        let index = self.nodes.len();
        self.nodes.push(NodeData {
            value: value,
            edges: Vec::new(),
        });
        return index;
    }

    pub fn add_edge(
        &mut self,
        source: NodeIndex,
        target: NodeIndex,
        weight: E,
    ) -> Result<(), GraphOperationError> {
        if source >= self.nodes.len() || target >= self.nodes.len() {
            return Err(GraphOperationError::NodeDoesNotExist);
        }

        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];

        self.edges.push(EdgeData {
            target: target,
            weight: weight,
        });

        node_data.edges.push(edge_index);
        return Ok(());
    }

    pub fn get_node_index(&self, node_value: N) -> Option<NodeIndex> {
        let mut i = 0;
        println!("Nodes in graph: {}", self.nodes.len());
        for node in &self.nodes {
            println!("Is {:?} == {:?}", node.value, node_value);
            if node.value == node_value {
                println!("True, return {}", i);
                return Some(i);
            }
            i += 1;
        }
        None
    }

    pub fn get_edge_weight(&self, source: NodeIndex, target: NodeIndex) -> Option<&E> {
        if source >= self.nodes.len() || target >= self.nodes.len() {
            return None;
        }

        for edge in &self.nodes[source].edges {
            let edge_data = &self.edges[*edge];
            if edge_data.target == target {
                return Some(&edge_data.weight);
            }
        }

        return None;
    }

    pub fn shortest_path(&self, source: NodeIndex, target: NodeIndex) -> Vec<(&N, &E)> {
        //let mut results = Vec::<(i32, i32)>::new();
        let mut distance = vec![usize::MAX; self.nodes.len()];
        let mut priority = BTreeMap::new();
        //let mut heap = BinaryHeap::new();
        //let mut prev = vec![None; self.nodes.len()];
        let mut prev = vec![source];

        distance[source] = 0;
        priority.insert(source, 0);

        while let Some((node_index, weight)) = priority.pop_first() {
            if node_index == target {
                println!("Is target node, breaking loop");
                break;
            }

            if weight > distance[node_index] {
                println!("Weight > Distance, continueing to next node");
                continue;
            }

            println!("Checking for node {}", node_index);
            for edge in &self.nodes[node_index].edges {
                let edge_data = &self.edges[*edge];
                //let alt = weight + edge_data.weight as usize;
                let alt = weight + 1;

                if alt < distance[edge_data.target] {
                    println!(
                        "Alt value {} is less than {} for adjcent node {}",
                        alt, distance[edge_data.target], edge_data.target
                    );
                    priority.insert(edge_data.target, alt);
                    distance[edge_data.target] = alt;

                    //let node_data = &self.nodes[edge_data.target];
                    // results.push((node_data.value, edge_data.weight));

                    if !prev.contains(&node_index) {
                        prev.push(node_index);
                    }
                }
            }
        }
        prev.push(target);

        println!("Distance");
        for (index, weight) in distance.iter().enumerate() {
            println!("Index: {}, Weight: {}", index, weight);
        }

        println!("Path");
        let mut results2 = Vec::new();
        for i in 0..prev.len() - 1 {
            let node_data = &self.nodes[prev[i + 1]];
            let node_value = &node_data.value;
            let edge_weight = self.get_edge_weight(prev[i], prev[i + 1]).unwrap();
            results2.push((node_value, edge_weight));
        }

        println!();
        println!();

        return results2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_nodes_no_edges() {
        let mut graph = Graph::<i32, i32>::new();

        let n0 = graph.add_node(1);
        let n1 = graph.add_node(2);

        assert_eq!(graph.get_edge_weight(n0, n1), None);
        assert_eq!(graph.get_edge_weight(n1, n0), None);
    }

    #[test]
    fn two_nodes_with_single_edge() {
        let mut graph = Graph::new();

        let n0 = graph.add_node(1);
        let n1 = graph.add_node(2);

        _ = graph.add_edge(n0, n1, 5);

        assert_eq!(graph.get_edge_weight(n0, n1), Some(&5));
        assert_eq!(graph.get_edge_weight(n1, n0), None);
    }

    #[test]
    fn multiple_nodes_multiple_edges() {
        let mut graph = Graph::new();

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
        let mut graph = Graph::new();

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
        let mut graph = Graph::new();

        let n0 = graph.add_node(1);
        let n1 = graph.add_node(2);
        _ = graph.add_edge(n0, n1, 5);

        let actual = graph.add_node(1);
        assert_eq!(actual, n0);
    }

    #[test]
    fn shortest_path_dijkstra() {
        let mut graph = Graph::new();

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
        let mut graph = Graph::new();

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
        let mut graph = Graph::new();

        let n0 = graph.add_node(1);
        let n1 = graph.add_node(2);
        _ = graph.add_edge(n0, n1, 30);

        let actual = graph.shortest_path(n0, n1);
        assert_eq!(actual, vec![(&2, &30)])
    }
}
