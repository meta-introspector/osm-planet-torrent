#!/usr/bin/env python3
"""Unified Proof Planner - Pick optimal grant focus using FRACTRAN + Monster symmetries"""

import json
import cbor2
from pathlib import Path
from typing import Dict, List, Tuple

# Our proven capabilities (from checkpoint)
OUR_PROOFS = {
    "ai_life_uucp": {
        "shard": 23,  # Consciousness
        "proof_type": "multiplayer",
        "lines_of_code": 7400,
        "protocols": 71,
        "proven": True
    },
    "leech_lattice": {
        "shard": 17,  # Cusp
        "proof_type": "mathematical",
        "dimensions": 24,
        "repos_analyzed": 143,
        "proven": True
    },
    "nix_builds": {
        "shard": 59,  # Memory
        "proof_type": "reproducible",
        "packages": 5,
        "pure_build": True,
        "proven": True
    },
    "osm_performance": {
        "shard": 23,  # Consciousness
        "proof_type": "optimization",
        "cycle_reduction": 0.26,
        "cpu_reduction": 0.087,
        "proven": True
    },
    "dasl_cbor": {
        "shard": 71,  # Omega
        "proof_type": "encoding",
        "compression": 0.44,
        "grants_mapped": 143,
        "proven": True
    }
}

# FRACTRAN scoring program
def score_grant_match(grant: Dict, proof: Dict) -> int:
    """Score how well a grant matches our proof capabilities"""
    score = 0
    
    # Shard alignment (Monster symmetry)
    if grant["shard"] == proof["shard"]:
        score += 100
    
    # Issue count = opportunity
    score += min(grant["issues"], 100)
    
    # Money potential
    score += min(grant["money_usd"] // 100, 100)
    
    # FRACTRAN encoding alignment
    if grant["fractran_encoding"] > 500:
        score += 50
    
    return score

def analyze_grant_fit(grant: Dict) -> Dict:
    """Analyze which of our proofs best fits this grant"""
    fits = {}
    
    for proof_name, proof in OUR_PROOFS.items():
        score = score_grant_match(grant, proof)
        fits[proof_name] = score
    
    best_proof = max(fits, key=fits.get)
    
    return {
        "grant": grant["name"],
        "best_proof": best_proof,
        "score": fits[best_proof],
        "all_scores": fits,
        "shard": grant["shard"],
        "money": grant["money_usd"],
        "issues": grant["issues"]
    }

def plan_optimal_focus(ontology: Dict) -> Dict:
    """Use FRACTRAN + Monster symmetries to pick optimal grant focus"""
    
    # Analyze all grants
    analyses = []
    for grant in ontology["grants"]:
        if grant["issues"] > 0:  # Only active grants
            analysis = analyze_grant_fit(grant)
            analyses.append(analysis)
    
    # Sort by score
    analyses.sort(key=lambda x: x["score"], reverse=True)
    
    # Group by proof type
    by_proof = {}
    for analysis in analyses:
        proof = analysis["best_proof"]
        if proof not in by_proof:
            by_proof[proof] = []
        by_proof[proof].append(analysis)
    
    # Pick top 3 per proof type
    recommendations = {}
    for proof, grants in by_proof.items():
        recommendations[proof] = grants[:3]
    
    # Overall top 10
    top_10 = analyses[:10]
    
    # Calculate focus recommendation
    proof_scores = {}
    for proof in OUR_PROOFS:
        proof_scores[proof] = sum(
            a["score"] for a in analyses[:20] if a["best_proof"] == proof
        )
    
    optimal_focus = max(proof_scores, key=proof_scores.get)
    
    return {
        "optimal_focus": optimal_focus,
        "focus_score": proof_scores[optimal_focus],
        "proof_scores": proof_scores,
        "top_10_grants": top_10,
        "recommendations_by_proof": recommendations,
        "total_analyzed": len(analyses)
    }

def main():
    print("🎯 UNIFIED PROOF PLANNER")
    print("=" * 70)
    
    # Load ontology
    ontology_file = Path("/mnt/data1/time-2026/02-february/24/GRANT_ONTOLOGY_20260224.json")
    ontology = json.loads(ontology_file.read_text())
    
    print(f"📊 Analyzing {len(ontology['grants'])} grants against {len(OUR_PROOFS)} proven capabilities\n")
    
    # Plan optimal focus
    plan = plan_optimal_focus(ontology)
    
    print(f"🎯 OPTIMAL FOCUS: {plan['optimal_focus'].upper()}")
    print(f"   Score: {plan['focus_score']}")
    print()
    
    # Show proof scores
    print("📊 PROOF CAPABILITY SCORES\n")
    for proof, score in sorted(plan["proof_scores"].items(), key=lambda x: x[1], reverse=True):
        proof_info = OUR_PROOFS[proof]
        print(f"{proof}: {score}")
        print(f"  Shard: {proof_info['shard']}, Type: {proof_info['proof_type']}")
    print()
    
    # Show top 10 overall
    print("🏆 TOP 10 GRANT OPPORTUNITIES\n")
    for i, grant in enumerate(plan["top_10_grants"], 1):
        print(f"{i}. {grant['grant']} (Score: {grant['score']})")
        print(f"   Best fit: {grant['best_proof']}")
        print(f"   Money: ${grant['money']:,}, Issues: {grant['issues']}, Shard: {grant['shard']}")
    print()
    
    # Show recommendations by proof
    print("💡 RECOMMENDATIONS BY PROOF TYPE\n")
    for proof, grants in plan["recommendations_by_proof"].items():
        if grants:
            print(f"📌 {proof.upper()} ({OUR_PROOFS[proof]['proof_type']})")
            for g in grants:
                print(f"   • {g['grant']}: ${g['money']:,} ({g['issues']} issues)")
            print()
    
    # Save plan
    output = Path("/mnt/data1/time-2026/02-february/24/PROOF_PLAN_20260224.json")
    output.write_text(json.dumps(plan, indent=2))
    
    cbor_output = Path("/mnt/data1/time-2026/02-february/24/PROOF_PLAN_20260224.cbor")
    cbor_output.write_bytes(cbor2.dumps(plan))
    
    print(f"💾 Plan saved:")
    print(f"   JSON: {output}")
    print(f"   CBOR: {cbor_output}")
    
    # Action items
    print(f"\n✅ RECOMMENDED ACTIONS\n")
    optimal = plan["optimal_focus"]
    top_grants = plan["recommendations_by_proof"][optimal]
    
    print(f"1. Focus on: {optimal.upper()}")
    print(f"2. Target grants:")
    for g in top_grants:
        print(f"   • {g['grant']}")
    print(f"3. Leverage proof: {OUR_PROOFS[optimal]['proof_type']}")
    print(f"4. Monster shard: {OUR_PROOFS[optimal]['shard']}")

if __name__ == "__main__":
    main()
