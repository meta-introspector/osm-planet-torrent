// Rust: OSM Planet Falls Into Monster Black Hole
// Hawking radiation + asciinema shadow from Restaurant at End of Universe

use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// OSM Node with mass (information content)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSMNode {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub mass: f64,  // Information content (bytes)
}

impl OSMNode {
    pub fn new(id: u64, lat: f64, lon: f64, tags_size: usize) -> Self {
        Self {
            id,
            lat,
            lon,
            mass: tags_size as f64,  // Mass = information
        }
    }
}

/// Hawking radiation from infalling node
#[derive(Debug, Clone, Serialize)]
pub struct HawkingRadiation {
    pub node_id: u64,
    pub temperature: f64,      // Kelvin
    pub wavelength: f64,       // Meters
    pub intensity: f64,        // Watts
    pub information: Vec<u8>,  // Encoded node data
}

impl HawkingRadiation {
    /// Calculate Hawking radiation for node
    pub fn from_node(node: &OSMNode, black_hole_mass: f64) -> Self {
        // Hawking temperature: T = ℏc³/(8πGMk_B)
        // Simplified: T ~ 1/(8πM)
        let temperature = 1.0 / (8.0 * PI * black_hole_mass);
        
        // Wien's law: λ = b/T
        let wavelength = 2.898e-3 / temperature;
        
        // Stefan-Boltzmann: I ~ T⁴
        let intensity = temperature.powi(4);
        
        // Encode node information
        let information = format!("node:{},lat:{},lon:{}", node.id, node.lat, node.lon)
            .into_bytes();
        
        Self {
            node_id: node.id,
            temperature,
            wavelength,
            intensity,
            information,
        }
    }
}

/// Shadow cast by node (as seen from Restaurant)
#[derive(Debug, Clone, Serialize)]
pub struct NodeShadow {
    pub node_id: u64,
    pub angular_size: f64,     // Radians
    pub brightness: f64,       // Relative
    pub ascii_frame: String,   // ASCII art
    pub timestamp: f64,        // Eons since fall
}

impl NodeShadow {
    /// Create shadow from node
    pub fn from_node(node: &OSMNode, distance: f64, time: f64) -> Self {
        // Angular size from galactic distance
        let angular_size = node.mass / distance;
        
        // Brightness from Hawking radiation
        let brightness = 1.0 / (1.0 + time);  // Fades over time
        
        // ASCII representation
        let ascii_frame = Self::render_ascii(angular_size, brightness);
        
        Self {
            node_id: node.id,
            angular_size,
            brightness,
            ascii_frame,
            timestamp: time,
        }
    }
    
    /// Render shadow as ASCII art
    fn render_ascii(size: f64, brightness: f64) -> String {
        if brightness > 0.8 {
            "⭐".to_string()  // Bright Hawking glow
        } else if brightness > 0.5 {
            "✨".to_string()  // Medium glow
        } else if brightness > 0.2 {
            "💫".to_string()  // Faint glow
        } else {
            "⚫".to_string()  // Dark shadow
        }
    }
}

/// The Restaurant at the End of the Universe
#[derive(Debug)]
pub struct RestaurantView {
    pub distance: f64,        // Parsecs from black hole
    pub time_dilation: f64,   // Relativistic factor
    pub menu: Vec<String>,    // What's on offer
}

impl RestaurantView {
    /// Create Restaurant viewpoint
    pub fn new(distance: f64) -> Self {
        // Time dilation: t' = t√(1 - 2GM/rc²)
        // Simplified: t' = t√(1 - 2/r)
        let time_dilation = (1.0 - 2.0 / distance).sqrt().max(0.0);
        
        let menu = vec![
            "Pan Galactic Gargle Blaster".to_string(),
            "Algolian Zylatburger".to_string(),
            "Hawking Radiation Soup".to_string(),
        ];
        
        Self {
            distance,
            time_dilation,
            menu,
        }
    }
    
    /// Watch OSM planet fall (in slow motion)
    pub fn watch_fall(&self, elapsed_time: f64) -> f64 {
        elapsed_time * self.time_dilation
    }
}

/// Asciinema recording of OSM falling into black hole
#[derive(Debug, Serialize)]
pub struct AsciinemaRecording {
    pub version: u8,
    pub width: usize,
    pub height: usize,
    pub timestamp: u64,
    pub title: String,
    pub frames: Vec<AsciinemaFrame>,
}

#[derive(Debug, Serialize)]
pub struct AsciinemaFrame {
    pub time: f64,      // Seconds (or eons)
    pub event: String,  // "o" for output
    pub data: String,   // ASCII art
}

