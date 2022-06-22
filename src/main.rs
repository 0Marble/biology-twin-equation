use integrals::TrapezoidIntegrator;
use linear_equation::LUSolver;
use testing::test_method;

use galerkin::GalerkinMethod;
use galerkin_fourier::GalerkinMethodWithFourier;
use neumann::NeumannMethod;
use nystrom::NystromMethod;

mod functions;
mod galerkin;
mod galerkin_fourier;
mod integrals;
mod linear_equation;
mod method;
mod neumann;
mod nystrom;
mod testing;

fn main() {
    let a = 1.0;
    let b = 1.0;
    let y = 2.0 / 3.0 * b + 52.0 / 27.0 * a;
    let q =
        |x: f64| 1.0 / 3.0 * a * x * x - 16.0 / 9.0 * a * x.abs() + 56.0 / 27.0 * a + 1.0 / 3.0 * b;
    let r = |x: f64| a * x * x + b;
    let m = |x: f64| (-2.0 * x.abs()).exp();
    let w = |x: f64| (-(x.abs())).exp() * q(x) / (1.0 + (-(x.abs())).exp() * r(x));
    let c = |x: f64| 1.0 + (-(x.abs())).exp() * r(x);

    let width = 15.0;
    let node_count = 5000;
    let comparison_point_count = 5000;

    let nystrom = NystromMethod::new(Box::new(LUSolver), node_count);
    let galerkin = GalerkinMethod::new(
        Box::new(TrapezoidIntegrator::new(node_count / 4)),
        Box::new(LUSolver),
        Box::new(|t: f64| (1.0f64 - t.powi(2)).sqrt()),
        60,
    );
    let galerkin_fourier = GalerkinMethodWithFourier::new(
        Box::new(TrapezoidIntegrator::new(node_count / 10)),
        Box::new(LUSolver),
        Box::new(|t: f64| (1.0f64 - t.powi(2)).sqrt()),
        400,
    );
    let neumann = NeumannMethod::new(
        500,
        node_count,
        Box::new(TrapezoidIntegrator::new(node_count)),
    );

    let prefix = "exponent";
    test_method(
        &galerkin,
        &m,
        &w,
        y,
        &c,
        comparison_point_count,
        width,
        "galerkin_taylor",
        "results",
        prefix,
    );
    test_method(
        &galerkin_fourier,
        &m,
        &w,
        y,
        &c,
        comparison_point_count,
        width,
        "galerkin_fourier",
        "results",
        prefix,
    );

    test_method(
        &neumann,
        &m,
        &w,
        y,
        &c,
        comparison_point_count,
        width,
        "neumann",
        "results",
        prefix,
    );

    test_method(
        &nystrom,
        &m,
        &w,
        y,
        &c,
        comparison_point_count,
        width,
        "nystrom",
        "results",
        prefix,
    );

    let p = 1.0;
    let a = 1.0;
    let n = 2;
    let y = a * std::f64::consts::PI * (a + 5.0 * p * p) * (a + 8.0 * p * p)
        / (p * (a * a + 21.0 * a * p * p + 120.0 * p * p * p * p));
    let m = |x: f64| p / (x * x + p * p) * std::f64::consts::FRAC_1_PI;
    let w = |x: f64| a / (x * x + (n as f64 + 1.0).powi(2) * p * p);
    let c = |x: f64| 1.0 + 24.0 / (71.0 * (x * x + 1.0)) + 40.0 / (71.0 * (x * x + 4.0));
    let prefix = "rational";
    test_method(
        &galerkin,
        &m,
        &w,
        y,
        &c,
        comparison_point_count,
        width,
        "galerkin_taylor",
        "results",
        prefix,
    );
    test_method(
        &galerkin_fourier,
        &m,
        &w,
        y,
        &c,
        comparison_point_count,
        width,
        "galerkin_fourier",
        "results",
        prefix,
    );

    test_method(
        &neumann,
        &m,
        &w,
        y,
        &c,
        comparison_point_count,
        width,
        "neumann",
        "results",
        prefix,
    );

    test_method(
        &nystrom,
        &m,
        &w,
        y,
        &c,
        comparison_point_count,
        width,
        "nystrom",
        "results",
        prefix,
    );
}
