//! Deliberation Protocol — Consider/Resolve/Forfeit
//! Multi-agent consensus engine with confidence propagation.

use std::collections::HashMap;

/// A deliberation proposal from an agent
#[derive(Debug, Clone)]
pub struct Proposal {
    pub id: u64,
    pub agent: String,
    pub approach: String,
    pub code: String,
    pub confidence: f64,
    pub constraints_met: Vec<String>,
    pub constraints_violated: Vec<String>,
    pub forfeited: bool,
}

/// Deliberation state for a single intent
pub struct DeliberationState {
    proposals: Vec<Proposal>,
    round: usize,
    max_rounds: usize,
    convergence_threshold: f64,
    history: Vec<RoundRecord>,
}

#[derive(Debug, Clone)]
pub struct RoundRecord {
    pub round: usize,
    pub proposals_count: usize,
    pub best_confidence: f64,
    pub forfeits: usize,
    pub converged: bool,
}

impl DeliberationState {
    pub fn new(max_rounds: usize, threshold: f64) -> Self {
        Self {
            proposals: vec![], round: 0, max_rounds, convergence_threshold: threshold,
            history: vec![],
        }
    }

    /// Submit a proposal for consideration
    pub fn consider(&mut self, agent: &str, approach: &str, code: &str, confidence: f64) -> u64 {
        let id = self.proposals.len() as u64;
        self.proposals.push(Proposal {
            id, agent: agent.to_string(), approach: approach.to_string(),
            code: code.to_string(), confidence,
            constraints_met: vec![], constraints_violated: vec![], forfeited: false,
        });
        id
    }

    /// Resolve: mark constraints as met or violated for a proposal
    pub fn resolve(&mut self, proposal_id: u64, met: Vec<String>, violated: Vec<String>) {
        if let Some(p) = self.proposals.get_mut(proposal_id as usize) {
            p.constraints_met = met;
            p.constraints_violated = violated;
            // Adjust confidence based on violations
            let penalty = p.constraints_violated.len() as f64 * 0.15;
            p.confidence = (p.confidence - penalty).max(0.1);
        }
    }

    /// Forfeit: agent concedes, transfers confidence to winner
    pub fn forfeit(&mut self, proposal_id: u64) {
        if let Some(p) = self.proposals.get_mut(proposal_id as usize) {
            if p.forfeited { return; }
            p.forfeited = true;
            // Find best non-forfeited proposal
            if let Some(best) = self.proposals.iter_mut()
                .filter(|pp| !pp.forfeited && pp.id != proposal_id)
                .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap())
            {
                let transfer = p.confidence * 0.2;
                best.confidence = (best.confidence + transfer).min(0.99);
            }
        }
    }

    /// Execute one deliberation round
    pub fn deliberate_round(&mut self) -> RoundRecord {
        self.round += 1;
        let active: Vec<&Proposal> = self.proposals.iter().filter(|p| !p.forfeited).collect();
        let best = active.iter().max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap());
        let best_conf = best.map(|b| b.confidence).unwrap_or(0.0);
        let forfeits = self.proposals.iter().filter(|p| p.forfeited).count();

        // Auto-forfeit proposals significantly below best
        if let Some(best_p) = best {
            for p in &mut self.proposals {
                if !p.forfeited && p.confidence < best_p.confidence - 0.3 {
                    self.forfeit(p.id);
                }
            }
        }

        let converged = best_conf >= self.convergence_threshold;
        let record = RoundRecord {
            round: self.round, proposals_count: active.len(),
            best_confidence: best_conf, forfeits,
            converged,
        };
        self.history.push(record.clone());
        record
    }

    /// Check if deliberation has converged
    pub fn is_converged(&self) -> bool {
        self.history.last().map(|r| r.converged).unwrap_or(false)
    }

    /// Get the winning proposal
    pub fn winner(&self) -> Option<&Proposal> {
        self.proposals.iter().filter(|p| !p.forfeited)
            .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap())
    }

    /// Get deliberation summary
    pub fn summary(&self) -> String {
        let mut lines = vec![format!("Deliberation: {} rounds, {} proposals", self.round, self.proposals.len())];
        for r in &self.history {
            let status = if r.converged { "CONVERGED" } else { "continuing" };
            lines.push(format!("  R{}: {} active, best={:.3}, {} forfeits, {}",
                r.round, r.proposals_count, r.best_confidence, r.forfeits, status));
        }
        if let Some(w) = self.winner() {
            lines.push(format!("Winner: {} (conf={:.3})", w.agent, w.confidence));
        }
        lines.join("\n")
    }
}

/// Bayesian confidence combiner
pub fn combine_confidence(c1: f64, c2: f64) -> f64 {
    1.0 / (1.0 / c1.max(0.001) + 1.0 / c2.max(0.001))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consider_and_resolve() {
        let mut state = DeliberationState::new(10, 0.85);
        let id = state.consider("architect", "builtin sorted", "sorted(data)", 0.9);
        state.resolve(id, vec!["descending".to_string()], vec![]);
        assert!(!state.is_converged());
    }

    #[test]
    fn test_forfeit_transfers_confidence() {
        let mut state = DeliberationState::new(10, 0.85);
        let good = state.consider("architect", "good approach", "good()", 0.9);
        let bad = state.consider("novice", "bad approach", "bad()", 0.4);
        state.forfeit(bad);
        assert!(state.proposals[good as usize].confidence > 0.9);
    }

    #[test]
    fn test_convergence() {
        let mut state = DeliberationState::new(10, 0.85);
        let id = state.consider("architect", "perfect", "fn()", 0.95);
        state.resolve(id, vec![], vec![]);
        state.deliberate_round();
        assert!(state.is_converged());
    }

    #[test]
    fn test_bayesian() {
        assert!((combine_confidence(0.5, 0.5) - 0.25).abs() < 0.01);
        assert!((combine_confidence(0.9, 0.9) - 0.45).abs() < 0.01);
    }
}
