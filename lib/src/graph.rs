use std::collections::{BTreeMap, BinaryHeap, VecDeque};
use std::fmt::Debug;

use log::{debug, info, warn};

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
    N: Clone + PartialEq + Debug,
    E: Copy + Debug,
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
            id: id,
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, value: N) -> NodeIndex {
        let mut existing: NodeIndex = 0;
        for node in &self.nodes {
            if node.value == value {
                //return Err(GraphOperationError::NodeAlreadyExistsForValue);
                /*debug!(
                    "Attempted to insert a node with a value {:?} that already exists, returning existing ({})",
                    &value,
                    existing
                );*/
                return existing;
            }
            existing += 1;
        }

        let index = self.nodes.len();
        debug!(
            "Inserting new node for value {:?} at index {}",
            &value, &index
        );
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
            warn!("Failed trying to create edge because at least one node doesn't exist (Source: {}, Target: {})", source, target);
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
        for node in &self.nodes {
            if node.value == node_value {
                return Some(i);
            }
            i += 1;
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
        return None;
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
                if visited[edge_data.target] == false {
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

    pub fn shortest_path2(&self, source: NodeIndex, target: NodeIndex) -> Vec<(&N, &E)> {
        //let mut results = Vec::<(i32, i32)>::new();
        let mut distance = vec![usize::MAX; self.nodes.len()];
        let mut priority = BTreeMap::new();
        //let mut priority = BinaryHeap::new();
        //let mut prev = vec![None; self.nodes.len()];
        let mut prev = vec![source];
        //let mut ans = BTreeMap::new();

        info!(
            "Getting shortest path between nodes {} [{:?}] and {} [{:?}]",
            source, self.nodes[source].value, target, self.nodes[target].value
        );

        //debug!("Graph Nodes: {:?}", self.nodes);
        //debug!("Graph Edges: {:?}", self.edges);

        distance[source] = 0;
        priority.insert(source, 0);

        //ans.insert(source, Some(0));
        /*while let Some((node_index, weight)) = priority.pop_first() {
            debug!(
                "Checking index {} [{:?}]",
                node_index, self.nodes[node_index].value
            );
            //if node_index == target {
            //    break;
            //}
            for edge in &self.nodes[node_index].edges {
                let new_weight = weight + 1;
                let edge_data = &self.edges[*edge];
                /*match ans.get(&node_index) {
                    Some(Some((_, dist_next))) if new_weight >= *dist_next => {}
                    Some(None) => {}
                    _ => {
                        ans.insert(edge_data.target, Some((node_index, new_weight)));
                        priority.insert(edge_data.target, new_weight);
                    }
                }*/

                debug!(
                    "Checking edge between {} [{:?}] and {} [{:?}]",
                    node_index,
                    self.nodes[node_index].value,
                    edge_data.target,
                    self.nodes[edge_data.target].value
                );

                match ans.get(&node_index) {
                    Some(Some((_, dist_next))) if new_weight >= *dist_next => {}
                    Some(None) => {}
                    _ => {
                        debug!(
                            "Adding ({}, ({}, {}))",
                            edge_data.target, node_index, new_weight
                        );
                        ans.insert(edge_data.target, Some((node_index, new_weight)));
                        priority.insert(edge_data.target, new_weight);
                    }
                }
                /*if let Some(Some(dist_next)) = ans.get(&node_index) {
                    debug!("Is {} < {}", new_weight, dist_next);
                    if new_weight < *dist_next {
                        ans.insert(edge_data.target, Some(new_weight));
                        priority.insert(edge_data.target, new_weight);
                    }
                }*/
            }
        }
        //let max = ans.keys().max();
        //ans.insert(target, Some(0));
        //ans.insert(target, Some((0, 100)));

        debug!("ANSWER 1 {:?}", ans);
        if let Some(max) = ans.keys().max() {
            if *max < target {
                ans.insert(target, Some((*max, 100)));
            }
        }

        debug!("ANSWER 2 {:?}", ans);

        let mut results = vec![];
        while let Some((b, Some((a, _)))) = ans.pop_first() {
            if let Some(edge_weight) = self.get_edge_weight(a, b) {
                let node_value = &self.nodes[b].value;
                results.push((node_value, edge_weight));
            } else {
                warn!("Unable to get edge weight between nodes {} and {}", a, b);
            }
        }
        return results;*/

        while let Some((node_index, weight)) = priority.pop_first() {
            debug!(
                "Popped {} [{:?}] from the priority queue with weight {}",
                node_index, self.nodes[node_index].value, weight
            );

            /*if node_index == target {
                break;
            }*/

            /*if weight > distance[node_index] {
                continue;
            }*/

            for edge in &self.nodes[node_index].edges {
                let edge_data = &self.edges[*edge];
                //let alt = weight + edge_data.weight as usize;
                let alt = weight + 1;

                /*debug!(
                    "Checking edge with target {} [{:?}]",
                    edge_data.target, self.nodes[edge_data.target].value
                );*/

                if alt < distance[edge_data.target] {
                    /*debug!(
                        "Alt value {} is less than distance {}",
                        alt, distance[edge_data.target]
                    );*/
                    debug!(
                        "Adding {} [{:?}] to priority queue with weight {}",
                        edge_data.target, self.nodes[edge_data.target].value, alt
                    );
                    priority.insert(edge_data.target, alt);
                    distance[edge_data.target] = alt;

                    if !prev.contains(&node_index) {
                        debug!(
                            "Adding node {} [{:?}] to prev structure",
                            node_index, self.nodes[node_index].value
                        );
                        prev.push(node_index);
                    }
                }
            }
        }
        prev.push(target);
        debug!(
            "Djikstra shortest path calculated distances: {:?}",
            distance
        );
        debug!("{:?}", prev);
        debug!("Priority Queue: {:?}", priority);

        let mut results2 = Vec::new();
        for i in 0..prev.len() - 1 {
            let node_data = &self.nodes[prev[i + 1]];
            let node_value = &node_data.value;
            let edge_weight = self.get_edge_weight(prev[i], prev[i + 1]).unwrap();
            results2.push((node_value, edge_weight));
        }

        return results2;
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
