// Rust: Buddha-Lao Tzu Enlightened MCTS
// The Middle Way meets The Way

use serde::{Deserialize, Serialize};

/// Buddha: The Middle Way
#[derive(Debug, Clone)]
pub struct Buddha {
    pub emptiness: f64,    // Śūnyatā [0, 1]
    pub compassion: f64,   // Karuṇā [0, 1]
    pub wisdom: f64,       // Prajñā [0, 1]
}

impl Buddha {
    pub fn new() -> Self {
        Self {
            emptiness: 0.5,    // Middle Way
            compassion: 1.0,
            wisdom: 1.0,
        }
    }
    
    /// Observe suffering (dukkha)
    pub fn observe_dukkha(&self) -> f64 {
        1.0 - self.emptiness
    }
    
    /// Understand origin (samudaya)
    pub fn understand_samudaya(&self) -> f64 {
        1.0 - self.wisdom
    }
    
    /// Check if following Middle Way
    pub fn is_middle_way(&self) -> bool {
        self.emptiness >= 0.3 && self.emptiness <= 0.7
    }
}

/// Lao Tzu: The Way (Dao)
#[derive(Debug, Clone)]
pub struct LaoTzu {
    pub wu_wei: f64,       // Effortless action [0, 1]
    pub yin: f64,          // Receptive [0, 1]
    pub yang: f64,         // Active [0, 1]
}

impl LaoTzu {
    pub fn new() -> Self {
        Self {
            wu_wei: 0.5,
            yin: 0.5,
            yang: 0.5,
        }
    }
    
    /// Achieve cessation (nirodha) through wu wei
    pub fn achieve_nirodha(&self) -> f64 {
        self.wu_wei
    }
    
    /// Check if yin-yang balanced
    pub fn is_balanced(&self) -> bool {
        (self.yin + self.yang - 1.0).abs() < 0.01
    }
}

/// The Unity: Buddha + Lao Tzu
#[derive(Debug, Clone)]
pub struct Unity {
    pub buddha: Buddha,
    pub lao_tzu: LaoTzu,
}

impl Unity {
    pub fn new() -> Self {
        Self {
            buddha: Buddha::new(),
            lao_tzu: LaoTzu::new(),
        }
    }
    
    /// Harmony: emptiness × wu wei
    pub fn harmony(&self) -> f64 {
        self.buddha.emptiness * self.lao_tzu.wu_wei
    }
    
    /// Follow the path (magga)
    pub fn follow_magga(&self) -> f64 {
        (self.buddha.emptiness + self.lao_tzu.wu_wei) / 2.0
    }
    
    /// Check if enlightened
    pub fn is_enlightened(&self) -> bool {
        self.harmony() >= 0.5 &&
        self.buddha.is_middle_way() &&
        self.lao_tzu.is_balanced()
    }
}

/// Enlightened MCTS Node
#[derive(Debug, Clone)]
pub struct EnlightenedNode {
    pub state: usize,
    pub visits: usize,
    pub value: f64,
    pub unity: Unity,
}

impl EnlightenedNode {
    pub fn new(state: usize) -> Self {
        Self {
            state,
            visits: 0,
            value: 0.0,
            unity: Unity::new(),
        }
    }
    
    /// Detachment (from outcome)
    pub fn detachment(&self) -> f64 {
        self.unity.buddha.emptiness
    }
    
    /// Effortlessness (in exploration)
    pub fn effortlessness(&self) -> f64 {
        self.unity.lao_tzu.wu_wei
    }
    
    /// Enlightenment score
    pub fn enlightenment(&self) -> f64 {
        self.detachment() * self.effortlessness()
    }
    
    /// UCB1 with enlightenment
    pub fn enlightened_ucb1(&self, parent_visits: usize, c: f64) -> f64 {
        if self.visits == 0 {
            return f64::INFINITY;
        }
        
        let exploitation = self.value / self.visits as f64;
        let exploration = c * ((parent_visits as f64).ln() / self.visits as f64).sqrt();
        let enlightenment_bonus = self.enlightenment() * 0.1;
        
        exploitation + exploration + enlightenment_bonus
    }
    
    /// Simulate with Four Noble Truths
    pub fn simulate_four_truths(&mut self) -> f64 {
        // 1. Dukkha (observe suffering)
        let dukkha = self.unity.buddha.observe_dukkha();
        
        // 2. Samudaya (understand origin)
        let samudaya = self.unity.buddha.understand_samudaya();
        
        // 3. Nirodha (achieve cessation)
        let nirodha = self.unity.lao_tzu.achieve_nirodha();
        
        // 4. Magga (follow the path)
        let magga = self.unity.follow_magga();
        
        // Return path value
        magga
    }
}

