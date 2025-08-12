# 🧠 Naive SumCheck Protocol in Rust

A simple Rust implementation of the **SumCheck protocol**, demonstrating how a Prover and Verifier can interact to verify claims about the sum of a multilinear polynomial over binary inputs.  
Inspired by interactive proof systems and zero-knowledge techniques used in zk-SNARKs and probabilistic checking.

---

## ✨ Features

- SumCheck protocol over {0,1}ⁿ domains
- Prover calculates the true sum of a function over all 2ⁿ binary inputs
- Verifier probabilistically checks the claim using random challenges
- Fully self-contained and does not rely on external math libraries
- Clear, educational output for each verification round

---

## 🧪 Example Output
```
The prover claims the total sum is: 44  
✅ Round 0: f(0)=4, f(1)=40, r=0 → f(r)=4 (✓)  
✅ Round 1: f(0)=-4, f(1)=8, r=1 → f(r)=8 (✓)  
✅ Round 2: f(0)=4, f(1)=4, r=0 → f(r)=4 (✓)  
✅ Round 3: f(0)=3, f(1)=1, r=1 → f(r)=1 (✓)  
✅ Round 4: f(0)=0, f(1)=1, r=0 → f(r)=0 (✓)  
✅ Verifier: SumCheck passed!
```


---

## 🧰 Getting Started

### 📦 Prerequisites

- Rust (version 1.70+ recommended)
- Cargo (comes with Rust)

Install Rust via [https://rustup.rs](https://rustup.rs)

### 🚀 Run the protocol

Clone the repository:

```bash
git clone https://github.com/yourusername/naive_sumcheck_protocol.git
cd naive_sumcheck_protocol
cargo run

🛠 How It Works
🧮 Function

The protocol verifies the sum of a function f(x₁, x₂, ..., xₙ) over all binary inputs (0 or 1). The Prover claims the sum:

S = ∑ f(x₁, ..., xₙ) over {0,1}ⁿ

The Verifier:

    Does not compute the full sum

    Asks the Prover to reduce the function via univariate polynomials at each round

    Samples a random challenge rᵢ ∈ {0,1} and verifies the claimed sum incrementally

🔐 Multilinearity

The function used must be multilinear (i.e. no exponents, just additions and multiplications over binary variables).
You can customize the function inside example_function:

fn example_function(input: &[i32]) -> i32 {
    let (b1, b2, b3, b4, b5) = (input[0], input[1], input[2], input[3], input[4]);
    b1 + b2 * b3 - b4 + b5 // Must remain multilinear
}

📁 Project Structure

.
├── src
│   └── main.rs        # Prover, Verifier and protocol logic
├── Cargo.toml         # Dependencies
└── README.md