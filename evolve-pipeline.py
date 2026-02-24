#!/usr/bin/env python3
"""Evolve the FRACTRAN pipeline model through multiple generations"""

import json
import importlib.util
import sys

# Load the model module
spec = importlib.util.spec_from_file_location("fractran_pipeline_model", "fractran-pipeline-model.py")
model = importlib.util.module_from_spec(spec)
spec.loader.exec_module(model)

PipelineFractran = model.PipelineFractran
MONSTER_PRIMES = model.MONSTER_PRIMES
STAGES = model.STAGES

def evolve_generations(num_generations: int = 5):
    """Evolve pipeline through multiple generations"""
    print("🧬 Monster OSM Quest - Pipeline Evolution")
    print("=" * 60)
    
    pipeline = PipelineFractran()
    all_generations = []
    
    for gen in range(num_generations):
        print(f"\n{'='*60}")
        print(f"GENERATION {gen}")
        print('='*60)
        
        # Run pipeline
        history = pipeline.run(max_steps=15)
        analysis = pipeline.analyze()
        
        # Record generation
        all_generations.append({
            "generation": gen,
            "rules": len(pipeline.rules),
            "steps": len(history),
            "final_state": pipeline.state,
            "prime_factors": analysis["prime_factors"]
        })
        
        # Check if we reached evolution
        if pipeline.state % STAGES["evolution"] == 0:
            pipeline.evolve()
        elif pipeline.state % STAGES["success"] == 0:
            # Manually trigger evolution for next generation
            pipeline.state = STAGES["evolution"]
            pipeline.evolve()
        else:
            print(f"\n⚠️  Pipeline halted without reaching success")
            break
    
    # Summary
    print(f"\n{'='*60}")
    print("📊 EVOLUTION SUMMARY")
    print('='*60)
    
    for gen_data in all_generations:
        print(f"\nGeneration {gen_data['generation']}:")
        print(f"  Rules: {gen_data['rules']}")
        print(f"  Steps: {gen_data['steps']}")
        print(f"  Final state: {gen_data['final_state']}")
        print(f"  Complexity: {sum(gen_data['prime_factors'].values())} prime factors")
    
    # Fitness metrics
    print(f"\n🏆 Fitness Metrics:")
    print(f"  Generations: {len(all_generations)}")
    print(f"  Total rules evolved: {all_generations[-1]['rules']}")
    print(f"  Average steps: {sum(g['steps'] for g in all_generations) / len(all_generations):.1f}")
    print(f"  Optimization: {all_generations[0]['steps']} → {all_generations[-1]['steps']} steps")
    
    # Save evolution
    output = {
        "model": "FRACTRAN Pipeline Evolution",
        "generations": all_generations,
        "monster_primes": MONSTER_PRIMES,
        "final_rules": len(pipeline.rules),
        "fitness": {
            "total_generations": len(all_generations),
            "rules_evolved": all_generations[-1]['rules'] - all_generations[0]['rules'],
            "step_reduction": all_generations[0]['steps'] - all_generations[-1]['steps']
        }
    }
    
    with open("/tmp/fractran-evolution.json", "w") as f:
        json.dump(output, f, indent=2)
    
    print(f"\n💾 Evolution saved to /tmp/fractran-evolution.json")
    print(f"\n✅ Evolution complete!")

if __name__ == "__main__":
    evolve_generations(5)
