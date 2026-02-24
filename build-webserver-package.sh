#!/usr/bin/env bash
# Build drop-in webserver package

set -e

VERSION="1.0.0"
OUTPUT="osm-torrent-web-${VERSION}.tar.gz"

echo "🌐 Building webserver drop-in package..."

# Create package structure
mkdir -p package/{www,nginx,apache,docs}

# Copy website files
echo "📦 Copying website..."
cp -r downloaded-site/* package/www/

# Generate nginx config
cat > package/nginx/osm-torrent.conf << 'NGINX'
# OSM Torrent Demo - Nginx Configuration
# Drop-in for any nginx server

# Option 1: Subdirectory (recommended)
location /osm-torrent/ {
    alias /var/www/osm-torrent/;
    try_files $uri $uri/ =404;
    add_header X-Torrent-Pluck "piece-13668" always;
    add_header X-Reduction "99.995%" always;
}

# Option 2: Subdomain
# server {
#     listen 80;
#     server_name osm-torrent.example.com;
#     root /var/www/osm-torrent;
#     
#     location / {
#         try_files $uri $uri/ =404;
#     }
# }

# Option 3: User directory
# location /~username/osm-torrent/ {
#     alias /home/username/public_html/osm-torrent/;
#     try_files $uri $uri/ =404;
# }
NGINX

# Generate Apache config
cat > package/apache/osm-torrent.conf << 'APACHE'
# OSM Torrent Demo - Apache Configuration
# Drop-in for any Apache server

# Option 1: Subdirectory (recommended)
Alias /osm-torrent /var/www/osm-torrent
<Directory /var/www/osm-torrent>
    Options Indexes FollowSymLinks
    AllowOverride None
    Require all granted
    Header set X-Torrent-Pluck "piece-13668"
    Header set X-Reduction "99.995%"
</Directory>

# Option 2: VirtualHost
# <VirtualHost *:80>
#     ServerName osm-torrent.example.com
#     DocumentRoot /var/www/osm-torrent
#     
#     <Directory /var/www/osm-torrent>
#         Options Indexes FollowSymLinks
#         AllowOverride None
#         Require all granted
#     </Directory>
# </VirtualHost>

# Option 3: User directory
# <Directory /home/*/public_html/osm-torrent>
#     Options Indexes FollowSymLinks
#     AllowOverride None
#     Require all granted
# </Directory>
APACHE

# Generate .htaccess for Apache
cat > package/www/.htaccess << 'HTACCESS'
# OSM Torrent Demo - Apache .htaccess
Options +FollowSymLinks
DirectoryIndex index.html

# Custom headers
Header set X-Torrent-Pluck "piece-13668"
Header set X-Reduction "99.995%"

# Enable compression
<IfModule mod_deflate.c>
    AddOutputFilterByType DEFLATE text/html text/css application/javascript
</IfModule>

# Cache static files
<IfModule mod_expires.c>
    ExpiresActive On
    ExpiresByType text/html "access plus 1 hour"
    ExpiresByType text/css "access plus 1 week"
    ExpiresByType application/javascript "access plus 1 week"
</IfModule>
HTACCESS

# Generate installation docs
cat > package/docs/INSTALL.md << 'DOCS'
# OSM Torrent Demo - Installation Guide

## Quick Start

### Nginx

1. Copy files:
   ```bash
   sudo cp -r www/* /var/www/osm-torrent/
   sudo cp nginx/osm-torrent.conf /etc/nginx/conf.d/
   ```

2. Test and reload:
   ```bash
   sudo nginx -t
   sudo systemctl reload nginx
   ```

3. Access: http://your-server/osm-torrent/

### Apache

1. Copy files:
   ```bash
   sudo cp -r www/* /var/www/osm-torrent/
   sudo cp apache/osm-torrent.conf /etc/apache2/conf-available/
   ```

2. Enable and reload:
   ```bash
   sudo a2enconf osm-torrent
   sudo systemctl reload apache2
   ```

3. Access: http://your-server/osm-torrent/

### User Directory (Nginx)

1. Copy to home:
   ```bash
   mkdir -p ~/public_html/osm-torrent
   cp -r www/* ~/public_html/osm-torrent/
   ```

2. Add to nginx config:
   ```nginx
   location /~username/osm-torrent/ {
       alias /home/username/public_html/osm-torrent/;
   }
   ```

### User Directory (Apache)

1. Copy to home:
   ```bash
   mkdir -p ~/public_html/osm-torrent
   cp -r www/* ~/public_html/osm-torrent/
   ```

2. Enable userdir:
   ```bash
   sudo a2enmod userdir
   sudo systemctl reload apache2
   ```

3. Access: http://your-server/~username/osm-torrent/

## What's Included

- **Interactive Leaflet map** showing Kumbakonam (piece 13668)
- **22 doors** from 71 Doors Gallery
- **Maps viewer** with OSM integration
- **Torrents page** with download links

## Features

- 🧲 Torrent piece 13668 (4MB from 86GB planet)
- 📍 Kumbakonam location (Ramanujan's birthplace)
- 🗺️ Interactive Leaflet map
- 📊 99.995% data reduction
- 🚪 22 interactive doors

## Requirements

- Web server (Nginx or Apache)
- No backend needed (pure static HTML)
- Network access for OSM tiles (at runtime)

## Customization

Edit `www/index.html` to change:
- Map center coordinates
- Zoom level
- Marker text
- Info panel content

## Support

- GitHub: https://github.com/meta-introspector/osm-planet-torrent
- Docs: https://meta-introspector.github.io/osm-planet-71-doors/
DOCS

# Generate README
cat > package/README.txt << 'README'
OSM Torrent Demo - Webserver Drop-in Package
=============================================

Version: 1.0.0
Size: ~100KB
License: ODbL 1.0

CONTENTS
--------
www/          - Website files (HTML, CSS, JS)
nginx/        - Nginx configuration
apache/       - Apache configuration  
docs/         - Installation guide

QUICK INSTALL
-------------
1. Extract: tar -xzf osm-torrent-web-1.0.0.tar.gz
2. Copy www/ to your webserver root
3. Add nginx or apache config
4. Reload webserver
5. Access in browser

See docs/INSTALL.md for detailed instructions.

FEATURES
--------
✅ Pure static HTML (no backend)
✅ Interactive Leaflet map
✅ 22 doors from 71 Doors Gallery
✅ Torrent piece demo (99.995% reduction)
✅ Works with any webserver

SUPPORT
-------
GitHub: meta-introspector/osm-planet-torrent
README

# Create tarball
echo "📦 Creating tarball..."
tar -czf "$OUTPUT" -C package .

# Cleanup
rm -rf package

# Show results
SIZE=$(ls -lh "$OUTPUT" | awk '{print $5}')
echo ""
echo "✅ Package created: $OUTPUT ($SIZE)"
echo ""
echo "Contents:"
tar -tzf "$OUTPUT" | head -20
echo "..."
echo ""
echo "Install:"
echo "  tar -xzf $OUTPUT"
echo "  cd package"
echo "  cat docs/INSTALL.md"
