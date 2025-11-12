# Quantum Limit-Graph v2.4.0 Level 5 MetaAgent - README

## Overview

Level 5 introduces **MetaAgent** - an advanced reasoning trace management system with memory folding, provenance logging, and contributor leaderboards for multilingual scientific research.

## Quick Start

```rust
use quantum_limit_graph::level5::{MetaAgent, AgentType, Leaderboard, RankingCriteria};

// Create MetaAgent
let mut meta = MetaAgent::new("researcher_id", "quantum_backend_v3");

// Log reasoning steps
meta.log_event(
    AgentType::Classification,
    "What is quantum entanglement?",
    "Task: quantum_physics_explanation",
    "en",
    0.95,
);

meta.log_event(
    AgentType::Reasoning,
    "Analyze quantum entanglement",
    "Entanglement is quantum correlation between particles",
    "en",
    0.92,
);

// Generate provenance
let provenance = meta.emit_provenance();
println!("Trace hash: {}", provenance.trace_hash);
println!("Uniqueness: {:.3}", provenance.uniqueness_score);

// Fold memory
let folded = meta.fold_memory();
println!("Compression: {:.2}%", folded.compression_ratio * 100.0);

// Add to leaderboard
let mut leaderboard = Leaderboard::new();
leaderboard.add_entry(provenance, vec!["en".to_string()]);
leaderboard.display(RankingCriteria::Combined);
```

## Features

### 🧠 Memory Folding
Hierarchical compression of reasoning traces:
- Automatic compression ratio calculation
- Key insights extraction
- Language distribution tracking
- Session-based organization

### 🔐 Provenance Logging
SHA-256 hash-based originality detection:
- Cryptographic trace fingerprinting
- Uniqueness score computation
- Temporal audit trail
- Agent sequence recording

### 🔄 Agent Transition Tracking
Monitor reasoning flow between agents:
- Automatic transition detection
- Quality scoring for each transition
- Pattern identification
- Flow optimization

### 🌐 Multilingual Support
Cross-language scientific reasoning:
- Language-tagged events
- Translation tracking
- Multilingual insights
- Cultural context preservation

### 👤 Contributor Personalization
Profile-based optimization:
- Preferred languages
- Expertise domains
- Reasoning style
- Performance metrics

### 🏆 Leaderboard System
Rank contributors by multiple criteria:
- **Trace Depth**: Number of reasoning steps
- **Uniqueness Score**: Originality metric
- **Total Submissions**: Contribution frequency
- **Average Depth**: Consistency metric
- **Combined Score**: Weighted ranking

## Agent Types

| Type | Purpose | Example |
|------|---------|---------|
| `Classification` | Task categorization | "Identify query type" |
| `Reasoning` | Logical analysis | "Analyze problem" |
| `Translation` | Language conversion | "Translate to English" |
| `Retrieval` | Information gathering | "Search database" |
| `Validation` | Result verification | "Verify accuracy" |
| `Synthesis` | Answer compilation | "Generate final answer" |
| `Action` | Task execution | "Execute command" |
| `Meta` | High-level orchestration | "Coordinate agents" |

## Examples

### Multilingual Reasoning

```rust
let mut meta = MetaAgent::new("multilingual_user", "quantum_backend_v3");

// Indonesian query
meta.log_event(
    AgentType::Classification,
    "Apa itu komputasi kuantum?",
    "Task: quantum_explanation",
    "id",
    0.93,
);

// Translation
meta.log_event(
    AgentType::Translation,
    "Translate to English",
    "What is quantum computing?",
    "en",
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

// Indonesian synthesis
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

### Leaderboard

```rust
let mut leaderboard = Leaderboard::new();

// Add multiple contributors
for i in 0..5 {
    let mut meta = MetaAgent::new(&format!("user{}", i), "backend");
    for _ in 0..(i + 1) * 5 {
        meta.log_event(AgentType::Reasoning, "input", "output", "en", 0.9);
    }
    let prov = meta.emit_provenance();
    leaderboard.add_entry(prov, vec!["en".to_string()]);
}

// Display rankings
leaderboard.display(RankingCriteria::Combined);

