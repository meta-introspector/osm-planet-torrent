#!/bin/bash
# Build and run BBS map viewer

set -e

cd /home/mdupont/projects/osm-planet-torrent

echo "🔨 Building BBS map..."
nix-shell -p cargo rustc pkg-config openssl --run "cargo build --release --bin bbs-map"

echo ""
echo "🗺️  Running BBS map..."
./target/release/bbs-map
