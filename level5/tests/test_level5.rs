// tests/test_level5.rs - Level 5 MetaAgent Integration Tests

use quantum_limit_graph::level5::{
    MetaAgent, AgentType, Leaderboard, RankingCriteria, ContributorProfile,
};

#[test]
fn test_meta_agent_creation() {
    let meta = MetaAgent::new("test_user", "test_backend");
    assert_eq!(meta.contributor_id, "test_user");
    assert_eq!(meta.backend_used, "test_backend");
    assert_eq!(meta.get_trace_depth(), 0);
    assert_eq!(meta.get_transition_count(), 0);
}

#[test]
fn test_meta_agent_with_profile() {
    let profile = ContributorProfile {
        contributor_id: "test_user".to_string(),
        preferred_languages: vec!["en".to_string(), "id".to_string()],
        expertise_domains: vec!["NLP".to_string()],
        reasoning_style: "analytical".to_string(),
        total_traces: 5,
        avg_trace_depth: 12.5,
    };

    let meta = MetaAgent::with_profile("test_user", "test_backend", profile.clone());
    assert_eq!(meta.profile.total_traces, 5);
    assert_eq!(meta.profile.avg_trace_depth, 12.5);
    assert_eq!(meta.profile.preferred_languages.len(), 2);
}

#[test]
fn test_event_logging() {
    let mut meta = MetaAgent::new("test_user", "test_backend");
    
    meta.log_event(
        AgentType::Classification,
        "test input",
        "test output",
        "en",
        0.95,
    );

    assert_eq!(meta.get_trace_depth(), 1);
    assert_eq!(meta.trace[0].language, "en");
    assert_eq!(meta.trace[0].confidence, 0.95);
}

#[test]
fn test_agent_transitions() {
    let mut meta = MetaAgent::new("test_user", "test_backend");
    
    meta.log_event(AgentType::Classification, "input1", "output1", "en", 0.9);
    meta.log_event(AgentType::Reasoning, "input2", "output2", "en", 0.92);
    meta.log_event(AgentType::Retrieval, "input3", "output3", "en", 0.88);

    assert_eq!(meta.get_transition_count(), 2); // 2 transitions between 3 different agents
}

#[test]
fn test_multilingual_logging() {
    let mut meta = MetaAgent::new("test_user", "test_backend");
    
    meta.log_event(AgentType::Classification, "English input", "output", "en", 0.9);
    meta.log_event(AgentType::Translation, "Indonesian input", "output", "id", 0.85);
    meta.log_event(AgentType::Reasoning, "Chinese input", "output", "zh", 0.88);

    let folded = meta.fold_memory();
    assert_eq!(folded.language_distribution.len(), 3);
    assert!(folded.language_distribution.contains_key("en"));
    assert!(folded.language_distribution.contains_key("id"));
    assert!(folded.language_distribution.contains_key("zh"));
}

#[test]
fn test_memory_folding() {
    let mut meta = MetaAgent::new("test_user", "test_backend");
    
    for i in 0..10 {
        meta.log_event(
            AgentType::Reasoning,
            &format!("input {}", i),
            &format!("output {}", i),
            "en",
            0.9,
        );
    }

    let folded = meta.fold_memory();
    assert_eq!(folded.folded_trace.len(), 10);
    assert!(!folded.summary.is_empty());
    assert!(folded.compression_ratio > 0.0);
    assert!(folded.compression_ratio <= 1.0);
}

#[test]
fn test_provenance_generation() {
    let mut meta = MetaAgent::new("test_user", "test_backend");
    
    meta.log_event(AgentType::Classification, "input1", "output1", "en", 0.9);
    meta.log_event(AgentType::Reasoning, "input2", "output2", "en", 0.92);

    let provenance = meta.emit_provenance();
    
    assert_eq!(provenance.contributor_id, "test_user");
    assert_eq!(provenance.backend_used, "test_backend");
    assert_eq!(provenance.trace_depth, 2);
    assert_eq!(provenance.trace_hash.len(), 64); // SHA-256 produces 64 hex chars
    assert!(provenance.uniqueness_score >= 0.0);
    assert!(provenance.uniqueness_score <= 1.0);
}

