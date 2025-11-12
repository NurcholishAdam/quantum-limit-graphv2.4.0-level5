// level5/mod.rs - Level 5 MetaAgent Module
// 
// Level 5 introduces:
// - Memory Folding: Hierarchical compression of reasoning traces
// - Provenance Logging: SHA-256 hash-based originality detection
// - Agent Transition Tracking: Monitor reasoning flow between agents
// - Multilingual Scientific Reasoning: Cross-language research capabilities
// - Contributor Personalization: Profile-based optimization
// - Leaderboard System: Rank contributors by trace depth and uniqueness

pub mod meta_agent;
pub mod leaderboard;
pub mod sample_integration;

pub use meta_agent::{
    MetaAgent,
    AgentType,
    AgentEvent,
    AgentTransition,
    MemoryFold,
    ProvenanceLog,
    ContributorProfile,
};

pub use leaderboard::{
    Leaderboard,
    ContributorStats,
    RankingCriteria,
};

pub use sample_integration::{
    demo_meta_agent,
    demo_leaderboard,
    demo_complete_integration,
    run_all_demos,
};
