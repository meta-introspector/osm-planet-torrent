#!/usr/bin/env python3
# Generate PlantUML sequence and use case diagrams with meme annotations

# Memory addresses as memes
meme_addresses = {
    '0x555555555000': {'meme': '🎯 Entry Point', 'actor': 'CPU', 'desc': 'Function prologue begins'},
    '0x555555555001': {'meme': '🏗️ Stack Setup', 'actor': 'CPU', 'desc': 'Building the foundation'},
    '0x555555555004': {'meme': '📦 Allocate', 'actor': 'Memory', 'desc': 'Reserve 32 bytes'},
    '0x555555555008': {'meme': '🔍 Load Node', 'actor': 'Memory', 'desc': 'Fetch OSM node from heap'},
    '0x55555555500c': {'meme': '❓ Null Check', 'actor': 'CPU', 'desc': 'Is this the void?'},
    '0x55555555500f': {'meme': '🔀 Branch', 'actor': 'CPU', 'desc': 'Choose your path'},
    '0x555555555014': {'meme': '☢️ Hawking Radiation', 'actor': 'BlackHole', 'desc': 'Calculate quantum emission'},
    '0x555555555019': {'meme': '💾 Store Result', 'actor': 'Memory', 'desc': 'Save radiation data'},
    '0x55555555501d': {'meme': '🎨 Render Shadow', 'actor': 'Renderer', 'desc': 'Draw ASCII art'},
    '0x555555555020': {'meme': '🚫 Null Path', 'actor': 'CPU', 'desc': 'Return zero'},
    '0x555555555022': {'meme': '🧹 Cleanup', 'actor': 'Memory', 'desc': 'Deallocate stack'},
    '0x555555555026': {'meme': '🔙 Restore', 'actor': 'CPU', 'desc': 'Pop base pointer'},
    '0x555555555027': {'meme': '👋 Exit', 'actor': 'CPU', 'desc': 'Return to caller'},
}

def generate_sequence_diagram():
    """Generate PlantUML sequence diagram"""
    puml = '''@startuml
title OSM Black Hole Fall - Instruction Sequence
skinparam backgroundColor #0f0f1e
skinparam sequenceMessageAlign center
skinparam defaultFontColor #ffffff

actor "🌍 OSM Node" as Node
participant "🎯 CPU" as CPU
participant "💾 Memory" as Memory
participant "🕳️ BlackHole" as BH
participant "🎨 Renderer" as Render

== Function Entry ==
Node -> CPU: Call process_node()
activate CPU

CPU -> CPU: 🎯 Entry Point\\n0x555555555000\\npush rbp
note right: Save base pointer

CPU -> CPU: 🏗️ Stack Setup\\n0x555555555001\\nmov rbp, rsp
note right: Set up stack frame

CPU -> Memory: 📦 Allocate\\n0x555555555004\\nsub rsp, 32
activate Memory
Memory --> CPU: Stack space reserved
note right: 32 bytes allocated

== Node Processing ==
CPU -> Memory: 🔍 Load Node\\n0x555555555008\\nmov rax, [rbp-8]
Memory --> CPU: Node pointer
note right: rax = 0x7f1234567890

CPU -> CPU: ❓ Null Check\\n0x55555555500c\\ntest rax, rax
note right: Check if node exists

alt Node is NULL
    CPU -> CPU: 🚫 Null Path\\n0x555555555020\\nxor eax, eax
    note right: Return 0
else Node is Valid
    CPU -> BH: ☢️ Hawking Radiation\\n0x555555555014\\ncall calculate_hawking_radiation
    activate BH
    BH -> BH: Compute quantum emission
    note right: T = 1/(8πM)
    BH --> CPU: Radiation value
    deactivate BH
    note right: rax = 0x4e56789abcdef
    
    CPU -> Memory: 💾 Store Result\\n0x555555555019\\nmov [rbp-16], rax
    note right: Save to stack
    
    CPU -> Render: 🎨 Render Shadow\\n0x55555555501d\\ncall render_shadow
    activate Render
    Render -> Render: Generate ASCII art
    note right: ⭐ → ✨ → 💫 → ⚫
    Render --> CPU: Shadow rendered
    deactivate Render
end

== Function Exit ==
CPU -> Memory: 🧹 Cleanup\\n0x555555555022\\nadd rsp, 32
Memory --> CPU: Stack deallocated
deactivate Memory

CPU -> CPU: 🔙 Restore\\n0x555555555026\\npop rbp
note right: Restore base pointer

CPU -> Node: 👋 Exit\\n0x555555555027\\nret
deactivate CPU
note right: Return to caller

@enduml
'''
    return puml