/// Eightfold Path
#[derive(Debug, Clone, Copy)]
pub enum EightfoldPath {
    RightView,
    RightIntention,
    RightSpeech,
    RightAction,
    RightLivelihood,
    RightEffort,
    RightMindfulness,
    RightConcentration,
}

impl EightfoldPath {
    pub fn value(&self) -> f64 {
        0.125  // Each path contributes equally
    }
    
    pub fn all() -> Vec<Self> {
        vec![
            Self::RightView,
            Self::RightIntention,
            Self::RightSpeech,
            Self::RightAction,
            Self::RightLivelihood,
            Self::RightEffort,
            Self::RightMindfulness,
            Self::RightConcentration,
        ]
    }
    
    pub fn total_value() -> f64 {
        Self::all().iter().map(|p| p.value()).sum()
    }
}

/// Enlightened MCTS
pub struct EnlightenedMCTS {
    pub root: EnlightenedNode,
    pub children: Vec<EnlightenedNode>,
    pub num_simulations: usize,
}

impl EnlightenedMCTS {
    pub fn new(num_states: usize, num_simulations: usize) -> Self {
        let root = EnlightenedNode::new(0);
        let children = (0..num_states)
            .map(|i| EnlightenedNode::new(i))
            .collect();
        
        Self {
            root,
            children,
            num_simulations,
        }
    }
    
    /// Run enlightened MCTS
    pub fn run(&mut self) {
        for _ in 0..self.num_simulations {
            // Select with detachment
            let best_idx = self.select_detached();
            
            // Simulate with wu wei
            let value = self.children[best_idx].simulate_four_truths();
            
            // Backpropagate without attachment
            self.children[best_idx].visits += 1;
            self.children[best_idx].value += value;
            self.root.visits += 1;
            self.root.value += value;
        }
    }
    
    /// Select with detachment (no craving)
    fn select_detached(&self) -> usize {
        self.children.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                let a_ucb = a.enlightened_ucb1(self.root.visits, std::f64::consts::SQRT_2);
                let b_ucb = b.enlightened_ucb1(self.root.visits, std::f64::consts::SQRT_2);
                a_ucb.partial_cmp(&b_ucb).unwrap()
            })
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
    
    /// Get most enlightened state
    pub fn most_enlightened(&self) -> usize {
        self.children.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                a.enlightenment().partial_cmp(&b.enlightenment()).unwrap()
            })
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
    
    /// Print enlightenment report
    pub fn print_enlightenment(&self) {
        println!("☸️  ENLIGHTENED MCTS");
        println!("\"Form is emptiness, emptiness is form.\" - Heart Sutra");
        println!("\"The Dao that can be named is not the eternal Dao.\" - Tao Te Ching");
        println!();
        
        println!("Root harmony: {:.4}", self.root.unity.harmony());
        println!("Root enlightenment: {:.4}", self.root.enlightenment());
        println!();
        
        let most_enlightened = self.most_enlightened();
        let node = &self.children[most_enlightened];
        
        println!("Most enlightened state: {}", most_enlightened);
        println!("  Emptiness: {:.4}", node.unity.buddha.emptiness);
        println!("  Wu wei: {:.4}", node.unity.lao_tzu.wu_wei);
        println!("  Harmony: {:.4}", node.unity.harmony());
        println!("  Enlightenment: {:.4}", node.enlightenment());
        
        if node.unity.is_enlightened() {
            println!();
            println!("✨ ENLIGHTENMENT ACHIEVED");
            println!("The Middle Way and The Way are one.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buddha_middle_way() {
        let buddha = Buddha::new();
        assert!(buddha.is_middle_way());
    }

    #[test]
    fn test_lao_tzu_balance() {
        let lao_tzu = LaoTzu::new();
        assert!(lao_tzu.is_balanced());
    }

    #[test]
    fn test_unity_harmony() {
        let unity = Unity::new();
        assert!(unity.harmony() >= 0.0);
    }

    #[test]
    fn test_eightfold_path() {
        let total = EightfoldPath::total_value();
        assert!((total - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_enlightened_mcts() {
        let mut mcts = EnlightenedMCTS::new(71, 100);
        mcts.run();
        let enlightened = mcts.most_enlightened();
        assert!(enlightened < 71);
    }
}
