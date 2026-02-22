//! 71 Doors - Monster Group OSM Visualizations

pub mod door_17_hawkins;

/// Door trait
pub trait Door {
    fn prime(&self) -> u32;
    fn name(&self) -> &str;
    fn render(&self, data: &[u8]) -> Vec<u8>;
}
