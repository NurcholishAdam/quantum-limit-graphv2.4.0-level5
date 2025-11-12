// meta_agent.rs - Level 5 MetaAgent with Memory Folding, Provenance Logging, and Agent Transition Tracking
// Designed for multilingual scientific reasoning and contributor personalization

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

/// Agent types in the MetaAgent system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentType {
    Classification,
    Reasoning,
    Action,
    Retrieval,
    Meta,
    Synthesis,
    Validation,
    Translation,
}

impl std::fmt::Display for AgentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentType::Classification => write!(f, "Classification"),
            AgentType::Reasoning => write!(f, "Reasoning"),
            AgentType::Action => write!(f, "Action"),
            AgentType::Retrieval => write!(f, "Retrieval"),
            AgentType::Meta => write!(f, "Meta"),
            AgentType::Synthesis => write!(f, "Synthesis"),
            AgentType::Validation => write!(f, "Validation"),
            AgentType::Translation => write!(f, "Translation"),
        }
    }
}

/// Individual agent event with full context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEvent {
    pub timestamp: DateTime<Utc>,
    pub agent: AgentType,
    pub input: String,
    pub output: String,
    pub language: String,
    pub confidence: f64,
    pub metadata: HashMap<String, String>,
}

/// Agent transition tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTransition {
    pub from_agent: AgentType,
    pub to_agent: AgentType,
    pub timestamp: DateTime<Utc>,
    pub reason: String,
    pub transition_score: f64,
}

/// Memory fold with hierarchical compression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryFold {
    pub session_id: String,
    pub folded_trace: Vec<AgentEvent>,
    pub summary: String,
    pub compression_ratio: f64,
    pub key_insights: Vec<String>,
    pub language_distribution: HashMap<String, usize>,
}

/// Provenance log with SHA-256 hash for originality detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceLog {
    pub trace_hash: String,
    pub agent_sequence: Vec<AgentType>,
    pub contributor_id: String,
    pub backend_used: String,
    pub timestamp: DateTime<Utc>,
    pub trace_depth: usize,
    pub uniqueness_score: f64,
    pub transitions: Vec<AgentTransition>,
}

/// Contributor personalization profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributorProfile {
    pub contributor_id: String,
    pub preferred_languages: Vec<String>,
    pub expertise_domains: Vec<String>,
    pub reasoning_style: String,
    pub total_traces: usize,
    pub avg_trace_depth: f64,
}

/// Level 5 MetaAgent with advanced capabilities
pub struct MetaAgent {
    pub trace: Vec<AgentEvent>,
    pub transitions: Vec<AgentTransition>,
    pub contributor_id: String,
    pub backend_used: String,
    pub profile: ContributorProfile,
    pub session_id: String,
    pub current_agent: Option<AgentType>,
}

impl MetaAgent {
    /// Create new MetaAgent with contributor profile
    pub fn new(contributor_id: &str, backend_used: &str) -> Self {
        let profile = ContributorProfile {
            contributor_id: contributor_id.to_string(),
            preferred_languages: vec!["en".to_string()],
            expertise_domains: vec![],
            reasoning_style: "analytical".to_string(),
            total_traces: 0,
            avg_trace_depth: 0.0,
        };

        MetaAgent {
            trace: Vec::new(),
            transitions: Vec::new(),
            contributor_id: contributor_id.to_string(),
            backend_used: backend_used.to_string(),
            profile,
            session_id: format!("session_{}", Utc::now().timestamp()),
            current_agent: None,
        }
    }

    /// Create MetaAgent with custom profile
    pub fn with_profile(contributor_id: &str, backend_used: &str, profile: ContributorProfile) -> Self {
        MetaAgent {
            trace: Vec::new(),
            transitions: Vec::new(),
            contributor_id: contributor_id.to_string(),
            backend_used: backend_used.to_string(),
            profile,
            session_id: format!("session_{}", Utc::now().timestamp()),
            current_agent: None,
        }
    }

    /// Log agent event with full context
    pub fn log_event(&mut self, agent: AgentType, input: &str, output: &str, language: &str, confidence: f64) {
        // Track agent transition
        if let Some(ref prev_agent) = self.current_agent {
            if prev_agent != &agent {
                self.track_transition(prev_agent.clone(), agent.clone(), "natural_flow");
            }
        }

        let event = AgentEvent {
            timestamp: Utc::now(),
            agent: agent.clone(),
            input: input.to_string(),
            output: output.to_string(),
            language: language.to_string(),
            confidence,
            metadata: HashMap::new(),
        };
        
        self.trace.push(event);
        self.current_agent = Some(agent);
    }

    /// Log event with metadata
    pub fn log_event_with_metadata(
        &mut self,
        agent: AgentType,
        input: &str,
        output: &str,
        language: &str,
        confidence: f64,
        metadata: HashMap<String, String>,
    ) {
        if let Some(ref prev_agent) = self.current_agent {
            if prev_agent != &agent {
                self.track_transition(prev_agent.clone(), agent.clone(), "natural_flow");
            }
        }

        let event = AgentEvent {
            timestamp: Utc::now(),
            agent: agent.clone(),
            input: input.to_string(),
            output: output.to_string(),
            language: language.to_string(),
            confidence,
            metadata,
        };
        
        self.trace.push(event);
        self.current_agent = Some(agent);
    }

    /// Track agent transition
    pub fn track_transition(&mut self, from: AgentType, to: AgentType, reason: &str) {
        let transition = AgentTransition {
            from_agent: from,
            to_agent: to,
            timestamp: Utc::now(),
            reason: reason.to_string(),
            transition_score: self.compute_transition_score(),
        };
        self.transitions.push(transition);
    }

