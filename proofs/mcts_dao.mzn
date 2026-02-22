% MiniZinc: MCTS with Thinker-Prover Duality
% Value model (Thinker) + Policy model (Prover) = Optimal search

include "globals.mzn";

% MCTS parameters
float: EXPLORATION_C = 1.414;  % sqrt(2)
int: MAX_DEPTH = 10;
int: NUM_SIMULATIONS = 1000;

% State space
int: num_states = 71;  % Monster shards
array[1..num_states] of var 0..NUM_SIMULATIONS: visits;
array[1..num_states] of var 0.0..1.0: value;  % Thinker (value model)
array[1..num_states] of var 0.0..1.0: policy;  % Prover (policy model)

% Dao harmony constraint
constraint forall(s in 1..num_states) (
  value[s] * policy[s] >= 0.5  % Thinker-Prover harmony
);

% UCB1 selection
array[1..num_states] of var float: ucb1;
constraint forall(s in 1..num_states) (
  let {
    float: exploitation = value[s];
    var float: exploration = EXPLORATION_C * sqrt(ln(sum(visits)) / visits[s]);
  } in
    ucb1[s] = exploitation + exploration
);

% Visit distribution (must sum to NUM_SIMULATIONS)
constraint sum(visits) = NUM_SIMULATIONS;

% All states visited at least once
constraint forall(s in 1..num_states) (
  visits[s] >= 1
);

% Value increases with visits (learning)
constraint forall(s in 1..num_states) (
  value[s] >= visits[s] / NUM_SIMULATIONS * 0.5
);

% Policy follows value (Prover validates Thinker)
constraint forall(s in 1..num_states) (
  abs(policy[s] - value[s]) <= 0.1  % 10% tolerance
);

% Eight Circuits (RAW) - special states
array[1..8] of int: circuits = [1, 9, 18, 27, 36, 45, 54, 63];
constraint forall(c in circuits) (
  value[c] >= 0.7  % Higher value for circuit states
);

% Metaprogramming circuit (6th) achieves perfection
constraint value[circuits[6]] = 1.0;
constraint policy[circuits[6]] = 1.0;

% Objective: Maximize total value while maintaining harmony
var float: total_value = sum(s in 1..num_states)(value[s] * visits[s]);
var float: harmony = sum(s in 1..num_states)(value[s] * policy[s]);
var float: objective = total_value + harmony;

solve maximize objective;

% Output: MCTS tree with Thinker-Prover values
output [
  "{\n",
  "  \"exploration_c\": ", show(EXPLORATION_C), ",\n",
  "  \"num_simulations\": ", show(NUM_SIMULATIONS), ",\n",
  "  \"total_value\": ", show(total_value), ",\n",
  "  \"harmony\": ", show(harmony), ",\n",
  "  \"states\": [\n"
] ++
[
  "    {\n",
  "      \"state\": ", show(s), ",\n",
  "      \"visits\": ", show(visits[s]), ",\n",
  "      \"value\": ", show_float(4, value[s]), ",\n",
  "      \"policy\": ", show_float(4, policy[s]), ",\n",
  "      \"ucb1\": ", show_float(4, ucb1[s]), ",\n",
  "      \"dao_harmony\": ", show_float(4, value[s] * policy[s]), "\n",
  "    }",
  if s < num_states then ",\n" else "\n" endif
  | s in 1..num_states
] ++
[
  "  ]\n",
  "}\n"
];
