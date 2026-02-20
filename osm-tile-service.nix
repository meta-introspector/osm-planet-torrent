{ config, pkgs, lib, ... }:

let
  osm-tile-shard = pkgs.rustPlatform.buildRustPackage {
    pname = "osm-tile-shard";
    version = "0.1.0";
    
    src = /home/mdupont/projects/osm-planet-torrent;
    
    cargoLock = {
      lockFile = /home/mdupont/projects/osm-planet-torrent/Cargo.lock;
    };
    
    nativeBuildInputs = with pkgs; [ pkg-config ];
    buildInputs = with pkgs; [ openssl ];
    
    buildPhase = ''
      cargo build --release --bin tile-shard
    '';
    
    installPhase = ''
      mkdir -p $out/bin
      cp target/release/tile-shard $out/bin/
    '';
  };

in {
  systemd.services.osm-tile-shard = {
    description = "OSM Planet Tile Sharding Service (86 GB → 196,883 tiles)";
    after = [ "network.target" ];
    wantedBy = [ "multi-user.target" ];
    
    serviceConfig = {
      Type = "simple";
      User = "osm-shard";
      Group = "osm-shard";
      ExecStart = "${osm-tile-shard}/bin/tile-shard";
      WorkingDirectory = "/mnt/data1/osm-planet-torrent";
      StateDirectory = "osm-shard";
      Restart = "on-failure";
      RestartSec = "10";
      
      # Access to planet file and output
      ReadOnlyPaths = [ "/mnt/data1/osm-planet" ];
      ReadWritePaths = [ "/mnt/data1/osm-planet-torrent" ];
      
      # Resource limits
      MemoryMax = "8G";
      CPUQuota = "2400%";  # 24 cores
    };
  };
  
  users.users.osm-shard = {
    isSystemUser = true;
    group = "osm-shard";
    home = "/var/lib/osm-shard";
    createHome = true;
  };
  
  users.groups.osm-shard = {};
  
  # Ensure output directory exists
  systemd.tmpfiles.rules = [
    "d /mnt/data1/osm-planet-torrent 0755 osm-shard osm-shard -"
    "d /mnt/data1/osm-planet-torrent/tiles 0755 osm-shard osm-shard -"
    "d /mnt/data1/osm-planet-torrent/admin 0755 osm-shard osm-shard -"
  ];
}
