# Task Planner System

## Overview

MiniZinc-based constraint solver for optimal task prioritization with dependency resolution.

## Components

### 1. MiniZinc Model (`proofs/task_planner.mzn`)
- **16 tasks** from quality management system
- **20 dependencies** enforcing execution order
- **Priority levels**: 10 (critical), 8-9 (high), 5-7 (medium), 4 (low)
- **Objective**: Minimize weighted completion time

### 2. Rust Planner Module (`src/planner.rs`)
- Loads optimized schedule from MiniZinc
- Provides execution order
- Tracks completed tasks
- Identifies critical path

### 3. Execution Scripts
- `plan_tasks.sh` - Generate optimized schedule
- `execute_plan.sh` - Execute tasks in order

## Task List

### Critical Path (Priority 10)
1. **IMPL_PARSE_DENSE** - Parse OSM PBF DenseNodes (5m)
2. **IMPL_SHARD_ASSIGN** - Gielis 71-fold assignment (3m)
3. **IMPL_COMPRESS** - 94× compression (8m)
4. **IMPL_DECOMPRESS** - Lossless decompression (5m)
5. **RUN_UNIT_TESTS** - Execute test suite (1m)
6. **FIX_FAILING_TESTS** - Iterate until 100% pass (10m)

### High Priority (8-9)
7. **IMPL_SYMMETRY** - Monster symmetry application (4m)
8. **IMPL_PIPELINE** - Main pipeline struct (6m)
9. **MEASURE_COVERAGE** - Target 70% minimum (2m)
10. **RUN_VALIDATE** - Pre-deployment validation (3m)

### Medium Priority (5-7)
11. **RUN_LINT** - Clippy + fmt (1m)
12. **RUN_AUDIT** - Generate audit trail (2m)
13. **INTEGRATE_MONSTER** - Connect to existing system (15m)
14. **PERF_BENCHMARK** - 1000 nodes/s target (10m)

### Low Priority (4-5)
15. **LOAD_TEST** - 1M+ nodes (20m)
16. **DEPLOY_SOLANA** - Production deployment (30m)

## Dependencies

```
IMPL_* → RUN_UNIT_TESTS → MEASURE_COVERAGE → FIX_FAILING_TESTS
                                                      ↓
                                              RUN_LINT, RUN_AUDIT
                                                      ↓
                                                RUN_VALIDATE
                                                      ↓
IMPL_PIPELINE → INTEGRATE_MONSTER → PERF_BENCHMARK → LOAD_TEST → DEPLOY_SOLANA
```

## Usage

```bash
# Generate optimal schedule
./plan_tasks.sh

# View schedule
cat proofs/task_schedule.json | jq

# In Rust
use osm_planet_torrent::planner::Schedule;

let schedule = Schedule::load()?;
schedule.print_plan();

let next = schedule.next_task(&completed)?;
println!("Execute: {}", next.task);
```

## Optimization

MiniZinc minimizes:
```
Σ (priority[t] × end_time[t])
```

Subject to:
- Dependency constraints
- Critical tasks start within 50 minutes
- No task overlap

## Output

```json
{
  "makespan": 125,
  "total_weighted_time": 8450,
  "schedule": [
    {
      "task": "IMPL_PARSE_DENSE",
      "start": 0,
      "end": 5,
      "duration": 5,
      "priority": 10
    },
    ...
  ]
}
```

## Integration with Quality System

- Loads proven constants from `proofs/model.json`
- Validates against Lean4 invariants
- Enforces Six Sigma quality gates
- Generates ITIL audit trail

**Status**: Task planner ready. Run `./plan_tasks.sh` to generate optimal schedule.
