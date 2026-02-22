% Find optimal primes for hierarchical OSM sharding
% Level 1: node_id % (p1 * p2 * p3)
% Level 2: lat % p4, lon % p5
% Level 3: height % p6
% Level 4: type % p7

include "globals.mzn";

% Prime ranges
var 67..73: p1;  % ~71
var 53..61: p2;  % ~59
var 43..53: p3;  % ~47
var 37..43: p4;  % ~41 (lat)
var 29..37: p5;  % ~31 (lon)
var 23..31: p6;  % ~29 (height)
var 5..11: p7;   % ~7 (type)

% All must be prime
constraint forall(p in [p1, p2, p3, p4, p5, p6, p7]) (
    forall(d in 2..ceil(sqrt(int2float(p)))) (p mod d != 0)
);

% All different
constraint all_different([p1, p2, p3, p4, p5, p6, p7]);

% Calculate hierarchy
var int: level1_buckets = p1 * p2 * p3;
var int: level2_buckets = p4 * p5;
var int: level3_buckets = p6;
var int: level4_buckets = p7;

% Total leaf buckets
var int: total_buckets = level1_buckets * level2_buckets * level3_buckets * level4_buckets;

% Target: reasonable bucket sizes for 86 GB
constraint total_buckets >= 1000;    % 86 MB avg
constraint total_buckets <= 1000000; % 86 KB avg

% Minimize total buckets (larger buckets = less overhead)
solve minimize total_buckets;

output [
  "Level 1 (node_id): \(p1) × \(p2) × \(p3) = \(level1_buckets) buckets\n",
  "Level 2 (lat,lon): \(p4) × \(p5) = \(level2_buckets) buckets\n",
  "Level 3 (height): \(p6) buckets\n",
  "Level 4 (type): \(p7) buckets\n",
  "Total: \(total_buckets) buckets\n",
  "Avg size: \(86000000000 div total_buckets) bytes = \(86000000000 div total_buckets div 1048576) MB\n"
];
