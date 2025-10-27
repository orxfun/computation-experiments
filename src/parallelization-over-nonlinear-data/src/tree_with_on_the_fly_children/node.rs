use crate::amount_of_work;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Node {
    pub id: usize,
    pub symbols: Vec<String>,
    pub symbols_out: Vec<String>,
}

impl Node {
    /// Fibonacci as example computation on each of the node values.
    pub fn compute(&self) -> u64 {
        (0..*amount_of_work())
            .map(|j| {
                let n = core::hint::black_box(40 + self.id as u64 + j as u64);
                let mut a = 0;
                let mut b = 1;
                for _ in 0..n {
                    let c = a + b;
                    a = b;
                    b = c;
                }
                a
            })
            .sum()
    }
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
