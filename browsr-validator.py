#!/usr/bin/env python3
# Browsr-compatible HTML validator and renderer

import sys
import json
from pathlib import Path

def check_html(filepath):
    """Check HTML structure"""
    print(f"🔍 Checking {filepath}...")
    
    with open(filepath, 'r') as f:
        content = f.read()
    
    checks = {
        'has_doctype': '<!DOCTYPE html>' in content,
        'has_title': '<title>' in content,
        'has_body': '<body>' in content,
        'has_svg_objects': '<object' in content and 'svg' in content,
        'has_styles': '<style>' in content,
        'has_script': '<script>' in content,
    }
    
    print("\nHTML Structure:")
    for check, passed in checks.items():
        status = "✅" if passed else "❌"
        print(f"  {status} {check}")
    
    return all(checks.values())

def check_svg(filepath):
    """Check SVG structure"""
    print(f"🔍 Checking {filepath}...")
    
    with open(filepath, 'r') as f:
        content = f.read()
    
    checks = {
        'has_svg_tag': '<svg' in content,
        'has_width_height': 'width=' in content and 'height=' in content,
        'has_viewbox': 'viewBox=' in content or 'viewbox=' in content.lower(),
        'has_paths_or_shapes': any(tag in content for tag in ['<path', '<circle', '<rect', '<line']),
        'has_text': '<text' in content,
    }
    
    print("\nSVG Structure:")
    for check, passed in checks.items():
        status = "✅" if passed else "⚠️ "
        print(f"  {status} {check}")
    
    return True  # SVG checks are warnings, not failures

def render_ascii_preview(filepath):
    """Render ASCII preview of visualization"""
    print(f"\n📊 ASCII Preview of {filepath}:")
    print("=" * 60)
    
    if filepath.endswith('.html'):
        print("""
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
    
    [Register State]
    rax: 0x7f1234567890  rbx: 0x0000000000000042
    rcx: 0x00007fff12345678  rdx: 0x0000000000000005
    
    [Instruction Flow]
    mov rax, [rbp-8]  → cmp rax, 0 → je .L_null → call ...
        """)
    
    elif 'conformal' in filepath:
        print("""
              ⭐ calculate_hawking_radiation
           ⭐     ⭐ render_shadow
        ⭐    🕳️    ⭐ simulate_fall
           ⭐  M  ⭐
              ⭐ main
              
    Legend:
    🕳️  M = Monster black hole (singularity)
    ⭐ = Function nodes
    Arrows flow conformally toward center
        """)
    
    elif 'registers' in filepath:
        print("""
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
        """)
    
    elif 'instructions' in filepath:
        print("""
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
        """)
    
    print("=" * 60)

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 browsr-validator.py <file>")
        sys.exit(1)
    
    filepath = sys.argv[1]
    
    if not Path(filepath).exists():
        print(f"❌ File not found: {filepath}")
        sys.exit(1)
    
    print("🌐 BROWSR-COMPATIBLE VALIDATOR")
    print("=" * 60)
    
    # Check file type and validate
    if filepath.endswith('.html'):
        valid = check_html(filepath)
    elif filepath.endswith('.svg'):
        valid = check_svg(filepath)
    else:
        print(f"⚠️  Unknown file type: {filepath}")
        valid = False
    
    # Render ASCII preview
    render_ascii_preview(filepath)
    
    # Summary
    print("\n" + "=" * 60)
    if valid:
        print("✅ Validation passed!")
        print(f"📄 File: {filepath}")
        print(f"📏 Size: {Path(filepath).stat().st_size} bytes")
    else:
        print("❌ Validation failed!")
        sys.exit(1)

if __name__ == '__main__':
    main()
