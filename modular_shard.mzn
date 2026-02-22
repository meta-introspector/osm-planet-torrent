% Find optimal primes for modular sharding
% Goal: Minimize max bucket size variance

include "globals.mzn";

% Primary sharding (node ID mod p1, p2, p3)
var 67..73: p1;  % Around 71
var 53..61: p2;  % Around 59  
var 43..53: p3;  % Around 47

% Secondary sharding (lat mod p4, lon mod p5)
var 37..43: p4;  % Around 41
var 29..37: p5;  % Around 31

% Tertiary sharding (height/elevation mod p6)
var 23..31: p6;  % Around 29

% Type sharding (node type mod p7)
var 5..11: p7;   % Small prime (5 or 7)

constraint is_prime(p1);
constraint is_prime(p2);
constraint is_prime(p3);
constraint is_prime(p4);
constraint is_prime(p5);
constraint is_prime(p6);
constraint is_prime(p7);

% All different
constraint all_different([p1, p2, p3, p4, p5, p6, p7]);

% Total buckets
var int: total_buckets = p1 * p2 * p3 * p4 * p5 * p6 * p7;

% Minimize total buckets while keeping reasonable distribution
solve minimize total_buckets;

output [
  "Primary: \(p1) × \(p2) × \(p3) = \(p1*p2*p3)\n",
  "Secondary: \(p4) × \(p5) = \(p4*p5)\n", 
  "Tertiary: \(p6)\n",
  "Type: \(p7)\n",
  "Total buckets: \(total_buckets)\n",
  "Avg size (86GB): \(86000000000 div total_buckets) bytes\n"
];

predicate is_prime(var int: n) =
  n > 1 /\ forall(i in 2..ceil(sqrt(int2float(ub(n))))) (
    i >= n \/ n mod i != 0
  );
