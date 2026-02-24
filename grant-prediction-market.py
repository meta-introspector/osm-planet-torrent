#!/usr/bin/env python3
"""Grant Prediction Market - AI Life agents simulate grant economy with Lobster game"""

import json
import cbor2
from pathlib import Path
from typing import Dict, List
import hashlib

# Lobster prediction market (from CICADA-71)
class GrantMarket:
    def __init__(self, grant_id: str, amount: int):
        self.grant_id = grant_id
        self.amount = amount
        self.yes_stake = 0
        self.no_stake = 0
        self.resolved = False
        self.outcome = None
        self.positions = {}  # agent -> (yes, no)
    
    def bet_yes(self, agent: str, amount: int):
        """Agent bets YES on grant approval"""
        self.yes_stake += amount
        if agent not in self.positions:
            self.positions[agent] = [0, 0]
        self.positions[agent][0] += amount
    
    def bet_no(self, agent: str, amount: int):
        """Agent bets NO on grant approval"""
        self.no_stake += amount
        if agent not in self.positions:
            self.positions[agent] = [0, 0]
        self.positions[agent][1] += amount
    
    def resolve(self, approved: bool):
        """Resolve market with outcome"""
        self.resolved = True
        self.outcome = approved
    
    def claim_winnings(self, agent: str) -> int:
        """Calculate winnings for agent"""
        if not self.resolved or agent not in self.positions:
            return 0
        
        yes_pos, no_pos = self.positions[agent]
        total_pool = self.yes_stake + self.no_stake
        
        if self.outcome:  # YES won
            if self.yes_stake == 0:
                return 0
            return int((yes_pos * total_pool) / self.yes_stake)
        else:  # NO won
            if self.no_stake == 0:
                return 0
            return int((no_pos * total_pool) / self.no_stake)

# AI Life agents with prediction strategies
class GrantAgent:
    def __init__(self, name: str, shard: int, strategy: str):
        self.name = name
        self.shard = shard
        self.strategy = strategy
        self.tokens = 1000  # Starting MMC
        self.predictions = []
    
    def predict(self, grant: Dict) -> tuple:
        """Predict grant approval and bet amount"""
        # Strategy based on shard alignment
        if self.strategy == "optimistic":
            # Bet YES if shard matches
            confidence = 0.8 if grant["shard"] == self.shard else 0.3
            bet_yes = int(self.tokens * confidence * 0.1)
            return (True, bet_yes)
        
        elif self.strategy == "pessimistic":
            # Bet NO unless strong signals
            confidence = 0.7 if grant["issues"] < 10 else 0.2
            bet_no = int(self.tokens * confidence * 0.1)
            return (False, bet_no)
        
        elif self.strategy == "analytical":
            # Analyze FRACTRAN encoding
            if grant["fractran_encoding"] > 500:
                bet_yes = int(self.tokens * 0.15)
                return (True, bet_yes)
            else:
                bet_no = int(self.tokens * 0.1)
                return (False, bet_no)
        
        elif self.strategy == "momentum":
            # Follow money
            if grant["money_usd"] > 5000:
                bet_yes = int(self.tokens * 0.2)
                return (True, bet_yes)
            else:
                bet_no = int(self.tokens * 0.05)
                return (False, bet_no)
        
        return (True, 10)  # Default

