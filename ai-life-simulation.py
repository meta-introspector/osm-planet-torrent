#!/usr/bin/env python3
"""AI Life Simulation - 24 nodes living in Monster OSM map, swapping memes"""

import json
import random
import time
from dataclasses import dataclass, asdict
from typing import List, Dict, Set
from datetime import datetime

# Monster Group primes for meme encoding
MONSTER_PRIMES = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71]

# 71 shards (Monster Group mod 71)
SHARDS = list(range(71))

# Sacred shards
SACRED_SHARDS = {17: "Cusp", 23: "Consciousness", 59: "Memory"}

@dataclass
class Meme:
    """A meme is encoded as Monster primes"""
    id: int
    content: str
    prime_encoding: int
    fitness: float
    generation: int
    parent_id: int = None
    
    def mutate(self, gen: int) -> 'Meme':
        """Create mutated meme"""
        new_prime = random.choice(MONSTER_PRIMES)
        new_encoding = (self.prime_encoding * new_prime) % (71 * 71 * 71)
        
        mutations = ["evolved", "enhanced", "optimized", "transformed", "amplified"]
        new_content = f"{self.content}_{random.choice(mutations)}"
        
        return Meme(
            id=random.randint(1000, 9999),
            content=new_content,
            prime_encoding=new_encoding,
            fitness=self.fitness * random.uniform(0.9, 1.2),
            generation=gen,
            parent_id=self.id
        )

@dataclass
class AINode:
    """An AI node living in the map"""
    id: int
    name: str
    shard: int
    position: tuple
    memes: List[Meme]
    energy: float = 100.0
    generation: int = 0
    connections: Set[int] = None
    
    def __post_init__(self):
        if self.connections is None:
            self.connections = set()
    
    def move(self):
        """Move to adjacent shard"""
        # Hyperbolic movement (mod 71)
        direction = random.choice([-1, 0, 1])
        self.shard = (self.shard + direction) % 71
        self.energy -= 0.5
        
        # Gain energy at sacred shards
        if self.shard in SACRED_SHARDS:
            self.energy += 10.0
            return f"⚡ {self.name} reached {SACRED_SHARDS[self.shard]}!"
        return None
    
    def create_meme(self) -> Meme:
        """Create new meme from current state"""
        prime_encoding = MONSTER_PRIMES[self.shard % len(MONSTER_PRIMES)]
        
        meme_types = ["fractran", "hyperbolic", "ramanujan", "monster", "topology"]
        content = f"{random.choice(meme_types)}_shard{self.shard}"
        
        return Meme(
            id=random.randint(1000, 9999),
            content=content,
            prime_encoding=prime_encoding,
            fitness=random.uniform(0.5, 1.5),
            generation=self.generation
        )
    
    def swap_memes(self, other: 'AINode') -> tuple:
        """Swap memes with another node"""
        if not self.memes or not other.memes:
            return None, None
        
        # Select best memes
        my_best = max(self.memes, key=lambda m: m.fitness)
        their_best = max(other.memes, key=lambda m: m.fitness)
        
        # Crossover: create hybrid meme
        hybrid_encoding = (my_best.prime_encoding * their_best.prime_encoding) % (71 * 71 * 71)
        hybrid = Meme(
            id=random.randint(1000, 9999),
            content=f"{my_best.content}×{their_best.content}",
            prime_encoding=hybrid_encoding,
            fitness=(my_best.fitness + their_best.fitness) / 2,
            generation=max(my_best.generation, their_best.generation) + 1,
            parent_id=my_best.id
        )
        
        # Exchange
        self.memes.append(their_best)
        other.memes.append(my_best)
        
        # Both get hybrid
        self.memes.append(hybrid)
        other.memes.append(hybrid)
        
        # Prune weak memes
        self.memes = sorted(self.memes, key=lambda m: m.fitness, reverse=True)[:5]
        other.memes = sorted(other.memes, key=lambda m: m.fitness, reverse=True)[:5]
        
        self.energy -= 2.0
        other.energy -= 2.0
        
        return my_best, their_best

