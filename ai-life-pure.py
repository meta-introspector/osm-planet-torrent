#!/usr/bin/env python3
"""Pure functional AI life - all constants as secret functions"""

from dataclasses import dataclass
from typing import Callable, List, Tuple, Any
import random

@dataclass
class State:
    """Pure state container"""
    data: dict
    
    def get(self, key: str, default=None):
        return self.data.get(key, default)
    
    def set(self, key: str, value):
        return State({**self.data, key: value})
    
    def update(self, updates: dict):
        return State({**self.data, **updates})

# Pure secret functions (no constants)
def make_secrets():
    """Generate secret functions - no hardcoded values"""
    return {
        'primes': lambda: [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71],
        'shard_count': lambda: 71,
        'node_count': lambda: 24,
        'sacred': lambda: {17: "Cusp", 23: "Consciousness", 59: "Memory"},
        'names': lambda: ["α", "β", "γ", "δ", "ε", "ζ", "η", "θ", "ι", "κ", "λ", "μ",
                         "ν", "ξ", "ο", "π", "ρ", "σ", "τ", "υ", "φ", "χ", "ψ", "ω"],
        'meme_types': lambda: ["fractran", "hyperbolic", "ramanujan", "monster", "topology"],
        'mutations': lambda: ["evolved", "enhanced", "optimized", "transformed", "amplified"],
        'energy_init': lambda: 100.0,
        'energy_move': lambda: 0.5,
        'energy_sacred': lambda: 10.0,
        'energy_create': lambda: 5.0,
        'energy_swap': lambda: 2.0,
        'energy_regen': lambda: 2.0,
        'fitness_init': lambda: 1.0,
        'fitness_mutation': lambda: (0.9, 1.2),
        'max_memes': lambda: 5,
        'connections': lambda: (3, 5),
    }

# Pure functions
def create_meme(secrets: dict, content: str, prime: int, fitness: float, gen: int, parent=None) -> dict:
    """Pure meme creation"""
    return {
        'id': random.randint(1000, 9999),
        'content': content,
        'prime': prime,
        'fitness': fitness,
        'generation': gen,
        'parent': parent
    }

def mutate_meme(secrets: dict, meme: dict, gen: int) -> dict:
    """Pure meme mutation"""
    primes = secrets['primes']()
    mutations = secrets['mutations']()
    shard_count = secrets['shard_count']()
    fitness_range = secrets['fitness_mutation']()
    
    new_prime = random.choice(primes)
    new_encoding = (meme['prime'] * new_prime) % (shard_count ** 3)
    new_content = f"{meme['content']}_{random.choice(mutations)}"
    new_fitness = meme['fitness'] * random.uniform(*fitness_range)
    
    return create_meme(secrets, new_content, new_encoding, new_fitness, gen, meme['id'])

def create_node(secrets: dict, node_id: int, shard: int) -> dict:
    """Pure node creation"""
    names = secrets['names']()
    primes = secrets['primes']()
    energy = secrets['energy_init']()
    fitness = secrets['fitness_init']()
    
    genesis_meme = create_meme(
        secrets,
        f"genesis_{names[node_id]}",
        primes[node_id % len(primes)],
        fitness,
        0
    )
    
    return {
        'id': node_id,
        'name': f"AI-{names[node_id]}",
        'shard': shard,
        'memes': [genesis_meme],
        'energy': energy,
        'generation': 0,
        'connections': set()
    }

def move_node(secrets: dict, node: dict) -> Tuple[dict, str]:
    """Pure node movement"""
    shard_count = secrets['shard_count']()
    energy_move = secrets['energy_move']()
    energy_sacred = secrets['energy_sacred']()
    sacred = secrets['sacred']()
    
    direction = random.choice([-1, 0, 1])
    new_shard = (node['shard'] + direction) % shard_count
    new_energy = node['energy'] - energy_move
    
    event = None
    if new_shard in sacred:
        new_energy += energy_sacred
        event = f"⚡ {node['name']} reached {sacred[new_shard]}!"
    
    return {**node, 'shard': new_shard, 'energy': new_energy}, event

def create_node_meme(secrets: dict, node: dict) -> Tuple[dict, str]:
    """Pure meme creation for node"""
    primes = secrets['primes']()
    meme_types = secrets['meme_types']()
    energy_create = secrets['energy_create']()
    fitness = secrets['fitness_init']()
    
    prime = primes[node['shard'] % len(primes)]
    content = f"{random.choice(meme_types)}_shard{node['shard']}"
    
    new_meme = create_meme(secrets, content, prime, random.uniform(0.5, 1.5), node['generation'])
    
    return (
        {**node, 'memes': node['memes'] + [new_meme], 'energy': node['energy'] - energy_create},
        f"💡 {node['name']} created {content}"
    )

def swap_memes(secrets: dict, node1: dict, node2: dict) -> Tuple[dict, dict, str]:
    """Pure meme swap"""
    if not node1['memes'] or not node2['memes']:
        return node1, node2, None
    
    max_memes = secrets['max_memes']()
    energy_swap = secrets['energy_swap']()
    shard_count = secrets['shard_count']()
    
    # Get best memes
    best1 = max(node1['memes'], key=lambda m: m['fitness'])
    best2 = max(node2['memes'], key=lambda m: m['fitness'])
    
    # Create hybrid
    hybrid_prime = (best1['prime'] * best2['prime']) % (shard_count ** 3)
    hybrid_fitness = (best1['fitness'] + best2['fitness']) / 2
    hybrid_gen = max(best1['generation'], best2['generation']) + 1
    
    hybrid = create_meme(
        secrets,
        f"{best1['content'][:20]}×{best2['content'][:20]}",
        hybrid_prime,
        hybrid_fitness,
        hybrid_gen,
        best1['id']
    )
    
    # Update memes
    memes1 = sorted(node1['memes'] + [best2, hybrid], key=lambda m: m['fitness'], reverse=True)[:max_memes]
    memes2 = sorted(node2['memes'] + [best1, hybrid], key=lambda m: m['fitness'], reverse=True)[:max_memes]
    
    new_node1 = {**node1, 'memes': memes1, 'energy': node1['energy'] - energy_swap}
    new_node2 = {**node2, 'memes': memes2, 'energy': node2['energy'] - energy_swap}
    
    event = f"🔄 {node1['name']}@{node1['shard']} ↔ {node2['name']}@{node2['shard']}: {best1['content'][:20]} × {best2['content'][:20]}"
    
    return new_node1, new_node2, event