    /// Compute transition quality score
    fn compute_transition_score(&self) -> f64 {
        if self.trace.is_empty() {
            return 1.0;
        }
        
        // Score based on recent event confidence
        let recent_confidence: f64 = self.trace.iter()
            .rev()
            .take(3)
            .map(|e| e.confidence)
            .sum::<f64>() / 3.0.min(self.trace.len() as f64);
        
        recent_confidence
    }

    /// Fold memory with hierarchical compression
    pub fn fold_memory(&self) -> MemoryFold {
        let total_chars: usize = self.trace.iter()
            .map(|e| e.input.len() + e.output.len())
            .sum();
        
        let summary = self.generate_summary();
        let compression_ratio = if total_chars > 0 {
            summary.len() as f64 / total_chars as f64
        } else {
            1.0
        };

        let key_insights = self.extract_key_insights();
        let language_distribution = self.compute_language_distribution();

        MemoryFold {
            session_id: self.session_id.clone(),
            folded_trace: self.trace.clone(),
            summary,
            compression_ratio,
            key_insights,
            language_distribution,
        }
    }

    /// Generate intelligent summary
    fn generate_summary(&self) -> String {
        let agent_counts = self.count_agent_types();
        let languages: Vec<String> = self.trace.iter()
            .map(|e| e.language.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        format!(
            "Session {} with {} reasoning steps across {} languages. Agent distribution: {:?}. Transitions: {}",
            self.session_id,
            self.trace.len(),
            languages.len(),
            agent_counts,
            self.transitions.len()
        )
    }

    /// Extract key insights from trace
    fn extract_key_insights(&self) -> Vec<String> {
        let mut insights = Vec::new();

        // High confidence outputs
        let high_conf_events: Vec<_> = self.trace.iter()
            .filter(|e| e.confidence > 0.8)
            .collect();
        
        if !high_conf_events.is_empty() {
            insights.push(format!("{} high-confidence reasoning steps", high_conf_events.len()));
        }

        // Multilingual reasoning
        let languages: std::collections::HashSet<_> = self.trace.iter()
            .map(|e| e.language.as_str())
            .collect();
        
        if languages.len() > 1 {
            insights.push(format!("Multilingual reasoning across {} languages", languages.len()));
        }

        // Complex agent transitions
        if self.transitions.len() > 5 {
            insights.push(format!("Complex reasoning with {} agent transitions", self.transitions.len()));
        }

        insights
    }

    /// Compute language distribution
    fn compute_language_distribution(&self) -> HashMap<String, usize> {
        let mut dist = HashMap::new();
        for event in &self.trace {
            *dist.entry(event.language.clone()).or_insert(0) += 1;
        }
        dist
    }

    /// Count agent types
    fn count_agent_types(&self) -> HashMap<AgentType, usize> {
        let mut counts = HashMap::new();
        for event in &self.trace {
            *counts.entry(event.agent.clone()).or_insert(0) += 1;
        }
        counts
    }

    /// Emit provenance log with SHA-256 hash
    pub fn emit_provenance(&self) -> ProvenanceLog {
        let mut hasher = Sha256::new();
        
        // Hash the entire reasoning trace for uniqueness
        for event in &self.trace {
            hasher.update(event.input.as_bytes());
            hasher.update(event.output.as_bytes());
            hasher.update(event.language.as_bytes());
            hasher.update(format!("{}", event.agent).as_bytes());
        }
        
        let trace_hash = format!("{:x}", hasher.finalize());
        let uniqueness_score = self.compute_uniqueness_score(&trace_hash);

        ProvenanceLog {
            trace_hash,
            agent_sequence: self.trace.iter().map(|e| e.agent.clone()).collect(),
            contributor_id: self.contributor_id.clone(),
            backend_used: self.backend_used.clone(),
            timestamp: Utc::now(),
            trace_depth: self.trace.len(),
            uniqueness_score,
            transitions: self.transitions.clone(),
        }
    }

    /// Compute uniqueness score (simplified - in production would check against database)
    fn compute_uniqueness_score(&self, hash: &str) -> f64 {
        // Score based on trace complexity and diversity
        let agent_diversity = self.count_agent_types().len() as f64 / 8.0; // 8 agent types
        let language_diversity = self.compute_language_distribution().len() as f64 / 5.0; // normalize
        let transition_complexity = (self.transitions.len() as f64 / self.trace.len() as f64).min(1.0);
        
        (agent_diversity + language_diversity + transition_complexity) / 3.0
    }

    /// Update contributor profile based on current trace
    pub fn update_profile(&mut self) {
        self.profile.total_traces += 1;
        
        // Update average trace depth
        let new_avg = (self.profile.avg_trace_depth * (self.profile.total_traces - 1) as f64 
                      + self.trace.len() as f64) / self.profile.total_traces as f64;
        self.profile.avg_trace_depth = new_avg;

        // Update preferred languages
        let lang_dist = self.compute_language_distribution();
        let mut langs: Vec<_> = lang_dist.into_iter().collect();
        langs.sort_by(|a, b| b.1.cmp(&a.1));
        self.profile.preferred_languages = langs.into_iter()
            .take(3)
            .map(|(lang, _)| lang)
            .collect();
    }

    /// Get trace depth (number of reasoning steps)
    pub fn get_trace_depth(&self) -> usize {
        self.trace.len()
    }

    /// Get transition count
    pub fn get_transition_count(&self) -> usize {
        self.transitions.len()
    }

    /// Export trace for analysis
    pub fn export_trace_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.trace)
    }

    /// Export provenance for verification
    pub fn export_provenance_json(&self) -> Result<String, serde_json::Error> {
        let provenance = self.emit_provenance();
        serde_json::to_string_pretty(&provenance)
    }
}
