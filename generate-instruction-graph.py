#!/usr/bin/env python3
# Generate DOT graph from instructions and render to PlantUML ASCII

import subprocess
import sys

# Sample instruction trace (would come from perf data)
instructions = [
    {
        'addr': '0x555555555000',
        'op': 'push',
        'args': 'rbp',
        'desc': 'Save base pointer',
        'next': ['0x555555555001']
    },
    {
        'addr': '0x555555555001',
        'op': 'mov',
        'args': 'rbp, rsp',
        'desc': 'Set up stack frame',
        'next': ['0x555555555004']
    },
    {
        'addr': '0x555555555004',
        'op': 'sub',
        'args': 'rsp, 32',
        'desc': 'Allocate stack space',
        'next': ['0x555555555008']
    },
    {
        'addr': '0x555555555008',
        'op': 'mov',
        'args': 'rax, [rbp-8]',
        'desc': 'Load node pointer',
        'next': ['0x55555555500c']
    },
    {
        'addr': '0x55555555500c',
        'op': 'test',
        'args': 'rax, rax',
        'desc': 'Check if null',
        'next': ['0x55555555500f', '0x555555555020']  # Branch
    },
    {
        'addr': '0x55555555500f',
        'op': 'je',
        'args': '.L_null',
        'desc': 'Jump if null',
        'next': ['0x555555555020']
    },
    {
        'addr': '0x555555555014',
        'op': 'call',
        'args': 'calculate_hawking_radiation',
        'desc': 'Compute Hawking radiation',
        'next': ['0x555555555019']
    },
    {
        'addr': '0x555555555019',
        'op': 'mov',
        'args': '[rbp-16], rax',
        'desc': 'Store radiation result',
        'next': ['0x55555555501d']
    },
    {
        'addr': '0x55555555501d',
        'op': 'call',
        'args': 'render_shadow',
        'desc': 'Render ASCII shadow',
        'next': ['0x555555555022']
    },
    {
        'addr': '0x555555555020',
        'op': 'xor',
        'args': 'eax, eax',
        'desc': 'Return 0 (null case)',
        'next': ['0x555555555022']
    },
    {
        'addr': '0x555555555022',
        'op': 'add',
        'args': 'rsp, 32',
        'desc': 'Deallocate stack',
        'next': ['0x555555555026']
    },
    {
        'addr': '0x555555555026',
        'op': 'pop',
        'args': 'rbp',
        'desc': 'Restore base pointer',
        'next': ['0x555555555027']
    },
    {
        'addr': '0x555555555027',
        'op': 'ret',
        'args': '',
        'desc': 'Return to caller',
        'next': []
    },
]

def generate_dot():
    """Generate Graphviz DOT format"""
    dot = '''digraph InstructionFlow {
    rankdir=TB;
    node [shape=box, style=filled, fillcolor=lightblue, fontname="Courier"];
    edge [fontname="Courier", fontsize=10];
    
    // Title
    labelloc="t";
    label="OSM Black Hole Fall - Instruction Flow\\nConformal Arrows Toward Singularity";
    fontsize=16;
    
'''
    
    # Add nodes
    for inst in instructions:
        label = f"{inst['addr']}\\n{inst['op']} {inst['args']}\\n{inst['desc']}"
        
        # Color based on instruction type
        if inst['op'] in ['call']:
            color = 'lightcoral'
        elif inst['op'] in ['je', 'jne', 'jmp']:
            color = 'lightyellow'
        elif inst['op'] in ['ret']:
            color = 'lightgreen'
        else:
            color = 'lightblue'
        
        dot += f'    "{inst["addr"]}" [label="{label}", fillcolor={color}];\n'
    
    dot += '\n'
    
    # Add edges
    for inst in instructions:
        for next_addr in inst['next']:
            # Find next instruction
            next_inst = next((i for i in instructions if i['addr'] == next_addr), None)
            if next_inst:
                # Label edge with flow type
                if inst['op'] in ['je', 'jne']:
                    label = 'branch'
                elif inst['op'] == 'call':
                    label = 'call'
                elif inst['op'] == 'ret':
                    label = 'return'
                else:
                    label = 'next'
                
                dot += f'    "{inst["addr"]}" -> "{next_addr}" [label="{label}"];\n'
    
    dot += '}\n'
    
    return dot

def generate_plantuml():
    """Generate PlantUML format"""
    puml = '''@startuml
title OSM Black Hole Fall - Instruction Flow
skinparam backgroundColor #0f0f1e
skinparam defaultFontColor #ffffff
skinparam activity {
  BackgroundColor #1a1a2e
  BorderColor #4a90e2
  FontColor #ffffff
}

start

'''
    
    for i, inst in enumerate(instructions):
        label = f"{inst['op']} {inst['args']}\\n{inst['desc']}"
        
        if inst['op'] == 'call':
            puml += f':🔵 **{inst["addr"]}**\\n{label};\n'
        elif inst['op'] in ['je', 'jne']:
            puml += f'if (🔶 **{inst["addr"]}**\\n{label}) then (yes)\n'
        elif inst['op'] == 'ret':
            puml += f':🟢 **{inst["addr"]}**\\n{label};\n'
        else:
            puml += f':⚪ **{inst["addr"]}**\\n{label};\n'
        
        # Handle branches
        if inst['op'] in ['je', 'jne'] and len(inst['next']) > 1:
            puml += 'else (no)\n'
    
    puml += '''
stop

@enduml
'''
    
    return puml

