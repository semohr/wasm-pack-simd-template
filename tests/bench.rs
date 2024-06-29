use wasm_bindgen::prelude::*;

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Function to run another function n times and record the execution times
pub fn measure_execution_time<F>(func: F, n: usize, i: usize)
where
    F: Fn(),
{
    let mut durations: Vec<f64> = Vec::with_capacity(n);

    for _ in 0..n {
        let start = js_sys::Date::now();
        for _ in 0..i {
            func();
        }
        let end = js_sys::Date::now();
        durations.push(end - start);
    }

    // Calculate mean
    let total_duration: f64 = durations.iter().sum();
    let mean_duration = total_duration / (n as f64);

    // Calculate median
    durations.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let median_duration = if n % 2 == 0 {
        (durations[n / 2 - 1] + durations[n / 2]) / 2.
    } else {
        durations[n / 2]
    };

    console_log!("Mean execution time: {:?}", mean_duration * 1000.);
    console_log!("Median execution time: {:?}", median_duration * 1000.);
}