#[test]
fn test_provenance_uniqueness() {
    let mut meta1 = MetaAgent::new("user1", "backend1");
    let mut meta2 = MetaAgent::new("user2", "backend2");
    
    // Same inputs should produce same hash
    meta1.log_event(AgentType::Reasoning, "test", "result", "en", 0.9);
    meta2.log_event(AgentType::Reasoning, "test", "result", "en", 0.9);
    
    let prov1 = meta1.emit_provenance();
    let prov2 = meta2.emit_provenance();
    
    assert_eq!(prov1.trace_hash, prov2.trace_hash);
    
    // Different inputs should produce different hash
    let mut meta3 = MetaAgent::new("user3", "backend3");
    meta3.log_event(AgentType::Reasoning, "different", "result", "en", 0.9);
    let prov3 = meta3.emit_provenance();
    
    assert_ne!(prov1.trace_hash, prov3.trace_hash);
}

#[test]
fn test_profile_update() {
    let mut meta = MetaAgent::new("test_user", "test_backend");
    
    for i in 0..5 {
        meta.log_event(AgentType::Reasoning, "input", "output", "en", 0.9);
    }
    
    assert_eq!(meta.profile.total_traces, 0);
    meta.update_profile();
    assert_eq!(meta.profile.total_traces, 1);
    assert_eq!(meta.profile.avg_trace_depth, 5.0);
}

#[test]
fn test_leaderboard_creation() {
    let leaderboard = Leaderboard::new();
    assert_eq!(leaderboard.total_contributors(), 0);
    assert_eq!(leaderboard.total_submissions(), 0);
}

#[test]
fn test_leaderboard_add_entry() {
    let mut leaderboard = Leaderboard::new();
    let mut meta = MetaAgent::new("user1", "backend1");
    
    for i in 0..10 {
        meta.log_event(AgentType::Reasoning, "input", "output", "en", 0.9);
    }
    
    let provenance = meta.emit_provenance();
    leaderboard.add_entry(provenance, vec!["en".to_string()]);
    
    assert_eq!(leaderboard.total_contributors(), 1);
    assert_eq!(leaderboard.total_submissions(), 1);
}

#[test]
fn test_leaderboard_multiple_submissions() {
    let mut leaderboard = Leaderboard::new();
    
    // Same user, multiple submissions
    for _ in 0..3 {
        let mut meta = MetaAgent::new("user1", "backend1");
        for i in 0..5 {
            meta.log_event(AgentType::Reasoning, "input", "output", "en", 0.9);
        }
        let provenance = meta.emit_provenance();
        leaderboard.add_entry(provenance, vec!["en".to_string()]);
    }
    
    assert_eq!(leaderboard.total_contributors(), 1);
    assert_eq!(leaderboard.total_submissions(), 3);
}

#[test]
fn test_leaderboard_ranking_by_depth() {
    let mut leaderboard = Leaderboard::new();
    
    // User 1: 5 steps
    let mut meta1 = MetaAgent::new("user1", "backend1");
    for _ in 0..5 {
        meta1.log_event(AgentType::Reasoning, "input", "output", "en", 0.9);
    }
    leaderboard.add_entry(meta1.emit_provenance(), vec!["en".to_string()]);
    
    // User 2: 10 steps
    let mut meta2 = MetaAgent::new("user2", "backend2");
    for _ in 0..10 {
        meta2.log_event(AgentType::Reasoning, "input", "output", "en", 0.9);
    }
    leaderboard.add_entry(meta2.emit_provenance(), vec!["en".to_string()]);
    
    let ranked = leaderboard.rank_by_depth();
    assert_eq!(ranked[0].contributor_id, "user2"); // Higher depth first
    assert_eq!(ranked[1].contributor_id, "user1");
}