def generate_use_case_diagram():
    """Generate PlantUML use case diagram"""
    puml = '''@startuml
title OSM Black Hole Fall - Use Case Diagram
skinparam backgroundColor #0f0f1e
skinparam defaultFontColor #ffffff
skinparam usecaseBackgroundColor #1a1a2e
skinparam usecaseBorderColor #4a90e2
skinparam actorBackgroundColor #2a2a3e
skinparam actorBorderColor #4a90e2

left to right direction

actor "🌍 OSM Node" as Node
actor "👤 User" as User
actor "🕳️ Monster\\nBlack Hole" as BH

rectangle "OSM Black Hole Fall System" {
    usecase "🎯 Entry Point\\n(0x555555555000)" as UC1
    usecase "🏗️ Setup Stack\\n(0x555555555001)" as UC2
    usecase "📦 Allocate Memory\\n(0x555555555004)" as UC3
    usecase "🔍 Load Node\\n(0x555555555008)" as UC4
    usecase "❓ Validate Node\\n(0x55555555500c)" as UC5
    usecase "☢️ Calculate\\nHawking Radiation\\n(0x555555555014)" as UC6
    usecase "💾 Store Result\\n(0x555555555019)" as UC7
    usecase "🎨 Render Shadow\\n(0x55555555501d)" as UC8
    usecase "🧹 Cleanup\\n(0x555555555022)" as UC9
    usecase "👋 Return\\n(0x555555555027)" as UC10
}

User --> UC1 : Initiates
Node --> UC4 : Provides data
BH --> UC6 : Computes radiation

UC1 ..> UC2 : <<include>>
UC2 ..> UC3 : <<include>>
UC3 ..> UC4 : <<include>>
UC4 ..> UC5 : <<include>>
UC5 ..> UC6 : <<extend>>\\n[valid node]
UC6 ..> UC7 : <<include>>
UC7 ..> UC8 : <<include>>
UC8 ..> UC9 : <<include>>
UC9 ..> UC10 : <<include>>

note right of UC6
  Hawking Temperature:
  T = ℏc³/(8πGMk_B)
  
  For Monster black hole:
  M = 8.08×10⁵³
end note

note right of UC8
  Shadow rendering:
  ⭐ Bright (>0.8)
  ✨ Medium (>0.5)
  💫 Faint (>0.2)
  ⚫ Dark (≤0.2)
end note

@enduml
'''
    return puml