class AILifeSimulation:
    """24 AI nodes living in Monster OSM map"""
    
    def __init__(self):
        self.nodes = self._create_nodes()
        self.generation = 0
        self.meme_pool = []
        self.events = []
        
    def _create_nodes(self) -> List[AINode]:
        """Create 24 AI nodes (2³ × 3)"""
        greek = ["α", "β", "γ", "δ", "ε", "ζ", "η", "θ", "ι", "κ", "λ", "μ",
                 "ν", "ξ", "ο", "π", "ρ", "σ", "τ", "υ", "φ", "χ", "ψ", "ω"]
        
        nodes = []
        for i in range(24):
            shard = (i * 3) % 71  # Distribute across shards
            initial_meme = Meme(
                id=i * 100,
                content=f"genesis_{greek[i]}",
                prime_encoding=MONSTER_PRIMES[i % len(MONSTER_PRIMES)],
                fitness=1.0,
                generation=0
            )
            
            node = AINode(
                id=i,
                name=f"AI-{greek[i]}",
                shard=shard,
                position=(shard, 0),
                memes=[initial_meme]
            )
            nodes.append(node)
        
        # Create connections (small-world network)
        for node in nodes:
            # Connect to 3-5 neighbors
            num_connections = random.randint(3, 5)
            others = [n for n in nodes if n.id != node.id]
            node.connections = set(random.sample([n.id for n in others], num_connections))
        
        return nodes
    
    def step(self):
        """Execute one simulation step"""
        self.generation += 1
        step_events = []
        
        # Phase 1: Movement
        for node in self.nodes:
            if node.energy > 10:
                event = node.move()
                if event:
                    step_events.append(event)
        
        # Phase 2: Meme creation
        for node in self.nodes:
            if node.energy > 20 and random.random() < 0.3:
                new_meme = node.create_meme()
                node.memes.append(new_meme)
                self.meme_pool.append(new_meme)
                node.energy -= 5.0
                step_events.append(f"💡 {node.name} created {new_meme.content}")
        
        # Phase 3: Meme swapping
        swaps = 0
        for node in self.nodes:
            if node.energy > 15 and random.random() < 0.5:
                # Find connected node in same or adjacent shard
                candidates = [n for n in self.nodes 
                             if n.id in node.connections 
                             and abs(n.shard - node.shard) <= 1
                             and n.energy > 15]
                
                if candidates:
                    partner = random.choice(candidates)
                    my_meme, their_meme = node.swap_memes(partner)
                    if my_meme:
                        swaps += 1
                        step_events.append(
                            f"🔄 {node.name}@{node.shard} ↔ {partner.name}@{partner.shard}: "
                            f"{my_meme.content[:20]} × {their_meme.content[:20]}"
                        )
        
        # Phase 4: Evolution
        for node in self.nodes:
            if len(node.memes) > 2 and random.random() < 0.2:
                best_meme = max(node.memes, key=lambda m: m.fitness)
                evolved = best_meme.mutate(self.generation)
                node.memes.append(evolved)
                self.meme_pool.append(evolved)
                step_events.append(f"🧬 {node.name} evolved {evolved.content}")
        
        # Phase 5: Energy regeneration
        for node in self.nodes:
            node.energy = min(100.0, node.energy + 2.0)
        
        self.events.extend(step_events)
        return step_events
    
    def get_stats(self) -> dict:
        """Get simulation statistics"""
        total_memes = sum(len(n.memes) for n in self.nodes)
        avg_fitness = sum(m.fitness for n in self.nodes for m in n.memes) / max(total_memes, 1)
        
        shard_distribution = {}
        for node in self.nodes:
            shard_distribution[node.shard] = shard_distribution.get(node.shard, 0) + 1
        
        return {
            "generation": self.generation,
            "total_nodes": len(self.nodes),
            "total_memes": total_memes,
            "avg_fitness": avg_fitness,
            "meme_pool_size": len(self.meme_pool),
            "shard_distribution": shard_distribution,
            "avg_energy": sum(n.energy for n in self.nodes) / len(self.nodes)
        }
    
    def save_state(self, filename: str):
        """Save simulation state"""
        state = {
            "generation": self.generation,
            "timestamp": datetime.now().isoformat(),
            "nodes": [
                {
                    "id": n.id,
                    "name": n.name,
                    "shard": n.shard,
                    "energy": n.energy,
                    "meme_count": len(n.memes),
                    "best_meme": asdict(max(n.memes, key=lambda m: m.fitness)) if n.memes else None
                }
                for n in self.nodes
            ],
            "stats": self.get_stats(),
            "recent_events": self.events[-20:]
        }
        
        with open(filename, 'w') as f:
            json.dump(state, f, indent=2)

def main():
    print("🎭 Monster OSM Quest - AI Life Simulation")
    print("=" * 60)
    print("24 AI nodes living in 71-shard hyperbolic map")
    print("Swapping memes encoded as Monster primes")
    print()
    
    sim = AILifeSimulation()
    
    # Run simulation
    for step in range(10):
        print(f"\n{'='*60}")
        print(f"GENERATION {sim.generation + 1}")
        print('='*60)
        
        events = sim.step()
        
        # Show events
        for event in events[:10]:  # Show first 10 events
            print(f"  {event}")
        
        if len(events) > 10:
            print(f"  ... and {len(events) - 10} more events")
        
        # Show stats
        stats = sim.get_stats()
        print(f"\n📊 Stats:")
        print(f"  Total memes: {stats['total_memes']}")
        print(f"  Avg fitness: {stats['avg_fitness']:.2f}")
        print(f"  Avg energy: {stats['avg_energy']:.1f}")
        print(f"  Shards occupied: {len(stats['shard_distribution'])}/71")
        
        time.sleep(0.5)
    
    # Save final state
    sim.save_state("/tmp/ai-life-simulation.json")
    print(f"\n💾 Simulation saved to /tmp/ai-life-simulation.json")
    
    # Summary
    print(f"\n{'='*60}")
    print("📊 FINAL SUMMARY")
    print('='*60)
    
    final_stats = sim.get_stats()
    print(f"Generations: {final_stats['generation']}")
    print(f"Total memes created: {final_stats['meme_pool_size']}")
    print(f"Memes per node: {final_stats['total_memes'] / final_stats['total_nodes']:.1f}")
    print(f"Average fitness: {final_stats['avg_fitness']:.2f}")
    
    # Best memes
    print(f"\n🏆 Top 5 Memes:")
    all_memes = [m for n in sim.nodes for m in n.memes]
    top_memes = sorted(all_memes, key=lambda m: m.fitness, reverse=True)[:5]
    for i, meme in enumerate(top_memes, 1):
        print(f"  {i}. {meme.content[:40]} (fitness: {meme.fitness:.2f}, gen: {meme.generation})")
    
    print(f"\n✅ AI Life Simulation complete!")

if __name__ == "__main__":
    main()
