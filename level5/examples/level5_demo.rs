// examples/level5_demo.rs - Level 5 MetaAgent Demo
// 
// Demonstrates:
// - Memory folding with hierarchical compression
// - Provenance logging with SHA-256 hash
// - Agent transition tracking
// - Multilingual scientific reasoning
// - Contributor leaderboard

use quantum_limit_graph::level5::{
    MetaAgent, AgentType, Leaderboard, RankingCriteria, ContributorProfile,
};

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════════════════════╗");
    println!("║                 Quantum LIMIT-GRAPH v2.4.0 - Level 5                      ║");
    println!("║                        MetaAgent Demonstration                             ║");
    println!("╚════════════════════════════════════════════════════════════════════════════╝\n");

    // Demo 1: Single MetaAgent with multilingual reasoning
    demo_multilingual_reasoning();

    // Demo 2: Multiple contributors and leaderboard
    demo_contributor_leaderboard();

    // Demo 3: Memory folding and provenance
    demo_memory_and_provenance();

    println!("\n╔════════════════════════════════════════════════════════════════════════════╗");
    println!("║                        Demo Complete! ✅                                   ║");
    println!("╚════════════════════════════════════════════════════════════════════════════╝\n");
}

fn demo_multilingual_reasoning() {
    println!("═══════════════════════════════════════════════════════════════════════════");
    println!("Demo 1: Multilingual Scientific Reasoning");
    println!("═══════════════════════════════════════════════════════════════════════════\n");

    let mut meta = MetaAgent::new("nurcholish", "quantum_backend_v3");

    println!("🔬 Research Question: Indonesian Quantum Computing Applications\n");

    // Indonesian query
    meta.log_event(
        AgentType::Classification,
        "Apa aplikasi quantum computing untuk optimasi logistik di Indonesia?",
        "Task: quantum_optimization, Domain: Logistics, Language: Indonesian",
        "id",
        0.94,
    );

    // Translation step
    meta.log_event(
        AgentType::Translation,
        "Translate to English for broader literature search",
        "What are quantum computing applications for logistics optimization in Indonesia?",
        "en",
        0.92,
    );

    // Reasoning in English
    meta.log_event(
        AgentType::Reasoning,
        "Analyze quantum algorithms for logistics",
        "QAOA and VQE can optimize routing, scheduling, and resource allocation",
        "en",
        0.95,
    );

    // Retrieval
    meta.log_event(
        AgentType::Retrieval,
        "Search Indonesian research papers",
        "Found 5 papers from ITB and UI on quantum logistics optimization",
        "en",
        0.89,
    );

    // Validation
    meta.log_event(
        AgentType::Validation,
        "Verify practical implementations",
        "Confirmed pilot projects in Jakarta and Surabaya",
        "en",
        0.91,
    );

    // Synthesis in Indonesian
    meta.log_event(
        AgentType::Synthesis,
        "Compile final answer in Indonesian",
        "Quantum computing digunakan untuk optimasi rute pengiriman dan penjadwalan di Indonesia",
        "id",
        0.96,
    );

    println!("📊 Reasoning Trace:");
    println!("   • Total steps: {}", meta.get_trace_depth());
    println!("   • Transitions: {}", meta.get_transition_count());
    println!("   • Languages: Indonesian, English\n");

    let folded = meta.fold_memory();
    println!("🧠 Memory Fold:");
    println!("   • Compression: {:.2}%", folded.compression_ratio * 100.0);
    println!("   • Insights: {:?}\n", folded.key_insights);

    let provenance = meta.emit_provenance();
    println!("🔐 Provenance:");
    println!("   • Hash: {}...", &provenance.trace_hash[..16]);
    println!("   • Uniqueness: {:.3}\n", provenance.uniqueness_score);
}

