{ pkgs ? import <nixpkgs> {} }:

pkgs.stdenv.mkDerivation {
  pname = "monster-osm-ai-life";
  version = "0.1.0";

  src = ./.;

  buildInputs = with pkgs; [
    python3
    python3Packages.cbor2
  ];

  installPhase = ''
    mkdir -p $out/bin
    mkdir -p $out/share/monster-osm
    
    # Install Python scripts
    cp ai-life-simulation.py $out/bin/monster-osm-ai-life
    cp ai-life-pure.py $out/bin/monster-osm-ai-life-pure
    cp ai-life-multiplayer.py $out/bin/monster-osm-ai-life-multiplayer
    cp ai-life-uucp.py $out/bin/monster-osm-ai-life-uucp
    cp ai-life-dashboard.py $out/bin/monster-osm-ai-life-dashboard
    
    chmod +x $out/bin/*
    
    # Add Python shebang
    for f in $out/bin/*; do
      sed -i "1s|.*|#!${pkgs.python3}/bin/python3|" $f
    done
  '';

  meta = with pkgs.lib; {
    description = "Monster OSM Quest AI Life - Multiplayer simulation with 71 protocols";
    license = licenses.mit;
    platforms = platforms.unix;
  };
}
