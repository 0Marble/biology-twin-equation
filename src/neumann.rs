use crate::{functions::*, integrals::Integrator, method::Method};
use rayon::prelude::*;

pub struct NeumannMethod {
    iter_count: usize,
    node_count: usize,
    integrator: Box<dyn Integrator>,
}

impl NeumannMethod {
    pub fn new(iter_count: usize, node_count: usize, integrator: Box<dyn Integrator>) -> Self {
        Self {
            iter_count,
            node_count,
            integrator,
        }
    }
}

impl Method for NeumannMethod {
    fn solve(
        &self,
        birth_probability: &dyn Function,
        death_probability: &dyn Function,
        parameter: f64,
        width: f64,
    ) -> Box<dyn Function> {
        let mut v: Vec<f64> = (0..self.node_count).map(|_| 0.0).collect();
        let step = width / (self.node_count - 1) as f64;

        for _ in 0..self.iter_count {
            let c = |x: f64| v[(x / step) as usize];
            v = (0..self.node_count)
                .into_par_iter()
                .map(|i| {
                    let x = (i as f64) * step;

                    (self.integrator.integrate(
                        &|t| (birth_probability.get(t - x) + birth_probability.get(t + x)) * c(t),
                        0.0,
                        width,
                    ) + (birth_probability.get(x) * parameter - death_probability.get(x)))
                        / (1.0 + death_probability.get(x))
                })
                .collect();
        }

        Box::new(PointFunction::new(
            v.iter().map(|v| v + 1.0).collect(),
            0.0,
            width,
        ))
    }
}
