#!/usr/bin/env bash
# Build and verify proven model

set -e

echo "🔍 Step 1: Verify Lean4 proofs..."
cd proofs
lean osm_invariants.lean

echo "✓ Lean4 proofs verified"

echo ""
echo "🔍 Step 2: Generate model from Lean4..."
lean --run osm_invariants.lean > model.json

echo "✓ Model generated: proofs/model.json"

echo ""
echo "🔍 Step 3: Solve MiniZinc constraints..."
minizinc osm_constraints.mzn osm_data.dzn > minizinc_solution.json

echo "✓ MiniZinc solution: proofs/minizinc_solution.json"

echo ""
echo "🔍 Step 4: Validate model in Rust..."
cd ..
cargo test --lib model::tests

echo "✓ Model validated in Rust"

echo ""
echo "✅ All proofs verified, model ready to use!"
echo ""
echo "Model location: proofs/model.json"
echo "Usage in Rust: use crate::model::MODEL;"
