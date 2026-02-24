#!/usr/bin/env bash
# Build and run ZK witness with perf recording

set -e

echo "🔐 PURE NIX ZK WITNESS BUILDER"
echo ""

# Check if running in Nix environment
if [ -z "$IN_NIX_SHELL" ]; then
    echo "Building with Nix..."
    nix-build zk-witness.nix -o result-zk-witness
    
    echo ""
    echo "✅ Build complete!"
    echo ""
    echo "Available commands:"
    echo "  ./result-zk-witness/bin/black-hole-fall    - Run simulation"
    echo "  ./result-zk-witness/bin/generate-zk-witness - Generate ZK witness"
    echo "  ./result-zk-witness/bin/verify-zk-witness   - Verify ZK witness"
    echo "  ./result-zk-witness/bin/analyze-perf        - Analyze performance"
    echo ""
    echo "Quick start:"
    echo "  ./result-zk-witness/bin/generate-zk-witness"
else
    echo "Already in Nix shell"
fi

# If built, run the witness generator
if [ -d "result-zk-witness" ]; then
    echo "Running ZK witness generator..."
    echo ""
    ./result-zk-witness/bin/generate-zk-witness
    
    echo ""
    echo "Verifying witness..."
    echo ""
    ./result-zk-witness/bin/verify-zk-witness
    
    echo ""
    echo "Performance analysis:"
    echo ""
    ./result-zk-witness/bin/analyze-perf | head -50
fi