def dot_to_ascii(dot_content):
    """Convert DOT to ASCII using graph-easy"""
    try:
        # Try graph-easy first
        result = subprocess.run(
            ['graph-easy', '--input=dot', '--output=ascii'],
            input=dot_content.encode(),
            capture_output=True,
            timeout=10
        )
        if result.returncode == 0:
            return result.stdout.decode()
    except (FileNotFoundError, subprocess.TimeoutExpired):
        pass
    
    # Fallback: simple ASCII representation
    return generate_simple_ascii()

def generate_simple_ascii():
    """Generate simple ASCII flow"""
    ascii_art = '''
OSM Black Hole Fall - Instruction Flow
=======================================

    ┌─────────────────────────────────┐
    │ 0x555555555000                  │
    │ push rbp                        │
    │ Save base pointer               │
    └─────────────────────────────────┘
                  │
                  ▼
    ┌─────────────────────────────────┐
    │ 0x555555555001                  │
    │ mov rbp, rsp                    │
    │ Set up stack frame              │
    └─────────────────────────────────┘
                  │
                  ▼
    ┌─────────────────────────────────┐
    │ 0x555555555004                  │
    │ sub rsp, 32                     │
    │ Allocate stack space            │
    └─────────────────────────────────┘
                  │
                  ▼
    ┌─────────────────────────────────┐
    │ 0x555555555008                  │
    │ mov rax, [rbp-8]                │
    │ Load node pointer               │
    └─────────────────────────────────┘
                  │
                  ▼
    ┌─────────────────────────────────┐
    │ 0x55555555500c                  │
    │ test rax, rax                   │
    │ Check if null                   │
    └─────────────────────────────────┘
                  │
         ┌────────┴────────┐
         │                 │
         ▼                 ▼
    [null case]      [valid node]
         │                 │
         │                 ▼
         │    ┌─────────────────────────────────┐
         │    │ 0x555555555014                  │
         │    │ call calculate_hawking_radiation│
         │    │ Compute Hawking radiation       │
         │    └─────────────────────────────────┘
         │                 │
         │                 ▼
         │    ┌─────────────────────────────────┐
         │    │ 0x555555555019                  │
         │    │ mov [rbp-16], rax               │
         │    │ Store radiation result          │
         │    └─────────────────────────────────┘
         │                 │
         │                 ▼
         │    ┌─────────────────────────────────┐
         │    │ 0x55555555501d                  │
         │    │ call render_shadow              │
         │    │ Render ASCII shadow             │
         │    └─────────────────────────────────┘
         │                 │
         └─────────────────┘
                  │
                  ▼
    ┌─────────────────────────────────┐
    │ 0x555555555022                  │
    │ add rsp, 32                     │
    │ Deallocate stack                │
    └─────────────────────────────────┘
                  │
                  ▼
    ┌─────────────────────────────────┐
    │ 0x555555555026                  │
    │ pop rbp                         │
    │ Restore base pointer            │
    └─────────────────────────────────┘
                  │
                  ▼
    ┌─────────────────────────────────┐
    │ 0x555555555027                  │
    │ ret                             │
    │ Return to caller                │
    └─────────────────────────────────┘
                  │
                  ▼
            [End] 🕳️
'''
    return ascii_art

def plantuml_to_ascii(puml_content):
    """Convert PlantUML to ASCII"""
    try:
        # Try plantuml with ASCII output
        result = subprocess.run(
            ['plantuml', '-tutxt', '-pipe'],
            input=puml_content.encode(),
            capture_output=True,
            timeout=10
        )
        if result.returncode == 0:
            return result.stdout.decode()
    except (FileNotFoundError, subprocess.TimeoutExpired):
        pass
    
    # Fallback: return PlantUML source
    return puml_content

def main():
    print("🎨 INSTRUCTION FLOW GRAPH GENERATOR")
    print("=" * 60)
    print()
    
    # Generate DOT
    print("📊 Generating Graphviz DOT...")
    dot_content = generate_dot()
    with open('instruction_flow.dot', 'w') as f:
        f.write(dot_content)
    print("✓ Saved: instruction_flow.dot")
    
    # Generate PlantUML
    print()
    print("🌱 Generating PlantUML...")
    puml_content = generate_plantuml()
    with open('instruction_flow.puml', 'w') as f:
        f.write(puml_content)
    print("✓ Saved: instruction_flow.puml")
    
    # Convert DOT to ASCII
    print()
    print("🔤 Converting to ASCII...")
    ascii_content = dot_to_ascii(dot_content)
    with open('instruction_flow_ascii.txt', 'w') as f:
        f.write(ascii_content)
    print("✓ Saved: instruction_flow_ascii.txt")
    
    # Display ASCII
    print()
    print("=" * 60)
    print(ascii_content)
    print("=" * 60)
    
    # Try to render DOT to SVG
    print()
    print("🎨 Rendering DOT to SVG...")
    try:
        subprocess.run(
            ['dot', '-Tsvg', 'instruction_flow.dot', '-o', 'instruction_flow.svg'],
            check=True,
            timeout=10
        )
        print("✓ Saved: instruction_flow.svg")
    except (FileNotFoundError, subprocess.CalledProcessError, subprocess.TimeoutExpired):
        print("⚠️  Graphviz not available, skipping SVG generation")
    
    print()
    print("✅ Graph generation complete!")
    print()
    print("Files generated:")
    print("  📄 instruction_flow.dot       - Graphviz DOT format")
    print("  📄 instruction_flow.puml      - PlantUML format")
    print("  📄 instruction_flow_ascii.txt - ASCII art")
    print("  📄 instruction_flow.svg       - SVG diagram (if dot available)")

if __name__ == '__main__':
    main()
