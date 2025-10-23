use rand::Rng;
use std::fmt::Debug;

const MAX_NUM_SYMBOLS: usize = 5;

#[derive(Clone)]
pub struct Node {
    pub id: usize,
    pub symbols: Vec<String>,
    pub symbols_out: Vec<String>,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("id", &self.id)
            .field("symbols", &format!("{:?}", &self.symbols).replace("\"", ""))
            .field(
                "symbols_out",
                &format!("{:?}", &self.symbols_out).replace("\"", ""),
            )
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct NodesStorage {
    pub all_nodes: Vec<Node>,
}

impl NodesStorage {
    pub fn new(len: usize, rng: &mut impl Rng) -> Self {
        let mut all_nodes: Vec<_> = (0..len)
            .map(|id| Node {
                id,
                symbols: vec![],
                symbols_out: vec![],
            })
            .collect();

        for i in 0..len {
            let n = rng.random_range(0..MAX_NUM_SYMBOLS);
            let symbols: Vec<_> = (0..n).map(|j| format!("{i}_{j}")).collect();
            all_nodes[i].symbols = symbols;

            if !all_nodes[i].symbols.is_empty() {
                let num_incoming_nodes = rng.random_range(0..MAX_NUM_SYMBOLS);
                let incoming_node_indices: Vec<_> = (0..num_incoming_nodes)
                    .map(|_| rng.random_range(0..len))
                    .collect();

                for j in incoming_node_indices {
                    if i != j {
                        let symbol_idx = rng.random_range(0..all_nodes[i].symbols.len());
                        let symbol = all_nodes[i].symbols[symbol_idx].clone();
                        if !all_nodes[j].symbols_out.contains(&symbol) {
                            all_nodes[j].symbols_out.push(symbol);
                        }
                    }
                }
            }
        }

        Self { all_nodes }
    }

    pub fn get_relevant_node(&self, symbol_out: &str) -> &Node {
        self.all_nodes
            .iter()
            .find(|x| x.symbols.iter().any(|s| s == symbol_out))
            .unwrap()
    }

    pub fn get_roots(&self, number_of_roots: usize, rng: &mut impl Rng) -> Vec<&Node> {
        (0..number_of_roots)
            .map(|_| rng.random_range(0..self.all_nodes.len()))
            .map(|idx| &self.all_nodes[idx])
            .collect()
    }
}
