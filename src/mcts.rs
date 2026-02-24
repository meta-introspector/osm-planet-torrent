// Rust: Thinker-Prover MCTS
// "The Thinker thinks, the Prover proves" - Robert Anton Wilson

use serde::{Deserialize, Serialize};
use std::f64::consts::SQRT_2;

/// The Thinker: Value model (proposes hypotheses)
#[derive(Debug, Clone)]
pub struct Thinker {
    pub hypothesis: String,
    pub value: f64,        // [0, 1]
    pub confidence: f64,   // [0, 1]
}

impl Thinker {
    pub fn new(hypothesis: String) -> Self {
        Self {
            hypothesis,
            value: 0.5,
            confidence: 0.5,
        }
    }
    
    /// Think: Generate value estimate
    pub fn think(&mut self, observations: &[f64]) {
        self.value = observations.iter().sum::<f64>() / observations.len() as f64;
        self.confidence = 1.0 - observations.iter()
            .map(|&x| (x - self.value).abs())
            .sum::<f64>() / observations.len() as f64;
    }
}

/// The Prover: Policy model (validates hypotheses)
#[derive(Debug, Clone)]
pub struct Prover {
    pub policy: Vec<f64>,  // Action probabilities [0, 1]
}

impl Prover {
    pub fn new(num_actions: usize) -> Self {
        Self {
            policy: vec![1.0 / num_actions as f64; num_actions],
        }
    }
    
    /// Prove: Validate and adjust policy
    pub fn prove(&mut self, thinker_value: f64, action: usize) {
        // Prover follows Thinker
        self.policy[action] = (self.policy[action] + thinker_value) / 2.0;
        
        // Normalize
        let sum: f64 = self.policy.iter().sum();
        for p in &mut self.policy {
            *p /= sum;
        }
    }
}

/// The Dao: Unity of Thinker and Prover
#[derive(Debug, Clone)]
pub struct Dao {
    pub thinker: Thinker,
    pub prover: Prover,
}

impl Dao {
    pub fn new(hypothesis: String, num_actions: usize) -> Self {
        Self {
            thinker: Thinker::new(hypothesis),
            prover: Prover::new(num_actions),
        }
    }
    
    /// Harmony: Thinker value × Prover policy
    pub fn harmony(&self, action: usize) -> f64 {
        self.thinker.value * self.prover.policy[action]
    }
    
    /// Check if in harmony (≥ 0.5)
    pub fn is_harmonious(&self, action: usize) -> bool {
        self.harmony(action) >= 0.5
    }
}

/// MCTS Node with Thinker-Prover
#[derive(Debug, Clone)]
pub struct MCTSNode {
    pub state: usize,
    pub visits: usize,
    pub value_sum: f64,
    pub dao: Dao,
    pub children: Vec<MCTSNode>,
}

impl MCTSNode {
    pub fn new(state: usize, hypothesis: String, num_actions: usize) -> Self {
        Self {
            state,
            visits: 0,
            value_sum: 0.0,
            dao: Dao::new(hypothesis, num_actions),
            children: Vec::new(),
        }
    }
    
    /// UCB1: Upper Confidence Bound
    pub fn ucb1(&self, parent_visits: usize, c: f64) -> f64 {
        if self.visits == 0 {
            return f64::INFINITY;
        }
        
        let exploitation = self.value_sum / self.visits as f64;
        let exploration = c * ((parent_visits as f64).ln() / self.visits as f64).sqrt();
        
        exploitation + exploration
    }
    
    /// Select best child using UCB1
    pub fn select(&self) -> Option<usize> {
        if self.children.is_empty() {
            return None;
        }
        
        let mut best_idx = 0;
        let mut best_ucb = f64::NEG_INFINITY;
        
        for (i, child) in self.children.iter().enumerate() {
            let ucb = child.ucb1(self.visits, SQRT_2);
            if ucb > best_ucb {
                best_ucb = ucb;
                best_idx = i;
            }
        }
        
        Some(best_idx)
    }
    
    /// Expand: Create children
    pub fn expand(&mut self, num_children: usize) {
        for i in 0..num_children {
            let child = MCTSNode::new(
                i,
                format!("State {}", i),
                num_children,
            );
            self.children.push(child);
        }
    }
    
    /// Simulate: Thinker thinks, Prover proves
    pub fn simulate(&mut self) -> f64 {
        // Thinker generates value
        let observations = vec![0.7, 0.8, 0.9];
        self.dao.thinker.think(&observations);
        
        // Prover validates
        let action = self.state % self.dao.prover.policy.len();
        self.dao.prover.prove(self.dao.thinker.value, action);
        
        // Return harmony
        self.dao.harmony(action)
    }
    
    /// Backpropagate: Update values
    pub fn backprop(&mut self, value: f64) {
        self.visits += 1;
        self.value_sum += value;
    }
}

/// MCTS with Thinker-Prover Dao
pub struct MCTS {
    pub root: MCTSNode,
    pub num_simulations: usize,
}

impl MCTS {
    pub fn new(num_states: usize, num_simulations: usize) -> Self {
        let mut root = MCTSNode::new(0, "Root".to_string(), num_states);
        root.expand(num_states);
        
        Self {
            root,
            num_simulations,
        }
    }
    
    /// Run MCTS
    pub fn run(&mut self) {
        for _ in 0..self.num_simulations {
            // Selection
            let child_idx = self.root.select().unwrap_or(0);
            
            // Simulation
            let value = self.root.children[child_idx].simulate();
            
            // Backpropagation
            self.root.children[child_idx].backprop(value);
            self.root.backprop(value);
        }
    }
    
    /// Get best action
    pub fn best_action(&self) -> usize {
        self.root.children.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                let a_val = a.value_sum / a.visits.max(1) as f64;
                let b_val = b.value_sum / b.visits.max(1) as f64;
                a_val.partial_cmp(&b_val).unwrap()
            })
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
    
    /// Print tree
    pub fn print_tree(&self) {
        println!("🌳 MCTS Tree (Thinker-Prover Dao)");
        println!("Root visits: {}", self.root.visits);
        println!("Root value: {:.4}", self.root.value_sum / self.root.visits as f64);
        println!();
        
        for (i, child) in self.root.children.iter().enumerate() {
            if child.visits > 0 {
                let value = child.value_sum / child.visits as f64;
                let harmony = child.dao.harmony(i % child.dao.prover.policy.len());
                println!("State {}: visits={}, value={:.4}, harmony={:.4}",
                    i, child.visits, value, harmony);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thinker() {
        let mut thinker = Thinker::new("Test".to_string());
        thinker.think(&[0.7, 0.8, 0.9]);
        assert!(thinker.value > 0.7 && thinker.value < 0.9);
    }

    #[test]
    fn test_prover() {
        let mut prover = Prover::new(3);
        prover.prove(0.8, 0);
        assert!(prover.policy[0] > 0.3);
    }

    #[test]
    fn test_dao_harmony() {
        let dao = Dao::new("Test".to_string(), 3);
        let harmony = dao.harmony(0);
        assert!(harmony >= 0.0 && harmony <= 1.0);
    }

    #[test]
    fn test_mcts() {
        let mut mcts = MCTS::new(71, 100);
        mcts.run();
        let best = mcts.best_action();
        assert!(best < 71);
    }
}
