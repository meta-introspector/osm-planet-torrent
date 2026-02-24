// BBS Door Game: The Black Hole (1979) - Rust Edition
// "A Journey That Begins Where Everything Ends"

use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use rand::Rng;

const TITLE_ART: &str = r#"
╔═══════════════════════════════════════════════════════════════╗
║                                                               ║
║  ████████╗██╗  ██╗███████╗    ██████╗ ██╗      █████╗  ██████╗██╗  ██╗    ║
║  ╚══██╔══╝██║  ██║██╔════╝    ██╔══██╗██║     ██╔══██╗██╔════╝██║ ██╔╝    ║
║     ██║   ███████║█████╗      ██████╔╝██║     ███████║██║     █████╔╝     ║
║     ██║   ██╔══██║██╔══╝      ██╔══██╗██║     ██╔══██║██║     ██╔═██╗     ║
║     ██║   ██║  ██║███████╗    ██████╔╝███████╗██║  ██║╚██████╗██║  ██╗    ║
║     ╚═╝   ╚═╝  ╚═╝╚══════╝    ╚═════╝ ╚══════╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝    ║
║                                                               ║
║              ██╗  ██╗ ██████╗ ██╗     ███████╗               ║
║              ██║  ██║██╔═══██╗██║     ██╔════╝               ║
║              ███████║██║   ██║██║     █████╗                 ║
║              ██╔══██║██║   ██║██║     ██╔══╝                 ║
║              ██║  ██║╚██████╔╝███████╗███████╗               ║
║              ╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚══════╝               ║
║                                                               ║
║                    A BBS Door Game (1979)                     ║
║              "A Journey That Begins Where Everything Ends"    ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝
"#;

const USS_PALOMINO: &str = r#"
        _____
    ___/     \___
   /   USS      \
  |   PALOMINO   |
   \___       ___/
       \_____/
      /  |  \
     /   |   \
    /____|____\
"#;

const CYGNUS: &str = r#"
           ___
       ___/   \___
   ___/   CYGNUS  \___
  /                   \
 |  ╔═══════════════╗  |
 |  ║   MAXIMILIAN  ║  |
 |  ╚═══════════════╝  |
  \___________________/
      |||||||||||||
      |||||||||||||
"#;

const BLACK_HOLE: &str = r#"
           ⚫⚫⚫⚫⚫
        ⚫⚫       ⚫⚫
      ⚫⚫           ⚫⚫
     ⚫               ⚫
    ⚫    🕳️ EVENT    ⚫
    ⚫     HORIZON    ⚫
     ⚫               ⚫
      ⚫⚫           ⚫⚫
        ⚫⚫       ⚫⚫
           ⚫⚫⚫⚫⚫
"#;

const MAXIMILIAN: &str = r#"
    ╔═══════╗
    ║ ◉   ◉ ║
    ║   ▼   ║
    ╚═══════╝
   ╔═════════╗
   ║ ▓▓▓▓▓▓▓ ║
   ║ ▓▓▓▓▓▓▓ ║
   ╚═════════╝
    ║║║║║║║
    ║║║║║║║
"#;

const VINCENT: &str = r#"
    ╔═══╗
    ║ ◉ ║
    ║ ▼ ║
    ╚═══╝
   ╔═════╗
   ║ ░░░ ║
   ╚═════╝
    ║║║
"#;

struct Game {
    player_name: String,
    ship_integrity: i32,
    crew_morale: i32,
    distance_to_hole: i32,
    has_vincent: bool,
    maximilian_defeated: bool,
}

impl Game {
    fn new() -> Self {
        Self {
            player_name: String::new(),
            ship_integrity: 100,
            crew_morale: 100,
            distance_to_hole: 1000,
            has_vincent: true,
            maximilian_defeated: false,
        }
    }