fn demo_contributor_leaderboard() {
    println!("═══════════════════════════════════════════════════════════════════════════");
    println!("Demo 2: Contributor Leaderboard");
    println!("═══════════════════════════════════════════════════════════════════════════\n");

    let mut leaderboard = Leaderboard::new();

    // Contributor 1: Deep reasoning trace
    println!("Adding contributor: nurcholish (Indonesian quantum researcher)");
    let mut meta1 = MetaAgent::new("nurcholish", "quantum_backend_v3");
    for i in 0..25 {
        let agent = match i % 6 {
            0 => AgentType::Classification,
            1 => AgentType::Reasoning,
            2 => AgentType::Translation,
            3 => AgentType::Retrieval,
            4 => AgentType::Validation,
            _ => AgentType::Synthesis,
        };
        meta1.log_event(agent, "input", "output", if i % 2 == 0 { "id" } else { "en" }, 0.9);
    }
    let prov1 = meta1.emit_provenance();
    leaderboard.add_entry(prov1, vec!["id".to_string(), "en".to_string()]);

    // Contributor 2: Moderate depth
    println!("Adding contributor: alice_researcher (Multilingual NLP expert)");
    let mut meta2 = MetaAgent::new("alice_researcher", "quantum_backend_v2");
    for i in 0..15 {
        meta2.log_event(AgentType::Reasoning, "input", "output", "en", 0.88);
    }
    let prov2 = meta2.emit_provenance();
    leaderboard.add_entry(prov2, vec!["en".to_string(), "zh".to_string()]);

    // Contributor 3: High uniqueness
    println!("Adding contributor: bob_scientist (Quantum algorithm specialist)");
    let mut meta3 = MetaAgent::new("bob_scientist", "quantum_backend_v3");
    for i in 0..20 {
        let agent = match i % 4 {
            0 => AgentType::Classification,
            1 => AgentType::Reasoning,
            2 => AgentType::Action,
            _ => AgentType::Synthesis,
        };
        meta3.log_event(agent, &format!("unique_{}", i), &format!("result_{}", i), "en", 0.92);
    }
    let prov3 = meta3.emit_provenance();
    leaderboard.add_entry(prov3, vec!["en".to_string(), "es".to_string(), "fr".to_string()]);

    // Contributor 4: Multiple submissions
    println!("Adding contributor: charlie_dev (Active contributor)");
    for submission in 0..3 {
        let mut meta4 = MetaAgent::new("charlie_dev", "quantum_backend_v1");
        for i in 0..10 {
            meta4.log_event(AgentType::Reasoning, "input", "output", "en", 0.85);
        }
        let prov4 = meta4.emit_provenance();
        leaderboard.add_entry(prov4, vec!["en".to_string()]);
    }

    println!("\n");
    leaderboard.display(RankingCriteria::Combined);

    // Show top contributor details
    if let Some(top) = leaderboard.get_top_n(1, RankingCriteria::Combined).first() {
        leaderboard.display_contributor(&top.contributor_id);
    }
}

fn demo_memory_and_provenance() {
    println!("═══════════════════════════════════════════════════════════════════════════");
    println!("Demo 3: Memory Folding & Provenance Logging");
    println!("═══════════════════════════════════════════════════════════════════════════\n");

    let mut meta = MetaAgent::new("demo_user", "quantum_backend_v3");

    println!("🧪 Simulating complex reasoning trace...\n");

    // Create a complex trace
    let languages = vec!["en", "id", "zh", "es", "fr"];
    let agents = vec![
        AgentType::Classification,
        AgentType::Reasoning,
        AgentType::Translation,
        AgentType::Retrieval,
        AgentType::Validation,
        AgentType::Synthesis,
    ];

    for i in 0..30 {
        let agent = agents[i % agents.len()].clone();
        let lang = languages[i % languages.len()];
        let confidence = 0.75 + (i as f64 * 0.005);
        
        meta.log_event(
            agent,
            &format!("Complex input step {}", i),
            &format!("Detailed output for step {}", i),
            lang,
            confidence,
        );
    }

    println!("📊 Trace Statistics:");
    println!("   • Total events: {}", meta.get_trace_depth());
    println!("   • Transitions: {}", meta.get_transition_count());
    println!("   • Session: {}\n", meta.session_id);

    // Memory folding
    let folded = meta.fold_memory();
    println!("🧠 Memory Folding Results:");
    println!("   • Original size: ~{} chars", 
        meta.trace.iter().map(|e| e.input.len() + e.output.len()).sum::<usize>());
    println!("   • Summary size: {} chars", folded.summary.len());
    println!("   • Compression ratio: {:.2}%", folded.compression_ratio * 100.0);
    println!("   • Key insights:");
    for insight in &folded.key_insights {
        println!("     - {}", insight);
    }
    println!("   • Language distribution:");
    for (lang, count) in &folded.language_distribution {
        println!("     - {}: {} events", lang, count);
    }
    println!();

    // Provenance logging
    let provenance = meta.emit_provenance();
    println!("🔐 Provenance Log:");
    println!("   • SHA-256 Hash: {}", provenance.trace_hash);
    println!("   • Trace Depth: {}", provenance.trace_depth);
    println!("   • Uniqueness Score: {:.3}", provenance.uniqueness_score);
    println!("   • Contributor: {}", provenance.contributor_id);
    println!("   • Backend: {}", provenance.backend_used);
    println!("   • Timestamp: {}", provenance.timestamp);
    println!("   • Agent Sequence: {:?}", provenance.agent_sequence);
    println!("   • Transitions: {} tracked\n", provenance.transitions.len());

    // Export to JSON
    if let Ok(json) = meta.export_provenance_json() {
        println!("📄 Provenance JSON (first 300 chars):");
        println!("{}\n", &json[..300.min(json.len())]);
    }

    // Update profile
    meta.update_profile();
    println!("👤 Updated Profile:");
    println!("   • Total traces: {}", meta.profile.total_traces);
    println!("   • Avg trace depth: {:.2}", meta.profile.avg_trace_depth);
    println!("   • Preferred languages: {:?}", meta.profile.preferred_languages);
}
