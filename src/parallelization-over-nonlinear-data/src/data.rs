use crate::AMOUNT_OF_WORK;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Clone)]
pub struct Node {
    pub value: Vec<u64>,
    pub children: Vec<Node>,
    pub fib_n: Vec<u64>,
}

impl Node {
    pub fn new(mut n: u32, rng: &mut impl Rng) -> Self {
        let mut children = Vec::new();
        if n < 5 {
            for _ in 0..n {
                children.push(Node::new(0, rng));
            }
        } else {
            while n > 0 {
                let n2 = rng.random_range(0..=n);
                children.push(Node::new(n2, rng));
                n -= n2;
            }
        }
        Self {
            value: (0..rng.random_range(1..500))
                .map(|_| rng.random_range(0..40))
                .collect(),
            children,
            fib_n: vec![],
        }
    }

    pub fn num_nodes(&self) -> usize {
        1 + self
            .children
            .iter()
            .map(|node| node.num_nodes())
            .sum::<usize>()
    }

    /// Fibonacci as example computation on each of the node values.
    pub fn compute(value: u64) -> u64 {
        (0..AMOUNT_OF_WORK)
            .map(|j| {
                let n = core::hint::black_box(value + j as u64);
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

    pub fn compute_using_mut_var(value: u64, rng: &mut impl Rng) -> u64 {
        match rng.random_bool(0.7) {
            true => Self::compute(value),
            false => Self::compute(value / 2),
        }
    }

    pub fn example_roots(seed: u64) -> Vec<Node> {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        #[cfg(not(miri))]
        return vec![
            Node::new(5000, &mut rng),
            Node::new(2000, &mut rng),
            Node::new(4000, &mut rng),
        ];

        #[cfg(miri)]
        vec![
            Node::new(50, &mut rng),
            Node::new(20, &mut rng),
            Node::new(40, &mut rng),
        ]
    }
}