    fn slow_print(&self, text: &str) {
        for ch in text.chars() {
            print!("{}", ch);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(30));
        }
        println!();
    }

    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().unwrap();
    }

    fn wait_for_key(&self) {
        print!("\n[Press ENTER to continue]");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }

    fn get_input(&self, prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    fn intro(&mut self) {
        self.clear_screen();
        println!("{}", TITLE_ART);
        thread::sleep(Duration::from_secs(2));

        self.slow_print("\nWelcome to THE BLACK HOLE BBS Door Game!");
        self.slow_print("Based on the 1979 Disney film");
        println!();

        self.player_name = self.get_input("Enter your name, Captain: ");
        if self.player_name.is_empty() {
            self.player_name = "Captain".to_string();
        }

        self.slow_print(&format!("\nWelcome aboard, Captain {}!", self.player_name));
        self.wait_for_key();
    }

    fn scene_discovery(&self) {
        self.clear_screen();
        self.slow_print("═".repeat(60).as_str());
        self.slow_print("SCENE 1: DISCOVERY");
        self.slow_print("═".repeat(60).as_str());
        println!();

        println!("{}", USS_PALOMINO);
        self.slow_print(&format!("\nCaptain {}, we've detected a massive object ahead...", self.player_name));
        self.slow_print("It's the USS Cygnus! The ship that disappeared 20 years ago!");
        self.slow_print("But wait... there's something else...");
        println!();
        println!("{}", BLACK_HOLE);
        self.slow_print("\nA BLACK HOLE! The Cygnus is on the edge of the event horizon!");

        self.wait_for_key();
    }

    fn scene_cygnus(&mut self) {
        self.clear_screen();
        self.slow_print("═".repeat(60).as_str());
        self.slow_print("SCENE 2: ABOARD THE CYGNUS");
        self.slow_print("═".repeat(60).as_str());
        println!();

        println!("{}", CYGNUS);
        self.slow_print("\nYou board the Cygnus. The ship is eerily quiet...");
        self.slow_print("Suddenly, a figure emerges from the shadows...");
        println!();
        println!("{}", MAXIMILIAN);
        self.slow_print("\nIt's MAXIMILIAN, Dr. Reinhardt's robot enforcer!");
        self.slow_print("Behind him, you see Dr. Hans Reinhardt...");
        self.slow_print("\nDr. Reinhardt: 'Welcome! I've been expecting you...'");
        self.slow_print("Dr. Reinhardt: 'I'm going to ride the Cygnus INTO the black hole!'");
        self.slow_print("Dr. Reinhardt: 'Join me, or die!'");

        println!();
        println!("What do you do?");
        println!("1. Accept Reinhardt's offer");
        println!("2. Refuse and try to escape");
        println!("3. Challenge Maximilian to a fight");

        let choice = self.get_input("\nYour choice (1-3): ");

        match choice.as_str() {
            "1" => self.ending_bad(),
            "3" => self.fight_maximilian(),
            _ => self.escape_sequence(),
        }
    }

    fn fight_maximilian(&mut self) {
        self.clear_screen();
        self.slow_print("═".repeat(60).as_str());
        self.slow_print("BATTLE: MAXIMILIAN");
        self.slow_print("═".repeat(60).as_str());
        println!();

        println!("{}", MAXIMILIAN);
        println!("     VS");
        println!("{}", VINCENT);

        self.slow_print("\nVINCENT steps forward to protect you!");
        self.slow_print("The two robots clash in an epic battle!");

        let mut vincent_hp = 100;
        let mut max_hp = 150;
        let mut rng = rand::thread_rng();

        while vincent_hp > 0 && max_hp > 0 {
            println!();
            println!("VINCENT HP: {}/100", vincent_hp);
            println!("MAXIMILIAN HP: {}/150", max_hp);
            println!();
            println!("1. Laser blast");
            println!("2. Dodge and counter");
            println!("3. Use environment");

            let choice = self.get_input("\nYour command: ");

            match choice.as_str() {
                "1" => {
                    let damage = rng.gen_range(15..=25);
                    max_hp -= damage;
                    self.slow_print(&format!("\nVINCENT fires! {} damage!", damage));
                }
                "2" => {
                    if rng.gen_bool(0.5) {
                        let damage = rng.gen_range(20..=30);
                        max_hp -= damage;
                        self.slow_print(&format!("\nPerfect dodge! Counter attack: {} damage!", damage));
                    } else {
                        let damage = rng.gen_range(10..=20);
                        vincent_hp -= damage;
                        self.slow_print(&format!("\nMissed! Maximilian strikes: {} damage!", damage));
                    }
                }
                "3" => {
                    let damage = rng.gen_range(25..=35);
                    max_hp -= damage;
                    self.slow_print(&format!("\nYou drop debris on Maximilian! {} damage!", damage));
                }
                _ => {}
            }

            if max_hp > 0 {
                let damage = rng.gen_range(10..=20);
                vincent_hp -= damage;
                self.slow_print(&format!("Maximilian attacks: {} damage!", damage));
            }

            thread::sleep(Duration::from_secs(1));
        }

        if vincent_hp > 0 {
            self.slow_print("\n🎉 VINCENT WINS!");
            self.slow_print("Maximilian is defeated!");
            self.maximilian_defeated = true;
            self.wait_for_key();
            self.escape_sequence();
        } else {
            self.slow_print("\n💀 VINCENT is destroyed!");
            self.has_vincent = false;
            self.ending_bad();
        }
    }

    fn escape_sequence(&mut self) {
        self.clear_screen();
        self.slow_print("═".repeat(60).as_str());
        self.slow_print("SCENE 3: ESCAPE!");
        self.slow_print("═".repeat(60).as_str());
        println!();

        self.slow_print("The Cygnus is breaking apart!");
        self.slow_print("You must reach the Palomino before it's too late!");
        self.slow_print(&format!("\nDistance to Palomino: {}m", self.distance_to_hole));
        self.slow_print(&format!("Ship Integrity: {}%", self.ship_integrity));

        let mut rng = rand::thread_rng();

        while self.distance_to_hole > 0 && self.ship_integrity > 0 {
            println!();
            println!("1. Full speed ahead!");
            println!("2. Navigate carefully");
            println!("3. Help injured crew");

            let choice = self.get_input("\nYour action: ");

            match choice.as_str() {
                "1" => {
                    self.distance_to_hole -= rng.gen_range(200..=300);
                    self.ship_integrity -= rng.gen_range(10..=20);
                    self.slow_print("\nFull throttle! But taking damage!");
                }
                "2" => {
                    self.distance_to_hole -= rng.gen_range(100..=150);
                    self.ship_integrity -= rng.gen_range(5..=10);
                    self.slow_print("\nCareful navigation...");
                }
                "3" => {
                    self.crew_morale += 20;
                    self.distance_to_hole -= rng.gen_range(50..=100);
                    self.slow_print("\nCrew morale improved!");
                }
                _ => {}
            }

            self.slow_print(&format!("Distance: {}m", self.distance_to_hole.max(0)));
            self.slow_print(&format!("Integrity: {}%", self.ship_integrity));

            if self.distance_to_hole <= 0 {
                break;
            }

            thread::sleep(Duration::from_secs(1));
        }

        if self.ship_integrity > 0 {
            self.ending_good();
        } else {
            self.ending_bad();
        }
    }

    fn ending_good(&self) {
        self.clear_screen();
        self.slow_print("═".repeat(60).as_str());
        self.slow_print("ENDING: ESCAPE!");
        self.slow_print("═".repeat(60).as_str());
        println!();

        self.slow_print("You made it to the Palomino!");
        self.slow_print("The ship breaks free from the black hole's gravity!");
        self.slow_print("\nAs you look back, you see the Cygnus falling into the event horizon...");
        self.slow_print("Dr. Reinhardt and Maximilian are consumed by the singularity...");
        println!();
        println!("{}", BLACK_HOLE);
        self.slow_print("\nYou've survived THE BLACK HOLE!");
        self.slow_print(&format!("\nFinal Score: {}", self.ship_integrity + self.crew_morale));

        if self.maximilian_defeated {
            self.slow_print("\n🏆 BONUS: Defeated Maximilian!");
        }

        self.slow_print("\n✨ CONGRATULATIONS! ✨");
    }

    fn ending_bad(&self) {
        self.clear_screen();
        self.slow_print("═".repeat(60).as_str());
        self.slow_print("ENDING: INTO THE VOID");
        self.slow_print("═".repeat(60).as_str());
        println!();

        self.slow_print("The gravitational pull is too strong...");
        self.slow_print("Your ship is drawn into the black hole...");
        println!();
        println!("{}", BLACK_HOLE);
        self.slow_print("\nYou cross the event horizon...");
        self.slow_print("Time and space become meaningless...");
        self.slow_print("You see visions of heaven and hell...");
        self.slow_print("\n💀 GAME OVER 💀");
    }

    fn play(&mut self) {
        self.intro();
        self.scene_discovery();
        self.scene_cygnus();

        println!();
        self.slow_print("Thanks for playing THE BLACK HOLE!");
        self.slow_print("A BBS Door Game by OSM Monster Systems");
        self.slow_print("\nPress ENTER to return to BBS...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }
}

fn main() {
    let mut game = Game::new();
    game.play();
}
