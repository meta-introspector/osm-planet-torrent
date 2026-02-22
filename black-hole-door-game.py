#!/usr/bin/env python3
# BBS Door Game: The Black Hole (1979 Disney Movie)
# ASCII art adventure game

import sys
import time
import random

# ASCII Art
TITLE_ART = r"""
╔═══════════════════════════════════════════════════════════════╗
║                                                               ║
║  ████████╗██╗  ██╗███████╗    ██████╗ ██╗      █████╗  ██████╗██╗  ██╗    ║
║  ╚══██╔══╝██║  ██║██╔════╝    ██╔══██╗██║     ██╔══██╗██╔════╝██║ ██╔╝    ║
║     ██║   ███████║█████╗      ██████╔╝██║     ███████║██║     █████╔╝     ║
║     ██║   ██╔══██║██╔══╝      ██╔══██╗██║     ██╔══██║██║     ██╔═██╗     ║
║     ██║   ██║  ██║███████╗    ██████╔╝███████╗██║  ██║╚██████╗██║  ██╗    ║
║     ╚═╝   ╚═╝  ╚═╝╚══════╝    ╚═════╝ ╚══════╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝    ║
║                                                               ║
║              ██╗  ██╗ ██████╗ ██╗     ███████╗               ║
║              ██║  ██║██╔═══██╗██║     ██╔════╝               ║
║              ███████║██║   ██║██║     █████╗                 ║
║              ██╔══██║██║   ██║██║     ██╔══╝                 ║
║              ██║  ██║╚██████╔╝███████╗███████╗               ║
║              ╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚══════╝               ║
║                                                               ║
║                    A BBS Door Game (1979)                     ║
║              "A Journey That Begins Where Everything Ends"    ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝
"""

USS_PALOMINO = r"""
        _____
    ___/     \___
   /   USS      \
  |   PALOMINO   |
   \___       ___/
       \_____/
      /  |  \
     /   |   \
    /____|____\
"""

CYGNUS = r"""
           ___
       ___/   \___
   ___/   CYGNUS  \___
  /                   \
 |  ╔═══════════════╗  |
 |  ║   MAXIMILIAN  ║  |
 |  ╚═══════════════╝  |
  \___________________/
      |||||||||||||
      |||||||||||||
"""

BLACK_HOLE = r"""
           ⚫⚫⚫⚫⚫
        ⚫⚫       ⚫⚫
      ⚫⚫           ⚫⚫
     ⚫               ⚫
    ⚫    🕳️ EVENT    ⚫
    ⚫     HORIZON    ⚫
     ⚫               ⚫
      ⚫⚫           ⚫⚫
        ⚫⚫       ⚫⚫
           ⚫⚫⚫⚫⚫
"""

MAXIMILIAN = r"""
    ╔═══════╗
    ║ ◉   ◉ ║
    ║   ▼   ║
    ╚═══════╝
   ╔═════════╗
   ║ ▓▓▓▓▓▓▓ ║
   ║ ▓▓▓▓▓▓▓ ║
   ╚═════════╝
    ║║║║║║║
    ║║║║║║║
"""

VINCENT = r"""
    ╔═══╗
    ║ ◉ ║
    ║ ▼ ║
    ╚═══╝
   ╔═════╗
   ║ ░░░ ║
   ╚═════╝
    ║║║
"""

def slow_print(text, delay=0.03):
    """Print text with typewriter effect"""
    for char in text:
        sys.stdout.write(char)
        sys.stdout.flush()
        time.sleep(delay)
    print()

def clear_screen():
    """Clear screen (BBS style)"""
    print("\033[2J\033[H", end="")

def wait_for_key():
    """Wait for user to press key"""
    input("\n[Press ENTER to continue]")

