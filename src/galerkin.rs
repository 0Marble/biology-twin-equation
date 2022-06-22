use crate::{
    functions::Function, integrals::Integrator, linear_equation::LinearEquationSolver,
    method::Method,
};

use rayon::prelude::*;

pub struct GalerkinMethod {
    integrator: Box<dyn Integrator>,
    equation_solver: Box<dyn LinearEquationSolver>,
    weight_func: Box<dyn Function>,

    polynome_degree: usize,
}

impl GalerkinMethod {
    pub fn new(
        integrator: Box<dyn Integrator>,
        equation_solver: Box<dyn LinearEquationSolver>,
        weight_func: Box<dyn Function>,
        polynome_degree: usize,
    ) -> Self {
        Self {
            integrator,
            equation_solver,
            weight_func,
            polynome_degree,
        }
    }
}

struct EvenPowerPolynomial {
    coefficients: Vec<f64>,
}

impl EvenPowerPolynomial {
    fn new(coefficients: Vec<f64>) -> Self {
        Self { coefficients }
    }
}

impl Function for EvenPowerPolynomial {
    fn get(&self, x: f64) -> f64 {
        (0..self.coefficients.len())
            .map(|n| (x).powf((n * 2) as f64) * self.coefficients[n])
            .sum()
    }
}

impl Method for GalerkinMethod {
    fn solve(
        &self,
        birth_probability: &dyn Function,
        death_probability: &dyn Function,
        parameter: f64,
        width: f64,
    ) -> Box<dyn Function> {
        let h = |t: f64, x: f64| -birth_probability.get(t - x) / (1.0 + death_probability.get(x));
        let y = |x: f64| {
            (birth_probability.get(x) * parameter - death_probability.get(x))
                / (1.0 + death_probability.get(x))
        };

        let base = |t: f64, n: usize| t.powi(n as i32 * 2);
        let ajk = |j: usize, k: usize| {
            self.integrator.integrate(
                &|x: f64| {
                    self.weight_func.get((x / width).clamp(-1.0, 1.0))
                        * base(x, j)
                        * (base(x, k)
                            + self.integrator.integrate(
                                &|t: f64| h(t, x) * base(t, k),
                                -width,
                                width,
                            ))
                },
                -width,
                width,
            )
        };

        let bj = |j: usize| {
            self.integrator.integrate(
                &|x: f64| self.weight_func.get((x / width).clamp(-1.0, 1.0)) * y(x) * base(x, j),
                -width,
                width,
            )
        };

        let mat = (0..self.polynome_degree / 2)
            .into_par_iter()
            .map(|j| {
                (0..self.polynome_degree / 2)
                    .into_par_iter()
                    .map(|k| ajk(j, k))
                    .collect::<Vec<f64>>()
            })
            .flatten()
            .collect::<Vec<f64>>();
        let b = (0..self.polynome_degree / 2)
            .into_par_iter()
            .map(|j| bj(j))
            .collect::<Vec<f64>>();

        let mut coefficients = self
            .equation_solver
            .solve(&mat, self.polynome_degree / 2, &b)
            .unwrap();
        coefficients[0] += 1.0;

        //println!("{:?}\n{:?}\n{:?}", mat, b, coefficients);

        Box::new(EvenPowerPolynomial::new(coefficients))
    }
}
