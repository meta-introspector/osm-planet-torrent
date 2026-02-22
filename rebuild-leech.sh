#!/bin/bash
# Rebuild tile writer with flush

set -e

cd /home/mdupont/projects/osm-planet-torrent

echo "🔨 Rebuilding with flush..."
nix-shell -p cargo rustc pkg-config openssl --run \
  "cargo build --release --bin tile-shard-leech" 2>&1 | tail -3

echo ""
echo "✅ Built: target/release/tile-shard-leech"
