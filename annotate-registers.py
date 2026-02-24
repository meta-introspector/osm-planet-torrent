#!/usr/bin/env python3
# Add register value frequencies to instruction flow arrows

import re
from collections import defaultdict

# Sample register trace (would come from perf data)
register_trace = [
    {'addr': '0x555555555000', 'rax': 0x0, 'rbx': 0x42, 'rcx': 0x7fff12345678, 'rdx': 0x5},
    {'addr': '0x555555555001', 'rax': 0x0, 'rbx': 0x42, 'rcx': 0x7fff12345678, 'rdx': 0x5},
    {'addr': '0x555555555004', 'rax': 0x0, 'rbx': 0x42, 'rcx': 0x7fff12345678, 'rdx': 0x5},
    {'addr': '0x555555555008', 'rax': 0x7f1234567890, 'rbx': 0x42, 'rcx': 0x7fff12345678, 'rdx': 0x5},
    {'addr': '0x55555555500c', 'rax': 0x7f1234567890, 'rbx': 0x42, 'rcx': 0x7fff12345678, 'rdx': 0x5},
    {'addr': '0x555555555014', 'rax': 0x7f1234567890, 'rbx': 0x42, 'rcx': 0x7fff12345678, 'rdx': 0x5},
    {'addr': '0x555555555019', 'rax': 0x4e56789abcdef, 'rbx': 0x42, 'rcx': 0x7fff12345678, 'rdx': 0x5},
    {'addr': '0x55555555501d', 'rax': 0x4e56789abcdef, 'rbx': 0x42, 'rcx': 0x7fff12345678, 'rdx': 0x5},
    {'addr': '0x555555555022', 'rax': 0x0, 'rbx': 0x42, 'rcx': 0x7fff12345678, 'rdx': 0x5},
]

# Instructions with register usage
instructions = [
    {
        'addr': '0x555555555000',
        'op': 'push',
        'args': 'rbp',
        'desc': 'Save base pointer',
        'reads': [],
        'writes': ['rsp'],
        'next': ['0x555555555001']
    },
    {
        'addr': '0x555555555001',
        'op': 'mov',
        'args': 'rbp, rsp',
        'desc': 'Set up stack frame',
        'reads': ['rsp'],
        'writes': ['rbp'],
        'next': ['0x555555555004']
    },
    {
        'addr': '0x555555555004',
        'op': 'sub',
        'args': 'rsp, 32',
        'desc': 'Allocate stack space',
        'reads': ['rsp'],
        'writes': ['rsp'],
        'next': ['0x555555555008']
    },
    {
        'addr': '0x555555555008',
        'op': 'mov',
        'args': 'rax, [rbp-8]',
        'desc': 'Load node pointer',
        'reads': ['rbp'],
        'writes': ['rax'],
        'next': ['0x55555555500c']
    },
    {
        'addr': '0x55555555500c',
        'op': 'test',
        'args': 'rax, rax',
        'desc': 'Check if null',
        'reads': ['rax'],
        'writes': ['flags'],
        'next': ['0x55555555500f', '0x555555555020']
    },
    {
        'addr': '0x55555555500f',
        'op': 'je',
        'args': '.L_null',
        'desc': 'Jump if null',
        'reads': ['flags'],
        'writes': [],
        'next': ['0x555555555020']
    },
    {
        'addr': '0x555555555014',
        'op': 'call',
        'args': 'calculate_hawking_radiation',
        'desc': 'Compute Hawking radiation',
        'reads': ['rax', 'rdi', 'rsi'],
        'writes': ['rax'],
        'next': ['0x555555555019']
    },
    {
        'addr': '0x555555555019',
        'op': 'mov',
        'args': '[rbp-16], rax',
        'desc': 'Store radiation result',
        'reads': ['rax', 'rbp'],
        'writes': [],
        'next': ['0x55555555501d']
    },
    {
        'addr': '0x55555555501d',
        'op': 'call',
        'args': 'render_shadow',
        'desc': 'Render ASCII shadow',
        'reads': ['rax', 'rdi'],
        'writes': ['rax'],
        'next': ['0x555555555022']
    },
    {
        'addr': '0x555555555020',
        'op': 'xor',
        'args': 'eax, eax',
        'desc': 'Return 0 (null case)',
        'reads': [],
        'writes': ['rax'],
        'next': ['0x555555555022']
    },
    {
        'addr': '0x555555555022',
        'op': 'add',
        'args': 'rsp, 32',
        'desc': 'Deallocate stack',
        'reads': ['rsp'],
        'writes': ['rsp'],
        'next': ['0x555555555026']
    },
    {
        'addr': '0x555555555026',
        'op': 'pop',
        'args': 'rbp',
        'desc': 'Restore base pointer',
        'reads': ['rsp'],
        'writes': ['rbp'],
        'next': ['0x555555555027']
    },
    {
        'addr': '0x555555555027',
        'op': 'ret',
        'args': '',
        'desc': 'Return to caller',
        'reads': ['rsp'],
        'writes': [],
        'next': []
    },
]

