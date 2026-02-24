#!/usr/bin/env python3
"""DASL-CBOR Grant Ontology with Monster Group symmetries and FRACTRAN"""

import cbor2
import json
from pathlib import Path
from typing import Dict, List, Tuple

# FRACTRAN program for grant state transitions
GRANT_FRACTRAN = [
    (17, 91),   # proposed -> under_review (shard 17: Cusp)
    (23, 17),   # under_review -> funded (shard 23: Consciousness)
    (59, 23),   # funded -> completed (shard 59: Memory)
    (71, 59),   # completed -> archived (shard 71: Omega)
    (2, 71),    # archived -> proposed (cycle)
]

# Monster Group shard mapping (71 shards, 10-fold way)
MONSTER_SHARDS = {
    0: {"name": "Origin", "topology": "A", "grants": []},
    17: {"name": "Cusp", "topology": "AIII", "grants": []},      # Proposed
    23: {"name": "Consciousness", "topology": "AI", "grants": []}, # Funded
    59: {"name": "Memory", "topology": "BDI", "grants": []},     # Completed
    71: {"name": "Omega", "topology": "D", "grants": []}         # Archived
}

def fractran_step(n: int, program: List[Tuple[int, int]]) -> int:
    """Execute one FRACTRAN step"""
    for num, den in program:
        if n % den == 0:
            return (n // den) * num
    return n

def grant_to_shard(grant: Dict) -> int:
    """Map grant to Monster shard via FRACTRAN"""
    # Encode grant state as prime product
    state_primes = {
        "proposed": 91,
        "under_review": 17,
        "funded": 23,
        "in_progress": 23,
        "completed": 59,
        "archived": 71,
        "rejected": 2
    }
    
    state = grant.get("state", "proposed").lower()
    for key in state_primes:
        if key in state:
            return state_primes[key]
    return 91  # Default: proposed

def money_to_prime(amount: int) -> int:
    """Encode money amount as prime factorization"""
    if amount == 0:
        return 2
    
    # Use small primes for amounts
    primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71]
    
    # Encode thousands
    k = amount // 1000
    if k < len(primes):
        return primes[k]
    return 71  # Max shard

def create_grant_ontology(recon_data: Dict) -> Dict:
    """Create DASL-CBOR ontology from grant recon data"""
    
    ontology = {
        "@context": {
            "@vocab": "http://monster-osm.org/grant-ontology#",
            "dasl": "http://webdav.org/dasl/",
            "cbor": "http://cbor.io/",
            "monster": "http://monster-group.org/",
            "fractran": "http://fractran.org/"
        },
        "monster_shards": MONSTER_SHARDS.copy(),
        "fractran_program": GRANT_FRACTRAN,
        "grants": []
    }
    
    # Process each repo as a grant source
    for repo in recon_data.get("repos", []):
        if "error" in repo:
            continue
        
        # Extract grant info
        name = repo.get("name", "")
        issues = repo.get("issue_count", 0)
        
        # Determine state from activity
        if issues > 50:
            state = "funded"
        elif issues > 10:
            state = "under_review"
        elif issues > 0:
            state = "proposed"
        else:
            state = "archived"
        
        # Map to shard
        shard = grant_to_shard({"state": state})
        
        # Estimate money (issues as proxy)
        money = issues * 100  # $100 per issue estimate
        money_prime = money_to_prime(money)
        
        grant = {
            "id": repo.get("owner_repo", name),
            "name": name,
            "state": state,
            "shard": shard,
            "money_usd": money,
            "money_prime": money_prime,
            "issues": issues,
            "branches": repo.get("branch_count", 0),
            "fractran_encoding": shard * money_prime
        }
        
        ontology["grants"].append(grant)
        
        # Add to shard
        if shard in ontology["monster_shards"]:
            ontology["monster_shards"][shard]["grants"].append(grant["id"])
    
    # Compute shard statistics
    for shard_id, shard in ontology["monster_shards"].items():
        shard["grant_count"] = len(shard["grants"])
        shard["total_money"] = sum(
            g["money_usd"] for g in ontology["grants"] 
            if g["shard"] == shard_id
        )
    
    return ontology

def encode_cbor(ontology: Dict) -> bytes:
    """Encode ontology as CBOR"""
    return cbor2.dumps(ontology)

def decode_cbor(data: bytes) -> Dict:
    """Decode CBOR ontology"""
    return cbor2.loads(data)

def dasl_query(ontology: Dict, shard: int = None, min_money: int = 0) -> List[Dict]:
    """DASL-style query on ontology"""
    results = ontology["grants"]
    
    if shard is not None:
        results = [g for g in results if g["shard"] == shard]
    
    if min_money > 0:
        results = [g for g in results if g["money_usd"] >= min_money]
    
    return results

def main():
    print("🔮 DASL-CBOR GRANT ONTOLOGY")
    print("=" * 60)
    
    # Load recon data
    recon_file = Path("/mnt/data1/time-2026/02-february/24/RECON_COMPLETE_20260224_101537.json")
    recon_data = json.loads(recon_file.read_text())
    
    # Create ontology
    print("📊 Creating ontology from 136 repos...")
    ontology = create_grant_ontology(recon_data)
    
    # Encode as CBOR
    cbor_data = encode_cbor(ontology)
    
    # Save both formats
    output_dir = Path("/mnt/data1/time-2026/02-february/24")
    
    json_file = output_dir / "GRANT_ONTOLOGY_20260224.json"
    json_file.write_text(json.dumps(ontology, indent=2))
    
    cbor_file = output_dir / "GRANT_ONTOLOGY_20260224.cbor"
    cbor_file.write_bytes(cbor_data)
    
    print(f"✅ JSON: {json_file} ({json_file.stat().st_size} bytes)")
    print(f"✅ CBOR: {cbor_file} ({cbor_file.stat().st_size} bytes)")
    
    # Show shard distribution
    print(f"\n🌀 MONSTER SHARD DISTRIBUTION\n")
    for shard_id in [17, 23, 59, 71]:
        shard = ontology["monster_shards"][shard_id]
        print(f"Shard {shard_id} ({shard['name']}):")
        print(f"  Grants: {shard['grant_count']}")
        print(f"  Money: ${shard['total_money']:,}")
        print(f"  Topology: {shard['topology']}")
    
    # DASL queries
    print(f"\n🔍 DASL QUERIES\n")
    
    funded = dasl_query(ontology, shard=23)
    print(f"Funded grants (shard 23): {len(funded)}")
    
    big_money = dasl_query(ontology, min_money=5000)
    print(f"Grants >$5K: {len(big_money)}")
    
    # Show top 5 by FRACTRAN encoding
    top = sorted(ontology["grants"], key=lambda g: g["fractran_encoding"], reverse=True)[:5]
    print(f"\n💰 TOP 5 BY FRACTRAN ENCODING\n")
    for g in top:
        print(f"{g['name']}: ${g['money_usd']:,} (shard {g['shard']}, encoding {g['fractran_encoding']})")
    
    print(f"\n✅ Ontology complete: {len(ontology['grants'])} grants mapped to Monster symmetries")

if __name__ == "__main__":
    main()
