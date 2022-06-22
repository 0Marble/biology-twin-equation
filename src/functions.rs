// use exmex::prelude::*;

pub trait Function: Sync {
    fn get(&self, x: f64) -> f64;
    fn to_vec(&self, left: f64, right: f64, node_count: usize) -> Vec<(f64, f64)> {
        let step = (right - left) / (node_count - 1) as f64;
        (0..node_count)
            .map(|i| {
                let x = (i as f64) * step + left;
                (x, self.get(x))
            })
            .collect()
    }
}

// impl Function for exmex::FlatEx<f64> {
//     fn get(&self, x: f64) -> f64 {
//         self.eval(&[x]).unwrap()
//     }
// }

pub trait Function2d: Sync {
    fn get(&self, x: f64, y: f64) -> f64;
}

impl<T> Function for T
where
    T: Fn(f64) -> f64 + Sync,
{
    fn get(&self, x: f64) -> f64 {
        self(x)
    }
}

impl<T> Function2d for T
where
    T: Fn(f64, f64) -> f64 + Sync,
{
    fn get(&self, x: f64, y: f64) -> f64 {
        self(x, y)
    }
}

pub struct PointFunction {
    pts: Vec<f64>,
    left: f64,
    right: f64,
}

impl PointFunction {
    pub fn new(pts: Vec<f64>, left: f64, right: f64) -> Self {
        Self { pts, left, right }
    }
}

impl Function for PointFunction {
    fn get(&self, x: f64) -> f64 {
        if self.left > x {
            return self.pts[0];
        }

        let step = (self.right - self.left) / (self.pts.len() - 1) as f64;
        let i = ((x - self.left) / step).floor() as usize;
        let t = ((x - self.left) / step).fract();

        if i + 2 > self.pts.len() {
            return self.pts.last().unwrap().to_owned();
        }

        self.pts[i] * t + self.pts[i + 1] * (1.0 - t)
    }
}