class BlackHoleGame:
    def __init__(self):
        self.player_name = ""
        self.ship_integrity = 100
        self.crew_morale = 100
        self.distance_to_hole = 1000
        self.has_vincent = True
        self.has_old_bob = True
        self.maximilian_defeated = False
        
    def intro(self):
        clear_screen()
        print(TITLE_ART)
        time.sleep(2)
        
        slow_print("\nWelcome to THE BLACK HOLE BBS Door Game!")
        slow_print("Based on the 1979 Disney film")
        print()
        
        self.player_name = input("Enter your name, Captain: ").strip() or "Captain"
        
        slow_print(f"\nWelcome aboard, Captain {self.player_name}!")
        wait_for_key()
        
    def scene_discovery(self):
        clear_screen()
        slow_print("═" * 60)
        slow_print("SCENE 1: DISCOVERY")
        slow_print("═" * 60)
        print()
        
        print(USS_PALOMINO)
        slow_print(f"\nCaptain {self.player_name}, we've detected a massive object ahead...")
        slow_print("It's the USS Cygnus! The ship that disappeared 20 years ago!")
        slow_print("But wait... there's something else...")
        print()
        print(BLACK_HOLE)
        slow_print("\nA BLACK HOLE! The Cygnus is on the edge of the event horizon!")
        
        wait_for_key()
        
    def scene_cygnus(self):
        clear_screen()
        slow_print("═" * 60)
        slow_print("SCENE 2: ABOARD THE CYGNUS")
        slow_print("═" * 60)
        print()
        
        print(CYGNUS)
        slow_print("\nYou board the Cygnus. The ship is eerily quiet...")
        slow_print("Suddenly, a figure emerges from the shadows...")
        print()
        print(MAXIMILIAN)
        slow_print("\nIt's MAXIMILIAN, Dr. Reinhardt's robot enforcer!")
        slow_print("Behind him, you see Dr. Hans Reinhardt...")
        slow_print("\nDr. Reinhardt: 'Welcome! I've been expecting you...'")
        slow_print("Dr. Reinhardt: 'I'm going to ride the Cygnus INTO the black hole!'")
        slow_print("Dr. Reinhardt: 'Join me, or die!'")
        
        print()
        print("What do you do?")
        print("1. Accept Reinhardt's offer")
        print("2. Refuse and try to escape")
        print("3. Challenge Maximilian to a fight")
        
        choice = input("\nYour choice (1-3): ").strip()
        
        if choice == "1":
            self.ending_bad()
        elif choice == "3":
            self.fight_maximilian()
        else:
            self.escape_sequence()
            
    def fight_maximilian(self):
        clear_screen()
        slow_print("═" * 60)
        slow_print("BATTLE: MAXIMILIAN")
        slow_print("═" * 60)
        print()
        
        print(MAXIMILIAN)
        print("     VS")
        print(VINCENT)
        
        slow_print("\nVINCENT steps forward to protect you!")
        slow_print("The two robots clash in an epic battle!")
        
        # Simple battle mechanic
        vincent_hp = 100
        max_hp = 150
        
        while vincent_hp > 0 and max_hp > 0:
            print()
            print(f"VINCENT HP: {vincent_hp}/100")
            print(f"MAXIMILIAN HP: {max_hp}/150")
            print()
            print("1. Laser blast")
            print("2. Dodge and counter")
            print("3. Use environment")
            
            choice = input("\nYour command: ").strip()
            
            if choice == "1":
                damage = random.randint(15, 25)
                max_hp -= damage
                slow_print(f"\nVINCENT fires! {damage} damage!")
            elif choice == "2":
                if random.random() > 0.5:
                    damage = random.randint(20, 30)
                    max_hp -= damage
                    slow_print(f"\nPerfect dodge! Counter attack: {damage} damage!")
                else:
                    damage = random.randint(10, 20)
                    vincent_hp -= damage
                    slow_print(f"\nMissed! Maximilian strikes: {damage} damage!")
            elif choice == "3":
                damage = random.randint(25, 35)
                max_hp -= damage
                slow_print(f"\nYou drop debris on Maximilian! {damage} damage!")
            
            if max_hp > 0:
                damage = random.randint(10, 20)
                vincent_hp -= damage
                slow_print(f"Maximilian attacks: {damage} damage!")
            
            time.sleep(1)
        
        if vincent_hp > 0:
            slow_print("\n🎉 VINCENT WINS!")
            slow_print("Maximilian is defeated!")
            self.maximilian_defeated = True
            wait_for_key()
            self.escape_sequence()
        else:
            slow_print("\n💀 VINCENT is destroyed!")
            self.has_vincent = False
            self.ending_bad()
            
    def escape_sequence(self):
        clear_screen()
        slow_print("═" * 60)
        slow_print("SCENE 3: ESCAPE!")
        slow_print("═" * 60)
        print()
        
        slow_print("The Cygnus is breaking apart!")
        slow_print("You must reach the Palomino before it's too late!")
        slow_print(f"\nDistance to Palomino: {self.distance_to_hole}m")
        slow_print(f"Ship Integrity: {self.ship_integrity}%")
        
        while self.distance_to_hole > 0 and self.ship_integrity > 0:
            print()
            print("1. Full speed ahead!")
            print("2. Navigate carefully")
            print("3. Help injured crew")
            
            choice = input("\nYour action: ").strip()
            
            if choice == "1":
                self.distance_to_hole -= random.randint(200, 300)
                self.ship_integrity -= random.randint(10, 20)
                slow_print("\nFull throttle! But taking damage!")
            elif choice == "2":
                self.distance_to_hole -= random.randint(100, 150)
                self.ship_integrity -= random.randint(5, 10)
                slow_print("\nCareful navigation...")
            elif choice == "3":
                self.crew_morale += 20
                self.distance_to_hole -= random.randint(50, 100)
                slow_print("\nCrew morale improved!")
            
            slow_print(f"Distance: {max(0, self.distance_to_hole)}m")
            slow_print(f"Integrity: {self.ship_integrity}%")
            
            if self.distance_to_hole <= 0:
                break
                
            time.sleep(1)
        
        if self.ship_integrity > 0:
            self.ending_good()
        else:
            self.ending_bad()
            
    def ending_good(self):
        clear_screen()
        slow_print("═" * 60)
        slow_print("ENDING: ESCAPE!")
        slow_print("═" * 60)
        print()
        
        slow_print("You made it to the Palomino!")
        slow_print("The ship breaks free from the black hole's gravity!")
        slow_print("\nAs you look back, you see the Cygnus falling into the event horizon...")
        slow_print("Dr. Reinhardt and Maximilian are consumed by the singularity...")
        print()
        print(BLACK_HOLE)
        slow_print("\nYou've survived THE BLACK HOLE!")
        slow_print(f"\nFinal Score: {self.ship_integrity + self.crew_morale}")
        
        if self.maximilian_defeated:
            slow_print("\n🏆 BONUS: Defeated Maximilian!")
        
        slow_print("\n✨ CONGRATULATIONS! ✨")
        
    def ending_bad(self):
        clear_screen()
        slow_print("═" * 60)
        slow_print("ENDING: INTO THE VOID")
        slow_print("═" * 60)
        print()
        
        slow_print("The gravitational pull is too strong...")
        slow_print("Your ship is drawn into the black hole...")
        print()
        print(BLACK_HOLE)
        slow_print("\nYou cross the event horizon...")
        slow_print("Time and space become meaningless...")
        slow_print("You see visions of heaven and hell...")
        slow_print("\n💀 GAME OVER 💀")
        
    def play(self):
        self.intro()
        self.scene_discovery()
        self.scene_cygnus()
        
        print()
        slow_print("Thanks for playing THE BLACK HOLE!")
        slow_print("A BBS Door Game by OSM Monster Systems")
        slow_print("\nPress ENTER to return to BBS...")
        input()

def main():
    game = BlackHoleGame()
    game.play()

if __name__ == "__main__":
    main()
