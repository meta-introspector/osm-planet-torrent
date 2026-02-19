// BBS Door Game: Ramanujan's World Map
// 1980s style ANSI map viewer
use std::io::{self, Write};

const RESET: &str = "\x1b[0m";
const BLUE: &str = "\x1b[34m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const CYAN: &str = "\x1b[36m";
const BOLD: &str = "\x1b[1m";

struct Location {
    name: &'static str,
    lat: f64,
    lon: f64,
    emoji: &'static str,
}

fn main() {
    let locations = vec![
        Location { name: "Kumbakonam", lat: 10.9617, lon: 79.3881, emoji: "🏛️" },
        Location { name: "Chennai", lat: 13.0827, lon: 80.2707, emoji: "🏙️" },
        Location { name: "London", lat: 51.5074, lon: -0.1278, emoji: "🏰" },
        Location { name: "Cambridge", lat: 52.2053, lon: 0.1218, emoji: "🎓" },
        Location { name: "Trinity College", lat: 52.2067, lon: 0.1165, emoji: "📚" },
    ];
    
    clear_screen();
    draw_header();
    draw_map(&locations);
    draw_legend(&locations);
    draw_footer();
}

fn clear_screen() {
    print!("\x1b[2J\x1b[H");
}

fn draw_header() {
    println!("{}{}", BOLD, CYAN);
    println!("╔════════════════════════════════════════════════════════════════════════════╗");
    println!("║                    🗺️  RAMANUJAN'S WORLD MAP 🗺️                           ║");
    println!("║                    BBS Door Game - Est. 1987                               ║");
    println!("╚════════════════════════════════════════════════════════════════════════════╝");
    println!("{}", RESET);
}

fn draw_map(locations: &[Location]) {
    let width = 76;
    let height = 20;
    
    // Create map grid
    let mut grid = vec![vec![' '; width]; height];
    
    // Draw ocean
    for y in 0..height {
        for x in 0..width {
            grid[y][x] = '~';
        }
    }
    
    // Plot locations
    for loc in locations {
        let x = ((loc.lon + 180.0) / 360.0 * width as f64) as usize;
        let y = ((90.0 - loc.lat) / 180.0 * height as f64) as usize;
        
        if x < width && y < height {
            grid[y][x] = '*';
        }
    }
    
    // Draw grid
    println!("{}┌{}┐{}", BLUE, "─".repeat(width), RESET);
    for row in grid {
        print!("{}│{}", BLUE, RESET);
        for cell in row {
            if cell == '*' {
                print!("{}{}{}", RED, cell, RESET);
            } else {
                print!("{}{}{}", CYAN, cell, RESET);
            }
        }
        println!("{}│{}", BLUE, RESET);
    }
    println!("{}└{}┘{}", BLUE, "─".repeat(width), RESET);
}

fn draw_legend(locations: &[Location]) {
    println!("\n{}{}LOCATIONS:{}", BOLD, YELLOW, RESET);
    for (i, loc) in locations.iter().enumerate() {
        let tile_lat = (((loc.lat + 90.0) * 100.0) as i64 % 71) as u8;
        let tile_lon = (((loc.lon + 180.0) * 100.0) as i64 % 59) as u8;
        
        println!("{}{}. {} {} - {:.4}°N, {:.4}°E - Tile({},{},0){}", 
            GREEN, i+1, loc.emoji, loc.name, loc.lat, loc.lon, tile_lat, tile_lon, RESET);
    }
}

fn draw_footer() {
    println!("\n{}{}Press [ENTER] to continue...{}", BOLD, CYAN, RESET);
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
}
