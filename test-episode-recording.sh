#!/bin/bash
# Quick test: Record one AI life episode

set -e

cd /home/mdupont/projects/osm-planet-torrent

echo "🎬 Recording AI Life Episode (Quick Test)"
echo "=========================================="

# Create simple recording script
cat > /tmp/ai_episode_test.py << 'EOF'
import sys
import time
sys.path.insert(0, '/home/mdupont/projects/osm-planet-torrent')

# Import and run simulation
exec(open('ai-life-simulation.py').read())
EOF

# Record with asciinema
asciinema rec \
  -c "python3 /tmp/ai_episode_test.py" \
  --overwrite \
  /tmp/ai-life-episode-test.cast

echo ""
echo "✅ Episode recorded!"
echo "📁 /tmp/ai-life-episode-test.cast"
echo ""
echo "To upload:"
echo "  asciinema upload /tmp/ai-life-episode-test.cast"
