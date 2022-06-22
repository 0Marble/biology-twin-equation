use rayon::prelude::*;

use crate::{functions::*, linear_equation::LinearEquationSolver, method::Method};

pub struct NystromMethod {
    solver: Box<dyn LinearEquationSolver>,
    node_count: usize,
}

impl NystromMethod {
    pub fn new(solver: Box<dyn LinearEquationSolver>, node_count: usize) -> Self {
        Self { solver, node_count }
    }
}

impl Method for NystromMethod {
    fn solve(
        &self,
        birth_probability: &dyn Function,
        death_probability: &dyn Function,
        parameter: f64,
        width: f64,
    ) -> Box<dyn Function> {
        let k = |x, y| birth_probability.get(y - x) / (1.0 + death_probability.get(x));
        let f = |x| {
            (birth_probability.get(x) * parameter - death_probability.get(x))
                / (1.0 + death_probability.get(x))
        };

        let step = width / (self.node_count - 1) as f64;

        let mut mat: Vec<f64> = (0..self.node_count * self.node_count)
            .map(|_| 0.0)
            .collect();

        for j in 0..self.node_count {
            let x = (j as f64) * step;
            for i in 0..self.node_count {
                let y = (i as f64) * step;

                let k = if i == self.node_count - 1 {
                    (k(x, y) + k(x, -y)) / 2.0
                } else if i == 0 {
                    k(x, y) / 2.0
                } else {
                    k(x, y) + k(x, -y)
                };

                mat[j * self.node_count + i] = if i == j { k * step - 1.0 } else { k * step };
            }
        }

        let b: Vec<f64> = (0..self.node_count)
            .into_par_iter()
            .map(|i| -f((i as f64) * step))
            .collect();

        Box::new(PointFunction::new(
            self.solver
                .solve(&mat, self.node_count, &b)
                .unwrap()
                .iter()
                .map(|x| x + 1.0)
                .collect(),
            0.0,
            width,
        ))
    }
}
