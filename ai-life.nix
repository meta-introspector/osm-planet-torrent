{ pkgs ? import <nixpkgs> {} }:

pkgs.python3Packages.buildPythonApplication {
  pname = "monster-osm-ai-life";
  version = "0.1.0";
  
  format = "other";

  src = pkgs.lib.cleanSourceWith {
    src = ./.;
    filter = path: type:
      let baseName = baseNameOf path;
      in baseName == "ai-life-simulation.py" ||
         baseName == "ai-life-pure.py" ||
         baseName == "ai-life-multiplayer.py" ||
         baseName == "ai-life-uucp.py" ||
         baseName == "ai-life-dashboard.py";
  };

  propagatedBuildInputs = with pkgs.python3Packages; [
    cbor2
  ];

  dontBuild = true;
  dontCheck = true;

  installPhase = ''
    mkdir -p $out/bin
    
    cp ai-life-simulation.py $out/bin/monster-osm-ai-life
    cp ai-life-pure.py $out/bin/monster-osm-ai-life-pure
    cp ai-life-multiplayer.py $out/bin/monster-osm-ai-life-multiplayer
    cp ai-life-uucp.py $out/bin/monster-osm-ai-life-uucp
    cp ai-life-dashboard.py $out/bin/monster-osm-ai-life-dashboard
    
    chmod +x $out/bin/*
  '';

  meta = with pkgs.lib; {
    description = "Monster OSM Quest AI Life - Multiplayer with 71 protocols";
    license = licenses.mit;
  };
}
