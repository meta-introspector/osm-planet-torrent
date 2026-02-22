# Escaped RDFa Semantics

## What Was Added

**5th archive format**: `osm-torrent-rdfa.html` with full semantic metadata

## Semantic Vocabularies Used

### Schema.org
- `SoftwareApplication` - The demo application
- `DigitalDocument` - Torrent piece 13668
- `Dataset` - OSM Planet (86GB)
- `Place` - Kumbakonam location
- `GeoCoordinates` - Lat/lon coordinates
- `Observation` - Data reduction statistics

### WGS84 Geo
- `geo:lat` / `geo:long` - Geographic coordinates

### DOAP (Description of a Project)
- `doap:Project` - Software project metadata
- `doap:repository` - GitHub repository

### Escaped RDFa
- `erdfa:embedded` - Escaped RDFa content
- `erdfa:content` - Embedded semantic markup

## Metadata Included

```turtle
@prefix schema: <http://schema.org/> .
@prefix geo: <http://www.w3.org/2003/01/geo/wgs84_pos#> .

<#piece-13668> a schema:DigitalDocument ;
    schema:name "OSM Planet Piece 13668" ;
    schema:encodingFormat "application/x-protobuf" ;
    schema:contentSize "4194304" ;
    schema:spatialCoverage <#kumbakonam> .

<#kumbakonam> a schema:Place ;
    schema:name "Kumbakonam" ;
    schema:description "Ramanujan's birthplace" ;
    schema:geo [
        a schema:GeoCoordinates ;
        schema:latitude "10.9617" ;
        schema:longitude "79.3881"
    ] ;
    schema:sameAs <https://www.wikidata.org/wiki/Q1011807> .

<#reduction> a schema:Observation ;
    schema:name "Data Reduction" ;
    schema:value "99.995" ;
    schema:description "Downloaded 4MB instead of 86GB" .
```

## Share URL

**Compressed base64 data URL** embedded in RDFa:
```html
<a href="data:text/html;base64,PCFET0NUWVBFIGh0bWw+...">
  Share URL (base64 compressed)
</a>
```

Opens directly in browser - no server needed!

## Extract Semantics

### Using rapper (Redland)
```bash
rapper -i rdfa osm-torrent-rdfa.html
```

### Using rdflib (Python)
```python
from rdflib import Graph
g = Graph()
g.parse("osm-torrent-rdfa.html", format="rdfa")
print(g.serialize(format="turtle"))
```

### Using Apache Any23
```bash
any23 rover -f turtle osm-torrent-rdfa.html
```

## Benefits

1. **Machine-readable** - Bots can extract structured data
2. **Linked data** - Links to Wikidata, Schema.org
3. **Searchable** - Search engines understand semantics
4. **Shareable** - Compressed URL embeds entire map
5. **Preservable** - Escaped RDFa survives HTML sanitizers

## Use Cases

### Search Engine Optimization
- Google understands Schema.org markup
- Rich snippets in search results
- Knowledge graph integration

### Data Integration
- Extract to RDF triple store
- Query with SPARQL
- Link to other datasets

### Archival
- Escaped RDFa preserved in restrictive platforms
- Semantic metadata survives copy-paste
- Machine-readable provenance

## Namespace

**Escaped RDFa**: https://escaped-rdfa.github.io/namespace

Maintained by: Jim Dupont (jmikedupont2)

## Download

All 5 formats available from GitHub Actions:
1. tar.gz
2. zip
3. standalone.html
4. README.md
5. **rdfa.html** (NEW!)

## Example Query

```sparql
PREFIX schema: <http://schema.org/>
PREFIX geo: <http://www.w3.org/2003/01/geo/wgs84_pos#>

SELECT ?place ?lat ?lon ?wikidata
WHERE {
  ?place a schema:Place ;
         schema:geo ?coords ;
         schema:sameAs ?wikidata .
  ?coords schema:latitude ?lat ;
          schema:longitude ?lon .
}
```

Result: Kumbakonam at [10.9617, 79.3881] linked to Wikidata Q1011807

## Links

- **Escaped RDFa**: https://escaped-rdfa.github.io/namespace
- **Schema.org**: https://schema.org
- **RDFa Primer**: https://www.w3.org/TR/rdfa-primer/
- **Wikidata**: https://www.wikidata.org/wiki/Q1011807