def decode_value(value):
    """Decode register value to meaningful representation"""
    if value == 0:
        return "NULL"
    elif value == 0x42:
        return "42 (Answer)"
    elif value & 0x7fff000000000000:
        return f"Stack@{hex(value)}"
    elif value & 0x7f0000000000:
        return f"Heap@{hex(value)}"
    elif value & 0x555500000000:
        return f"Code@{hex(value)}"
    else:
        return hex(value)

def calculate_frequencies():
    """Calculate register value frequencies"""
    frequencies = defaultdict(lambda: defaultdict(int))
    
    for trace in register_trace:
        for reg, val in trace.items():
            if reg != 'addr':
                frequencies[reg][val] += 1
    
    return frequencies

def get_edge_annotation(inst, next_addr, frequencies):
    """Generate edge annotation with register frequencies"""
    annotations = []
    
    # Get register values at this instruction
    trace = next((t for t in register_trace if t['addr'] == inst['addr']), None)
    if not trace:
        return ""
    
    # Annotate with register values that change
    for reg in inst['writes']:
        if reg in trace and reg != 'flags':
            val = trace[reg]
            decoded = decode_value(val)
            freq = frequencies[reg][val]
            annotations.append(f"{reg}={decoded} (×{freq})")
    
    return "\\n".join(annotations)

def generate_annotated_dot():
    """Generate DOT with register frequencies on arrows"""
    frequencies = calculate_frequencies()
    
    dot = '''digraph InstructionFlowAnnotated {
    rankdir=TB;
    node [shape=box, style=filled, fillcolor=lightblue, fontname="Courier", fontsize=10];
    edge [fontname="Courier", fontsize=8, color="#4a90e2"];
    
    // Title
    labelloc="t";
    label="OSM Black Hole Fall - Instruction Flow with Register Frequencies\\nConformal Arrows Annotated with Register Values";
    fontsize=14;
    
    // Legend
    subgraph cluster_legend {
        label="Legend";
        style=filled;
        fillcolor=lightyellow;
        
        legend [shape=plaintext, label=<
            <table border="0" cellborder="1" cellspacing="0">
                <tr><td><b>Register Decoding</b></td></tr>
                <tr><td>NULL = 0x0</td></tr>
                <tr><td>42 = Answer to Everything</td></tr>
                <tr><td>Stack@ = Stack pointer</td></tr>
                <tr><td>Heap@ = Heap allocation</td></tr>
                <tr><td>Code@ = Code pointer</td></tr>
                <tr><td>(×N) = Frequency count</td></tr>
            </table>
        >];
    }
    
'''
    
    # Add nodes with register state
    for inst in instructions:
        trace = next((t for t in register_trace if t['addr'] == inst['addr']), None)
        
        # Build register state string
        reg_state = ""
        if trace:
            reg_vals = []
            for reg in ['rax', 'rbx', 'rcx', 'rdx']:
                if reg in trace:
                    val = trace[reg]
                    decoded = decode_value(val)
                    reg_vals.append(f"{reg}={decoded}")
            reg_state = "\\n" + " ".join(reg_vals[:2]) + "\\n" + " ".join(reg_vals[2:])
        
        label = f"{inst['addr']}\\n{inst['op']} {inst['args']}\\n{inst['desc']}{reg_state}"
        
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
    
    # Add edges with register annotations
    for inst in instructions:
        for next_addr in inst['next']:
            annotation = get_edge_annotation(inst, next_addr, frequencies)
            
            # Label edge with flow type and register changes
            if inst['op'] in ['je', 'jne']:
                label = f"branch\\n{annotation}" if annotation else "branch"
                style = "dashed"
            elif inst['op'] == 'call':
                label = f"call\\n{annotation}" if annotation else "call"
                style = "bold"
            elif inst['op'] == 'ret':
                label = "return"
                style = "dotted"
            else:
                label = annotation if annotation else "next"
                style = "solid"
            
            dot += f'    "{inst["addr"]}" -> "{next_addr}" [label="{label}", style={style}];\n'
    
    dot += '}\n'
    
    return dot

