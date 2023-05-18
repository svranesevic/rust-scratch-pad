use fnv::FnvHashMap;
use std::hash::Hash;

pub struct Graph<NodeId, Edge = (), Node = ()> {
    nodes: FnvHashMap<NodeId, Node>,
    outgoing: FnvHashMap<NodeId, Vec<(NodeId, Edge)>>,
}

impl<NodeId, Edge, Node> Graph<NodeId, Edge, Node>
where
    NodeId: Eq + Hash,
{
    pub fn new() -> Graph<NodeId, Edge, Node> {
        Graph {
            nodes: FnvHashMap::default(),
            outgoing: FnvHashMap::default(),
        }
    }

    pub fn add_node(self: &mut Graph<NodeId, Edge, Node>, id: NodeId, node: Node) {
        self.nodes.insert(id, node);
    }

    pub fn add_edge(self: &mut Self, from: NodeId, to: NodeId, edge: Edge) {
        self.outgoing.entry(from).or_default().push((to, edge));
    }

    pub fn get_node(self: &Self, node: NodeId) -> Option<&Node> {
        self.nodes.get(&node)
    }

    pub fn get_node_mut(self: &mut Self, node: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(&node)
    }
}

impl<NodeId, Edge, Node> Graph<NodeId, Edge, Node>
where
    NodeId: Eq + Hash + Clone,
    Edge: Clone,
{
    pub fn add_symetric_edge(self: &mut Self, from: NodeId, to: NodeId, edge: Edge) {
        self.add_edge(from.clone(), to.clone(), edge.clone());
        self.add_edge(to, from, edge);
    }
}

impl<NodeId, Edge> Graph<NodeId, Edge, ()>
where
    NodeId: Eq + Hash,
{
    pub fn add_node_id(self: &mut Self, node: NodeId) {
        self.nodes.insert(node, ());
    }
}
