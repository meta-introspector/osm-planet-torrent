# Archive Formats - GitHub Actions Output

## What Gets Generated

Every push to the repo generates **4 downloadable formats**:

### 1. Web Archive (tar.gz)
**File**: `osm-torrent-web.tar.gz`  
**Size**: ~100KB  
**Use**: Deploy to any webserver  

```bash
tar -xzf osm-torrent-web.tar.gz
cd public
python3 -m http.server 8000
```

### 2. Zip Archive
**File**: `osm-torrent-web.zip`  
**Size**: ~100KB  
**Use**: Windows-friendly, same content as tar.gz  

```bash
unzip osm-torrent-web.zip
cd public
# Serve with your preferred method
```

### 3. Standalone HTML
**File**: `osm-torrent-standalone.html`  
**Size**: ~3KB  
**Use**: Copy-paste to JSFiddle, CodePen, Gist, Pastebin  

**JSFiddle**: https://jsfiddle.net  
**CodePen**: https://codepen.io  
**Gist**: https://gist.github.com  

Just paste the entire file content and run!

### 4. README
**File**: `osm-torrent-README.md`  
**Size**: ~2KB  
**Use**: Documentation for all formats  

## How to Download

### From GitHub Actions

1. Go to: https://github.com/meta-introspector/osm-planet-torrent/actions
2. Click latest successful run
3. Scroll to "Artifacts"
4. Download `osm-torrent-archives`
5. Extract and use any format

### From Command Line

```bash
# Using gh CLI
gh run list --workflow=build-demo-maps.yml --limit 1
gh run download <RUN_ID> -n osm-torrent-archives
```

## What's Inside

All formats contain:
- ✅ Interactive Leaflet map
- ✅ Kumbakonam location (piece 13668)
- ✅ 22 doors from 71 Doors Gallery
- ✅ Maps viewer
- ✅ Torrents page
- ✅ 99.995% reduction demo

## Use Cases

### Web Archive (tar.gz/zip)
- Deploy to nginx/apache
- Host on GitHub Pages
- Serve from user directory
- Add to existing website

### Standalone HTML
- Share on JSFiddle for live demo
- Post to CodePen for showcase
- Create GitHub Gist for embedding
- Paste to Pastebin for quick share
- Email as single file
- Open directly in browser

## Future Formats

Planned additions:
- [ ] PDF (map screenshot + docs)
- [ ] EPUB (Kindle-ready ebook)
- [ ] WARC (Web Archive format)
- [ ] Docker image
- [ ] Nix flake output

## Example: JSFiddle

1. Download `osm-torrent-standalone.html`
2. Go to https://jsfiddle.net
3. Paste entire content into HTML panel
4. Click "Run"
5. Share the JSFiddle URL!

## Example: Quick Deploy

```bash
# Download and deploy in one command
gh run download $(gh run list -w build-demo-maps.yml -L 1 --json databaseId -q '.[0].databaseId') \
  -n osm-torrent-archives && \
tar -xzf osm-torrent-web.tar.gz && \
cd public && \
python3 -m http.server 8000
```

## Retention

- Artifacts kept for **90 days**
- Download anytime within that period
- New artifacts generated on every push

## Size Comparison

| Format | Size | Compression |
|--------|------|-------------|
| tar.gz | ~100KB | Best |
| zip | ~100KB | Good |
| standalone.html | ~3KB | Single file |
| README.md | ~2KB | Text |

**Total**: ~205KB for all formats combined

## Links

- **Actions**: https://github.com/meta-introspector/osm-planet-torrent/actions
- **Live Site**: https://meta-introspector.github.io/osm-planet-torrent/
- **71 Doors**: https://meta-introspector.github.io/osm-planet-71-doors/
