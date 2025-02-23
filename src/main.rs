// main.rs
use burn::tensor::Tensor;
use burn::module::Module;
use burn::nn::Linear;
use burn::tensor::backend::Backend;
use rand::Rng;
use textplots::{Chart, Plot, Shape};


// Define the linear model
#[derive(Module, Debug)]
struct LinearRegression<B: Backend> {
    linear: Linear<B>,
}

impl<B: Backend> LinearRegression<B> {
    pub fn forward(&self, x: Tensor<B, 2>) -> Tensor<B, 2> {
        self.linear.forward(x)
    }
}

// Generate synthetic data
fn generate_data(n: usize) -> (Vec<f32>, Vec<f32>) {
    let mut rng = rand::thread_rng();
    let mut x_vals = Vec::new();
    let mut y_vals = Vec::new();

    for _ in 0..n {
        let x = rng.gen_range(-10.0..10.0);
        let y = 2.0 * x + 1.0 + rng.gen_range(-1.0..1.0);
        x_vals.push(x);
        y_vals.push(y);
    }

    (x_vals, y_vals)
}

fn main() {
    let (x_vals, y_vals) = generate_data(100);

    // Print sample data points
    for i in 0..10 {
        println!("x: {:.2}, y: {:.2}", x_vals[i], y_vals[i]);
    }

    // Plot results
    Chart::new(180, 60, -10.0, 10.0)
        .lineplot(&Shape::Points(&x_vals.iter().zip(y_vals.iter()).map(|(&x, &y)| (x, y)).collect::<Vec<_>>()))
        .display();
}
