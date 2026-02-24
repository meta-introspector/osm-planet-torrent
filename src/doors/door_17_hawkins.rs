//! Door 17 - Hawkins Radiation Model

use super::Door;

pub struct HawkinsRadiation;

impl Door for HawkinsRadiation {
    fn prime(&self) -> u32 { 17 }
    fn name(&self) -> &str { "Hawkins Radiation" }
    
    fn render(&self, _data: &[u8]) -> Vec<u8> {
        let html = include_str!("../../templates/door17.html");
        html.as_bytes().to_vec()
    }
}
