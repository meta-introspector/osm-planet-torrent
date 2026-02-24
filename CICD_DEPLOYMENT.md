# Monster OSM Quest - Full CI/CD Pipeline

## Complete Test Suite + Nginx Deployment

### Workflow: `full-test-suite.yml`

**Triggers**:
- Push to `main` or `test-*` branches
- Pull requests to `main`
- Manual dispatch

### Pipeline Stages

#### 1. Build All Binaries
- Uses Nix for reproducible builds
- Builds 6 binaries:
  - `zkperf_dense`
  - `fractran_osm`
  - `ramanujan_24_walkers`
  - `walkers_with_lmfdb`
  - `math_nodes_world`
  - `ramanujan_fractran_speedrun`
- Creates VERSION file with build metadata
- Uploads as artifact (90 days retention)

#### 2. Test zkperf (Matrix)
- Tests 5 Ramanujan locations in parallel:
  - 13668 (Kumbakonam)
  - 13645 (Namagiri Temple)
  - 14137 (Chennai)
  - 16793 (London)
  - 16945 (Cambridge)
- Downloads test OSM data
- Runs zkperf_dense extraction
- Uploads test results

#### 3. Test FRACTRAN
- Tests encoding for pieces: 13668, 17, 23, 59
- Generates FRACTRAN states
- Uploads encodings

#### 4. Test Walkers
- Creates test LMFDB data
- Runs 24 walker simulation
- Tests LMFDB discovery
- Projects math nodes
- Runs speedrun
- Generates markdown report

#### 5. Package for Nginx
- Downloads all test artifacts
- Creates deployment package:
  ```
  nginx-deploy/
  ├── bin/           # All binaries
  ├── results/       # Test results
  ├── web/           # HTML interface
  ├── VERSION        # Build info
  └── deploy.sh      # Deployment script
  ```
- Creates tarball
- Uploads as `nginx-deployment-{run_id}`

## Deployment

### Automatic (from CI)
```bash
# Download and deploy latest
./deploy-from-ci.sh

# Deploy specific run
./deploy-from-ci.sh 123
```

### Manual
```bash
# Download artifacts
gh run download {run-id}

# Extract and deploy
cd nginx-deployment-{run-id}
tar xzf nginx-deploy.tar.gz
cd nginx-deploy
./deploy.sh

# Custom location
NGINX_ROOT=/var/www/html/custom ./deploy.sh
```

## Testing Deployment

```bash
# Test all binaries
./test-deployment.sh

# Manual tests
/var/www/html/osm-monster/bin/zkperf_dense --help
/var/www/html/osm-monster/bin/fractran_osm --piece 13668

# Web interface
curl http://localhost/osm-monster/test-results.html
```

## Artifacts

Each run produces:
- `monster-osm-binaries-{run}` - All binaries + VERSION
- `test-results-piece-{N}` - zkperf test results (5 pieces)
- `fractran-encodings` - FRACTRAN states
- `walker-test-results` - Walker simulation + report
- `nginx-deployment-{run}` - Complete deployment package

## Nginx Structure

```
/var/www/html/osm-monster/
├── bin/
│   ├── zkperf_dense
│   ├── fractran_osm
│   ├── ramanujan_24_walkers
│   ├── walkers_with_lmfdb
│   ├── math_nodes_world
│   └── ramanujan_fractran_speedrun
├── results/
│   ├── test-13668.geojson
│   ├── fractran-13668.json
│   ├── walkers.json
│   └── test-report.md
├── web/
│   ├── index.html
│   ├── game.js
│   └── test-results.html
└── VERSION
```

## Quick Commands

```bash
# Trigger CI
git push origin test-deployment

# Watch run
gh run watch

# Download latest
./deploy-from-ci.sh

# Test deployment
./test-deployment.sh

# View results
curl http://localhost/osm-monster/test-results.html
```

## Environment Variables

- `NGINX_ROOT` - Deployment location (default: `/var/www/html/osm-monster`)
- `RUN_ID` - CI run number (default: `latest`)

## Example Session

```bash
# 1. Push to trigger CI
git push origin test-deployment

# 2. Wait for completion
gh run watch

# 3. Deploy to nginx
./deploy-from-ci.sh

# 4. Test deployment
./test-deployment.sh

# 5. Access web interface
firefox http://localhost/osm-monster/

# 6. Run binaries
/var/www/html/osm-monster/bin/zkperf_dense \
  --input /mnt/data1/osm-planet/planet-latest.osm.pbf \
  --piece 13668 \
  --limit 10
```

## Monitoring

```bash
# List runs
gh run list --workflow=full-test-suite.yml

# View specific run
gh run view {run-id}

# Download logs
gh run view {run-id} --log
```

---

🎭 Monster OSM Quest - Full CI/CD with Nginx deployment
