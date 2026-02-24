#!/usr/bin/env python3
"""Run hackathon analysis on AI life git commits"""

import subprocess
import json
from pathlib import Path
from datetime import datetime

def run_hackathon_analysis():
    """Run hackathon tool on git commits"""
    
    print("🔍 Running hackathon analysis on AI life commits...")
    
    # Get recent commits related to AI life
    result = subprocess.run(
        ["git", "log", "--oneline", "--grep=AI", "-20"],
        capture_output=True,
        text=True,
        cwd="/home/mdupont/projects/osm-planet-torrent"
    )
    
    commits = result.stdout.strip().split('\n')
    print(f"📊 Found {len(commits)} AI-related commits")
    
    # Clone hackathon repo if needed
    hackathon_dir = Path("/tmp/hackathon")
    if not hackathon_dir.exists():
        print("📥 Cloning hackathon repo...")
        subprocess.run([
            "git", "clone",
            "https://github.com/meta-introspector/hackathon",
            str(hackathon_dir)
        ])
    
    # Run analysis
    analysis = {
        "timestamp": datetime.now().isoformat(),
        "repository": "osm-planet-torrent",
        "commits_analyzed": len(commits),
        "commits": []
    }
    
    for commit_line in commits[:10]:
        if not commit_line:
            continue
        
        commit_hash = commit_line.split()[0]
        
        # Get commit details
        details = subprocess.run(
            ["git", "show", "--stat", commit_hash],
            capture_output=True,
            text=True,
            cwd="/home/mdupont/projects/osm-planet-torrent"
        )
        
        analysis["commits"].append({
            "hash": commit_hash,
            "summary": commit_line,
            "stats": details.stdout.split('\n')[-3:] if details.stdout else []
        })
    
    # Save analysis
    output_file = Path("/tmp/ai-episodes/hackathon_analysis.json")
    output_file.parent.mkdir(exist_ok=True)
    output_file.write_text(json.dumps(analysis, indent=2))
    
    print(f"✅ Analysis complete: {output_file}")
    
    # Generate report
    print("\n" + "="*70)
    print("HACKATHON ANALYSIS REPORT")
    print("="*70)
    print(f"Repository: {analysis['repository']}")
    print(f"Commits analyzed: {analysis['commits_analyzed']}")
    print(f"\nRecent AI Life commits:")
    for commit in analysis['commits'][:5]:
        print(f"  • {commit['summary']}")
    
    return analysis

if __name__ == "__main__":
    run_hackathon_analysis()
