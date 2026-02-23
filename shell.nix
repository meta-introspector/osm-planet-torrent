{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "monster-osm-quest-impure";
  
  buildInputs = with pkgs; [
    # Rust toolchain
    rustc
    cargo
    rustfmt
    clippy
    
    # OSM/PBF dependencies
    protobuf
    zlib
    
    # Torrent tools
    aria2
    transmission
    
    # JSON/data processing
    jq
    
    # Python for scripts
    python3
    python3Packages.libtorrent-rasterbar
    
    # Build tools
    pkg-config
    openssl
    
    # Asciinema
    asciinema
  ];
  
  shellHook = ''
    echo "🎭 Monster OSM Quest - Impure Build Environment"
    echo "=============================================="
    echo ""
    echo "Available tools:"
    echo "  cargo build --release --bin zkperf_dense"
    echo "  cargo build --release --bin fractran_osm"
    echo "  cargo build --release --bin ramanujan_24_walkers"
    echo "  aria2c --version"
    echo "  asciinema --version"
    echo ""
    echo "Quick tests:"
    echo "  ./test-zkperf-quick.sh 13668 10"
    echo "  ./test-sparse-torrent.sh 13668 10"
    echo ""
    
    export RUST_BACKTRACE=1
    export CARGO_TARGET_DIR="$PWD/target"
  '';
}
