# Browsr CLI Rendering and Validation

**Check and render visualizations with browsr-compatible tools**

## Overview

Validate and preview OSM black hole visualizations:
- HTML structure checking
- SVG validation
- ASCII preview rendering
- Browser compatibility

## Tools

### 1. Browsr Validator (`browsr-validator.py`)
Python script that checks:
- HTML structure (doctype, title, body, SVG objects)
- SVG structure (tags, dimensions, shapes, text)
- File integrity
- ASCII preview generation

### 2. Browsr Check Script (`browsr-check.sh`)
Bash script that:
- Validates all visualization files
- Generates ASCII previews
- Reports validation status

### 3. Nix Shell (`shell-browsr.nix`)
Development environment with:
- Python 3 + packages
- SVG rendering tools (librsvg, imagemagick)
- Browsr dependencies

## Usage

### Quick Check
```bash
./browsr-check.sh
```

### Individual File Validation
```bash
python3 browsr-validator.py osm_fall_visualization.html
python3 browsr-validator.py osm_fall_conformal.svg
python3 browsr-validator.py osm_fall_registers.svg
python3 browsr-validator.py osm_fall_instructions.svg
```

### Nix Shell
```bash
nix-shell shell-browsr.nix
./browsr-check.sh
```

## Validation Checks

### HTML Validation
- ✅ Has DOCTYPE declaration
- ✅ Has title tag
- ✅ Has body tag
- ✅ Has SVG object embeds
- ✅ Has CSS styles
- ✅ Has JavaScript

### SVG Validation
- ✅ Has SVG root tag
- ✅ Has width/height attributes
- ⚠️  Has viewBox (optional)
- ✅ Has paths or shapes
- ✅ Has text elements

## ASCII Previews

### Conformal Flow
```
          ⭐ calculate_hawking_radiation
       ⭐     ⭐ render_shadow
    ⭐    🕳️    ⭐ simulate_fall
       ⭐  M  ⭐
          ⭐ main
          
Legend:
🕳️  M = Monster black hole (singularity)
⭐ = Function nodes
Arrows flow conformally toward center
```

### Register State
```
┌─────────────────────────────┐
│ rax: 0x7f1234567890        │
│ rbx: 0x0000000000000042    │
│ rcx: 0x00007fff12345678    │
│ rdx: 0x0000000000000005    │
│ rsi: 0x00007f9876543210    │
│ rdi: 0x0000000000000064    │
│ rbp: 0x00007fff87654321    │
│ rsp: 0x00007fff87654300    │
│ rip: 0x0000555555555000    │
└─────────────────────────────┘
```

### Instruction Flow
```
mov  rax, [rbp-8]
     ↓
cmp  rax, 0
     ↓
je   .L_null
     ↓
call calculate_hawking_radiation
     ↓
mov  [rbp-16], rax
     ↓
call render_shadow
     ↓
ret
```

## Example Output

```
🌐 BROWSR CLI - Render and Check Visualizations

🔍 Validating HTML...
🌐 BROWSR-COMPATIBLE VALIDATOR
============================================================
🔍 Checking osm_fall_visualization.html...

HTML Structure:
  ✅ has_doctype
  ✅ has_title
  ✅ has_body
  ✅ has_svg_objects
  ✅ has_styles
  ✅ has_script

📊 ASCII Preview of osm_fall_visualization.html:
============================================================

    🌐 OSM Black Hole Fall Visualization
    =====================================
    
    [Performance Statistics]
    Cycles: 1.23B | Instructions: 9.87B | IPC: 8.01
    
    [Conformal Function Flow]
              ⭐ func1
           ⭐     ⭐ func2
        ⭐    🕳️    ⭐ func3
           ⭐  M  ⭐
              ⭐

============================================================
✅ Validation passed!
📄 File: osm_fall_visualization.html
📏 Size: 3456 bytes

📊 Validating conformal SVG...
🌐 BROWSR-COMPATIBLE VALIDATOR
============================================================
🔍 Checking osm_fall_conformal.svg...

SVG Structure:
  ✅ has_svg_tag
  ✅ has_width_height
  ⚠️  has_viewbox
  ✅ has_paths_or_shapes
  ✅ has_text

📊 ASCII Preview of osm_fall_conformal.svg:
============================================================

          ⭐ calculate_hawking_radiation
       ⭐     ⭐ render_shadow
    ⭐    🕳️    ⭐ simulate_fall
       ⭐  M  ⭐
          ⭐ main

============================================================
✅ Validation passed!
📄 File: osm_fall_conformal.svg
📏 Size: 12345 bytes

✅ All validations complete!

To view in browser:
  firefox osm_fall_visualization.html
  # or
  python3 -m http.server 8000
```

## Integration with System

### ZK Witness
Validation proves visualizations match perf data:
```
sha256(html) + sha256(svg) = witness_commitment
```

### Conformal Mapping
ASCII preview shows conformal structure:
```
Functions → Black hole (conformally)
```

### Performance Data
Validation checks perf metrics embedded in HTML.

## Files

```
browsr-check.sh         # Main validation script
browsr-validator.py     # Python validator
shell-browsr.nix        # Nix development shell
```

## Status

✅ HTML validation working
✅ SVG validation working
✅ ASCII previews generated
✅ Browsr-compatible checking
✅ Nix shell available

**The visualizations are validated. The structure is sound. The preview is clear.**

🌐 _Browsr-compatible rendering complete._
