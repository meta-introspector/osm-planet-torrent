{
  description = "Monster OSM Quest - Sparse torrent extraction with ZK witnesses";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" ];
        };
        
        # Build all Monster OSM binaries
        monsterOsmBinaries = pkgs.rustPlatform.buildRustPackage {
          pname = "monster-osm-quest";
          version = "0.1.0";
          
          src = ./.;
          
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          
          nativeBuildInputs = with pkgs; [
            pkg-config
            protobuf
          ];
          
          buildInputs = with pkgs; [
            zlib
            openssl
          ];
          
          # Build all binaries
          cargoBuildFlags = [
            "--bins"
          ];
          
          # Impure build - allow network access for dependencies
          __impure = true;
          
          meta = {
            description = "Monster OSM Quest - Sparse torrent extraction";
            homepage = "https://github.com/meta-introspector/osm-planet-torrent";
          };
        };
        
      in {
        packages = {
          default = monsterOsmBinaries;
          monster-osm = monsterOsmBinaries;
        };
        
        apps = {
          zkperf-dense = {
            type = "app";
            program = "${monsterOsmBinaries}/bin/zkperf_dense";
          };
          fractran-osm = {
            type = "app";
            program = "${monsterOsmBinaries}/bin/fractran_osm";
          };
          ramanujan-walkers = {
            type = "app";
            program = "${monsterOsmBinaries}/bin/ramanujan_24_walkers";
          };
          walkers-lmfdb = {
            type = "app";
            program = "${monsterOsmBinaries}/bin/walkers_with_lmfdb";
          };
        };
        
        devShells.default = pkgs.mkShell {
          name = "monster-osm-dev";
          
          buildInputs = with pkgs; [
            rustToolchain
            
            # OSM/PBF tools
            protobuf
            zlib
            openssl
            pkg-config
            
            # Torrent clients
            aria2
            transmission_4
            
            # Data processing
            jq
            
            # Python for scripts
            python3
            python3Packages.libtorrent-rasterbar
            
            # Visualization
            asciinema
            
            # GitHub CLI
            gh
          ];
          
          shellHook = ''
            echo "🎭 Monster OSM Quest - Development Shell"
            echo "========================================"
            echo ""
            echo "📦 Binaries to build:"
            echo "  - zkperf_dense       (sparse extraction + ZK witness)"
            echo "  - fractran_osm       (FRACTRAN encoding)"
            echo "  - ramanujan_24_walkers (walker simulation)"
            echo "  - walkers_with_lmfdb (LMFDB discovery)"
            echo "  - math_nodes_world   (database projection)"
            echo ""
            echo "🔨 Build commands:"
            echo "  cargo build --release --bin zkperf_dense"
            echo "  cargo build --release --bins  # all binaries"
            echo ""
            echo "🧪 Test commands:"
            echo "  ./test-zkperf-quick.sh 13668 10"
            echo "  ./test-sparse-torrent.sh 13668 10"
            echo ""
            echo "🚀 Run with nix:"
            echo "  nix run .#zkperf-dense -- --help"
            echo "  nix run .#fractran-osm -- --piece 13668"
            echo ""
            
            export RUST_BACKTRACE=1
            export CARGO_TARGET_DIR="$PWD/target"
            export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library"
          '';
        };
        
        # CI/CD helper
        checks = {
          build-all = monsterOsmBinaries;
          
          test-zkperf = pkgs.runCommand "test-zkperf" {
            buildInputs = [ monsterOsmBinaries pkgs.jq ];
          } ''
            echo "Testing zkperf_dense..."
            ${monsterOsmBinaries}/bin/zkperf_dense --help > $out
          '';
        };
      }
    );
}
