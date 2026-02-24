# Monster OSM Quest - AI Life Movies

Asciinema recordings of the AI life simulation system.

## Movies

1. **ai-life-simulation.cast** - 24 nodes with meme evolution
2. **ai-life-pure.cast** - Pure functional version with secret functions
3. **ai-life-uucp.cast** - UUCP multiplayer (4 agents)
4. **ai-life-dashboard.cast** - Live dashboard visualization

## View Online

Visit: https://meta-introspector.github.io/osm-planet-torrent/ai-life-movies/

## Generate Locally

```bash
# Record simulation
asciinema rec -c "python3 ai-life-simulation.py" ai-life-simulation.cast

# Record UUCP
asciinema rec -c "python3 ai-life-uucp.py" ai-life-uucp.cast

# Record dashboard
timeout 10 asciinema rec -c "python3 ai-life-dashboard.py" ai-life-dashboard.cast
```

## Play Locally

```bash
asciinema play ai-life-uucp.cast
```
