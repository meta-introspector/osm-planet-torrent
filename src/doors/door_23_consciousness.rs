//! Door 23 - Consciousness
//! Neural network visualization

use super::Door;

pub struct NeuralNetwork;

impl Door for NeuralNetwork {
    fn prime(&self) -> u32 { 23 }
    fn name(&self) -> &str { "Consciousness" }
    fn description(&self) -> &str { "Silicon Valley - Neural network of connections" }
    
    fn render(&self, _data: &[u8]) -> Vec<u8> {
        b"<html><body><h1>Door 23 - Consciousness</h1></body></html>".to_vec()
    }
}
