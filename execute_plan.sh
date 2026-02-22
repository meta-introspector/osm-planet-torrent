#!/usr/bin/env bash
# Execute tasks in MiniZinc-optimized order

set -e

cd /home/mdupont/projects/osm-planet-torrent

# Generate plan
./plan_tasks.sh

echo ""
echo "🚀 Ready to execute tasks in optimal order"
echo ""
echo "Next steps:"
echo "1. Implement critical functions (priority 10)"
echo "2. Run unit tests"
echo "3. Measure coverage"
echo "4. Fix failing tests"
echo "5. Validate and deploy"
echo ""
echo "Use: cargo run --bin execute_plan"
