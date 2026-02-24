#!/usr/bin/env python3
"""Record AI Life episodes with dashboard and asciinema"""

import json
import subprocess
import time
from datetime import datetime
from pathlib import Path

class AIEpisodeRecorder:
    """Record AI life simulation episodes"""
    
    def __init__(self, output_dir="/tmp/ai-episodes"):
        self.output_dir = Path(output_dir)
        self.output_dir.mkdir(exist_ok=True)
        self.episode_num = 0
    
    def create_dashboard(self, sim_state: dict) -> str:
        """Create ASCII dashboard"""
        stats = sim_state['stats']
        nodes = sim_state['nodes']
        events = sim_state.get('recent_events', [])
        
        # Header
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
            "",
            "┌─────────────────────────────────────────────────────────────────────┐",
            "│ SHARD MAP (71 shards, 24 nodes)                                    │",
            "└─────────────────────────────────────────────────────────────────────┘",
        ]
        
        # Shard distribution
        shard_dist = stats.get('shard_distribution', {})
        shard_line = ""
        for i in range(71):
            if i in shard_dist:
                count = shard_dist[i]
                if i == 17:
                    shard_line += f"🏛{count}"
                elif i == 23:
                    shard_line += f"🧠{count}"
                elif i == 59:
                    shard_line += f"🌙{count}"
                else:
                    shard_line += f"●{count}"
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
            "│ TOP NODES                                                           │",
            "└─────────────────────────────────────────────────────────────────────┘",
        ])
        
        # Top nodes by meme count
        top_nodes = sorted(nodes, key=lambda n: n['meme_count'], reverse=True)[:5]
        for node in top_nodes:
            best = node.get('best_meme', {})
            fitness = best.get('fitness', 0) if best else 0
            lines.append(
                f"  {node['name']:8} @shard{node['shard']:2d} | "
                f"⚡{node['energy']:5.1f} | 🧬{node['meme_count']} memes | "
                f"📊{fitness:.2f}"
            )
        
        lines.extend([
            "",
            "┌─────────────────────────────────────────────────────────────────────┐",
            "│ RECENT EVENTS                                                       │",
            "└─────────────────────────────────────────────────────────────────────┘",
        ])
        
        for event in events[-8:]:
            lines.append(f"  {event[:70]}")
        
        lines.extend([
            "",
            "═══════════════════════════════════════════════════════════════════════",
        ])
        
        return "\n".join(lines)
    
    def record_episode(self, generations=20, cast_file=None):
        """Record an episode"""
        self.episode_num += 1
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        
        if cast_file is None:
            cast_file = self.output_dir / f"episode_{self.episode_num}_{timestamp}.cast"
        
        print(f"🎬 Recording Episode {self.episode_num}")
        print(f"📹 Output: {cast_file}")
        
        # Start asciinema recording
        script = f"""
import sys
import time
sys.path.insert(0, '/home/mdupont/projects/osm-planet-torrent')
from ai_life_simulation import AILifeSimulation

sim = AILifeSimulation()

for step in range({generations}):
    events = sim.step()
    
    # Create dashboard
    state = {{
        'generation': sim.generation,
        'nodes': [
            {{
                'id': n.id,
                'name': n.name,
                'shard': n.shard,
                'energy': n.energy,
                'meme_count': len(n.memes),
                'best_meme': {{
                    'fitness': max(n.memes, key=lambda m: m.fitness).fitness
                }} if n.memes else None
            }}
            for n in sim.nodes
        ],
        'stats': sim.get_stats(),
        'recent_events': sim.events[-8:]
    }}
    
    # Import recorder
    from record_ai_episodes import AIEpisodeRecorder
    recorder = AIEpisodeRecorder()
    dashboard = recorder.create_dashboard(state)
    
    print('\\033[2J\\033[H')  # Clear screen
    print(dashboard)
    time.sleep(0.3)

print('\\n✅ Episode complete!')
"""
        
        # Write temp script
        script_file = self.output_dir / f"temp_episode_{self.episode_num}.py"
        script_file.write_text(script)
        
        # Record with asciinema
        cmd = [
            "asciinema", "rec",
            "-c", f"python3 {script_file}",
            "--overwrite",
            str(cast_file)
        ]
        
        subprocess.run(cmd)
        
        # Cleanup
        script_file.unlink()
        
        print(f"✅ Episode {self.episode_num} recorded!")
        print(f"📁 {cast_file}")
        
        return cast_file
    
    def upload_episode(self, cast_file: Path):
        """Upload to asciinema.org"""
        print(f"📤 Uploading {cast_file.name}...")
        
        result = subprocess.run(
            ["asciinema", "upload", str(cast_file)],
            capture_output=True,
            text=True
        )
        
        # Extract URL
        for line in result.stdout.split('\n'):
            if 'asciinema.org' in line:
                url = line.strip()
                print(f"✅ Uploaded: {url}")
                return url
        
        print("⚠️  Upload may have succeeded, check asciinema.org")
        return None

def main():
    recorder = AIEpisodeRecorder()
    
    # Record 3 episodes
    episodes = []
    for i in range(3):
        print(f"\n{'='*70}")
        print(f"EPISODE {i+1}/3")
        print('='*70)
        
        cast_file = recorder.record_episode(generations=15)
        episodes.append(cast_file)
        
        time.sleep(1)
    
    # Upload all
    print(f"\n{'='*70}")
    print("UPLOADING EPISODES")
    print('='*70)
    
    urls = []
    for cast_file in episodes:
        url = recorder.upload_episode(cast_file)
        if url:
            urls.append(url)
    
    # Save manifest
    manifest = {
        "timestamp": datetime.now().isoformat(),
        "episodes": [
            {
                "number": i+1,
                "file": str(cast_file),
                "url": url
            }
            for i, (cast_file, url) in enumerate(zip(episodes, urls))
        ]
    }
    
    manifest_file = recorder.output_dir / "episodes_manifest.json"
    manifest_file.write_text(json.dumps(manifest, indent=2))
    
    print(f"\n✅ All episodes recorded and uploaded!")
    print(f"📋 Manifest: {manifest_file}")

if __name__ == "__main__":
    main()
