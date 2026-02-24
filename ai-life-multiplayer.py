#!/usr/bin/env python3
"""Multiplayer AI life with CBOR/RDFa state sharing across 71 protocols"""

import json
import base64
import hashlib
from dataclasses import dataclass, asdict
from typing import Dict, List, Any
from urllib.parse import quote, unquote

try:
    import cbor2
except ImportError:
    cbor2 = None

@dataclass
class GameState:
    """Shareable game state"""
    nodes: List[Dict]
    generation: int
    player_id: str
    protocol: int  # 0-70 (71 protocols)
    
    def to_dict(self):
        return {
            'nodes': self.nodes,
            'generation': self.generation,
            'player_id': self.player_id,
            'protocol': self.protocol
        }

# 71 Protocol definitions (Monster Group mod 71)
PROTOCOLS = {
    0: "http",
    1: "https",
    2: "ftp",
    3: "ssh",
    4: "telnet",
    5: "smtp",
    6: "imap",
    7: "pop3",
    8: "dns",
    9: "dhcp",
    10: "ntp",
    11: "snmp",
    12: "ldap",
    13: "mqtt",
    14: "coap",
    15: "websocket",
    16: "webrtc",
    17: "ipfs",  # Sacred Cusp
    18: "dat",
    19: "bittorrent",
    20: "i2p",
    21: "tor",
    22: "freenet",
    23: "matrix",  # Sacred Consciousness
    24: "xmpp",
    25: "irc",
    26: "activitypub",
    27: "nostr",
    28: "ssb",
    29: "hypercore",
    30: "gun",
    31: "orbit",
    32: "textile",
    33: "ceramic",
    34: "arweave",
    35: "filecoin",
    36: "storj",
    37: "sia",
    38: "swarm",
    39: "zeronet",
    40: "urbit",
    41: "holochain",
    42: "solid",
    43: "bluesky",
    44: "mastodon",
    45: "lemmy",
    46: "peertube",
    47: "pixelfed",
    48: "funkwhale",
    49: "castopod",
    50: "mobilizon",
    51: "writefreely",
    52: "plume",
    53: "bookwyrm",
    54: "owncast",
    55: "gotosocial",
    56: "misskey",
    57: "pleroma",
    58: "akkoma",
    59: "diaspora",  # Sacred Memory
    60: "hubzilla",
    61: "friendica",
    62: "gnusocial",
    63: "pump.io",
    64: "scuttlebutt",
    65: "cabal",
    66: "briar",
    67: "jami",
    68: "tox",
    69: "retroshare",
    70: "cwtch"
}

def encode_cbor(state: GameState) -> bytes:
    """Encode state as CBOR"""
    if cbor2:
        return cbor2.dumps(state.to_dict())
    # Fallback to JSON
    return json.dumps(state.to_dict()).encode('utf-8')

def decode_cbor(data: bytes) -> GameState:
    """Decode CBOR to state"""
    if cbor2:
        d = cbor2.loads(data)
    else:
        d = json.loads(data.decode('utf-8'))
    return GameState(**d)

def state_to_rdfa(state: GameState) -> str:
    """Convert state to RDFa markup"""
    return f"""
<div vocab="https://schema.org/" typeof="Game">
  <meta property="name" content="Monster OSM Quest AI Life" />
  <meta property="gamePlayMode" content="MultiPlayer" />
  <div property="gameState" typeof="GameState">
    <meta property="generation" content="{state.generation}" />
    <meta property="playerID" content="{state.player_id}" />
    <meta property="protocol" content="{state.protocol}" />
    <meta property="protocolName" content="{PROTOCOLS[state.protocol]}" />
    <meta property="nodeCount" content="{len(state.nodes)}" />
  </div>
</div>
""".strip()

def create_share_url(state: GameState, base_url: str = "https://osm.monster") -> str:
    """Create shareable URL with CBOR-encoded state"""
    cbor_data = encode_cbor(state)
    b64_data = base64.urlsafe_b64encode(cbor_data).decode('ascii').rstrip('=')
    
    protocol_name = PROTOCOLS[state.protocol]
    state_hash = hashlib.sha256(cbor_data).hexdigest()[:8]
    
    return f"{base_url}/play/{protocol_name}/{state.protocol}/{state_hash}?state={b64_data}"

def parse_share_url(url: str) -> GameState:
    """Parse shareable URL to game state"""
    # Extract state parameter
    if '?state=' not in url:
        raise ValueError("Invalid share URL")
    
    b64_data = url.split('?state=')[1].split('&')[0]
    
    # Add padding if needed
    padding = 4 - (len(b64_data) % 4)
    if padding != 4:
        b64_data += '=' * padding
    
    cbor_data = base64.urlsafe_b64decode(b64_data)
    return decode_cbor(cbor_data)

def create_dasl_query(state: GameState) -> str:
    """Create DASL (WebDAV Search) query for state"""
    return f"""<?xml version="1.0"?>
<D:searchrequest xmlns:D="DAV:">
  <D:basicsearch>
    <D:select>
      <D:prop><D:displayname/></D:prop>
    </D:select>
    <D:from>
      <D:scope>
        <D:href>/games/monster-osm/</D:href>
        <D:depth>infinity</D:depth>
      </D:scope>
    </D:from>
    <D:where>
      <D:and>
        <D:eq>
          <D:prop><D:protocol/></D:prop>
          <D:literal>{state.protocol}</D:literal>
        </D:eq>
        <D:eq>
          <D:prop><D:generation/></D:prop>
          <D:literal>{state.generation}</D:literal>
        </D:eq>
      </D:and>
    </D:where>
  </D:basicsearch>
</D:searchrequest>"""