def evolve_node(secrets: dict, node: dict) -> Tuple[dict, str]:
    """Pure node evolution"""
    if len(node['memes']) < 2:
        return node, None
    
    best = max(node['memes'], key=lambda m: m['fitness'])
    evolved = mutate_meme(secrets, best, node['generation'])
    
    return (
        {**node, 'memes': node['memes'] + [evolved]},
        f"🧬 {node['name']} evolved {evolved['content']}"
    )

def regen_energy(secrets: dict, node: dict) -> dict:
    """Pure energy regeneration"""
    energy_regen = secrets['energy_regen']()
    energy_init = secrets['energy_init']()
    return {**node, 'energy': min(energy_init, node['energy'] + energy_regen)}

def init_world(secrets: dict) -> State:
    """Pure world initialization"""
    node_count = secrets['node_count']()
    shard_count = secrets['shard_count']()
    conn_range = secrets['connections']()
    
    # Create nodes
    nodes = [create_node(secrets, i, (i * 3) % shard_count) for i in range(node_count)]
    
    # Create connections
    for node in nodes:
        num_conn = random.randint(*conn_range)
        others = [n['id'] for n in nodes if n['id'] != node['id']]
        node['connections'] = set(random.sample(others, num_conn))
    
    return State({
        'nodes': nodes,
        'generation': 0,
        'events': [],
        'meme_pool': []
    })

def step_world(secrets: dict, state: State) -> State:
    """Pure world step"""
    nodes = state.get('nodes')
    events = []
    
    # Phase 1: Movement
    new_nodes = []
    for node in nodes:
        if node['energy'] > 10:
            node, event = move_node(secrets, node)
            if event:
                events.append(event)
        new_nodes.append(node)
    nodes = new_nodes
    
    # Phase 2: Meme creation
    new_nodes = []
    for node in nodes:
        if node['energy'] > 20 and random.random() < 0.3:
            node, event = create_node_meme(secrets, node)
            events.append(event)
        new_nodes.append(node)
    nodes = new_nodes
    
    # Phase 3: Meme swapping
    swapped = set()
    new_nodes = list(nodes)
    for i, node in enumerate(nodes):
        if i in swapped or node['energy'] <= 15 or random.random() >= 0.5:
            continue
        
        candidates = [j for j, n in enumerate(nodes) 
                     if n['id'] in node['connections']
                     and abs(n['shard'] - node['shard']) <= 1
                     and n['energy'] > 15
                     and j not in swapped]
        
        if candidates:
            j = random.choice(candidates)
            new_nodes[i], new_nodes[j], event = swap_memes(secrets, new_nodes[i], new_nodes[j])
            if event:
                events.append(event)
            swapped.add(i)
            swapped.add(j)
    nodes = new_nodes
    
    # Phase 4: Evolution
    new_nodes = []
    for node in nodes:
        if len(node['memes']) > 2 and random.random() < 0.2:
            node, event = evolve_node(secrets, node)
            if event:
                events.append(event)
        new_nodes.append(node)
    nodes = new_nodes
    
    # Phase 5: Energy regen
    nodes = [regen_energy(secrets, n) for n in nodes]
    
    return state.update({
        'nodes': nodes,
        'generation': state.get('generation') + 1,
        'events': state.get('events') + events
    })

def get_stats(secrets: dict, state: State) -> dict:
    """Pure statistics"""
    nodes = state.get('nodes')
    total_memes = sum(len(n['memes']) for n in nodes)
    avg_fitness = sum(m['fitness'] for n in nodes for m in n['memes']) / max(total_memes, 1)
    avg_energy = sum(n['energy'] for n in nodes) / len(nodes)
    
    shard_dist = {}
    for node in nodes:
        shard_dist[node['shard']] = shard_dist.get(node['shard'], 0) + 1
    
    return {
        'generation': state.get('generation'),
        'total_nodes': len(nodes),
        'total_memes': total_memes,
        'avg_fitness': avg_fitness,
        'avg_energy': avg_energy,
        'shard_distribution': shard_dist
    }

def main():
    print("🎭 Pure Functional AI Life")
    print("=" * 60)
    
    # Create secrets (no hardcoded constants)
    secrets = make_secrets()
    
    # Initialize world
    state = init_world(secrets)
    
    # Run simulation
    for _ in range(10):
        state = step_world(secrets, state)
        
        stats = get_stats(secrets, state)
        events = state.get('events')[-10:]
        
        print(f"\n{'='*60}")
        print(f"GENERATION {stats['generation']}")
        print('='*60)
        
        for event in events:
            print(f"  {event}")
        
        print(f"\n📊 Stats:")
        print(f"  Total memes: {stats['total_memes']}")
        print(f"  Avg fitness: {stats['avg_fitness']:.2f}")
        print(f"  Avg energy: {stats['avg_energy']:.1f}")
    
    print(f"\n✅ Pure functional simulation complete!")

if __name__ == "__main__":
    main()
