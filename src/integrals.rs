use rayon::prelude::*;

use crate::functions::Function;

pub trait Integrator: Sync {
    fn integrate(&self, f: &dyn Function, left: f64, right: f64) -> f64;
}

pub struct TrapezoidIntegrator {
    node_count: usize,
}

impl TrapezoidIntegrator {
    pub fn new(node_count: usize) -> Self {
        Self { node_count }
    }
}

impl Integrator for TrapezoidIntegrator {
    fn integrate(&self, f: &dyn Function, left: f64, right: f64) -> f64 {
        let step = (right - left) / (self.node_count - 1) as f64;

        (0..self.node_count - 1)
            .into_par_iter()
            .map(|i| {
                let x = (i as f64) * step + left;
                let f1 = f.get(x);
                let f2 = f.get(x + step);
                (f1 + f2) * step
            })
            .sum::<f64>()
            / 2.0
    }
}
