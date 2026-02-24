#!/usr/bin/env python3
"""Simple dashboard viewer for AI life simulation"""

import sys
import time
import os

# Import from same directory
import importlib.util
spec = importlib.util.spec_from_file_location("ai_life_simulation", 
    "/home/mdupont/projects/osm-planet-torrent/ai-life-simulation.py")
ai_sim = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ai_sim)
AILifeSimulation = ai_sim.AILifeSimulation

def clear_screen():
    print('\033[2J\033[H', end='')

def create_dashboard(sim):
    """Create ASCII dashboard"""
    stats = sim.get_stats()
    
    lines = [
        "╔═══════════════════════════════════════════════════════════════════════╗",
        "║           🎭 MONSTER OSM QUEST - AI LIFE SIMULATION                  ║",
        "╚═══════════════════════════════════════════════════════════════════════╝",
        "",
        f"⏱  Generation: {stats['generation']}",
        f"🤖 Nodes: {stats['total_nodes']}",
        f"🧬 Total Memes: {stats['total_memes']}",
        f"⚡ Avg Energy: {stats['avg_energy']:.1f}",
        f"📊 Avg Fitness: {stats['avg_fitness']:.2f}",
        f"🌍 Shards Occupied: {len(stats['shard_distribution'])}/71",
        "",
        "┌─────────────────────────────────────────────────────────────────────┐",
        "│ SHARD MAP (● = nodes, 🏛=Cusp, 🧠=Consciousness, 🌙=Memory)         │",
        "└─────────────────────────────────────────────────────────────────────┘",
    ]
    
    # Shard visualization
    shard_dist = stats['shard_distribution']
    shard_line = ""
    for i in range(71):
        if i in shard_dist:
            count = shard_dist[i]
            if i == 17:
                shard_line += f"🏛"
            elif i == 23:
                shard_line += f"🧠"
            elif i == 59:
                shard_line += f"🌙"
            else:
                shard_line += f"●"
        else:
            shard_line += "·"
        
        if (i + 1) % 10 == 0:
            lines.append(f"  {shard_line}")
            shard_line = ""
    
    if shard_line:
        lines.append(f"  {shard_line}")
    
    lines.extend([
        "",
        "┌─────────────────────────────────────────────────────────────────────┐",
        "│ TOP 5 NODES                                                         │",
        "└─────────────────────────────────────────────────────────────────────┘",
    ])
    
    # Top nodes
    nodes_with_fitness = [
        (n, max(n.memes, key=lambda m: m.fitness).fitness if n.memes else 0)
        for n in sim.nodes
    ]
    top_nodes = sorted(nodes_with_fitness, key=lambda x: x[1], reverse=True)[:5]
    
    for node, fitness in top_nodes:
        lines.append(
            f"  {node.name:8} @shard{node.shard:2d} | "
            f"⚡{node.energy:5.1f} | 🧬{len(node.memes)} memes | "
            f"📊{fitness:.2f}"
        )
    
    lines.extend([
        "",
        "┌─────────────────────────────────────────────────────────────────────┐",
        "│ RECENT EVENTS                                                       │",
        "└─────────────────────────────────────────────────────────────────────┘",
    ])
    
    for event in sim.events[-8:]:
        lines.append(f"  {event[:69]}")
    
    lines.extend([
        "",
        "═══════════════════════════════════════════════════════════════════════",
    ])
    
    return "\n".join(lines)

def main():
    print("🎬 Starting AI Life Dashboard...")
    time.sleep(1)
    
    sim = AILifeSimulation()
    
    for step in range(20):
        clear_screen()
        
        # Run simulation step
        sim.step()
        
        # Display dashboard
        dashboard = create_dashboard(sim)
        print(dashboard)
        
        time.sleep(0.4)
    
    print("\n✅ Simulation complete!")
    print(f"📊 Final stats: {sim.get_stats()['total_memes']} memes, "
          f"fitness {sim.get_stats()['avg_fitness']:.2f}")

if __name__ == "__main__":
    main()
