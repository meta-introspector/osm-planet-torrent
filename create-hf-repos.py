#!/usr/bin/env python3
"""Create HuggingFace dataset repositories"""

from huggingface_hub import HfApi, create_repo
import sys

api = HfApi()

DATASETS = [
    ("geo_shards", "Geographic shards (208 files, 1.1M)"),
    ("shards", "Monster Group shards 71×59 grid (90,270 files, 1.0G)"),
    ("chunks", "Torrent chunks (4,418 files, 41M)"),
    ("index", "Piece index (21,792 files, 87M)"),
    ("ramanujan_tiles", "Ramanujan-specific tiles (978 files, 4.1M)"),
    ("monster_shards", "Monster shards (248 files, 1.2M)"),
]

HECKE = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71]

print("Creating HuggingFace dataset repositories...")
print(f"Organization: introspector")
print()

created = []
failed = []

# Core datasets
for name, desc in DATASETS:
    repo_id = f"introspector/osm-planet-{name}"
    try:
        create_repo(
            repo_id=repo_id,
            repo_type="dataset",
            exist_ok=True,
            private=False,
        )
        print(f"✅ {repo_id}")
        created.append(repo_id)
    except Exception as e:
        print(f"⚠️  {repo_id}: {e}")
        failed.append((repo_id, str(e)))

# Hecke operators
for p in HECKE:
    repo_id = f"introspector/osm-planet-tiles-T{p}"
    desc = f"Hecke operator T_{p} tiles (~65K files, 2.5M)"
    try:
        create_repo(
            repo_id=repo_id,
            repo_type="dataset",
            exist_ok=True,
            private=False,
        )
        print(f"✅ {repo_id}")
        created.append(repo_id)
    except Exception as e:
        print(f"⚠️  {repo_id}: {e}")
        failed.append((repo_id, str(e)))

print()
print(f"Created: {len(created)}")
print(f"Failed: {len(failed)}")

if failed:
    print("\nFailed repos:")
    for repo, err in failed:
        print(f"  - {repo}: {err}")

sys.exit(0 if not failed else 1)
