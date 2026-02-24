#!/usr/bin/env python3
"""Upload datasets to HuggingFace"""

from huggingface_hub import HfApi
import sys
from pathlib import Path

api = HfApi()

DATASETS = [
    "geo_shards",
    "monster_shards", 
    "ramanujan_tiles",
    "chunks",
    "index",
    "shards",
]

HECKE = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71]

print("Uploading datasets to HuggingFace...")
print()

uploaded = []
failed = []

# Core datasets
for name in DATASETS:
    repo_id = f"introspector/osm-planet-{name}"
    folder = Path(name)
    
    if not folder.exists():
        print(f"⚠️  {name}: folder not found")
        failed.append((name, "folder not found"))
        continue
    
    print(f"📤 {repo_id}...")
    try:
        api.upload_folder(
            folder_path=str(folder),
            repo_id=repo_id,
            repo_type="dataset",
        )
        print(f"✅ {repo_id}")
        uploaded.append(repo_id)
    except Exception as e:
        print(f"⚠️  {repo_id}: {e}")
        failed.append((repo_id, str(e)))

# Hecke operators
for p in HECKE:
    repo_id = f"introspector/osm-planet-tiles-T{p}"
    folder = Path(f"tiles-T{p}")
    
    if not folder.exists():
        print(f"⚠️  tiles-T{p}: folder not found")
        failed.append((f"tiles-T{p}", "folder not found"))
        continue
    
    print(f"📤 {repo_id}...")
    try:
        api.upload_folder(
            folder_path=str(folder),
            repo_id=repo_id,
            repo_type="dataset",
        )
        print(f"✅ {repo_id}")
        uploaded.append(repo_id)
    except Exception as e:
        print(f"⚠️  {repo_id}: {e}")
        failed.append((repo_id, str(e)))

print()
print(f"Uploaded: {len(uploaded)}")
print(f"Failed: {len(failed)}")

if failed:
    print("\nFailed:")
    for name, err in failed:
        print(f"  - {name}: {err}")

sys.exit(0 if not failed else 1)
