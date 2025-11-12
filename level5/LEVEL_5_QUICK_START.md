# Level 5 MetaAgent - Quick Start Guide

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
quantum-limit-graph = "2.4.0"
chrono = "0.4"
sha2 = "0.10"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 5-Minute Tutorial

### Step 1: Create a MetaAgent

```rust
use quantum_limit_graph::level5::{MetaAgent, AgentType};

fn main() {
    let mut meta = MetaAgent::new("your_name", "quantum_backend_v3");
    println!("✅ MetaAgent created!");
}
```

### Step 2: Log Reasoning Steps

```rust
// Classification
meta.log_event(
    AgentType::Classification,
    "What is the capital of Indonesia?",
    "Task: geography_query",
    "en",
    0.95,
);

// Reasoning
meta.log_event(
    AgentType::Reasoning,
    "Analyze geography query",
    "Indonesia's capital is Jakarta",
    "en",
    0.98,
);

// Synthesis
meta.log_event(
    AgentType::Synthesis,
    "Compile final answer",
    "The capital of Indonesia is Jakarta",
    "en",
    0.99,
);

println!("✅ Logged {} reasoning steps", meta.get_trace_depth());
```

### Step 3: Generate Provenance

```rust
let provenance = meta.emit_provenance();

println!("🔐 Provenance:");
println!("  Hash: {}...", &provenance.trace_hash[..16]);
println!("  Depth: {}", provenance.trace_depth);
println!("  Uniqueness: {:.3}", provenance.uniqueness_score);
```

### Step 4: Fold Memory

```rust
let folded = meta.fold_memory();

println!("🧠 Memory Fold:");
println!("  Compression: {:.2}%", folded.compression_ratio * 100.0);
println!("  Insights: {:?}", folded.key_insights);
```

### Step 5: Add to Leaderboard

```rust
use quantum_limit_graph::level5::{Leaderboard, RankingCriteria};

let mut leaderboard = Leaderboard::new();
leaderboard.add_entry(provenance, vec!["en".to_string()]);

leaderboard.display(RankingCriteria::Combined);
```

## Multilingual Example

```rust
let mut meta = MetaAgent::new("multilingual_user", "quantum_backend_v3");

// Indonesian query
meta.log_event(
    AgentType::Classification,
    "Apa itu komputasi kuantum?",
    "Task: quantum_explanation",
    "id",  // Indonesian
    0.93,
);

// Translation
meta.log_event(
    AgentType::Translation,
    "Translate to English",
    "What is quantum computing?",
    "en",  // English
    0.91,
);

// English reasoning
meta.log_event(
    AgentType::Reasoning,
    "Explain quantum computing",
    "Uses qubits and superposition",
    "en",
    0.94,
);

// Indonesian answer
meta.log_event(
    AgentType::Synthesis,
    "Answer in Indonesian",
    "Komputasi kuantum menggunakan qubit",
    "id",
    0.96,
);

let folded = meta.fold_memory();
println!("Languages: {:?}", folded.language_distribution);
// Output: {"id": 2, "en": 2}
```

## Running the Demo

```bash
# Clone repository
git clone https://github.com/yourusername/quantum-limit-graph-v2.4.0
cd quantum-limit-graph-v2.4.0/rust

# Run Level 5 demo
cargo run --example level5_demo

# Run tests
cargo test test_level5 -- --nocapture
```

## Common Patterns

### Pattern 1: Scientific Research

```rust
let mut meta = MetaAgent::new("researcher", "quantum_backend_v3");

meta.log_event(AgentType::Classification, "Research question", "Task type", "en", 0.9);
meta.log_event(AgentType::Retrieval, "Search literature", "Found papers", "en", 0.88);
meta.log_event(AgentType::Reasoning, "Analyze findings", "Key insights", "en", 0.92);
meta.log_event(AgentType::Validation, "Verify results", "Confirmed", "en", 0.95);
meta.log_event(AgentType::Synthesis, "Write conclusion", "Final answer", "en", 0.97);
```

### Pattern 2: Cross-Language Translation

```rust
let mut meta = MetaAgent::new("translator", "quantum_backend_v3");

meta.log_event(AgentType::Classification, "Source text", "Translation task", "en", 0.9);
meta.log_event(AgentType::Translation, "Translate", "Target text", "id", 0.88);
meta.log_event(AgentType::Validation, "Check quality", "Validated", "id", 0.91);
```

