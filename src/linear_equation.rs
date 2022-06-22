pub trait LinearEquationSolver: Sync {
    fn solve(&self, mat: &[f64], width: usize, b: &[f64]) -> Option<Vec<f64>>;
}

pub struct LUSolver;

impl LUSolver {
    fn lu(&self, mat: &[f64], width: usize) -> Option<(Vec<f64>, Vec<f64>)> {
        if width != mat.len() / width {
            return None;
        }

        let mut l: Vec<f64> = (0..width * width).map(|_| 0.0).collect();
        let mut u = l.clone();
        let mut d = mat.to_owned();

        for layer in 0..width {
            let a = d[layer * width + layer];
            if a.clone() == 0.0.into() {
                return None;
            }

            l[layer * width + layer] = 1.0;
            u[layer * width + layer] = a;

            for i in layer + 1..width {
                l[i * width + layer] = d[i * width + layer] / a;
                u[layer * width + i] = d[layer * width + i];

                for j in layer + 1..width {
                    d[i * width + j] =
                        d[i * width + j] - (d[layer * width + j] * d[i * width + layer]) / a;
                }
            }
        }

        Some((l, u))
    }

    fn gauss_from_lu(&self, l: &[f64], u: &[f64], b: &[f64], width: usize) -> Option<Vec<f64>> {
        let l_height = l.len() / width;
        let u_height = u.len() / width;

        if l_height != width || u_height != width || b.len() != width {
            return None;
        }

        let v = self.l_gauss(l, b, width);
        let x = self.u_gauss(u, &v, width);

        Some(x)
    }

    fn l_gauss(&self, l: &[f64], b: &[f64], width: usize) -> Vec<f64> {
        let mut x: Vec<f64> = (0..width).map(|_| 1.0).collect();
        for i in 0..width {
            let mut xi = b[i];
            for j in 0..i {
                xi = xi - l[i * width + j] * x[j];
            }
            x[i] = xi;
        }

        x
    }

    fn u_gauss(&self, u: &[f64], b: &[f64], width: usize) -> Vec<f64> {
        let mut x: Vec<f64> = (0..width).map(|_| 1.0).collect();

        for i in 0..width {
            let mut xi = b[width - i - 1];
            for j in 0..i {
                xi = xi - u[(width - i - 1) * width + width - j - 1] * x[width - j - 1];
            }
            x[width - i - 1] = xi / u[(width - i - 1) * width + width - i - 1];
        }
        x
    }
}

impl LinearEquationSolver for LUSolver {
    fn solve(&self, mat: &[f64], width: usize, b: &[f64]) -> Option<Vec<f64>> {
        let (l, u) = self.lu(mat, width)?;
        self.gauss_from_lu(&l, &u, b, width)
    }
}
