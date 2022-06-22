use crate::functions::Function;

pub trait Method {
    fn solve(
        &self,
        birth_probability: &dyn Function,
        death_probability: &dyn Function,
        parameter: f64,
        width: f64,
    ) -> Box<dyn Function>;
}
