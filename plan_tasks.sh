#!/usr/bin/env bash
# Generate and display optimized task plan

set -e

echo "🔍 Solving task prioritization with MiniZinc..."
cd /home/mdupont/projects/osm-planet-torrent

minizinc proofs/task_planner.mzn --output-mode json > proofs/task_schedule.json

echo "✓ Schedule generated: proofs/task_schedule.json"
echo ""

# Pretty print schedule
echo "📋 Optimized Execution Plan:"
echo ""

jq -r '
  "Makespan: \(.makespan) minutes",
  "Total weighted time: \(.total_weighted_time)",
  "",
  "Tasks (in execution order):",
  (.schedule | sort_by(.start) | .[] | 
    "\(if .priority == 10 then "🔴" elif .priority >= 8 then "🟡" else "🟢" end) [\(.start)..\(.end)] \(.task) (\(.duration)m, priority \(.priority))"
  )
' proofs/task_schedule.json

echo ""
echo "✅ Use this schedule to execute tasks in optimal order"
