#!/usr/bin/env python3
# Monster TradeWars - Elite/Frontier style trading game in 15D Monster space
# Ships navigate through 71 Monster shards using existing shard infrastructure at `/mnt/data1/introspector/shards/`.

import random
import math
import json
import os
from dataclasses import dataclass
from typing import List, Tuple

# Monster Group 15 primes
MONSTER_PRIMES = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71]
NUM_SHARDS = 71

# FRACTRAN programs from plan.org
FRACTRAN_PROGRAMS = {
    "complete-system": [(4.18e21, 2.17e28)],
    "godel-scheduler": [(3.20e11, 4.49e14)],
    "harmonic-tick": [(2.34e13, 6.26e16)],
    "integrator-lattice": [(2.34e13, 1.37e17)]
}

@dataclass
class MonsterCoord:
    """15-dimensional Monster group coordinates"""
    coords: List[int]  # 15 coordinates, one per prime
    
    def __init__(self, seed: int = None):
        if seed is None:
            self.coords = [random.randint(0, p-1) for p in MONSTER_PRIMES]
        else:
            self.coords = [(seed * (i+1)) % p for i, p in enumerate(MONSTER_PRIMES)]
    
    def distance_to(self, other: 'MonsterCoord') -> float:
        """Euclidean distance in 15D space"""
        return math.sqrt(sum((a - b)**2 for a, b in zip(self.coords, other.coords)))
    
    def shard(self) -> int:
        """Map to 71 shards using largest prime"""
        return sum(self.coords) % NUM_SHARDS
    
    def __repr__(self):
        return f"M15D({self.shard()})"

@dataclass
class Ship:
    """Player ship with 15D position"""
    name: str
    ship_class: str  # Palomino, Cygnus, Maximilian, VINCENT
    position: MonsterCoord
    velocity: MonsterCoord
    cargo: int
    max_cargo: int
    credits: int
    fuel: int
    max_fuel: int
    weapons: int
    shields: int
    
    def __repr__(self):
        return f"{self.name} ({self.ship_class}) @ {self.position} | Cargo:{self.cargo}/{self.max_cargo} Credits:{self.credits} Fuel:{self.fuel}"

# Ship classes from The Black Hole (1979)
SHIP_CLASSES = {
    "Palomino": {"cargo": 100, "fuel": 100, "weapons": 2, "shields": 50, "speed": 1.0},
    "Cygnus": {"cargo": 200, "fuel": 150, "weapons": 5, "shields": 100, "speed": 0.7},
    "Maximilian": {"cargo": 50, "fuel": 80, "weapons": 10, "shields": 150, "speed": 1.2},
    "VINCENT": {"cargo": 75, "fuel": 120, "weapons": 4, "shields": 80, "speed": 1.1},
}

@dataclass
class Station:
    """Trading station at Monster coordinate"""
    name: str
    position: MonsterCoord
    commodity: str
    buy_price: int
    sell_price: int
    
    def __repr__(self):
        return f"{self.name} @ {self.position} | {self.commodity}: Buy {self.buy_price} Sell {self.sell_price}"

class MonsterTradeWars:
    def __init__(self):
        # Choose ship class
        self.ship = self.choose_ship()
        
        # Generate stations at different Monster coordinates
        self.stations = self.generate_stations()
        self.current_station = self.stations[0]
        self.game_mode = "trade"  # trade, combat, explore
        self.enemies = []
    
    def choose_ship(self) -> Ship:
        """Choose ship class at game start"""
        print("\n🚀 CHOOSE YOUR SHIP:")
        print("-" * 60)
        for i, (name, stats) in enumerate(SHIP_CLASSES.items(), 1):
            print(f"{i}. {name}")
            print(f"   Cargo: {stats['cargo']} | Fuel: {stats['fuel']} | Weapons: {stats['weapons']}")
            print(f"   Shields: {stats['shields']} | Speed: {stats['speed']}x")
            print()
        
        while True:
            try:
                choice = int(input("Select ship (1-4): "))
                if 1 <= choice <= 4:
                    ship_name = list(SHIP_CLASSES.keys())[choice - 1]
                    stats = SHIP_CLASSES[ship_name]
                    
                    return Ship(
                        name=f"USS {ship_name}",
                        ship_class=ship_name,
                        position=MonsterCoord(0),
                        velocity=MonsterCoord(0),
                        cargo=0,
                        max_cargo=stats["cargo"],
                        credits=1000,
                        fuel=stats["fuel"],
                        max_fuel=stats["fuel"],
                        weapons=stats["weapons"],
                        shields=stats["shields"]
                    )
            except (ValueError, IndexError):
                print("❌ Invalid choice")
        
    def generate_stations(self) -> List[Station]:
        """Generate trading stations across Monster space"""
        commodities = [
            "Brainrot Bisque", "Hawking Radiation", "Monster Symmetry",
            "Conformal Arrows", "j-invariant Fuel", "Enlightenment Crystals",
            "OODA Loops", "ZK Witnesses", "Thinker-Prover Pairs"
        ]
        
        stations = []
        for i in range(20):
            pos = MonsterCoord(i * 1000)
            commodity = random.choice(commodities)
            base_price = random.randint(50, 200)
            
            stations.append(Station(
                name=f"Station-{pos.shard()}",
                position=pos,
                commodity=commodity,
                buy_price=base_price,
                sell_price=int(base_price * 1.5)
            ))
        
        return stations
    
    def find_nearest_station(self) -> Station:
        """Find nearest station to current position"""
        return min(self.stations, 
                  key=lambda s: self.ship.position.distance_to(s.position))
    
    def navigate_to(self, target: MonsterCoord):
        """Navigate ship through 15D Monster space"""
        distance = self.ship.position.distance_to(target)
        fuel_cost = int(distance / 10)
        
        if fuel_cost > self.ship.fuel:
            print(f"❌ Not enough fuel! Need {fuel_cost}, have {self.ship.fuel}")
            return False
        
        print(f"🚀 Navigating through Monster space...")
        print(f"   From shard {self.ship.position.shard()} to {target.shard()}")
        print(f"   Distance: {distance:.2f} in 15D")
        print(f"   Fuel cost: {fuel_cost}")
        
        # Move ship
        self.ship.position = target
        self.ship.fuel -= fuel_cost
        
        # Random encounter
        if random.random() < 0.2:
            self.random_encounter()
        
        return True
    
    def random_encounter(self):
        """Random events in Monster space"""
        roll = random.random()
        
        if roll < 0.15:  # Combat encounter
            self.combat_encounter()
        elif roll < 0.3:  # Environmental
            events = [
                ("🕳️ Black hole detected! Gravity well drains fuel.", -10, 0, 0),
                ("⭐ Hawking radiation boost! Free energy.", 20, 0, 0),
                ("💎 Found floating cargo!", 0, 10, 0),
                ("🎨 Conformal anomaly! Credits gained.", 0, 0, 100),
                ("☢️ Radiation storm! Shield damage.", 0, 0, -20),
            ]
            
            event, fuel_change, cargo_change, shield_change = random.choice(events)
            print(f"\n{event}")
            
            self.ship.fuel = max(0, min(self.ship.max_fuel, self.ship.fuel + fuel_change))
            self.ship.cargo = max(0, min(self.ship.max_cargo, self.ship.cargo + cargo_change))
            self.ship.shields = max(0, self.ship.shields + shield_change)
    
    def combat_encounter(self):
        """Combat with enemy ship"""
        enemy_types = ["Pirate", "Raider", "Maximilian Bot", "Rogue AI"]
        enemy = random.choice(enemy_types)
        enemy_shields = random.randint(20, 100)
        
        print(f"\n⚔️ COMBAT: {enemy} detected!")
        print(f"   Enemy shields: {enemy_shields}")
        print(f"   Your weapons: {self.ship.weapons}")
        print(f"   Your shields: {self.ship.shields}")
        
        while True:
            print("\n1. Attack  2. Flee  3. Negotiate")
            choice = input("Action: ").strip()
            
            if choice == "1":  # Attack
                damage = self.ship.weapons * random.randint(5, 15)
                enemy_shields -= damage
                print(f"💥 You deal {damage} damage!")
                
                if enemy_shields <= 0:
                    loot = random.randint(100, 500)
                    self.ship.credits += loot
                    print(f"✅ Enemy destroyed! Looted {loot} credits")
                    break
                
                # Enemy counterattack
                enemy_damage = random.randint(10, 30)
                self.ship.shields -= enemy_damage
                print(f"💢 Enemy deals {enemy_damage} damage!")
                
                if self.ship.shields <= 0:
                    print("💀 SHIP DESTROYED! Game Over")
                    exit(0)
            
            elif choice == "2":  # Flee
                if random.random() < 0.6:
                    print("🏃 Escaped!")
                    break
                else:
                    print("❌ Failed to escape!")
                    enemy_damage = random.randint(10, 30)
                    self.ship.shields -= enemy_damage
                    print(f"💢 Enemy deals {enemy_damage} damage!")
            
            elif choice == "3":  # Negotiate
                bribe = random.randint(50, 200)
                if self.ship.credits >= bribe:
                    self.ship.credits -= bribe
                    print(f"💰 Paid {bribe} credits. Enemy leaves.")
                    break
                else:
                    print(f"❌ Need {bribe} credits to negotiate!")
            
            else:
                print("❌ Invalid action")
    
    def dock_at_station(self, station: Station):
        """Dock at trading station"""
        if not self.navigate_to(station.position):
            return
        
        self.current_station = station
        print(f"\n🛸 Docked at {station.name}")
        print(f"   Location: {station.position}")
        print(f"   Trading: {station.commodity}")
        print(f"   Buy: {station.buy_price} | Sell: {station.sell_price}")
    
    def buy_cargo(self, quantity: int):
        """Buy cargo at current station"""
        cost = self.current_station.buy_price * quantity
        
        if cost > self.ship.credits:
            print(f"❌ Not enough credits! Need {cost}, have {self.ship.credits}")
            return
        
        if self.ship.cargo + quantity > self.ship.max_cargo:
            print(f"❌ Not enough cargo space!")
            return
        
        self.ship.cargo += quantity
        self.ship.credits -= cost
        print(f"✅ Bought {quantity} {self.current_station.commodity} for {cost} credits")
    
    def sell_cargo(self, quantity: int):
        """Sell cargo at current station"""
        if quantity > self.ship.cargo:
            print(f"❌ Not enough cargo! Have {self.ship.cargo}")
            return
        
        profit = self.current_station.sell_price * quantity
        self.ship.cargo -= quantity
        self.ship.credits += profit
        print(f"✅ Sold {quantity} {self.current_station.commodity} for {profit} credits")
    
    def refuel(self):
        """Refuel at current station"""
        fuel_needed = self.ship.max_fuel - self.ship.fuel
        cost = fuel_needed * 2
        
        if cost > self.ship.credits:
            print(f"❌ Not enough credits for fuel! Need {cost}")
            return
        
        self.ship.fuel = self.ship.max_fuel
        self.ship.credits -= cost
        print(f"⛽ Refueled for {cost} credits")
    
    def show_status(self):
        """Display ship status"""
        print("\n" + "="*60)
        print(f"SHIP STATUS: {self.ship.name} ({self.ship.ship_class})")
        print("="*60)
        print(f"Position: {self.ship.position}")
        print(f"Shard: {self.ship.position.shard()}/71")
        print(f"Cargo: {self.ship.cargo}/{self.ship.max_cargo}")
        print(f"Credits: {self.ship.credits}")
        print(f"Fuel: {self.ship.fuel}/{self.ship.max_fuel}")
        print(f"Weapons: {self.ship.weapons} | Shields: {self.ship.shields}")
        print("="*60)
    
    def show_local_map(self):
        """Show nearby stations"""
        print("\n📡 LOCAL SCANNER - Nearby Stations:")
        print("-" * 60)
        
        # Sort by distance
        nearby = sorted(self.stations, 
                       key=lambda s: self.ship.position.distance_to(s.position))[:5]
        
        for i, station in enumerate(nearby, 1):
            dist = self.ship.position.distance_to(station.position)
            print(f"{i}. {station.name} @ Shard {station.position.shard()} - {dist:.1f} units")
            print(f"   {station.commodity}: Buy {station.buy_price} / Sell {station.sell_price}")
    
    def play(self):
        """Main game loop"""
        print("""
╔═══════════════════════════════════════════════════════════╗
║                                                           ║
║           MONSTER TRADEWARS 2026                          ║
║                                                           ║
║     Elite/Frontier Trading in 15D Monster Space           ║
║                                                           ║
║  Navigate 71 shards using Monster group coordinates      ║
║  Trade commodities across conformal space                 ║
║  Survive black holes and radiation storms                 ║
║                                                           ║
╚═══════════════════════════════════════════════════════════╝
        """)
        
        self.show_status()
        
        while True:
            print("\n" + "="*60)
            print("COMMANDS:")
            print("  1. Show status")
            print("  2. Show local map")
            print("  3. Navigate to station")
            print("  4. Buy cargo")
            print("  5. Sell cargo")
            print("  6. Refuel")
            print("  7. Jump to random shard")
            print("  8. Repair shields")
            print("  9. Upgrade ship")
            print("  M. Mini-games")
            print("  0. Quit")
            print("="*60)
            
            choice = input("\nCommand: ").strip()
            
            if choice == "1":
                self.show_status()
            
            elif choice == "2":
                self.show_local_map()
            
            elif choice == "3":
                self.show_local_map()
                try:
                    idx = int(input("\nSelect station (1-5): ")) - 1
                    nearby = sorted(self.stations, 
                                  key=lambda s: self.ship.position.distance_to(s.position))[:5]
                    if 0 <= idx < len(nearby):
                        self.dock_at_station(nearby[idx])
                except (ValueError, IndexError):
                    print("❌ Invalid selection")
            
            elif choice == "4":
                try:
                    qty = int(input(f"Buy how many {self.current_station.commodity}? "))
                    self.buy_cargo(qty)
                except ValueError:
                    print("❌ Invalid quantity")
            
            elif choice == "5":
                try:
                    qty = int(input(f"Sell how many {self.current_station.commodity}? "))
                    self.sell_cargo(qty)
                except ValueError:
                    print("❌ Invalid quantity")
            
            elif choice == "6":
                self.refuel()
            
            elif choice == "7":
                # Jump to random shard
                target = MonsterCoord(random.randint(0, 10000))
                print(f"🌀 Jumping to random shard {target.shard()}...")
                self.navigate_to(target)
                nearest = self.find_nearest_station()
                print(f"📡 Nearest station: {nearest.name}")
            
            elif choice == "8":
                # Repair shields
                max_shields = SHIP_CLASSES[self.ship.ship_class]["shields"]
                damage = max_shields - self.ship.shields
                cost = damage * 2
                
                if cost > self.ship.credits:
                    print(f"❌ Need {cost} credits for repairs")
                else:
                    self.ship.shields = max_shields
                    self.ship.credits -= cost
                    print(f"🔧 Shields repaired for {cost} credits")
            
            elif choice == "9":
                # Upgrade ship
                print("\n⚙️ UPGRADES:")
                print("1. Weapons (+1) - 500 credits")
                print("2. Cargo (+20) - 300 credits")
                print("3. Fuel Tank (+20) - 200 credits")
                
                upgrade = input("Select upgrade (1-3): ").strip()
                
                if upgrade == "1" and self.ship.credits >= 500:
                    self.ship.weapons += 1
                    self.ship.credits -= 500
                    print("✅ Weapons upgraded!")
                elif upgrade == "2" and self.ship.credits >= 300:
                    self.ship.max_cargo += 20
                    self.ship.credits -= 300
                    print("✅ Cargo bay expanded!")
                elif upgrade == "3" and self.ship.credits >= 200:
                    self.ship.max_fuel += 20
                    self.ship.fuel += 20
                    self.ship.credits -= 200
                    print("✅ Fuel tank upgraded!")
                else:
                    print("❌ Cannot afford upgrade")
            
            elif choice.upper() == "M":
                # Mini-games
                self.mini_games_menu()
            
            elif choice == "0":
                print("\n👋 Thanks for playing Monster TradeWars!")
                print(f"Final credits: {self.ship.credits}")
                break
            
            else:
                print("❌ Invalid command")
    
    def mini_games_menu(self):
        """Mini-games for earning credits"""
        print("\n" + "="*60)
        print("MINI-GAMES")
        print("="*60)
        print("1. Monster Prime Guess - Guess the prime (50 credits)")
        print("2. Shard Roulette - Bet on shard number (2x payout)")
        print("3. Conformal Maze - Navigate 15D maze (100 credits)")
        print("4. Black Hole Poker - 5-card draw (varies)")
        print("5. Back to main menu")
        print("="*60)
        
        choice = input("\nSelect game: ").strip()
        
        if choice == "1":
            self.prime_guess_game()
        elif choice == "2":
            self.shard_roulette()
        elif choice == "3":
            self.conformal_maze()
        elif choice == "4":
            self.black_hole_poker()
    
    def prime_guess_game(self):
        """Guess which Monster prime"""
        target = random.choice(MONSTER_PRIMES)
        print(f"\n🎲 Guess the Monster prime (2-71)")
        
        try:
            guess = int(input("Your guess: "))
            if guess == target:
                self.ship.credits += 50
                print(f"✅ Correct! Won 50 credits. It was {target}")
            else:
                print(f"❌ Wrong! It was {target}")
        except ValueError:
            print("❌ Invalid guess")
    
    def shard_roulette(self):
        """Bet on shard number"""
        print(f"\n🎰 Shard Roulette (0-70)")
        
        try:
            bet = int(input("Bet amount: "))
            if bet > self.ship.credits:
                print("❌ Not enough credits")
                return
            
            guess = int(input("Guess shard (0-70): "))
            result = random.randint(0, 70)
            
            if guess == result:
                winnings = bet * 10
                self.ship.credits += winnings
                print(f"🎉 JACKPOT! Shard {result}! Won {winnings} credits!")
            elif abs(guess - result) <= 5:
                winnings = bet * 2
                self.ship.credits += winnings
                print(f"✅ Close! Shard {result}. Won {winnings} credits!")
            else:
                self.ship.credits -= bet
                print(f"❌ Lost! Shard was {result}")
        except ValueError:
            print("❌ Invalid input")
    
    def conformal_maze(self):
        """Navigate 15D conformal maze"""
        print(f"\n🌀 Conformal Maze - Find exit in 5 moves")
        
        pos = [0] * 15
        target = [random.randint(-2, 2) for _ in range(15)]
        moves = 5
        
        while moves > 0:
            dist = math.sqrt(sum((a - b)**2 for a, b in zip(pos, target)))
            print(f"Distance to exit: {dist:.2f} | Moves left: {moves}")
            
            if dist < 1.0:
                self.ship.credits += 100
                print("✅ Found exit! Won 100 credits!")
                return
            
            print("Move in dimension (0-14):")
            try:
                dim = int(input("Dimension: "))
                direction = int(input("Direction (-1 or +1): "))
                
                if 0 <= dim < 15 and direction in [-1, 1]:
                    pos[dim] += direction
                    moves -= 1
                else:
                    print("❌ Invalid move")
            except ValueError:
                print("❌ Invalid input")
        
        print("❌ Out of moves! Lost in 15D space")
    
    def black_hole_poker(self):
        """Simple 5-card poker"""
        print(f"\n🃏 Black Hole Poker")
        
        try:
            bet = int(input("Bet amount: "))
            if bet > self.ship.credits:
                print("❌ Not enough credits")
                return
            
            # Simple hand evaluation
            hand = [random.randint(1, 13) for _ in range(5)]
            hand.sort()
            
            print(f"Your hand: {hand}")
            
            # Check for pairs, three of a kind, etc.
            unique = len(set(hand))
            
            if unique == 1:  # Five of a kind (impossible but fun)
                winnings = bet * 100
                self.ship.credits += winnings
                print(f"🎉 FIVE OF A KIND! Won {winnings}!")
            elif unique == 2:  # Four of a kind or full house
                winnings = bet * 10
                self.ship.credits += winnings
                print(f"✅ Four of a kind! Won {winnings}!")
            elif unique == 3:  # Three of a kind or two pair
                winnings = bet * 3
                self.ship.credits += winnings
                print(f"✅ Three of a kind! Won {winnings}!")
            elif unique == 4:  # One pair
                winnings = bet * 2
                self.ship.credits += winnings
                print(f"✅ Pair! Won {winnings}!")
            else:
                self.ship.credits -= bet
                print(f"❌ High card. Lost {bet}")
        except ValueError:
            print("❌ Invalid bet")

if __name__ == "__main__":
    game = MonsterTradeWars()
    game.play()
