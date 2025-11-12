// leaderboard.rs - Level 5 Contributor Leaderboard System
// Ranks contributors by trace depth and provenance uniqueness

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use super::meta_agent::ProvenanceLog;

/// Contributor statistics for leaderboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributorStats {
    pub contributor_id: String,
    pub trace_depth: usize,
    pub provenance_hash: String,
    pub backend_used: String,
    pub last_updated: DateTime<Utc>,
    pub uniqueness_score: f64,
    pub total_submissions: usize,
    pub avg_trace_depth: f64,
    pub languages_used: Vec<String>,
    pub rank: usize,
}

/// Leaderboard ranking criteria
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RankingCriteria {
    TraceDepth,
    UniquenessScore,
    TotalSubmissions,
    AvgTraceDepth,
    Combined,
}

/// Leaderboard system
pub struct Leaderboard {
    pub entries: Vec<ContributorStats>,
    contributor_history: HashMap<String, Vec<ProvenanceLog>>,
}

impl Leaderboard {
    /// Create new leaderboard
    pub fn new() -> Self {
        Leaderboard {
            entries: Vec::new(),
            contributor_history: HashMap::new(),
        }
    }

    /// Add entry from provenance log
    pub fn add_entry(&mut self, provenance: ProvenanceLog, languages: Vec<String>) {
        let contributor_id = provenance.contributor_id.clone();
        
        // Store in history
        self.contributor_history
            .entry(contributor_id.clone())
            .or_insert_with(Vec::new)
            .push(provenance.clone());

        // Check if contributor exists
        if let Some(existing) = self.entries.iter_mut().find(|e| e.contributor_id == contributor_id) {
            // Update existing entry
            existing.total_submissions += 1;
            existing.trace_depth = existing.trace_depth.max(provenance.trace_depth);
            existing.provenance_hash = provenance.trace_hash.clone();
            existing.backend_used = provenance.backend_used.clone();
            existing.last_updated = provenance.timestamp;
            existing.uniqueness_score = existing.uniqueness_score.max(provenance.uniqueness_score);
            
            // Update average trace depth
            let history = &self.contributor_history[&contributor_id];
            existing.avg_trace_depth = history.iter()
                .map(|p| p.trace_depth as f64)
                .sum::<f64>() / history.len() as f64;
            
            // Update languages
            for lang in languages {
                if !existing.languages_used.contains(&lang) {
                    existing.languages_used.push(lang);
                }
            }
        } else {
            // Create new entry
            let stats = ContributorStats {
                contributor_id: contributor_id.clone(),
                trace_depth: provenance.trace_depth,
                provenance_hash: provenance.trace_hash,
                backend_used: provenance.backend_used,
                last_updated: provenance.timestamp,
                uniqueness_score: provenance.uniqueness_score,
                total_submissions: 1,
                avg_trace_depth: provenance.trace_depth as f64,
                languages_used: languages,
                rank: 0,
            };
            self.entries.push(stats);
        }

        // Recompute ranks
        self.update_ranks(RankingCriteria::Combined);
    }

    /// Rank by trace depth (number of reasoning steps)
    pub fn rank_by_depth(&self) -> Vec<&ContributorStats> {
        let mut ranked = self.entries.iter().collect::<Vec<_>>();
        ranked.sort_by(|a, b| {
            b.trace_depth.cmp(&a.trace_depth)
                .then_with(|| b.uniqueness_score.partial_cmp(&a.uniqueness_score).unwrap())
        });
        ranked
    }

    /// Rank by uniqueness score (SHA-256 hash originality)
    pub fn rank_by_uniqueness(&self) -> Vec<&ContributorStats> {
        let mut ranked = self.entries.iter().collect::<Vec<_>>();
        ranked.sort_by(|a, b| {
            b.uniqueness_score.partial_cmp(&a.uniqueness_score).unwrap()
                .then_with(|| b.trace_depth.cmp(&a.trace_depth))
        });
        ranked
    }

    /// Rank by total submissions
    pub fn rank_by_submissions(&self) -> Vec<&ContributorStats> {
        let mut ranked = self.entries.iter().collect::<Vec<_>>();
        ranked.sort_by(|a, b| b.total_submissions.cmp(&a.total_submissions));
        ranked
    }

    /// Rank by average trace depth
    pub fn rank_by_avg_depth(&self) -> Vec<&ContributorStats> {
        let mut ranked = self.entries.iter().collect::<Vec<_>>();
        ranked.sort_by(|a, b| {
            b.avg_trace_depth.partial_cmp(&a.avg_trace_depth).unwrap()
        });
        ranked
    }

    /// Combined ranking (weighted score)
    pub fn rank_combined(&self) -> Vec<&ContributorStats> {
        let mut ranked = self.entries.iter().collect::<Vec<_>>();
        ranked.sort_by(|a, b| {
            let score_a = self.compute_combined_score(a);
            let score_b = self.compute_combined_score(b);
            score_b.partial_cmp(&score_a).unwrap()
        });
        ranked
    }

    /// Compute combined score
    fn compute_combined_score(&self, stats: &ContributorStats) -> f64 {
        // Weighted combination of metrics
        let depth_score = stats.trace_depth as f64 / 100.0; // normalize
        let uniqueness_score = stats.uniqueness_score;
        let submission_score = (stats.total_submissions as f64).ln() / 5.0; // log scale
        let avg_depth_score = stats.avg_trace_depth / 50.0; // normalize
        
        // Weights: depth=0.3, uniqueness=0.4, submissions=0.15, avg_depth=0.15
        0.3 * depth_score + 0.4 * uniqueness_score + 0.15 * submission_score + 0.15 * avg_depth_score
    }