def generate_annotated_ascii():
    """Generate ASCII with register frequencies"""
    frequencies = calculate_frequencies()
    
    ascii_art = '''
OSM Black Hole Fall - Instruction Flow with Register Frequencies
=================================================================

Legend:
  NULL = 0x0
  42 = Answer to Everything
  Stack@ = Stack pointer (0x7fff...)
  Heap@ = Heap allocation (0x7f...)
  Code@ = Code pointer (0x5555...)
  (×N) = Frequency count

'''
    
    for inst in instructions:
        trace = next((t for t in register_trace if t['addr'] == inst['addr']), None)
        
        # Instruction box
        ascii_art += f'''
    ┌─────────────────────────────────────────────┐
    │ {inst['addr']}                      │
    │ {inst['op']:4s} {inst['args']:30s} │
    │ {inst['desc']:43s} │
'''
        
        # Register state
        if trace:
            ascii_art += "    │ " + "─" * 43 + " │\n"
            for reg in ['rax', 'rbx', 'rcx', 'rdx']:
                if reg in trace:
                    val = trace[reg]
                    decoded = decode_value(val)
                    freq = frequencies[reg][val]
                    ascii_art += f"    │ {reg}: {decoded:30s} (×{freq:2d}) │\n"
        
        ascii_art += "    └─────────────────────────────────────────────┘\n"
        
        # Arrows with annotations
        if inst['next']:
            for next_addr in inst['next']:
                annotation = get_edge_annotation(inst, next_addr, frequencies)
                if annotation:
                    ascii_art += f"                  │ {annotation}\n"
                ascii_art += "                  ▼\n"
    
    ascii_art += '''
            [End] 🕳️ Monster Black Hole
'''
    
    return ascii_art

def generate_frequency_report():
    """Generate frequency analysis report"""
    frequencies = calculate_frequencies()
    
    report = '''
Register Value Frequency Analysis
==================================

'''
    
    for reg, values in sorted(frequencies.items()):
        report += f"\n{reg.upper()}:\n"
        report += "-" * 50 + "\n"
        
        # Sort by frequency
        sorted_vals = sorted(values.items(), key=lambda x: x[1], reverse=True)
        
        for val, count in sorted_vals:
            decoded = decode_value(val)
            pct = (count / len(register_trace)) * 100
            bar = "█" * int(pct / 5)
            report += f"  {decoded:30s} : {count:3d} ({pct:5.1f}%) {bar}\n"
    
    return report

def main():
    print("📊 INSTRUCTION FLOW WITH REGISTER FREQUENCIES")
    print("=" * 60)
    print()
    
    # Generate annotated DOT
    print("📊 Generating annotated DOT...")
    dot_content = generate_annotated_dot()
    with open('instruction_flow_annotated.dot', 'w') as f:
        f.write(dot_content)
    print("✓ Saved: instruction_flow_annotated.dot")
    
    # Generate annotated ASCII
    print()
    print("🔤 Generating annotated ASCII...")
    ascii_content = generate_annotated_ascii()
    with open('instruction_flow_annotated.txt', 'w') as f:
        f.write(ascii_content)
    print("✓ Saved: instruction_flow_annotated.txt")
    
    # Generate frequency report
    print()
    print("📈 Generating frequency report...")
    report = generate_frequency_report()
    with open('register_frequencies.txt', 'w') as f:
        f.write(report)
    print("✓ Saved: register_frequencies.txt")
    
    # Display ASCII
    print()
    print("=" * 60)
    print(ascii_content)
    print("=" * 60)
    
    # Display frequency report
    print(report)
    
    # Render to SVG
    print()
    print("🎨 Rendering annotated DOT to SVG...")
    import subprocess
    try:
        subprocess.run(
            ['dot', '-Tsvg', 'instruction_flow_annotated.dot', '-o', 'instruction_flow_annotated.svg'],
            check=True,
            timeout=10
        )
        print("✓ Saved: instruction_flow_annotated.svg")
    except (FileNotFoundError, subprocess.CalledProcessError, subprocess.TimeoutExpired):
        print("⚠️  Graphviz not available, skipping SVG generation")
    
    print()
    print("✅ Annotated graph generation complete!")
    print()
    print("Files generated:")
    print("  📄 instruction_flow_annotated.dot - DOT with register frequencies")
    print("  📄 instruction_flow_annotated.txt - ASCII with annotations")
    print("  📄 instruction_flow_annotated.svg - SVG diagram")
    print("  📄 register_frequencies.txt      - Frequency analysis")

if __name__ == '__main__':
    main()
