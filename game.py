#!/usr/bin/env python3
"""A terminal dungeon crawler with a randomly generated grid map."""

import random
import sys

# ── Map generation ───────────────────────────────────────────────────────
WIDTH, HEIGHT = 7, 5
TILE_EMPTY = "."
TILE_ROOM  = " "
TILE_PLAYER = "@"
TILE_ENEMY = "E"
TILE_ITEM  = "?"
TILE_EXIT  = "X"
TILE_FOG   = "#"

ENEMIES = [
    {"name": "Goblin",  "hp": 3, "atk": 1},
    {"name": "Skeleton","hp": 4, "atk": 2},
    {"name": "Troll",   "hp": 6, "atk": 2},
    {"name": "Wraith",  "hp": 5, "atk": 3},
]

ITEMS = [
    ("sword",  "a Sword (+2 ATK)",     lambda p: p.update({"atk": p["atk"] + 2})),
    ("potion", "a Healing Potion (+5 HP)", lambda p: None),
    ("shield", "a Shield (+3 max HP)",  lambda p: p.update({"max_hp": p["max_hp"] + 3, "hp": p["hp"] + 3})),
    ("dagger", "a Dagger (+1 ATK)",     lambda p: p.update({"atk": p["atk"] + 1})),
]

def generate_dungeon():
    """Generate a random connected dungeon on a grid."""
    grid = [[False] * WIDTH for _ in range(HEIGHT)]
    rooms = {}

    # Carve rooms using random walk from top-left toward bottom-right
    r, c = 0, 0
    grid[r][c] = True
    path = [(r, c)]
    while (r, c) != (HEIGHT - 1, WIDTH - 1):
        dirs = []
        if r < HEIGHT - 1: dirs.append((1, 0))
        if c < WIDTH - 1:  dirs.append((0, 1))
        if r > 0:          dirs.append((-1, 0))
        if c > 0:          dirs.append((0, -1))
        weighted = []
        for dr, dc in dirs:
            weight = 3 if (dr > 0 or dc > 0) else 1
            weighted.extend([(dr, dc)] * weight)
        dr, dc = random.choice(weighted)
        r, c = r + dr, c + dc
        if not grid[r][c]:
            grid[r][c] = True
            path.append((r, c))

    # Add extra random rooms for variety
    for _ in range(WIDTH * HEIGHT // 3):
        rr = random.randint(0, HEIGHT - 1)
        rc = random.randint(0, WIDTH - 1)
        if not grid[rr][rc]:
            for dr, dc in [(0,1),(0,-1),(1,0),(-1,0)]:
                nr, nc = rr + dr, rc + dc
                if 0 <= nr < HEIGHT and 0 <= nc < WIDTH and grid[nr][nc]:
                    grid[rr][rc] = True
                    break

    # Build room data
    for r in range(HEIGHT):
        for c in range(WIDTH):
            if grid[r][c]:
                rooms[(r, c)] = {"enemy": None, "item": None, "visited": False, "desc": ""}

    start = (0, 0)
    exit_pos = (HEIGHT - 1, WIDTH - 1)
    rooms[start]["desc"] = "The dungeon entrance. Faint light behind you."
    rooms[start]["visited"] = True
    rooms[exit_pos]["desc"] = "A grand door with golden runes — the EXIT!"

    # Place enemies
    placeable = [pos for pos in rooms if pos != start and pos != exit_pos]
    random.shuffle(placeable)
    n_enemies = max(2, len(placeable) // 3)
    for pos in placeable[:n_enemies]:
        rooms[pos]["enemy"] = dict(random.choice(ENEMIES))

    # Place items
    remaining = [pos for pos in placeable[n_enemies:]]
    random.shuffle(remaining)
    n_items = max(2, len(remaining) // 2)
    item_pool = list(ITEMS)
    random.shuffle(item_pool)
    for i, pos in enumerate(remaining[:n_items]):
        item = item_pool[i % len(item_pool)]
        rooms[pos]["item"] = item

    # Room descriptions
    descs = [
        "A damp stone chamber. Water drips from the ceiling.",
        "A dusty room with cobwebs in every corner.",
        "A narrow passage with scratch marks on the walls.",
        "A cold room. Your breath is visible.",
        "A musty chamber with broken furniture.",
        "Glowing mushrooms light this cavern.",
        "An old storage room with empty barrels.",
        "The walls are covered in strange runes.",
        "A crossroads of crumbling passages.",
        "A quiet alcove with a mossy floor.",
    ]
    for pos, room in rooms.items():
        if not room["desc"]:
            room["desc"] = random.choice(descs)

    return grid, rooms, start, exit_pos


# ── Display ──────────────────────────────────────────────────────────────
def draw_map(grid, rooms, player_pos, exit_pos, reveal_all=False):
    lines = []
    # Header
    lines.append("+" + "---+" * WIDTH)
    for r in range(HEIGHT):
        row_mid  = "|"
        row_bot  = "+"
        for c in range(WIDTH):
            pos = (r, c)
            # Cell content
            if pos == player_pos:
                cell = " @ "
            elif not grid[r][c]:
                cell = "///";
            elif not rooms[pos]["visited"] and not reveal_all:
                cell = " # "
            elif pos == exit_pos:
                cell = " X "
            elif rooms[pos]["enemy"]:
                cell = " E "
            elif rooms[pos]["item"]:
                cell = " ? "
            else:
                cell = "   "

            # Right wall: open if neighbor to the east is also a room
            if c < WIDTH - 1 and grid[r][c] and grid[r][c+1]:
                right = " "
            else:
                right = "|"
            row_mid += cell + right

            # Bottom wall: open if neighbor below is also a room
            if r < HEIGHT - 1 and grid[r][c] and grid[r+1][c]:
                bot = "   "
            else:
                bot = "---"
            row_bot += bot + "+"

        lines.append(row_mid)
        lines.append(row_bot)

    lines.append("")
    lines.append("@ = You   X = Exit   E = Enemy   ? = Item   # = Unexplored   /// = Wall")
    return "\n".join(lines)


# ── Game ─────────────────────────────────────────────────────────────────
def main():
    grid, rooms, start, exit_pos = generate_dungeon()
    player = {"hp": 12, "max_hp": 12, "atk": 2, "pos": start, "inv": []}

    def status():
        print(f"[HP: {player['hp']}/{player['max_hp']}  ATK: {player['atk']}  Items: {', '.join(player['inv']) or 'none'}]")

    def look():
        pos = player["pos"]
        room = rooms[pos]
        print(f"\n--- Room ({pos[0]},{pos[1]}) ---")
        print(room["desc"])
        if room["enemy"]:
            e = room["enemy"]
            print(f"  !! A {e['name']} is here! (HP:{e['hp']} ATK:{e['atk']})")
        if room["item"]:
            print(f"  You see {room['item'][1]} on the ground.")
        dirs = []
        for name, (dr, dc) in [("north",(-1,0)),("south",(1,0)),("east",(0,1)),("west",(0,-1))]:
            nr, nc = pos[0]+dr, pos[1]+dc
            if 0 <= nr < HEIGHT and 0 <= nc < WIDTH and grid[nr][nc]:
                dirs.append(name)
        print(f"  Exits: {', '.join(dirs)}")
        status()

    def move(direction):
        deltas = {"north":(-1,0),"south":(1,0),"east":(0,1),"west":(0,-1)}
        if direction not in deltas:
            print("Invalid direction."); return False
        dr, dc = deltas[direction]
        nr, nc = player["pos"][0]+dr, player["pos"][1]+dc
        if not (0 <= nr < HEIGHT and 0 <= nc < WIDTH and grid[nr][nc]):
            print("You can't go that way!"); return False
        room = rooms[player["pos"]]
        if room["enemy"]:
            print(f"The {room['enemy']['name']} blocks your way! Fight first!"); return False
        player["pos"] = (nr, nc)
        rooms[(nr, nc)]["visited"] = True
        return True

    def fight():
        room = rooms[player["pos"]]
        if not room["enemy"]:
            print("Nothing to fight here."); return
        e = room["enemy"]
        print(f"\n=== BATTLE: You vs {e['name']}! ===")
        while e["hp"] > 0 and player["hp"] > 0:
            dmg = random.randint(1, player["atk"])
            e["hp"] -= dmg
            print(f"  You deal {dmg} dmg -> {e['name']} HP: {max(0,e['hp'])}")
            if e["hp"] <= 0:
                print(f"  Victory! The {e['name']} is slain!")
                room["enemy"] = None
                return
            edmg = random.randint(1, e["atk"])
            player["hp"] -= edmg
            print(f"  {e['name']} deals {edmg} dmg -> Your HP: {max(0,player['hp'])}")
            if player["hp"] <= 0:
                print("\n  You have fallen... GAME OVER.")
                return

    def take():
        room = rooms[player["pos"]]
        if not room["item"]:
            print("Nothing to pick up."); return
        name, desc, effect = room["item"]
        player["inv"].append(name)
        if name == "potion":
            print(f"Picked up {desc}. Use with 'use potion'.")
        else:
            effect(player)
            print(f"Picked up and equipped {desc}!")
        room["item"] = None
        status()

    def use(item_name):
        if item_name == "potion" and "potion" in player["inv"]:
            old_hp = player["hp"]
            player["hp"] = min(player["hp"] + 5, player["max_hp"])
            player["inv"].remove("potion")
            print(f"Healed {player['hp']-old_hp} HP! (HP: {player['hp']}/{player['max_hp']})")
        else:
            print("You don't have that or can't use it.")

    # ── Start ────────────────────────────────────────────────────────────
    print()
    print("=" * 50)
    print("     DUNGEON CRAWLER - Random Grid Edition")
    print("=" * 50)
    print("Reach the EXIT [X] at the bottom-right to win!")
    print("Commands: north/south/east/west (or n/s/e/w)")
    print("          look, map, take, fight, use potion, quit")
    print()
    print(draw_map(grid, rooms, player["pos"], exit_pos))
    look()

    while player["hp"] > 0:
        try:
            cmd = input("\n> ").strip().lower()
        except (EOFError, KeyboardInterrupt):
            print("\nBye!"); break

        if not cmd:
            continue

        moved = False
        if cmd in ("n", "north"):   moved = move("north")
        elif cmd in ("s", "south"): moved = move("south")
        elif cmd in ("e", "east"):  moved = move("east")
        elif cmd in ("w", "west"):  moved = move("west")
        elif cmd == "map":
            print(draw_map(grid, rooms, player["pos"], exit_pos)); continue
        elif cmd == "revealmap":
            print(draw_map(grid, rooms, player["pos"], exit_pos, reveal_all=True)); continue
        elif cmd == "look":
            look(); continue
        elif cmd in ("take", "get"):
            take(); continue
        elif cmd in ("fight", "attack"):
            fight(); continue
        elif cmd.startswith("use "):
            use(cmd.split(" ",1)[1]); continue
        elif cmd in ("quit", "q"):
            print("Thanks for playing!"); break
        elif cmd == "help":
            print("Commands: n/s/e/w, look, map, take, fight, use potion, quit"); continue
        else:
            print("Unknown command. Type 'help'."); continue

        if moved:
            print()
            print(draw_map(grid, rooms, player["pos"], exit_pos))
            look()
            if player["pos"] == exit_pos:
                print("\n*** YOU FOUND THE EXIT! CONGRATULATIONS - YOU WIN! ***\n")
                break

    if player["hp"] <= 0:
        print("\n--- GAME OVER ---\n")
        print("Final map (revealed):")
        print(draw_map(grid, rooms, player["pos"], exit_pos, reveal_all=True))

if __name__ == "__main__":
    main()
