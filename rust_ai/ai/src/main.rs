use rand::RngExt;
use std::thread;
use std::time::Duration;

struct StepData {
    prediction: f64,
    error: f64,
    avg_abs_error: f64,
    w0: f64,
    w1: f64,
    bias: f64,
}

fn study(steps: usize, lr: f64) -> StepData {
    let mut rng = rand::rng();

    let mut step = StepData {
        prediction: 0.0,
        error: 0.0,
        avg_abs_error: 0.0,
        w0: rng.random_range(0.0..1.0),
        w1: rng.random_range(0.0..1.0),
        bias: rng.random_range(0.1..0.5),
    };

    let mut error_sum = 0.0;

    for step_i in 1..=steps {
        let x0 = rng.random_range(0..10);
        let x1 = rng.random_range(0..10);
        let res = x0 + x1;

        let x0_f = x0 as f64;
        let x1_f = x1 as f64;
        let target = res as f64;

        step.prediction = x0_f * step.w0 + x1_f * step.w1 + step.bias;
        step.error = target - step.prediction;
        error_sum += step.error.abs();

        step.w0 += lr * step.error * x0_f;
        step.w1 += lr * step.error * x1_f;
        step.bias += lr * step.error;

        if step_i % 1000 == 0 || step_i == 1 {
            step.avg_abs_error = error_sum / (step_i as f64);
            println!(
                "Step {:4} | {:2} + {:2} = {:2} | pred={:6.2} | err={:6.2} | avg_err={:6.4} | w0={:.4}, w1={:.4}, bias={:.4}",
                step_i, x0, x1, res, step.prediction, step.error, step.avg_abs_error, step.w0, step.w1, step.bias
            );
        }
    }

    step.avg_abs_error = error_sum / (steps as f64);
    step
}

fn main() {
    println!("AI on rust (stream SGD). Without framework");
    thread::sleep(Duration::from_millis(500));

    let steps = 10000000;
    let lr = 0.01;
    let step = study(steps, lr);

    println!(
        "\nFinal step: w0={:.4}, w1={:.4}, bias={:.4}, avg_abs_error={:.4}",
        step.w0, step.w1, step.bias, step.avg_abs_error
    );

    let test_x0 = 1000000.0;
    let test_x1 = 0.001;
    let test_pred = test_x0 * step.w0 + test_x1 * step.w1 + step.bias;
    println!(
        "Test {} + {} = {} -> prediction: {:.2}",
        test_x0, test_x1, test_x0 + test_x1, test_pred
    );
}