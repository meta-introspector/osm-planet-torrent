# Nix shell with browsr for visualization checking
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    python3
    python3Packages.pip
    python3Packages.rich
    python3Packages.textual
    
    # For SVG rendering
    librsvg
    imagemagick
    
    # Browsr dependencies
    (python3.withPackages (ps: with ps; [
      rich
      textual
      click
      beautifulsoup4
      lxml
    ]))
  ];
  
  shellHook = ''
    echo "🌐 Browsr Visualization Environment"
    echo ""
    
    # Install browsr if not available
    if ! command -v browsr &> /dev/null; then
      echo "📦 Installing browsr..."
      pip install --user browsr
      export PATH="$HOME/.local/bin:$PATH"
    fi
    
    echo "✅ Environment ready"
    echo ""
    echo "Commands:"
    echo "  ./browsr-check.sh           - Check all visualizations"
    echo "  browsr osm_fall_visualization.html - View HTML"
    echo "  browsr osm_fall_conformal.svg      - View SVG"
    echo ""
  '';
}