#[test]
fn test_leaderboard_ranking_criteria() {
    let mut leaderboard = Leaderboard::new();
    
    let mut meta = MetaAgent::new("user1", "backend1");
    for _ in 0..10 {
        meta.log_event(AgentType::Reasoning, "input", "output", "en", 0.9);
    }
    leaderboard.add_entry(meta.emit_provenance(), vec!["en".to_string()]);
    
    // Test different ranking criteria
    let by_depth = leaderboard.rank_by_depth();
    let by_uniqueness = leaderboard.rank_by_uniqueness();
    let by_submissions = leaderboard.rank_by_submissions();
    let combined = leaderboard.rank_combined();
    
    assert_eq!(by_depth.len(), 1);
    assert_eq!(by_uniqueness.len(), 1);
    assert_eq!(by_submissions.len(), 1);
    assert_eq!(combined.len(), 1);
}

#[test]
fn test_leaderboard_top_n() {
    let mut leaderboard = Leaderboard::new();
    
    for i in 0..5 {
        let mut meta = MetaAgent::new(&format!("user{}", i), "backend");
        for _ in 0..(i + 1) * 5 {
            meta.log_event(AgentType::Reasoning, "input", "output", "en", 0.9);
        }
        leaderboard.add_entry(meta.emit_provenance(), vec!["en".to_string()]);
    }
    
    let top_3 = leaderboard.get_top_n(3, RankingCriteria::TraceDepth);
    assert_eq!(top_3.len(), 3);
}

#[test]
fn test_leaderboard_export_json() {
    let mut leaderboard = Leaderboard::new();
    let mut meta = MetaAgent::new("user1", "backend1");
    
    meta.log_event(AgentType::Reasoning, "input", "output", "en", 0.9);
    leaderboard.add_entry(meta.emit_provenance(), vec!["en".to_string()]);
    
    let json = leaderboard.export_json(RankingCriteria::Combined);
    assert!(json.is_ok());
    assert!(!json.unwrap().is_empty());
}

#[test]
fn test_contributor_history() {
    let mut leaderboard = Leaderboard::new();
    
    for _ in 0..3 {
        let mut meta = MetaAgent::new("user1", "backend1");
        meta.log_event(AgentType::Reasoning, "input", "output", "en", 0.9);
        leaderboard.add_entry(meta.emit_provenance(), vec!["en".to_string()]);
    }
    
    let history = leaderboard.get_contributor_history("user1");
    assert!(history.is_some());
    assert_eq!(history.unwrap().len(), 3);
}

#[test]
fn test_json_export() {
    let mut meta = MetaAgent::new("test_user", "test_backend");
    meta.log_event(AgentType::Reasoning, "input", "output", "en", 0.9);
    
    let trace_json = meta.export_trace_json();
    assert!(trace_json.is_ok());
    
    let prov_json = meta.export_provenance_json();
    assert!(prov_json.is_ok());
}

#[test]
fn test_complex_multilingual_scenario() {
    let mut meta = MetaAgent::new("multilingual_user", "quantum_backend");
    
    // Simulate complex multilingual reasoning
    let languages = vec!["en", "id", "zh", "es", "fr"];
    let agents = vec![
        AgentType::Classification,
        AgentType::Translation,
        AgentType::Reasoning,
        AgentType::Retrieval,
        AgentType::Validation,
        AgentType::Synthesis,
    ];
    
    for i in 0..20 {
        let agent = agents[i % agents.len()].clone();
        let lang = languages[i % languages.len()];
        meta.log_event(agent, "input", "output", lang, 0.85 + (i as f64 * 0.005));
    }
    
    let folded = meta.fold_memory();
    assert_eq!(folded.language_distribution.len(), 5);
    assert!(folded.key_insights.len() > 0);
    
    let provenance = meta.emit_provenance();
    assert_eq!(provenance.trace_depth, 20);
    assert!(provenance.transitions.len() > 0);
}
