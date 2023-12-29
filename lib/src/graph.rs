pub type NodeIndex = usize;
pub type EdgeIndex = usize;

pub struct NodeData {
    value: i32,
    edges: Vec<EdgeIndex>,
}

pub struct EdgeData {
    target: NodeIndex,
    weight: i32,
}

pub struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, value: i32) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData {
            value: value,
            edges: Vec::new(),
        });
        return index;
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex, weight: i32) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];

        self.edges.push(EdgeData {
            target: target,
            weight: weight,
        });

        node_data.edges.push(edge_index);
    }

    pub fn get_edge_weight(&self, source: NodeIndex, target: NodeIndex) -> Option<i32> {
        for edge in &self.nodes[source].edges {
            let edge_data = &self.edges[*edge];
            if edge_data.target == target {
                return Some(edge_data.weight);
            }
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_nodes_no_edges() {
        let mut graph = Graph::new();

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

        graph.add_edge(n0, n1, 5);

        assert_eq!(graph.get_edge_weight(n0, n1), Some(5));
        assert_eq!(graph.get_edge_weight(n1, n0), None);
    }

    #[test]
    fn multiple_nodes_multiple_edges() {
        let mut graph = Graph::new();

        let n0 = graph.add_node(1);
        let n1 = graph.add_node(2);
        let n2 = graph.add_node(3);
        let n3 = graph.add_node(4);

        graph.add_edge(n0, n1, 6);
        graph.add_edge(n1, n0, 5);
        graph.add_edge(n2, n1, 2);
        graph.add_edge(n3, n0, 10);

        assert_eq!(graph.get_edge_weight(n0, n1), Some(6));
        assert_eq!(graph.get_edge_weight(n1, n0), Some(5));
        assert_eq!(graph.get_edge_weight(n2, n1), Some(2));
        assert_eq!(graph.get_edge_weight(n1, n2), None);
        assert_eq!(graph.get_edge_weight(n3, n0), Some(10));
        assert_eq!(graph.get_edge_weight(n0, n3), None);
    }
}