// Get top 3
let top_3 = leaderboard.get_top_n(3, RankingCriteria::TraceDepth);
```

## Running

### Demo
```bash
cargo run --example level5_demo
```

### Tests
```bash
cargo test test_level5 -- --nocapture
```

### Integration
```bash
cargo run --example sample_integration
```

## Documentation

- **Full Documentation**: `../LEVEL_5_COMPLETE.md`
- **Quick Start**: `../LEVEL_5_QUICK_START.md`
- **Delivery Summary**: `../LEVEL_5_DELIVERY_SUMMARY.md`

## API Reference

### MetaAgent

```rust
// Creation
let meta = MetaAgent::new(contributor_id, backend);
let meta = MetaAgent::with_profile(contributor_id, backend, profile);

// Event logging
meta.log_event(agent_type, input, output, language, confidence);
meta.log_event_with_metadata(agent_type, input, output, language, confidence, metadata);

// Transition tracking
meta.track_transition(from_agent, to_agent, reason);

// Memory folding
let folded = meta.fold_memory();

// Provenance
let provenance = meta.emit_provenance();

// Profile
meta.update_profile();

// Metrics
let depth = meta.get_trace_depth();
let transitions = meta.get_transition_count();

// Export
let trace_json = meta.export_trace_json()?;
let prov_json = meta.export_provenance_json()?;
```

### Leaderboard

```rust
// Creation
let leaderboard = Leaderboard::new();

// Add entries
leaderboard.add_entry(provenance, languages);

// Ranking
let by_depth = leaderboard.rank_by_depth();
let by_uniqueness = leaderboard.rank_by_uniqueness();
let by_submissions = leaderboard.rank_by_submissions();
let combined = leaderboard.rank_combined();

// Display
leaderboard.display(RankingCriteria::Combined);
leaderboard.display_contributor(contributor_id);

// Query
let top_n = leaderboard.get_top_n(n, criteria);
let history = leaderboard.get_contributor_history(contributor_id);

// Statistics
let total_contributors = leaderboard.total_contributors();
let total_submissions = leaderboard.total_submissions();

// Export
let json = leaderboard.export_json(criteria)?;
```

## Performance

| Operation | Time | Memory |
|-----------|------|--------|
| Event logging | <1μs | ~300 bytes |
| SHA-256 hash | 1-2ms | - |
| Memory folding | <5ms | 5-20% of original |
| Leaderboard ranking | <10ms | ~500 bytes/entry |

## Integration

### With Level 4

```rust
use quantum_limit_graph::level4::AgentMesh;
use quantum_limit_graph::level5::MetaAgent;

let mesh = AgentMesh::new();
let mut meta = MetaAgent::new("user", "backend");

// Track mesh operations
meta.log_event(AgentType::Action, "Execute via mesh", "Success", "en", 0.94);
```

### With Databases

```rust
// PostgreSQL
let provenance = meta.emit_provenance();
sqlx::query!(
    "INSERT INTO provenance (hash, contributor, depth) VALUES ($1, $2, $3)",
    provenance.trace_hash,
    provenance.contributor_id,
    provenance.trace_depth as i32
).execute(&pool).await?;
```

## Best Practices

1. **Log all significant steps**: Don't skip important reasoning
2. **Use correct agent types**: Choose appropriate agent for each step
3. **Tag languages accurately**: Essential for multilingual analysis
4. **Set realistic confidence**: Reflects actual certainty
5. **Fold memory regularly**: After long sessions or periodically
6. **Update profiles**: Keep contributor data current
7. **Export for backup**: Regularly save traces and provenance

## Troubleshooting

### Low uniqueness score
Use diverse agent types and languages:
```rust
// Good: Diverse agents and languages
meta.log_event(AgentType::Classification, "in", "out", "en", 0.9);
meta.log_event(AgentType::Translation, "in", "out", "id", 0.88);
meta.log_event(AgentType::Reasoning, "in", "out", "en", 0.92);
```

### High memory usage
Fold memory regularly:
```rust
if meta.get_trace_depth() % 50 == 0 {
    let folded = meta.fold_memory();
    // Store and clear if needed
}
```

### Leaderboard not updating
Call update_ranks:
```rust
leaderboard.add_entry(provenance, languages);
leaderboard.update_ranks(RankingCriteria::Combined);
```

## Contributing

See `../CONTRIBUTING.md` for contribution guidelines.

## License

See `../LICENSE` for license information.

## Support

- **Issues**: GitHub Issues
- **Discussions**: GitHub Discussions
- **Documentation**: See docs folder

---

**Version**: 2.4.0 Level 5
**Status**: Production Ready ✅