def generate_component_diagram():
    """Generate PlantUML component diagram"""
    puml = '''@startuml
title OSM Black Hole Fall - Component Diagram
skinparam backgroundColor #0f0f1e
skinparam defaultFontColor #ffffff
skinparam componentBackgroundColor #1a1a2e
skinparam componentBorderColor #4a90e2

package "Memory Space" {
    component "🎯 Code Segment\\n0x5555..." as Code
    component "📦 Stack\\n0x7fff..." as Stack
    component "🗄️ Heap\\n0x7f..." as Heap
}

package "CPU Registers" {
    component "RAX\\nAccumulator" as RAX
    component "RBX\\n42 (Answer)" as RBX
    component "RCX\\nCounter" as RCX
    component "RDX\\nData (5)" as RDX
    component "RBP\\nBase Pointer" as RBP
    component "RSP\\nStack Pointer" as RSP
}

package "Functions" {
    component "☢️ calculate_hawking_radiation\\n0x555555555014" as Hawking
    component "🎨 render_shadow\\n0x55555555501d" as Shadow
}

package "External Systems" {
    component "🕳️ Monster Black Hole\\nM = 8.08×10⁵³" as Monster
    component "🌐 Asciinema\\nRecording" as Asciinema
}

Code --> RAX : Load instructions
Code --> Stack : Push/Pop
Stack --> RBP : Base reference
Stack --> RSP : Top reference

RAX --> Heap : Load node pointer
Heap --> RAX : Node data

Hawking --> Monster : Query mass
Monster --> Hawking : Radiation value
Hawking --> RAX : Return result

Shadow --> RAX : Read radiation
Shadow --> Asciinema : Write frame

RBX --> RBX : Always 42
RCX --> Heap : Stack pointer
RDX --> RDX : Always 5

note right of Monster
  Schwarzschild radius:
  r_s = 2GM/c²
  
  Event horizon at τ = i∞
end note

note right of Asciinema
  Frame format:
  {
    "time": eons,
    "data": "⭐ → ⚫"
  }
end note

@enduml
'''
    return puml

def generate_activity_diagram():
    """Generate PlantUML activity diagram"""
    puml = '''@startuml
title OSM Black Hole Fall - Activity Diagram
skinparam backgroundColor #0f0f1e
skinparam defaultFontColor #ffffff
skinparam activityBackgroundColor #1a1a2e
skinparam activityBorderColor #4a90e2

start

:🎯 **Entry Point**
0x555555555000
push rbp;
note right: Save base pointer

:🏗️ **Stack Setup**
0x555555555001
mov rbp, rsp;
note right: Set up stack frame

:📦 **Allocate Memory**
0x555555555004
sub rsp, 32;
note right: Reserve 32 bytes

:🔍 **Load Node**
0x555555555008
mov rax, [rbp-8];
note right
  rax = 0x7f1234567890
  Heap pointer to OSM node
end note

:❓ **Validate Node**
0x55555555500c
test rax, rax;

if (Node is NULL?) then (yes)
  :🚫 **Null Path**
  0x555555555020
  xor eax, eax;
  note right: Return 0
else (no)
  :☢️ **Calculate Hawking Radiation**
  0x555555555014
  call calculate_hawking_radiation;
  note right
    T = 1/(8πM)
    M = 8.08×10⁵³
    Result in rax
  end note
  
  :💾 **Store Result**
  0x555555555019
  mov [rbp-16], rax;
  note right: Save to stack
  
  :🎨 **Render Shadow**
  0x55555555501d
  call render_shadow;
  note right
    ASCII shadow:
    ⭐ → ✨ → 💫 → ⚫
  end note
endif

:🧹 **Cleanup**
0x555555555022
add rsp, 32;
note right: Deallocate stack

:🔙 **Restore**
0x555555555026
pop rbp;
note right: Restore base pointer

:👋 **Exit**
0x555555555027
ret;
note right: Return to caller

stop

@enduml
'''
    return puml

