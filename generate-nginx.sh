#!/usr/bin/env bash
# Generate nginx config for OSM Planet using homedir convention

TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%S%z")
PROJECT_DIR="$HOME/projects/osm-planet-torrent"

cat > /tmp/osm-planet-homedir.nginx << NGINX
# OSM Planet - 71 Doors Gallery
# Generated: $TIMESTAMP
# Monster Group [71, 59, 47]
# Homedir Convention: /~osm-planet/

server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;

    server_name solana.solfunmeme.com;

    # Existing root
    root /nix/store/lbp0nzjn4g2f5c7khyvn9v7p7avmcvrw-cicada71-site;
    index index.html;

    ssl_certificate /etc/letsencrypt/live/solana.solfunmeme.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/solana.solfunmeme.com/privkey.pem;

    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_prefer_server_ciphers on;

    # Existing locations (keep)
    location / {
        try_files \$uri \$uri/ =404;
        add_header X-CICADA-71 "Monster-Portal" always;
    }

    location /door {
        alias /home/mdupont/projects/monster-osm-quest-standalone;
        try_files \$uri \$uri/index.html =404;
    }

    # NEW: OSM Planet 71 Doors (homedir convention)
    location /~osm-planet/ {
        alias $PROJECT_DIR/public_html/;
        try_files \$uri \$uri/ =404;
        add_header X-Monster-Group "71-59-47" always;
        add_header X-Doors "71" always;
    }

    # Door 17 - Hawkins Radiation
    location = /~osm-planet/doors/door-17.html {
        alias $PROJECT_DIR/public_html/doors/door-17.html;
        add_header X-Door "17-Hawkins-Radiation" always;
        add_header X-Cusp "Giza-Pyramids" always;
    }

    # Door 23 - Consciousness
    location = /~osm-planet/doors/door-23.html {
        alias $PROJECT_DIR/public_html/doors/door-23.html;
        add_header X-Door "23-Consciousness" always;
        add_header X-Cusp "Silicon-Valley" always;
    }

    # Door 59 - Memory
    location = /~osm-planet/doors/door-59.html {
        alias $PROJECT_DIR/public_html/doors/door-59.html;
        add_header X-Door "59-Memory" always;
        add_header X-Cusp "Ramanujan-Temple" always;
    }

    # Torrents
    location = /~osm-planet/torrents.html {
        alias $PROJECT_DIR/public_html/torrents.html;
        add_header X-Archive "Archive.org" always;
    }

    # Health check
    location = /~osm-planet/health {
        return 200 "71 Doors Gallery OK\n";
        add_header Content-Type text/plain;
    }
}
NGINX

echo "✅ Generated: /tmp/osm-planet-homedir.nginx"
echo ""
echo "To deploy:"
echo "  sudo cp /tmp/osm-planet-homedir.nginx /etc/nginx/sites-available/solana-solfunmeme"
echo "  sudo nginx -t && sudo systemctl reload nginx"
echo ""
echo "Access at:"
echo "  https://solana.solfunmeme.com/~osm-planet/"