### Pattern 3: Multi-Agent Collaboration

```rust
let mut meta = MetaAgent::new("team_lead", "quantum_backend_v3");

meta.log_event(AgentType::Classification, "Complex task", "Decompose", "en", 0.9);
meta.log_event(AgentType::Action, "Delegate subtask 1", "Completed", "en", 0.88);
meta.log_event(AgentType::Action, "Delegate subtask 2", "Completed", "en", 0.89);
meta.log_event(AgentType::Synthesis, "Merge results", "Final output", "en", 0.94);
```

## Agent Types Reference

| Agent Type | Use Case | Example |
|------------|----------|---------|
| `Classification` | Task categorization | "Identify query type" |
| `Reasoning` | Logical analysis | "Analyze problem" |
| `Translation` | Language conversion | "Translate to English" |
| `Retrieval` | Information gathering | "Search database" |
| `Validation` | Result verification | "Verify accuracy" |
| `Synthesis` | Answer compilation | "Generate final answer" |
| `Action` | Task execution | "Execute command" |
| `Meta` | High-level orchestration | "Coordinate agents" |

## Leaderboard Criteria

```rust
// Rank by trace depth (most reasoning steps)
leaderboard.display(RankingCriteria::TraceDepth);

// Rank by uniqueness (most original)
leaderboard.display(RankingCriteria::UniquenessScore);

// Rank by submissions (most active)
leaderboard.display(RankingCriteria::TotalSubmissions);

// Rank by average depth (most consistent)
leaderboard.display(RankingCriteria::AvgTraceDepth);

// Combined ranking (weighted)
leaderboard.display(RankingCriteria::Combined);
```

## Export Data

```rust
// Export trace to JSON
let trace_json = meta.export_trace_json()?;
std::fs::write("trace.json", trace_json)?;

// Export provenance to JSON
let prov_json = meta.export_provenance_json()?;
std::fs::write("provenance.json", prov_json)?;

// Export leaderboard to JSON
let board_json = leaderboard.export_json(RankingCriteria::Combined)?;
std::fs::write("leaderboard.json", board_json)?;
```

## Troubleshooting

### Issue: Low uniqueness score

**Solution**: Use diverse agent types and languages

```rust
// Bad: Repetitive
meta.log_event(AgentType::Reasoning, "step1", "out1", "en", 0.9);
meta.log_event(AgentType::Reasoning, "step2", "out2", "en", 0.9);

// Good: Diverse
meta.log_event(AgentType::Classification, "step1", "out1", "en", 0.9);
meta.log_event(AgentType::Reasoning, "step2", "out2", "en", 0.9);
meta.log_event(AgentType::Translation, "step3", "out3", "id", 0.9);
```

### Issue: Memory usage too high

**Solution**: Fold memory regularly

```rust
// After every 50 events
if meta.get_trace_depth() % 50 == 0 {
    let folded = meta.fold_memory();
    // Store folded memory
    // Clear trace if needed
}
```

### Issue: Leaderboard not updating

**Solution**: Call update_ranks

```rust
leaderboard.add_entry(provenance, languages);
leaderboard.update_ranks(RankingCriteria::Combined);
```

## Next Steps

1. **Read Full Documentation**: See `LEVEL_5_COMPLETE.md`
2. **Explore Examples**: Check `examples/level5_demo.rs`
3. **Run Tests**: Execute `cargo test test_level5`
4. **Integrate with Level 4**: Combine with agent mesh
5. **Deploy to Production**: See deployment guide

## Support

- **Documentation**: `LEVEL_5_COMPLETE.md`
- **Examples**: `examples/level5_demo.rs`
- **Tests**: `tests/test_level5.rs`
- **Integration**: `src/level5/sample_integration.rs`

## Quick Reference Card

```rust
// Create
let mut meta = MetaAgent::new("user", "backend");

// Log
meta.log_event(AgentType::Reasoning, "in", "out", "en", 0.9);

// Provenance
let prov = meta.emit_provenance();

// Memory
let fold = meta.fold_memory();

// Leaderboard
let mut board = Leaderboard::new();
board.add_entry(prov, vec!["en".to_string()]);
board.display(RankingCriteria::Combined);
```

Happy reasoning! 🚀
