#!/bin/bash
# Quick deploy script for local testing

set -e

RUN_ID=${1:-latest}
NGINX_ROOT=${NGINX_ROOT:-/var/www/html/osm-monster}

echo "🎭 Monster OSM Quest - Local Deployment"
echo "========================================"
echo "Run ID: $RUN_ID"
echo "Target: $NGINX_ROOT"
echo ""

if [ "$RUN_ID" = "latest" ]; then
    echo "📥 Downloading latest artifacts..."
    gh run list --workflow=full-test-suite.yml --limit 1 --json databaseId --jq '.[0].databaseId' > /tmp/latest-run
    RUN_ID=$(cat /tmp/latest-run)
    echo "Latest run: $RUN_ID"
fi

echo "📦 Downloading artifacts..."
gh run download "$RUN_ID" --dir /tmp/monster-deploy-$$

cd /tmp/monster-deploy-$$

if [ -d "nginx-deployment-$RUN_ID" ]; then
    cd "nginx-deployment-$RUN_ID"
    tar xzf nginx-deploy.tar.gz
    cd nginx-deploy
else
    echo "❌ Deployment package not found"
    exit 1
fi

echo ""
echo "🚀 Deploying to $NGINX_ROOT..."
sudo mkdir -p "$NGINX_ROOT"
sudo cp -r bin "$NGINX_ROOT/"
sudo cp -r results "$NGINX_ROOT/"
sudo cp -r web/* "$NGINX_ROOT/"
sudo cp VERSION "$NGINX_ROOT/"

echo ""
echo "✅ Deployment complete!"
echo ""
echo "📊 Deployed files:"
ls -lh "$NGINX_ROOT"

echo ""
echo "🧪 Testing binaries..."
"$NGINX_ROOT/bin/zkperf_dense" --help | head -3

echo ""
echo "🌐 Access points:"
echo "  - http://localhost/osm-monster/"
echo "  - http://localhost/osm-monster/test-results.html"
echo ""
echo "🎭 Monster OSM Quest deployed!"
