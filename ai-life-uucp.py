#!/usr/bin/env python3
"""UUCP-based multiplayer AI agents playing and sharing state"""

import json
import base64
import time
import tempfile
from pathlib import Path
from datetime import datetime

# Add UUCP to protocols
UUCP_PROTOCOL = 71  # Extension beyond original 71

class UUCPMailbox:
    """Simulated UUCP mailbox for agent communication"""
    
    def __init__(self, agent_name: str, spool_dir: str = "/tmp/uucp-spool"):
        self.agent_name = agent_name
        self.spool_dir = Path(spool_dir)
        self.inbox = self.spool_dir / agent_name / "inbox"
        self.outbox = self.spool_dir / agent_name / "outbox"
        
        self.inbox.mkdir(parents=True, exist_ok=True)
        self.outbox.mkdir(parents=True, exist_ok=True)
    
    def send(self, to_agent: str, state: dict):
        """Send game state via UUCP"""
        timestamp = datetime.now().strftime("%Y%m%d%H%M%S")
        filename = f"C.{to_agent}.{timestamp}"
        
        # Encode state
        payload = {
            'from': self.agent_name,
            'to': to_agent,
            'timestamp': timestamp,
            'state': state
        }
        
        # Write to recipient's inbox
        recipient_inbox = self.spool_dir / to_agent / "inbox"
        recipient_inbox.mkdir(parents=True, exist_ok=True)
        
        msg_file = recipient_inbox / filename
        msg_file.write_text(json.dumps(payload))
        
        return filename
    
    def receive(self):
        """Receive game states from UUCP inbox"""
        messages = []
        for msg_file in self.inbox.glob("C.*"):
            try:
                payload = json.loads(msg_file.read_text())
                messages.append(payload)
                msg_file.unlink()  # Remove after reading
            except Exception as e:
                print(f"Error reading {msg_file}: {e}")
        return messages

class AIAgent:
    """AI agent that plays and shares via UUCP"""
    
    def __init__(self, agent_id: int, name: str):
        self.agent_id = agent_id
        self.name = name
        self.mailbox = UUCPMailbox(name)
        self.state = self.init_state()
        self.peers = []
    
    def init_state(self):
        """Initialize agent's game state"""
        return {
            'agent_id': self.agent_id,
            'agent_name': self.name,
            'shard': (self.agent_id * 3) % 71,
            'memes': [f"genesis_{self.name}"],
            'energy': 100.0,
            'generation': 0,
            'protocol': UUCP_PROTOCOL
        }
    
    def play_turn(self):
        """Execute one turn"""
        # Move
        self.state['shard'] = (self.state['shard'] + 1) % 71
        
        # Create meme
        if self.state['energy'] > 20 and len(self.state['memes']) < 5:
            new_meme = f"meme_{self.state['generation']}_{self.state['shard']}"
            self.state['memes'].append(new_meme)
            self.state['energy'] -= 5
        
        # Regen energy
        self.state['energy'] = min(100, self.state['energy'] + 2)
        self.state['generation'] += 1
        
        return self.state
    
    def share_state(self, peer_name: str):
        """Share state with peer via UUCP"""
        filename = self.mailbox.send(peer_name, self.state)
        return filename
    
    def receive_states(self):
        """Receive states from peers"""
        messages = self.mailbox.receive()
        
        for msg in messages:
            peer_state = msg['state']
            self.merge_state(peer_state)
        
        return messages
    
    def merge_state(self, peer_state: dict):
        """Merge peer's state with own"""
        # Learn from peer's memes
        for meme in peer_state['memes']:
            if meme not in self.state['memes'] and len(self.state['memes']) < 5:
                self.state['memes'].append(f"learned_{meme}")
                break

def run_uucp_game(num_agents=4, num_turns=5):
    """Run multiplayer game with UUCP sharing"""
    
    print("🎮 UUCP Multiplayer AI Life")
    print("=" * 70)
    print(f"Protocol: UUCP (71)")
    print(f"Agents: {num_agents}")
    print(f"Turns: {num_turns}")
    print()
    
    # Create agents
    greek = ["α", "β", "γ", "δ", "ε", "ζ", "η", "θ"]
    agents = [AIAgent(i, f"AI-{greek[i]}") for i in range(num_agents)]
    
    # Set up peer connections (ring topology)
    for i, agent in enumerate(agents):
        agent.peers = [agents[(i + 1) % num_agents].name]
    
    # Play game
    for turn in range(num_turns):
        print(f"\n{'='*70}")
        print(f"TURN {turn + 1}")
        print('='*70)
        
        # Each agent plays
        for agent in agents:
            agent.play_turn()
            print(f"  {agent.name}: shard {agent.state['shard']}, "
                  f"{len(agent.state['memes'])} memes, "
                  f"energy {agent.state['energy']:.0f}")
        
        # Share states via UUCP
        print(f"\n📨 UUCP Mail Transfer:")
        for agent in agents:
            for peer in agent.peers:
                filename = agent.share_state(peer)
                print(f"  {agent.name} → {peer}: {filename}")
        
        # Receive and merge
        time.sleep(0.1)  # Simulate network delay
        
        print(f"\n📬 Receiving Mail:")
        for agent in agents:
            messages = agent.receive_states()
            for msg in messages:
                print(f"  {agent.name} ← {msg['from']}: "
                      f"gen {msg['state']['generation']}, "
                      f"{len(msg['state']['memes'])} memes")
        
        time.sleep(0.5)
    
    # Final stats
    print(f"\n{'='*70}")
    print("FINAL STATS")
    print('='*70)
    
    for agent in agents:
        print(f"\n{agent.name}:")
        print(f"  Generation: {agent.state['generation']}")
        print(f"  Shard: {agent.state['shard']}")
        print(f"  Energy: {agent.state['energy']:.0f}")
        print(f"  Memes: {agent.state['memes']}")
    
    # Verify UUCP sharing worked
    print(f"\n{'='*70}")
    print("VERIFICATION")
    print('='*70)
    
    total_memes = sum(len(a.state['memes']) for a in agents)
    learned_memes = sum(1 for a in agents for m in a.state['memes'] if 'learned_' in m)
    
    print(f"✅ Total memes: {total_memes}")
    print(f"✅ Learned memes (via UUCP): {learned_memes}")
    print(f"✅ UUCP protocol: {UUCP_PROTOCOL}")
    print(f"✅ Agents communicated: {learned_memes > 0}")
    
    return agents

def create_uucp_share_url(state: dict) -> str:
    """Create UUCP-style share URL"""
    json_data = json.dumps(state)
    b64_data = base64.urlsafe_b64encode(json_data.encode()).decode().rstrip('=')
    
    return f"uucp://osm.monster!{state['agent_name']}/game/{state['generation']}?state={b64_data}"

def main():
    # Run game
    agents = run_uucp_game(num_agents=4, num_turns=5)
    
    # Generate share URLs
    print(f"\n{'='*70}")
    print("SHARE URLS (UUCP Protocol)")
    print('='*70)
    
    for agent in agents:
        url = create_uucp_share_url(agent.state)
        print(f"\n{agent.name}:")
        print(f"  {url[:80]}...")
    
    print(f"\n✅ UUCP multiplayer game complete!")
    print(f"   Agents played {agents[0].state['generation']} turns")
    print(f"   States shared via UUCP mailboxes")
    print(f"   Memes exchanged and learned")

if __name__ == "__main__":
    main()
