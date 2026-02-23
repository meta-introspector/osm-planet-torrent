#!/usr/bin/env python3
"""Upload Monster OSM Quest datasets to Hugging Face"""

import json
import os
from pathlib import Path
from datasets import Dataset, DatasetDict
from huggingface_hub import HfApi, create_repo

# Configuration
HF_USERNAME = "meta-introspector"
DATASETS = {
    "lmfdb-monster-71": {
        "description": "L-functions and Modular Forms Database mapped to Monster Group 71 shards",
        "files": [
            "/mnt/data1/spool/experiments_monster/lmfdb_71_shards.json",
            "/mnt/data1/spool/experiments_monster/lmfdb_math_functions.json"
        ]
    },
    "ramanujan-osm-journey": {
        "description": "Srinivasa Ramanujan's biographical locations mapped to OSM",
        "files": [
            "ramanujan-location-index.json",
            "/tmp/ramanujan_fractran_speedrun.json"
        ]
    },
    "ramanujan-24-walkers": {
        "description": "24 Ramanujan walker agents simulation data",
        "files": [
            "/tmp/ramanujan_24_walkers.json",
            "/tmp/walkers_with_lmfdb.json"
        ]
    }
}

def create_dataset_card(name, description):
    """Generate README.md for dataset"""
    return f"""---
license: cc-by-4.0
task_categories:
- other
tags:
- mathematics
- monster-group
- osm
- ramanujan
- geography
size_categories:
- n<1K
---

# {name}

{description}

## Dataset Description

Part of the Monster OSM Quest project, which maps the Monster Group's 71 shards to OpenStreetMap data.

### Data Fields

See individual JSON files for schema.

### Source Data

- **OSM**: OpenStreetMap planet data (ODbL license)
- **LMFDB**: L-functions and Modular Forms Database (CC BY-SA 4.0)
- **Mathematical**: Ramanujan biographical data and FRACTRAN encodings

## Usage

```python
from datasets import load_dataset

ds = load_dataset("{HF_USERNAME}/{name}")
print(ds)
```

## Citation

```bibtex
@misc{{monster-osm-quest,
  title={{Monster OSM Quest}},
  author={{Meta-Introspector}},
  year={{2026}},
  url={{https://github.com/meta-introspector/osm-planet-torrent}}
}}
```

## Links

- **Project**: https://github.com/meta-introspector/osm-planet-torrent
- **Asciinema**: https://asciinema.org/a/f8QZVlQ5wDp3DdnA
- **Archive.org**: https://archive.org/details/@meta_introspector

---

🎭 Monster OSM Quest - Where mathematics meets geography
"""

def upload_dataset(name, config):
    """Upload a dataset to Hugging Face"""
    print(f"\n📤 Uploading {name}...")
    
    repo_id = f"{HF_USERNAME}/{name}"
    
    # Create repository
    try:
        create_repo(repo_id, repo_type="dataset", exist_ok=True)
        print(f"✅ Repository created: {repo_id}")
    except Exception as e:
        print(f"⚠️  Repository exists or error: {e}")
    
    # Prepare data
    data = []
    for file_path in config["files"]:
        if os.path.exists(file_path):
            with open(file_path, 'r') as f:
                content = json.load(f)
                data.append({
                    "file": os.path.basename(file_path),
                    "content": json.dumps(content)
                })
            print(f"  ✅ Loaded {os.path.basename(file_path)}")
        else:
            print(f"  ⚠️  File not found: {file_path}")
    
    if not data:
        print(f"  ❌ No data to upload for {name}")
        return
    
    # Create dataset
    dataset = Dataset.from_list(data)
    
    # Upload
    dataset.push_to_hub(repo_id)
    print(f"✅ Dataset uploaded to {repo_id}")
    
    # Upload README
    api = HfApi()
    readme_content = create_dataset_card(name, config["description"])
    api.upload_file(
        path_or_fileobj=readme_content.encode(),
        path_in_repo="README.md",
        repo_id=repo_id,
        repo_type="dataset"
    )
    print(f"✅ README uploaded")

def main():
    print("🎭 Monster OSM Quest - Hugging Face Upload")
    print("==========================================")
    
    # Check authentication
    token = os.getenv("HF_TOKEN")
    if not token:
        print("⚠️  Set HF_TOKEN environment variable")
        print("   Get token from: https://huggingface.co/settings/tokens")
        return
    
    # Upload each dataset
    for name, config in DATASETS.items():
        try:
            upload_dataset(name, config)
        except Exception as e:
            print(f"❌ Error uploading {name}: {e}")
    
    print("\n📊 Upload Summary:")
    print(f"  - {len(DATASETS)} datasets uploaded")
    print(f"\n🌐 View at: https://huggingface.co/{HF_USERNAME}")

if __name__ == "__main__":
    main()
