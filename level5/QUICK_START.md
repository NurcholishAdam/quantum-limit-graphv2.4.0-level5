# Quick Start Guide - Rust Implementation

Get up and running with Quantum Limit Graph v2.4.0 in Rust in under 5 minutes.

## Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs))
- Cargo (comes with Rust)
- Git

## Installation

### Step 1: Navigate to Rust Directory

```bash
cd quantum_integration/quantum-limit-graph-v2.4.0/rust
```

### Step 2: Build the Project

```bash
# Debug build (faster compilation)
cargo build

# Release build (optimized performance)
cargo build --release
```

### Step 3: Run Tests

```bash
cargo test
```

## Your First Program

### Option 1: Run the Example

```bash
cargo run --example simple_demo
```

### Option 2: Run the Main Binary

```bash
cargo run --release
```

### Option 3: Write Your Own

Create `my_demo.rs`:

```rust
use quantum_limit_graph::{Level3Agent, Level3Config, EditRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Create agent with default configuration
    let config = Level3Config::default();
    let agent = Level3Agent::new(config)?;
    
    // Create an edit request
    let request = EditRequest {
        edit_content: "The capital of France is Paris".to_string(),
        language: "english".to_string(),
        rank: 128,
        domain: Some("legal".to_string()),
        contributor_id: Some("user_001".to_string()),
        multimodal_data: None,
    };
    
    // Process the edit
    let result = agent.process_edit(request).await?;
    
    // Print results
    println!("Overall Score: {:.3}", result.overall_score);
    println!("Stages Completed: {}", result.stages.len());
    
    Ok(())
}
```

Run it:

```bash
cargo run --bin my_demo
```

## Common Tasks

### Running Benchmarks

```bash
cargo bench
```

### Checking Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check for errors without building
cargo check
```

### Building Documentation

```bash
cargo doc --open
```

### Running Specific Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_level3_agent_creation

# Run with output
cargo test -- --nocapture
```

## Configuration Examples

### Minimal Configuration

```rust
let config = Level3Config {
    enable_quantum_transformers: true,
    enable_meta_cognitive_rl: false,
    enable_multimodal: false,
    enable_adaptive_optimization: false,
    enable_entangled_memory: true,
    enable_domain_modules: vec![],
    num_qubits: 4,
    num_layers: 2,
    max_shards: 1000,
};
```

### Full Configuration

```rust
let config = Level3Config {
    enable_quantum_transformers: true,
    enable_meta_cognitive_rl: true,
    enable_multimodal: true,
    enable_adaptive_optimization: true,
    enable_entangled_memory: true,
    enable_domain_modules: vec![
        "legal".to_string(),
        "medical".to_string(),
        "scientific".to_string(),
    ],
    num_qubits: 8,
    num_layers: 4,
    max_shards: 10000,
};
```

## Performance Tips

### 1. Always Use Release Mode for Benchmarks

```bash
cargo run --release
cargo bench
```

### 2. Enable Link-Time Optimization

Add to `Cargo.toml`:

```toml
[profile.release]
lto = true
codegen-units = 1
```

### 3. Use Parallel Processing

The Rust implementation automatically uses all available CPU cores.

### 4. Monitor Memory Usage

```bash
cargo run --release -- --memory-profile
```

## Troubleshooting

### Build Errors

```bash
# Clean and rebuild
cargo clean
cargo build --release
```

### Missing Dependencies

```bash
# Update dependencies
cargo update
```

### Test Failures

```bash
# Run tests with backtrace
RUST_BACKTRACE=1 cargo test
```

## Next Steps

- Read the [Full Documentation](./README.md)
- Explore [Examples](./examples/)
- Check [Performance Guide](./DEPLOYMENT.md)
- Review [API Documentation](https://docs.rs/quantum-limit-graph)

## Getting Help

- GitHub Issues: [Report a bug](https://github.com/nurcholishadam/quantum-limit-graph-v.2.4.0-level-3-maturity/issues)
- Documentation: [Full docs](./README.md)
- Examples: [Code examples](./examples/)

## Performance Expectations

On a modern CPU (e.g., Intel i7 or AMD Ryzen 7):

- Single edit processing: **~150-200 µs**
- Benchmark (100 edits): **~20-25 ms**
- Quantum circuit simulation: **~10-15 µs**
- Memory usage: **~50-100 MB**

These are **10-100x faster** than the Python implementation!