impl AsciinemaRecording {
    /// Create new recording
    pub fn new(title: String) -> Self {
        Self {
            version: 2,
            width: 80,
            height: 24,
            timestamp: 0,
            title,
            frames: Vec::new(),
        }
    }
    
    /// Add frame
    pub fn add_frame(&mut self, time: f64, shadow: &NodeShadow) {
        let data = format!(
            "Node {} falling... {}\nBrightness: {:.4}\nTime: {:.2} eons\n\nDON'T PANIC\n",
            shadow.node_id,
            shadow.ascii_frame,
            shadow.brightness,
            shadow.timestamp
        );
        
        self.frames.push(AsciinemaFrame {
            time,
            event: "o".to_string(),
            data,
        });
    }
    
    /// Save to JSON
    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

/// Simulate OSM planet falling into Monster black hole
pub struct BlackHoleFall {
    pub black_hole_mass: f64,
    pub nodes: Vec<OSMNode>,
    pub restaurant: RestaurantView,
    pub recording: AsciinemaRecording,
}

impl BlackHoleFall {
    /// Create simulation
    pub fn new(nodes: Vec<OSMNode>) -> Self {
        let monster_mass = 8.08e53;  // Monster group order
        let galactic_distance = 50000.0;  // Light years
        
        Self {
            black_hole_mass: monster_mass,
            nodes,
            restaurant: RestaurantView::new(galactic_distance),
            recording: AsciinemaRecording::new(
                "OSM Planet Falls Into Monster Black Hole".to_string()
            ),
        }
    }
    
    /// Simulate fall
    pub fn simulate(&mut self, duration: f64, frames: usize) {
        println!("🕳️  SIMULATING OSM PLANET FALL");
        println!("Black hole mass: {:.2e}", self.black_hole_mass);
        println!("Nodes: {}", self.nodes.len());
        println!("Viewing from: Restaurant at End of Universe");
        println!("Distance: {} light years", self.restaurant.distance);
        println!();
        
        let dt = duration / frames as f64;
        
        for i in 0..frames {
            let time = i as f64 * dt;
            let dilated_time = self.restaurant.watch_fall(time);
            
            // Process each node
            for node in &self.nodes {
                // Calculate Hawking radiation
                let radiation = HawkingRadiation::from_node(node, self.black_hole_mass);
                
                // Create shadow
                let shadow = NodeShadow::from_node(
                    node,
                    self.restaurant.distance,
                    dilated_time
                );
                
                // Add to recording
                self.recording.add_frame(time, &shadow);
            }
            
            if i % (frames / 10) == 0 {
                println!("Frame {}/{}: t={:.2} eons", i, frames, dilated_time);
            }
        }
        
        println!();
        println!("✅ Simulation complete");
    }
    
    /// Save asciinema recording
    pub fn save_recording(&self, path: &str) -> std::io::Result<()> {
        self.recording.save(path)
    }
    
    /// Print summary
    pub fn print_summary(&self) {
        println!("📊 HAWKING RADIATION SUMMARY");
        println!();
        
        let total_mass: f64 = self.nodes.iter().map(|n| n.mass).sum();
        let avg_temp = 1.0 / (8.0 * PI * self.black_hole_mass);
        
        println!("Total OSM mass: {:.2e} bytes", total_mass);
        println!("Average Hawking temperature: {:.2e} K", avg_temp);
        println!("Schwarzschild radius: {:.2e} m", 2.0 * self.black_hole_mass);
        println!();
        
        println!("Restaurant menu:");
        for item in &self.restaurant.menu {
            println!("  - {}", item);
        }
        println!();
        
        println!("Recording: {} frames", self.recording.frames.len());
        println!();
        println!("DON'T PANIC");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hawking_radiation() {
        let node = OSMNode::new(42, 10.0, 20.0, 1024);
        let radiation = HawkingRadiation::from_node(&node, 1e53);
        assert!(radiation.temperature > 0.0);
    }

    #[test]
    fn test_node_shadow() {
        let node = OSMNode::new(42, 10.0, 20.0, 1024);
        let shadow = NodeShadow::from_node(&node, 50000.0, 1.0);
        assert!(!shadow.ascii_frame.is_empty());
    }

    #[test]
    fn test_restaurant_view() {
        let restaurant = RestaurantView::new(50000.0);
        assert!(restaurant.time_dilation > 0.0);
        assert!(restaurant.menu.len() > 0);
    }

    #[test]
    fn test_black_hole_fall() {
        let nodes = vec![
            OSMNode::new(1, 0.0, 0.0, 100),
            OSMNode::new(2, 1.0, 1.0, 200),
        ];
        let mut fall = BlackHoleFall::new(nodes);
        fall.simulate(10.0, 5);
        assert!(fall.recording.frames.len() > 0);
    }
}