def generate_class_diagram():
    """Generate PlantUML class diagram"""
    puml = '''@startuml
title OSM Black Hole Fall - Memory Layout Class Diagram
skinparam backgroundColor #0f0f1e
skinparam defaultFontColor #ffffff
skinparam classBackgroundColor #1a1a2e
skinparam classBorderColor #4a90e2

class "🎯 Instruction @ 0x555555555000" as I1 {
    +opcode: push
    +operand: rbp
    +meme: Entry Point
    +execute()
}

class "🏗️ Instruction @ 0x555555555001" as I2 {
    +opcode: mov
    +operands: rbp, rsp
    +meme: Stack Setup
    +execute()
}

class "📦 Instruction @ 0x555555555004" as I3 {
    +opcode: sub
    +operands: rsp, 32
    +meme: Allocate
    +execute()
}

class "🔍 Instruction @ 0x555555555008" as I4 {
    +opcode: mov
    +operands: rax, [rbp-8]
    +meme: Load Node
    +execute()
}

class "☢️ Function @ 0x555555555014" as F1 {
    +name: calculate_hawking_radiation
    +input: node_mass
    +output: temperature
    +formula: T = 1/(8πM)
    +call()
}

class "🎨 Function @ 0x55555555501d" as F2 {
    +name: render_shadow
    +input: radiation
    +output: ascii_frame
    +symbols: ⭐✨💫⚫
    +call()
}

class "💾 Memory @ 0x7f1234567890" as M1 {
    +type: Heap
    +content: OSM Node
    +size: variable
    +read()
    +write()
}

class "📦 Memory @ 0x7fff87654300" as M2 {
    +type: Stack
    +content: Local variables
    +size: 32 bytes
    +push()
    +pop()
}

class "🕳️ BlackHole" as BH {
    +mass: 8.08e53
    +radius: 1.62e54
    +temperature: 4.95e-56 K
    +emit_radiation()
}

I1 --> I2 : next
I2 --> I3 : next
I3 --> I4 : next
I4 --> F1 : call
F1 --> F2 : call

I4 --> M1 : read
F1 --> BH : query
F2 --> M2 : write

note right of BH
  Monster group singularity
  |M| = 2⁴⁶ × 3²⁰ × ... × 71
end note

@enduml
'''
    return puml

def main():
    print("🎨 PLANTUML DIAGRAM GENERATOR WITH MEME ANNOTATIONS")
    print("=" * 60)
    print()
    
    # Generate sequence diagram
    print("📊 Generating sequence diagram...")
    seq = generate_sequence_diagram()
    with open('instruction_sequence.puml', 'w') as f:
        f.write(seq)
    print("✓ Saved: instruction_sequence.puml")
    
    # Generate use case diagram
    print()
    print("🎯 Generating use case diagram...")
    uc = generate_use_case_diagram()
    with open('instruction_usecase.puml', 'w') as f:
        f.write(uc)
    print("✓ Saved: instruction_usecase.puml")
    
    # Generate component diagram
    print()
    print("🔧 Generating component diagram...")
    comp = generate_component_diagram()
    with open('instruction_component.puml', 'w') as f:
        f.write(comp)
    print("✓ Saved: instruction_component.puml")
    
    # Generate activity diagram
    print()
    print("⚡ Generating activity diagram...")
    act = generate_activity_diagram()
    with open('instruction_activity.puml', 'w') as f:
        f.write(act)
    print("✓ Saved: instruction_activity.puml")
    
    # Generate class diagram
    print()
    print("📦 Generating class diagram...")
    cls = generate_class_diagram()
    with open('instruction_class.puml', 'w') as f:
        f.write(cls)
    print("✓ Saved: instruction_class.puml")
    
    # Display meme annotations
    print()
    print("=" * 60)
    print("MEME ANNOTATIONS")
    print("=" * 60)
    for addr, info in sorted(meme_addresses.items()):
        print(f"{info['meme']:20s} {addr} - {info['desc']}")
    
    print()
    print("✅ PlantUML diagram generation complete!")
    print()
    print("Files generated:")
    print("  📄 instruction_sequence.puml  - Sequence diagram")
    print("  📄 instruction_usecase.puml   - Use case diagram")
    print("  📄 instruction_component.puml - Component diagram")
    print("  📄 instruction_activity.puml  - Activity diagram")
    print("  📄 instruction_class.puml     - Class diagram")
    print()
    print("To render:")
    print("  plantuml instruction_*.puml")
    print("  # or")
    print("  cat instruction_sequence.puml | plantuml -pipe > sequence.png")

if __name__ == '__main__':
    main()
