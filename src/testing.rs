use crate::{functions::Function, method::Method};
use std::fs::File;
use std::io::Write;

fn save_csv(func: &dyn Function, left: f64, right: f64, node_count: usize, file_path: &str) {
    let mut file = File::create(file_path).unwrap();
    let step = (right - left) / (node_count - 1) as f64;

    for i in 0..node_count {
        let x = (i as f64) * step + left;
        write!(file, "{x},{}\n", func.get(x)).unwrap();
    }
}

macro_rules! measure {
    ($func:expr) => {{
        use std::time::Instant;
        let start = Instant::now();
        let res = $func;
        (res, start.elapsed())
    }};
}

pub fn test_method(
    method: &dyn Method,
    m: &dyn Function,
    w: &dyn Function,
    parameter: f64,
    actual: &dyn Function,
    comparison_point_count: usize,
    width: f64,
    out_name: &str,
    out_dir: &str,
    name_prefix: &str,
) {
    let (answer, calc_duration) = measure!(method.solve(m, w, parameter, width));

    let diff = |x| (answer.get(x) - actual.get(x)).abs() / actual.get(x) * 100.0;

    let mut diff_vals: Vec<f64> = diff
        .to_vec(0.0, width, comparison_point_count)
        .iter()
        .map(|(_, y)| y.to_owned())
        .collect();
    diff_vals.sort_by(|x, y| x.partial_cmp(y).unwrap());

    let max_diff = diff_vals.iter().last().unwrap().to_owned();
    let mean = diff_vals.iter().sum::<f64>() / diff_vals.len() as f64;
    let median = diff_vals[diff_vals.len() / 2];

    let (_, save_duration) = measure!({
        save_csv(
            actual,
            0.0,
            width,
            comparison_point_count,
            &format!("{out_dir}/{name_prefix}_actual.csv"),
        );
        save_csv(
            answer.as_ref(),
            0.0,
            width,
            comparison_point_count,
            &format!("{out_dir}/{name_prefix}_{out_name}.csv"),
        );
        save_csv(
            &diff,
            0.0,
            width,
            comparison_point_count,
            &format!("{out_dir}/{name_prefix}_{out_name}_diff.csv"),
        )
    });

    let mut stats_file =
        File::create(&format!("{out_dir}/{name_prefix}_{out_name}_stats.csv")).unwrap();
    writeln!(
        stats_file,
        "{name_prefix}_{out_name}:
\tCalculation took {}ms
\tSaving took {}ms
\tMax difference {}%\tMean: {}%\tMedian: {}%",
        calc_duration.as_millis(),
        save_duration.as_millis(),
        max_diff,
        mean,
        median
    )
    .unwrap();

    println!(
        "{name_prefix}_{out_name}:
\tCalculation took {}ms
\tSaving took {}ms
\tMax difference {}%\tMean: {}%\tMedian: {}%",
        calc_duration.as_millis(),
        save_duration.as_millis(),
        max_diff,
        mean,
        median
    );
}
