{
  description = "HuggingFace Hub CLI";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
      python = pkgs.python3;
      
      huggingface-hub = python.pkgs.buildPythonPackage rec {
        pname = "huggingface-hub";
        version = "0.20.3";
        
        src = python.pkgs.fetchPypi {
          inherit pname version;
          sha256 = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
        };
        
        propagatedBuildInputs = with python.pkgs; [
          filelock
          fsspec
          requests
          tqdm
          pyyaml
          typing-extensions
          packaging
        ];
        
        doCheck = false;
      };
      
    in {
      packages.${system}.default = python.withPackages (ps: [ huggingface-hub ]);
      
      devShells.${system}.default = pkgs.mkShell {
        packages = [
          (python.withPackages (ps: [ huggingface-hub ]))
        ];
      };
    };
}