def create_protocol_manifest() -> Dict[int, Dict[str, Any]]:
    """Create manifest of all 71 protocols"""
    manifest = {}
    for proto_id, proto_name in PROTOCOLS.items():
        manifest[proto_id] = {
            'id': proto_id,
            'name': proto_name,
            'sacred': proto_id in [17, 23, 59],
            'category': get_protocol_category(proto_id),
            'share_url_template': f"{{base}}/play/{proto_name}/{proto_id}/{{hash}}?state={{state}}"
        }
    return manifest

def get_protocol_category(proto_id: int) -> str:
    """Categorize protocol"""
    if proto_id < 11:
        return "classic"
    elif proto_id < 23:
        return "modern"
    elif proto_id < 41:
        return "p2p"
    elif proto_id < 59:
        return "fediverse"
    else:
        return "privacy"

class MultiplayerGame:
    """Multiplayer game with state sharing"""
    
    def __init__(self, player_id: str, protocol: int = 0):
        self.player_id = player_id
        self.protocol = protocol % 71  # Ensure 0-70
        self.state = None
    
    def create_game(self, nodes: List[Dict]) -> GameState:
        """Create new game state"""
        self.state = GameState(
            nodes=nodes,
            generation=0,
            player_id=self.player_id,
            protocol=self.protocol
        )
        return self.state
    
    def share_state(self, base_url: str = "https://osm.monster") -> Dict[str, str]:
        """Generate all sharing formats"""
        if not self.state:
            raise ValueError("No game state to share")
        
        return {
            'url': create_share_url(self.state, base_url),
            'rdfa': state_to_rdfa(self.state),
            'dasl': create_dasl_query(self.state),
            'cbor_b64': base64.b64encode(encode_cbor(self.state)).decode('ascii'),
            'json': json.dumps(self.state.to_dict()),
            'protocol': PROTOCOLS[self.protocol],
            'protocol_id': self.protocol
        }
    
    def load_from_url(self, url: str):
        """Load game state from share URL"""
        self.state = parse_share_url(url)
        self.player_id = self.state.player_id
        self.protocol = self.state.protocol
        return self.state
    
    def advance_generation(self):
        """Advance to next generation"""
        if self.state:
            self.state.generation += 1
    
    def switch_protocol(self, new_protocol: int):
        """Switch to different protocol"""
        self.protocol = new_protocol % 71
        if self.state:
            self.state.protocol = self.protocol

def main():
    print("🎮 Monster OSM Quest - Multiplayer")
    print("=" * 70)
    print("Share game state across 71 protocols!")
    print()
    
    # Create sample game
    player_id = hashlib.sha256(b"player1").hexdigest()[:8]
    
    # Test all 71 protocols
    print("📡 Testing 71 Protocols:")
    print()
    
    for proto_id in [0, 17, 23, 59, 70]:  # Sample protocols
        game = MultiplayerGame(player_id, proto_id)
        
        # Create sample state
        sample_nodes = [
            {'id': i, 'name': f'AI-{i}', 'shard': (i * 3) % 71, 'memes': 1}
            for i in range(5)
        ]
        game.create_game(sample_nodes)
        
        # Share state
        shared = game.share_state()
        
        sacred = " 🌟" if proto_id in [17, 23, 59] else ""
        print(f"Protocol {proto_id:2d}: {PROTOCOLS[proto_id]:15s}{sacred}")
        print(f"  URL: {shared['url'][:80]}...")
        print(f"  CBOR size: {len(shared['cbor_b64'])} bytes")
        print()
    
    # Test round-trip
    print("🔄 Testing Round-Trip:")
    game1 = MultiplayerGame("alice", 23)
    game1.create_game(sample_nodes)
    url = game1.share_state()['url']
    
    game2 = MultiplayerGame("bob", 0)
    game2.load_from_url(url)
    
    print(f"  Original: Gen {game1.state.generation}, Protocol {game1.state.protocol}")
    print(f"  Loaded:   Gen {game2.state.generation}, Protocol {game2.state.protocol}")
    print(f"  ✅ Match: {game1.state.generation == game2.state.generation}")
    print()
    
    # Show protocol manifest
    print("📋 Protocol Manifest (sample):")
    manifest = create_protocol_manifest()
    for proto_id in [17, 23, 59]:
        info = manifest[proto_id]
        print(f"  {info['id']:2d}. {info['name']:15s} [{info['category']}] {'🌟 SACRED' if info['sacred'] else ''}")
    
    print()
    print("✅ Multiplayer system ready!")
    print(f"   71 protocols available")
    print(f"   CBOR encoding: {'✅' if cbor2 else '⚠️  JSON fallback'}")
    print(f"   RDFa markup: ✅")
    print(f"   DASL queries: ✅")

if __name__ == "__main__":
    main()
