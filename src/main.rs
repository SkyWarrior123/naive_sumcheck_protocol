use rand::Rng;

struct Prover {
    function: Box<dyn Fn(&[i32]) -> i32>,
    num_vars: usize,
}

struct Verifier {
    num_vars: usize,
}

impl Prover {
    fn new(function: Box<dyn Fn(&[i32]) -> i32>, num_vars: usize) -> Self {
        Self { function, num_vars }
    }

    fn calculate_sum(&self) -> i32 {
        let mut total = 0;
        for i in 0..(1 << self.num_vars) {
            let input: Vec<i32> = (0..self.num_vars).map(|j| (i >> j) & 1).collect();
            total += (self.function)(&input);
        }
        total
    }

    /// Simulates a univariate polynomial over the current variable,
    /// with fixed inputs so far. Returns values at x=0 and x=1.
    fn evaluate_polynomial(&self, fixed_inputs: &[i32], var_index: usize) -> [i32; 2] {
        let mut evaluations = [0; 2];
        for val in 0..=1 {
            let mut inputs = fixed_inputs.to_vec();
            inputs.push(val);
            // Pad remaining variables with 0 for consistent arity
            while inputs.len() < self.num_vars {
                inputs.push(0);
            }

            // Sum over all combinations of remaining variables
            let mut sum = 0;
            let remaining_vars = self.num_vars - (var_index + 1);
            for i in 0..(1 << remaining_vars) {
                let mut input = inputs.clone();
                for j in 0..remaining_vars {
                    input[var_index + 1 + j] = (i >> j) & 1;
                }
                sum += (self.function)(&input);
            }

            evaluations[val as usize] = sum;
        }
        evaluations
    }
}

impl Verifier {

    fn new(num_vars: usize) -> Self {
        Self { num_vars }
    }

    fn verify(&self, claimed_sum: i32, prover: &Prover) -> bool {
        let mut rng = rand::thread_rng();
        let mut r_values = Vec::new();     // Stores random values chosen so far
        let mut running_sum = claimed_sum; // Expected sum to verify at each step

        for i in 0..self.num_vars {
            // Prover returns f_i(0) and f_i(1) (sum over all continuations)
            let poly_vals = prover.evaluate_polynomial(&r_values, i);
            let expected_total = poly_vals[0] + poly_vals[1];

            // ✅ Check if the prover's polynomial adds up correctly
            if running_sum != expected_total {
                println!(
                    "❌ Round {}: mismatch. Expected total {} != f(0) + f(1) = {} + {} = {}",
                    i, running_sum, poly_vals[0], poly_vals[1], expected_total
                );
                return false;
            }

            // Sample r_i ∈ {0, 1}
            let r = rng.gen_range(0..=1);

            // Interpolate: f(r) = f(0) * (1 - r) + f(1) * r
            let next_sum = poly_vals[0] * (1 - r) + poly_vals[1] * r;

            println!(
                "✅ Round {}: f(0)={}, f(1)={}, r={} → f(r)={} (✓)",
                i, poly_vals[0], poly_vals[1], r, next_sum
            );

            // Advance to next round
            r_values.push(r);
            running_sum = next_sum;
        }

        // If we reach here, all rounds passed
        true
    }
}


/// Example multilinear function over 5 binary variables
fn example_function(input: &[i32]) -> i32 {
    let (b1, b2, b3, b4, b5) = (input[0], input[1], input[2], input[3], input[4]);
    2 * b1.pow(2) + b2 + b1 * b2 * b3 - b4 + b2 * b5.pow(3)
}

fn main() {
    let num_vars = 5;
    let prover = Prover::new(Box::new(example_function), num_vars);
    let claimed_sum = prover.calculate_sum();
    println!("The prover claims the total sum is: {}", claimed_sum);

    let verifier = Verifier::new(num_vars);
    let result = verifier.verify(claimed_sum, &prover);

    if result {
        println!("✅ Verifier: SumCheck passed!");
    } else {
        println!("❌ Verifier: SumCheck failed!");
    }
}