def simulate_grant_economy(ontology: Dict, num_rounds: int = 5) -> Dict:
    """Simulate grant prediction market with AI Life agents"""
    
    # Create 4 agents (Monster Group shards)
    agents = [
        GrantAgent("α", 17, "optimistic"),    # Cusp
        GrantAgent("β", 23, "analytical"),    # Consciousness
        GrantAgent("γ", 59, "pessimistic"),   # Memory
        GrantAgent("δ", 71, "momentum")       # Omega
    ]
    
    # Get active grants
    active_grants = [g for g in ontology["grants"] if g["issues"] > 10][:20]
    
    markets = {}
    results = []
    
    print("🦞 LOBSTER PREDICTION MARKET - GRANT ECONOMY SIMULATION")
    print("=" * 70)
    print(f"Agents: {len(agents)}, Grants: {len(active_grants)}, Rounds: {num_rounds}\n")
    
    for round_num in range(num_rounds):
        print(f"🎲 ROUND {round_num + 1}")
        print("-" * 70)
        
        # Each agent predicts on random grants
        for grant in active_grants[:5]:  # Top 5 per round
            grant_id = grant["id"]
            
            # Create market if new
            if grant_id not in markets:
                markets[grant_id] = GrantMarket(grant_id, grant["money_usd"])
            
            market = markets[grant_id]
            
            # Each agent makes prediction
            for agent in agents:
                prediction, amount = agent.predict(grant)
                
                if amount > agent.tokens:
                    amount = agent.tokens // 2
                
                if prediction:
                    market.bet_yes(agent.name, amount)
                    agent.tokens -= amount
                    print(f"  {agent.name} bets YES {amount} MMC on {grant['name'][:30]}")
                else:
                    market.bet_no(agent.name, amount)
                    agent.tokens -= amount
                    print(f"  {agent.name} bets NO {amount} MMC on {grant['name'][:30]}")
                
                agent.predictions.append({
                    "round": round_num,
                    "grant": grant_id,
                    "prediction": prediction,
                    "amount": amount
                })
        
        print()
    
    # Resolve markets (simulate outcomes)
    print("🎯 RESOLVING MARKETS")
    print("-" * 70)
    
    for grant_id, market in markets.items():
        # Outcome based on yes/no ratio
        outcome = market.yes_stake > market.no_stake
        market.resolve(outcome)
        
        grant_name = next(g["name"] for g in active_grants if g["id"] == grant_id)
        print(f"  {grant_name[:40]}: {'APPROVED' if outcome else 'REJECTED'}")
        print(f"    YES: {market.yes_stake} MMC, NO: {market.no_stake} MMC")
    
    print()
    
    # Claim winnings
    print("💰 CLAIMING WINNINGS")
    print("-" * 70)
    
    for agent in agents:
        total_winnings = 0
        for grant_id, market in markets.items():
            winnings = market.claim_winnings(agent.name)
            total_winnings += winnings
        
        agent.tokens += total_winnings
        print(f"  {agent.name} ({agent.strategy}): {agent.tokens} MMC (Δ{total_winnings:+d})")
    
    # Calculate Gödel numbers for zkSNARK proofs
    print("\n🔮 GÖDEL ENCODING (zkSNARK PROOFS)")
    print("-" * 70)
    
    for agent in agents:
        # Gödel number from predictions
        godel_hash = hashlib.sha256(
            json.dumps(agent.predictions, sort_keys=True).encode()
        ).hexdigest()
        godel_mod = int(godel_hash[:8], 16) % 1_000_000
        
        print(f"  {agent.name}: Gödel={godel_mod}, Predictions={len(agent.predictions)}")
    
    return {
        "agents": [
            {
                "name": a.name,
                "shard": a.shard,
                "strategy": a.strategy,
                "final_tokens": a.tokens,
                "predictions": len(a.predictions)
            }
            for a in agents
        ],
        "markets": [
            {
                "grant_id": m.grant_id,
                "amount": m.amount,
                "yes_stake": m.yes_stake,
                "no_stake": m.no_stake,
                "outcome": m.outcome
            }
            for m in markets.values()
        ],
        "rounds": num_rounds
    }

def main():
    # Load ontology
    ontology = json.load(open('/mnt/data1/time-2026/02-february/24/GRANT_ONTOLOGY_20260224.json'))
    
    # Run simulation
    results = simulate_grant_economy(ontology, num_rounds=5)
    
    # Save results
    output = Path("/mnt/data1/time-2026/02-february/24/PREDICTION_MARKET_20260224.json")
    output.write_text(json.dumps(results, indent=2))
    
    cbor_output = Path("/mnt/data1/time-2026/02-february/24/PREDICTION_MARKET_20260224.cbor")
    cbor_output.write_bytes(cbor2.dumps(results))
    
    print(f"\n💾 Results saved:")
    print(f"   JSON: {output}")
    print(f"   CBOR: {cbor_output}")
    
    # Winner
    winner = max(results["agents"], key=lambda a: a["final_tokens"])
    print(f"\n🏆 WINNER: {winner['name']} ({winner['strategy']}) with {winner['final_tokens']} MMC")

if __name__ == "__main__":
    main()
