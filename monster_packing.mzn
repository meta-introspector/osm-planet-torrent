% MiniZinc model for optimal packing of OSM nodes into Monster Group blocks
% Goal: Pack nodes into 71 blocks of 4KB each, minimizing geographic fragmentation

% Parameters
int: num_blocks = 71;           % Monster Group prime
int: block_size = 4096;         % 4KB per block
int: num_nodes;                 % Total nodes to pack (from input)
int: num_geo_buckets;           % Number of geographic buckets (71×59×47)

% Input arrays (from OSM data)
array[1..num_nodes] of int: node_id;
array[1..num_nodes] of int: node_size;        % Size in bytes
array[1..num_nodes] of int: geo_bucket;       % Which geographic bucket (0-196876)
array[1..num_nodes] of float: node_lat;
array[1..num_nodes] of float: node_lon;

% Decision variables
array[1..num_nodes] of var 1..num_blocks: node_to_block;  % Which block each node goes to

% Constraints

% 1. Block capacity constraint: each block <= 4KB
constraint forall(b in 1..num_blocks) (
    sum(n in 1..num_nodes where node_to_block[n] = b) (node_size[n]) <= block_size
);

% 2. Geographic locality: nodes in same geo bucket should be in same block (soft)
array[1..num_blocks] of var int: block_geo_diversity = 
    [sum(n in 1..num_nodes where node_to_block[n] = b) 
         (if exists(m in 1..num_nodes where m != n /\ node_to_block[m] = b /\ geo_bucket[m] != geo_bucket[n]) 
          then 1 else 0 endif) 
     | b in 1..num_blocks];

% 3. Load balancing: blocks should be roughly equal size
var int: min_block_size = min([sum(n in 1..num_nodes where node_to_block[n] = b) (node_size[n]) | b in 1..num_blocks]);
var int: max_block_size = max([sum(n in 1..num_nodes where node_to_block[n] = b) (node_size[n]) | b in 1..num_blocks]);
var int: block_size_variance = max_block_size - min_block_size;

% 4. Spatial coherence: minimize bounding box area per block
array[1..num_blocks] of var float: block_bbox_area;
constraint forall(b in 1..num_blocks) (
    let {
        var float: min_lat = min([node_lat[n] | n in 1..num_nodes where node_to_block[n] = b]),
        var float: max_lat = max([node_lat[n] | n in 1..num_nodes where node_to_block[n] = b]),
        var float: min_lon = min([node_lon[n] | n in 1..num_nodes where node_to_block[n] = b]),
        var float: max_lon = max([node_lon[n] | n in 1..num_nodes where node_to_block[n] = b]),
    } in
    block_bbox_area[b] = (max_lat - min_lat) * (max_lon - min_lon)
);

% Objective: Minimize geographic fragmentation + balance load
var int: total_geo_diversity = sum(block_geo_diversity);
var float: total_bbox_area = sum(block_bbox_area);

solve minimize (
    total_geo_diversity * 1000 +      % Heavily penalize geographic mixing
    block_size_variance +              % Penalize unbalanced blocks
    total_bbox_area                    % Minimize spatial spread
);

% Output
output [
    "Optimal packing found:\n",
    "  Geographic diversity: \(total_geo_diversity)\n",
    "  Block size variance: \(block_size_variance) bytes\n",
    "  Total bbox area: \(total_bbox_area)\n",
    "\nBlock assignments:\n"
] ++
[
    "Block \(b): " ++
    "\(sum(n in 1..num_nodes where node_to_block[n] = b) (node_size[n])) bytes, " ++
    "\(count(n in 1..num_nodes where node_to_block[n] = b)) nodes\n"
    | b in 1..num_blocks
];
