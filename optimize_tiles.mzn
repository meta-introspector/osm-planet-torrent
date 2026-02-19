% Optimize tile sizes for browser loading
% Goal: Minimize download size while maintaining query speed

% Variables
var 1..1000: tile_size_kb;        % Target tile size in KB
var 1..500000: total_tiles;       % Number of tiles
var 1..100: compression_ratio;    % Compression (1-100%)

% Constants
int: planet_size_gb = 86;
int: planet_size_kb = planet_size_gb * 1024 * 1024;

% Constraints
constraint total_tiles * tile_size_kb = planet_size_kb;

% Browser constraints
constraint tile_size_kb >= 10;    % Min 10 KB (too small = too many requests)
constraint tile_size_kb <= 5000;  % Max 5 MB (too large = slow initial load)

% Ramanujan primes: 71 × 59 × 47 = 196,883
constraint total_tiles >= 196883;
constraint total_tiles <= 200000;

% Compression (gzip typical: 70-80%)
constraint compression_ratio >= 70;
constraint compression_ratio <= 85;

var int: compressed_tile_kb = (tile_size_kb * compression_ratio) div 100;

% Optimize for fast browser loading
% Prefer: smaller tiles (faster initial load) but not too many (HTTP overhead)
solve minimize compressed_tile_kb + (total_tiles div 1000);

output [
  "Optimal tile configuration:\n",
  "Total tiles: \(total_tiles)\n",
  "Tile size: \(tile_size_kb) KB (uncompressed)\n",
  "Compressed: \(compressed_tile_kb) KB (gzip)\n",
  "Compression: \(compression_ratio)%\n",
  "\nBrowser performance:\n",
  "Initial load: \(compressed_tile_kb) KB\n",
  "HTTP requests: 1 per query\n",
  "Savings: \(100 - (compressed_tile_kb * 100 div planet_size_kb))% vs full planet\n"
];
