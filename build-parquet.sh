#!/bin/bash
# Build Parquet tile writer

set -e

cd /home/mdupont/projects/osm-planet-torrent

echo "🔨 Building Parquet tile writer..."
nix-shell -p cargo rustc pkg-config openssl --run \
  "cargo build --release --bin tile-parquet-leech"

echo ""
echo "✅ Built: target/release/tile-parquet-leech"
