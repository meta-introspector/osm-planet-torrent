% MiniZinc: Task Planner with Dependency Resolution
% Prioritize OSM Monster quality management tasks

include "globals.mzn";

% Task definitions
enum TASK = {
  IMPL_PARSE_DENSE,      % Implement parse_dense_nodes()
  IMPL_SHARD_ASSIGN,     % Implement assign_gielis_shard()
  IMPL_COMPRESS,         % Implement compress_emoji_cube()
  IMPL_DECOMPRESS,       % Implement decompress_emoji_cube()
  IMPL_SYMMETRY,         % Implement apply_monster_symmetry()
  IMPL_PIPELINE,         % Implement MonsterPipeline struct
  RUN_UNIT_TESTS,        % Run make test
  MEASURE_COVERAGE,      % Run make coverage (70% target)
  FIX_FAILING_TESTS,     % Fix tests until 100% pass
  RUN_LINT,              % Run make lint
  RUN_AUDIT,             % Generate audit trail
  RUN_VALIDATE,          % Pre-deployment validation
  INTEGRATE_MONSTER,     % Connect to existing Monster compression
  PERF_BENCHMARK,        % Performance benchmarking
  LOAD_TEST,             % Load testing (1M+ nodes)
  DEPLOY_SOLANA          % Deploy to Solana tile service
};

int: num_tasks = card(TASK);

% Task properties
array[TASK] of int: duration = [
  5,   % IMPL_PARSE_DENSE (5 min)
  3,   % IMPL_SHARD_ASSIGN (3 min)
  8,   % IMPL_COMPRESS (8 min)
  5,   % IMPL_DECOMPRESS (5 min)
  4,   % IMPL_SYMMETRY (4 min)
  6,   % IMPL_PIPELINE (6 min)
  1,   % RUN_UNIT_TESTS (1 min)
  2,   % MEASURE_COVERAGE (2 min)
  10,  % FIX_FAILING_TESTS (10 min)
  1,   % RUN_LINT (1 min)
  2,   % RUN_AUDIT (2 min)
  3,   % RUN_VALIDATE (3 min)
  15,  % INTEGRATE_MONSTER (15 min)
  10,  % PERF_BENCHMARK (10 min)
  20,  % LOAD_TEST (20 min)
  30   % DEPLOY_SOLANA (30 min)
];

array[TASK] of int: priority = [
  10,  % IMPL_PARSE_DENSE (critical)
  10,  % IMPL_SHARD_ASSIGN (critical)
  10,  % IMPL_COMPRESS (critical)
  10,  % IMPL_DECOMPRESS (critical)
  8,   % IMPL_SYMMETRY (high)
  9,   % IMPL_PIPELINE (high)
  10,  % RUN_UNIT_TESTS (critical)
  9,   % MEASURE_COVERAGE (high)
  10,  % FIX_FAILING_TESTS (critical)
  7,   % RUN_LINT (medium)
  6,   % RUN_AUDIT (medium)
  8,   % RUN_VALIDATE (high)
  7,   % INTEGRATE_MONSTER (medium)
  6,   % PERF_BENCHMARK (medium)
  5,   % LOAD_TEST (low)
  4    % DEPLOY_SOLANA (low)
];

% Dependencies (task X must complete before task Y)
set of int: DEPS = 1..20;
array[DEPS] of TASK: dep_from = [
  IMPL_PARSE_DENSE, IMPL_SHARD_ASSIGN, IMPL_COMPRESS, IMPL_DECOMPRESS, IMPL_SYMMETRY,
  IMPL_PIPELINE, IMPL_PIPELINE, IMPL_PIPELINE, IMPL_PIPELINE, IMPL_PIPELINE,
  RUN_UNIT_TESTS, MEASURE_COVERAGE, FIX_FAILING_TESTS, FIX_FAILING_TESTS, FIX_FAILING_TESTS,
  RUN_VALIDATE, RUN_VALIDATE, INTEGRATE_MONSTER, PERF_BENCHMARK, LOAD_TEST
];

array[DEPS] of TASK: dep_to = [
  RUN_UNIT_TESTS, RUN_UNIT_TESTS, RUN_UNIT_TESTS, RUN_UNIT_TESTS, RUN_UNIT_TESTS,
  RUN_UNIT_TESTS, INTEGRATE_MONSTER, PERF_BENCHMARK, LOAD_TEST, DEPLOY_SOLANA,
  MEASURE_COVERAGE, FIX_FAILING_TESTS, RUN_LINT, RUN_AUDIT, RUN_VALIDATE,
  INTEGRATE_MONSTER, DEPLOY_SOLANA, PERF_BENCHMARK, LOAD_TEST, DEPLOY_SOLANA
];

% Decision variables: start time for each task
array[TASK] of var 0..200: start_time;
array[TASK] of var 0..200: end_time;

% Constraint: end_time = start_time + duration
constraint forall(t in TASK) (
  end_time[t] = start_time[t] + duration[t]
);

% Constraint: dependencies must be satisfied
constraint forall(d in DEPS) (
  end_time[dep_from[d]] <= start_time[dep_to[d]]
);

% Constraint: critical path tasks start ASAP
constraint forall(t in TASK where priority[t] = 10) (
  start_time[t] <= 50  % Must start within first 50 minutes
);

% Objective: minimize weighted completion time
var int: total_weighted_time = sum(t in TASK)(
  priority[t] * end_time[t]
);

var int: makespan = max(t in TASK)(end_time[t]);

solve minimize total_weighted_time;

% Output: execution plan
output [
  "{\n",
  "  \"makespan\": ", show(makespan), ",\n",
  "  \"total_weighted_time\": ", show(total_weighted_time), ",\n",
  "  \"schedule\": [\n"
] ++
[
  "    {\n",
  "      \"task\": \"", show(t), "\",\n",
  "      \"start\": ", show(start_time[t]), ",\n",
  "      \"end\": ", show(end_time[t]), ",\n",
  "      \"duration\": ", show(duration[t]), ",\n",
  "      \"priority\": ", show(priority[t]), "\n",
  "    }",
  if t != DEPLOY_SOLANA then ",\n" else "\n" endif
  | t in TASK
] ++
[
  "  ]\n",
  "}\n"
];
