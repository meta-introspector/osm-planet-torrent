#!/usr/bin/env bash
# Run Jocko OODA loop

set -e

cd /home/mdupont/projects/osm-planet-torrent

echo "🎖️  JOCKO OODA LOOP"
echo "Discipline equals freedom."
echo ""

echo "🔍 Generating optimal OODA cycle plan..."
minizinc proofs/ooda_loop.mzn proofs/ooda_data.dzn --output-mode json > proofs/ooda_plan.json

echo "✓ OODA plan generated"
echo ""

# Display plan
jq -r '
  "Cycles: \(.num_cycles)",
  "Cycle time: \(.cycle_time)s",
  "Total time: \(.total_time)s",
  "Total quality: \(.total_quality)",
  "Total risk: \(.total_risk)",
  "",
  "OODA Cycles:",
  (.cycles[] | 
    "🔄 Cycle \(.cycle): \(.observe) → \(.orient) → \(.decide) → \(.act) (Q:\(.quality) R:\(.risk))"
  )
' proofs/ooda_plan.json

echo ""
echo "✅ OODA loop ready. Execute with: cargo run --bin ooda_loop"