    /// Update ranks based on criteria
    pub fn update_ranks(&mut self, criteria: RankingCriteria) {
        let ranked = match criteria {
            RankingCriteria::TraceDepth => self.rank_by_depth(),
            RankingCriteria::UniquenessScore => self.rank_by_uniqueness(),
            RankingCriteria::TotalSubmissions => self.rank_by_submissions(),
            RankingCriteria::AvgTraceDepth => self.rank_by_avg_depth(),
            RankingCriteria::Combined => self.rank_combined(),
        };

        // Update rank field
        for (i, entry) in ranked.iter().enumerate() {
            if let Some(stats) = self.entries.iter_mut().find(|e| e.contributor_id == entry.contributor_id) {
                stats.rank = i + 1;
            }
        }
    }

    /// Display leaderboard
    pub fn display(&self, criteria: RankingCriteria) {
        println!("\n╔════════════════════════════════════════════════════════════════════════════╗");
        println!("║          🏆 Quantum LIMIT-GRAPH Level 5 Leaderboard 🏆                    ║");
        println!("╠════════════════════════════════════════════════════════════════════════════╣");
        
        let criteria_name = match criteria {
            RankingCriteria::TraceDepth => "Trace Depth",
            RankingCriteria::UniquenessScore => "Uniqueness Score",
            RankingCriteria::TotalSubmissions => "Total Submissions",
            RankingCriteria::AvgTraceDepth => "Average Trace Depth",
            RankingCriteria::Combined => "Combined Score",
        };
        println!("║ Ranking Criteria: {:<58} ║", criteria_name);
        println!("╠════════════════════════════════════════════════════════════════════════════╣");

        let ranked = match criteria {
            RankingCriteria::TraceDepth => self.rank_by_depth(),
            RankingCriteria::UniquenessScore => self.rank_by_uniqueness(),
            RankingCriteria::TotalSubmissions => self.rank_by_submissions(),
            RankingCriteria::AvgTraceDepth => self.rank_by_avg_depth(),
            RankingCriteria::Combined => self.rank_combined(),
        };

        for (i, entry) in ranked.iter().enumerate().take(10) {
            let medal = match i {
                0 => "🥇",
                1 => "🥈",
                2 => "🥉",
                _ => "  ",
            };

            println!("║ {} {:2}. {:<20} │ Depth: {:4} │ Unique: {:.3} │ Subs: {:3} ║",
                medal,
                i + 1,
                truncate(&entry.contributor_id, 20),
                entry.trace_depth,
                entry.uniqueness_score,
                entry.total_submissions
            );
        }

        println!("╚════════════════════════════════════════════════════════════════════════════╝\n");
    }

    /// Display detailed stats for a contributor
    pub fn display_contributor(&self, contributor_id: &str) {
        if let Some(stats) = self.entries.iter().find(|e| e.contributor_id == contributor_id) {
            println!("\n╔════════════════════════════════════════════════════════════════════════════╗");
            println!("║                    Contributor Profile: {:<35} ║", truncate(contributor_id, 35));
            println!("╠════════════════════════════════════════════════════════════════════════════╣");
            println!("║ Rank:                  #{:<54} ║", stats.rank);
            println!("║ Max Trace Depth:       {:<55} ║", stats.trace_depth);
            println!("║ Avg Trace Depth:       {:<55.2} ║", stats.avg_trace_depth);
            println!("║ Uniqueness Score:      {:<55.3} ║", stats.uniqueness_score);
            println!("║ Total Submissions:     {:<55} ║", stats.total_submissions);
            println!("║ Backend Used:          {:<55} ║", truncate(&stats.backend_used, 55));
            println!("║ Languages:             {:<55} ║", stats.languages_used.join(", "));
            println!("║ Provenance Hash:       {:<55} ║", truncate(&stats.provenance_hash, 55));
            println!("║ Last Updated:          {:<55} ║", stats.last_updated.format("%Y-%m-%d %H:%M:%S UTC"));
            println!("╚════════════════════════════════════════════════════════════════════════════╝\n");
        } else {
            println!("Contributor '{}' not found in leaderboard.", contributor_id);
        }
    }

    /// Export leaderboard to JSON
    pub fn export_json(&self, criteria: RankingCriteria) -> Result<String, serde_json::Error> {
        let ranked = match criteria {
            RankingCriteria::TraceDepth => self.rank_by_depth(),
            RankingCriteria::UniquenessScore => self.rank_by_uniqueness(),
            RankingCriteria::TotalSubmissions => self.rank_by_submissions(),
            RankingCriteria::AvgTraceDepth => self.rank_by_avg_depth(),
            RankingCriteria::Combined => self.rank_combined(),
        };

        serde_json::to_string_pretty(&ranked)
    }

    /// Get top N contributors
    pub fn get_top_n(&self, n: usize, criteria: RankingCriteria) -> Vec<&ContributorStats> {
        let ranked = match criteria {
            RankingCriteria::TraceDepth => self.rank_by_depth(),
            RankingCriteria::UniquenessScore => self.rank_by_uniqueness(),
            RankingCriteria::TotalSubmissions => self.rank_by_submissions(),
            RankingCriteria::AvgTraceDepth => self.rank_by_avg_depth(),
            RankingCriteria::Combined => self.rank_combined(),
        };

        ranked.into_iter().take(n).collect()
    }

    /// Get contributor history
    pub fn get_contributor_history(&self, contributor_id: &str) -> Option<&Vec<ProvenanceLog>> {
        self.contributor_history.get(contributor_id)
    }

    /// Get total contributors
    pub fn total_contributors(&self) -> usize {
        self.entries.len()
    }

    /// Get total submissions
    pub fn total_submissions(&self) -> usize {
        self.entries.iter().map(|e| e.total_submissions).sum()
    }
}

impl Default for Leaderboard {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to truncate strings
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
