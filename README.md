# Ndumiso-Ngcobo-01
Linear Regression Model using Rust Burn Library

# Overview

This project implements a simple linear regression model in Rust using the Burn library (version 0.16.0). The model predicts the function:
using synthetic data generated with some noise to simulate real-world conditions.
Features
Generates synthetic training data
Defines a linear regression model using Burn
Trains the model using the Mean Squared Error (MSE) loss function
Evaluates and visualizes results using textplots

Setup Instructions
# 1. Install Rust
Ensure you have Rust installed:
If Rust is not installed, download it from Rust Lang.

# 2. Clone the Repository
git clone <your-repo-url>
cd linear_regression_model

# 3. Install Dependencies
cargo build
# 4. Run the Program
linear_regression_model/
│── src/
│   ├── main.rs   # Main Rust program
│── Cargo.toml    # Dependencies and project metadata
│── README.md     # Documentation
# Results & Evaluation

The program prints synthetic data points.

Plots a simple graph using textplots.

# Challenges faced:
Adapting the Burn library to the task.

# Resources used:
Burn documentation, Rust official guide, online tutorials.

# License

This project is open-source and licensed under the MIT License.



The required dependencies are listed in Cargo.toml. Run:
