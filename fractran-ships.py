#!/usr/bin/env python3
# FRACTRAN Ships - Evolving AI ships that play Monster TradeWars
# Based on plan.org FRACTRAN algebra and 15D Monster coordinates

import random
import math
from dataclasses import dataclass
from typing import List, Tuple
import json

# Monster Group 15 primes (from plan.org)
MONSTER_PRIMES = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71]

# FRACTRAN programs from plan.org (operators on (ℕ,×,1) monoid)
FRACTRAN_PROGRAMS = {
    "complete-system": [(4.18e21, 2.17e28)],  # 35 fractions folded
    "godel-scheduler": [(3.20e11, 4.49e14)],  # 15 fractions
    "harmonic-tick": [(2.34e13, 6.26e16)],    # 17 fractions
    "integrator-lattice": [(2.34e13, 1.37e17)]  # 20 fractions
}

@dataclass
class MonsterCoord:
    """15-dimensional Monster group coordinates"""
    coords: List[int]
    
    def __init__(self, seed: int = None):
        if seed is None:
            self.coords = [random.randint(0, p-1) for p in MONSTER_PRIMES]
        else:
            self.coords = [(seed * (i+1)) % p for i, p in enumerate(MONSTER_PRIMES)]
    
    def distance_to(self, other: 'MonsterCoord') -> float:
        return math.sqrt(sum((a - b)**2 for a, b in zip(self.coords, other.coords)))
    
    def shard(self) -> int:
        return sum(self.coords) % 71
    
    def to_fractran_state(self) -> int:
        """Convert 15D coords to FRACTRAN state (product of prime powers)"""
        state = 1
        for coord, prime in zip(self.coords, MONSTER_PRIMES):
            state *= prime ** coord
        return state
    
    def __repr__(self):
        return f"M15D({self.shard()})"

@dataclass
class FractranShip:
    """AI ship that evolves using FRACTRAN programs"""
    name: str
    position: MonsterCoord
    program: str  # Which FRACTRAN program
    state: int    # Current FRACTRAN state
    fitness: float
    generation: int
    cargo: int
    credits: int
    fuel: int
    
    def evolve(self):
        """Execute one FRACTRAN step"""
        if self.program not in FRACTRAN_PROGRAMS:
            return
        
        # Get program fractions
        fractions = FRACTRAN_PROGRAMS[self.program]
        
        # Try each fraction
        for num, denom in fractions:
            if self.state % denom == 0:
                self.state = int(self.state * num / denom)
                self.generation += 1
                break
        
        # Update fitness based on state
        self.fitness = math.log(self.state + 1) / 100
    
    def navigate(self, target: MonsterCoord) -> bool:
        """Navigate using FRACTRAN-guided movement"""
        distance = self.position.distance_to(target)
        fuel_cost = int(distance / 10)
        
        if fuel_cost > self.fuel:
            return False
        
        # Evolve before moving (FRACTRAN guides navigation)
        self.evolve()
        
        # Move to target
        self.position = target
        self.fuel -= fuel_cost
        
        return True
    
    def trade(self, buy_price: int, sell_price: int) -> int:
        """Trade using FRACTRAN-optimized strategy"""
        # Evolve to get trading decision
        self.evolve()
        
        # Use FRACTRAN state to decide quantity
        quantity = (self.state % 50) + 1
        
        # Buy if we have credits
        if buy_price * quantity <= self.credits:
            self.cargo += quantity
            self.credits -= buy_price * quantity
            return quantity
        
        return 0
    
    def __repr__(self):
        return f"{self.name} @ {self.position} | Gen:{self.generation} Fit:{self.fitness:.3f} Credits:{self.credits}"

class FractranFleet:
    """Fleet of evolving FRACTRAN ships"""
    
    def __init__(self, size: int = 4):
        self.ships = []
        programs = list(FRACTRAN_PROGRAMS.keys())
        
        for i in range(size):
            ship = FractranShip(
                name=f"FRACTRAN-{i+1}",
                position=MonsterCoord(i * 1000),
                program=programs[i % len(programs)],
                state=2 ** (i + 2),  # Start with power of 2
                fitness=0.0,
                generation=0,
                cargo=0,
                credits=1000,
                fuel=100
            )
            self.ships.append(ship)
    
    def evolve_all(self):
        """Evolve entire fleet"""
        for ship in self.ships:
            ship.evolve()
    
    def best_ship(self) -> FractranShip:
        """Get ship with highest fitness"""
        return max(self.ships, key=lambda s: s.fitness)
    
    def tournament_selection(self) -> FractranShip:
        """Select ship via tournament"""
        contestants = random.sample(self.ships, 2)
        return max(contestants, key=lambda s: s.fitness)
    
    def crossover(self, parent1: FractranShip, parent2: FractranShip) -> FractranShip:
        """Create offspring from two parents"""
        # Mix coordinates
        child_coords = []
        for c1, c2 in zip(parent1.position.coords, parent2.position.coords):
            child_coords.append(random.choice([c1, c2]))
        
        child_pos = MonsterCoord(0)
        child_pos.coords = child_coords
        
        # Create child ship
        child = FractranShip(
            name=f"FRACTRAN-Gen{parent1.generation + 1}",
            position=child_pos,
            program=random.choice([parent1.program, parent2.program]),
            state=(parent1.state + parent2.state) // 2,
            fitness=0.0,
            generation=max(parent1.generation, parent2.generation) + 1,
            cargo=0,
            credits=1000,
            fuel=100
        )
        
        return child
    
    def mutate(self, ship: FractranShip):
        """Mutate ship's FRACTRAN state"""
        # Random prime mutation
        prime = random.choice(MONSTER_PRIMES)
        if random.random() < 0.5:
            ship.state *= prime
        else:
            ship.state //= prime
        
        ship.state = max(1, ship.state)
    
    def genetic_algorithm(self, generations: int = 10):
        """Run genetic algorithm on fleet"""
        print(f"\n🧬 FRACTRAN Fleet Evolution ({generations} generations)")
        print("=" * 60)
        
        for gen in range(generations):
            # Evolve all ships
            self.evolve_all()
            
            # Show best
            best = self.best_ship()
            print(f"Gen {gen}: Best = {best.name} | Fit={best.fitness:.3f} | State={best.state}")
            
            # Selection and reproduction
            if gen < generations - 1:
                # Keep best 2
                self.ships.sort(key=lambda s: s.fitness, reverse=True)
                survivors = self.ships[:2]
                
                # Create 2 offspring
                offspring = []
                for _ in range(2):
                    parent1 = self.tournament_selection()
                    parent2 = self.tournament_selection()
                    child = self.crossover(parent1, parent2)
                    
                    # Mutate with 20% probability
                    if random.random() < 0.2:
                        self.mutate(child)
                    
                    offspring.append(child)
                
                # New generation
                self.ships = survivors + offspring
        
        print("\n✅ Evolution complete!")
        return self.best_ship()

def demo_fractran_ships():
    """Demonstrate FRACTRAN ships"""
    print("""
╔═══════════════════════════════════════════════════════════╗
║                                                           ║
║           FRACTRAN SHIPS - AI TRADERS                     ║
║                                                           ║
║     Evolving ships using FRACTRAN algebra                 ║
║     Navigate 15D Monster space autonomously               ║
║                                                           ║
╚═══════════════════════════════════════════════════════════╝
    """)
    
    # Create fleet
    fleet = FractranFleet(size=4)
    
    print("\n🚀 Initial Fleet:")
    print("-" * 60)
    for ship in fleet.ships:
        print(f"  {ship}")
        print(f"     Program: {ship.program}")
        print(f"     State: {ship.state}")
        print(f"     Position: {ship.position}")
    
    # Evolve fleet
    best = fleet.genetic_algorithm(generations=20)
    
    print(f"\n👑 CHAMPION SHIP:")
    print("-" * 60)
    print(f"  {best}")
    print(f"  Program: {best.program}")
    print(f"  State: {best.state}")
    print(f"  Position: {best.position}")
    print(f"  Shard: {best.position.shard()}/71")
    
    # Test navigation
    print(f"\n🧭 Navigation Test:")
    print("-" * 60)
    target = MonsterCoord(random.randint(0, 10000))
    print(f"  Target: {target}")
    
    if best.navigate(target):
        print(f"  ✅ Navigation successful!")
        print(f"  New position: {best.position}")
        print(f"  Fuel remaining: {best.fuel}")
        print(f"  Generation: {best.generation}")
    else:
        print(f"  ❌ Not enough fuel")
    
    # Save best ship
    ship_data = {
        "name": best.name,
        "program": best.program,
        "state": best.state,
        "generation": best.generation,
        "fitness": best.fitness,
        "position": best.position.coords,
        "shard": best.position.shard()
    }
    
    with open("/home/mdupont/projects/osm-planet-torrent/best_fractran_ship.json", "w") as f:
        json.dump(ship_data, f, indent=2)
    
    print(f"\n💾 Best ship saved to best_fractran_ship.json")

if __name__ == "__main__":
    demo_fractran_ships()
