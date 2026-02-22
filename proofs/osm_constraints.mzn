% MiniZinc: OSM Monster System Constraints
% Optimize shard assignment and validate invariants

include "globals.mzn";

% Constants from Lean4 proofs
array[1..17] of int: MONSTER_PRIMES = [2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,59,71];
int: NUM_SHARDS = 71;
int: EMOJI_CUBE_SIZE = 13824;  % 24³
int: COMPRESSED_SIZE = 150;
int: COMPRESSION_RATIO = 94;

% Performance constraints
float: MALLOC_MAX = 1.0;
int: THROUGHPUT_MIN = 1000;
int: LATENCY_MAX = 100;

% Coordinate bounds
float: LAT_MIN = -90.0;
float: LAT_MAX = 90.0;
float: LON_MIN = -180.0;
float: LON_MAX = 180.0;

% Decision variables: Shard assignment
int: num_nodes;  % Input parameter
array[1..num_nodes] of var 0..NUM_SHARDS-1: shard_assignment;

% Constraint: Uniform distribution
constraint forall(s in 0..NUM_SHARDS-1) (
  let {
    var int: count = sum(i in 1..num_nodes)(shard_assignment[i] = s);
    int: expected = num_nodes div NUM_SHARDS;
    int: tolerance = expected div 10;  % 10% tolerance
  } in
    count >= expected - tolerance /\
    count <= expected + tolerance
);

% Constraint: All shards used
constraint forall(s in 0..NUM_SHARDS-1) (
  exists(i in 1..num_nodes)(shard_assignment[i] = s)
);

% Objective: Minimize variance in shard sizes
var int: max_shard_size;
var int: min_shard_size;

constraint max_shard_size = max([
  sum(i in 1..num_nodes)(shard_assignment[i] = s) | s in 0..NUM_SHARDS-1
]);

constraint min_shard_size = min([
  sum(i in 1..num_nodes)(shard_assignment[i] = s) | s in 0..NUM_SHARDS-1
]);

var int: variance = max_shard_size - min_shard_size;

solve minimize variance;

% Output model
output [
  "{\n",
  "  \"num_shards\": ", show(NUM_SHARDS), ",\n",
  "  \"num_nodes\": ", show(num_nodes), ",\n",
  "  \"variance\": ", show(variance), ",\n",
  "  \"max_shard_size\": ", show(max_shard_size), ",\n",
  "  \"min_shard_size\": ", show(min_shard_size), ",\n",
  "  \"distribution\": [\n"
] ++
[
  "    {\"shard\": ", show(s), 
  ", \"count\": ", show(sum(i in 1..num_nodes)(shard_assignment[i] = s)), "}",
  if s < NUM_SHARDS-1 then ",\n" else "\n" endif
  | s in 0..NUM_SHARDS-1
] ++
[
  "  ]\n",
  "}\n"
];
