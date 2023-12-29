pub type NodeIndex = usize;
pub type EdgeIndex = usize;

pub struct NodeData {
    value: i32,
    first_outgoing_edge: Option<EdgeIndex>,
}

pub struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
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

    pub fn add_node(&mut self, value: i32) {
        self.nodes.push(NodeData {
            value: value,
            first_outgoing_edge: None,
        });
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex, value: i32) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target: target,
            next_outgoing_edge: node_data.first_outgoing_edge,
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }

    pub fn get_edge_weight(&self, source: NodeIndex, target: NodeIndex) -> Option<i32> {
        let node_data = &self.nodes[source];
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_nodes_no_edges() {
        let mut graph = Graph::new();

        graph.add_node(1);
        graph.add_node(2);

        assert_eq!(graph.get_edge_weight(1, 2), None);
        assert_eq!(graph.get_edge_weight(2, 1), None);
    }

    #[test]
    fn two_nodes_with_single_edge() {
        let mut graph = Graph::new();

        graph.add_node(1);
        graph.add_node(2);

        graph.add_edge(1, 2, 5);

        assert_eq!(graph.get_edge_weight(1, 2), Some(5));
        assert_eq!(graph.get_edge_weight(2, 1), None);
    }

    #[test]
    fn multiple_nodes_multiple_edges() {
        let mut graph = Graph::new();

        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_node(4);

        graph.add_edge(1, 2, 6);
        graph.add_edge(2, 1, 5);
        graph.add_edge(3, 2, 2);
        graph.add_edge(4, 1, 10);

        assert_eq!(graph.get_edge_weight(1, 2), Some(6));
        assert_eq!(graph.get_edge_weight(2, 1), Some(6));
        assert_eq!(graph.get_edge_weight(3, 2), Some(2));
        assert_eq!(graph.get_edge_weight(2, 3), None);
        assert_eq!(graph.get_edge_weight(4, 1), Some(10));
        assert_eq!(graph.get_edge_weight(1, 4), None);
    }
}
