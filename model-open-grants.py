#!/usr/bin/env python3
"""Model currently open grants"""

import json
import subprocess
from pathlib import Path
from datetime import datetime

def fetch_open_grants():
    """Fetch open RFPs"""
    print("🔍 Fetching open grants...")
    
    devgrants = Path("/tmp/ipfs-devgrants")
    if not devgrants.exists():
        subprocess.run(["git", "clone", "https://github.com/ipfs/devgrants", str(devgrants)])
    
    rfps = []
    rfp_dir = devgrants / "rfps"
    
    if rfp_dir.exists():
        for f in rfp_dir.glob("*.md"):
            content = f.read_text()
            title = next((l[2:].strip() for l in content.split('\n') if l.startswith('# ')), f.name)
            rfps.append({"file": f.name, "title": title, "content": content[:500]})
    
    return rfps

def model_fit(rfp):
    """Model our fit"""
    keywords = ["performance", "optimization", "test", "python", "rust", "cross-language"]
    text = (rfp['title'] + rfp['content']).lower()
    matches = [k for k in keywords if k in text]
    
    return {
        "rfp": rfp['title'],
        "file": rfp['file'],
        "matches": matches,
        "score": len(matches),
        "fit": "high" if len(matches) >= 3 else "medium" if matches else "low"
    }

def main():
    print("🎯 Open Grant Modeling")
    print("=" * 60)
    
    rfps = fetch_open_grants()
    print(f"✅ Found {len(rfps)} RFPs\n")
    
    models = sorted([model_fit(r) for r in rfps], key=lambda m: m['score'], reverse=True)
    
    for i, m in enumerate(models[:5], 1):
        print(f"{i}. {m['rfp']}")
        print(f"   Score: {m['score']} | Fit: {m['fit'].upper()}")
        if m['matches']:
            print(f"   Matches: {', '.join(m['matches'])}")
        print()
    
    Path("/tmp/open_grants.json").write_text(json.dumps(models, indent=2))
    print(f"✅ Saved to /tmp/open_grants.json")

if __name__ == "__main__":
    main()
