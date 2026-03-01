use std::collections::{HashMap, HashSet};
use rand::Rng;

use crate::items::{self, Item};
use crate::enemy::{self, Enemy};
use crate::npc::{self, Npc};

/// Position type: signed to support infinite expansion in all directions.
pub type Pos = (i32, i32);

// ── Room ────────────────────────────────────────────────────────────────

pub struct Room {
    pub description: String,
    pub enemy: Option<Enemy>,
    pub npc: Option<Npc>,
    pub items: Vec<Item>,
    pub visited: bool,
}

// ── Map ─────────────────────────────────────────────────────────────────

pub struct Map {
    pub rooms: HashMap<Pos, Room>,
    /// Positions we already decided about (room or wall).
    decided: HashSet<Pos>,
    /// Map seed for deterministic generation.
    pub seed: u64,
    /// Where the dragon boss guards the exit.
    pub exit_pos: Pos,
}

impl Map {
    /// Deterministic hash for a position — used to decide room existence & content.
    fn pos_hash(&self, pos: Pos) -> u64 {
        let mut h = self.seed;
        h = h.wrapping_add((pos.0 as u64).wrapping_mul(0x9E3779B97F4A7C15));
        h = h.wrapping_add((pos.1 as u64).wrapping_mul(0x517CC1B727220A95));
        h ^= h >> 30;
        h = h.wrapping_mul(0xBF58476D1CE4E5B9);
        h ^= h >> 27;
        h = h.wrapping_mul(0x94D049BB133111EB);
        h ^= h >> 31;
        h
    }

    /// Whether a room deterministically exists at `pos`.
    fn room_should_exist(&self, pos: Pos) -> bool {
        if pos == (0, 0) || pos == self.exit_pos { return true; }
        // 60% density
        (self.pos_hash(pos) % 100) < 60
    }

    /// Manhattan distance from origin.
    pub fn distance(pos: Pos) -> u32 {
        pos.0.unsigned_abs() + pos.1.unsigned_abs()
    }

    /// Difficulty tier (0-7) based on distance from origin.
    fn tier_at(pos: Pos) -> u32 {
        let d = Self::distance(pos);
        (d / 4).min(7)
    }

    /// Ensure all positions within `radius` of `center` have been decided.
    /// New rooms get populated with enemies, NPCs, and items.
    pub fn ensure_generated(&mut self, center: Pos, radius: i32, rng: &mut impl Rng) {
        let mut new_rooms: Vec<Pos> = Vec::new();

        for dr in -radius..=radius {
            for dc in -radius..=radius {
                let pos = (center.0 + dr, center.1 + dc);
                if self.decided.contains(&pos) {
                    continue;
                }
                self.decided.insert(pos);
                if self.room_should_exist(pos) {
                    let desc = ROOM_DESCS[self.pos_hash(pos) as usize % ROOM_DESCS.len()].to_string();
                    self.rooms.insert(pos, Room {
                        description: desc,
                        enemy: None,
                        npc: None,
                        items: Vec::new(),
                        visited: false,
                    });
                    new_rooms.push(pos);
                }
            }
        }

        // Populate new rooms (skip origin and exit — those are pre-built)
        for pos in new_rooms {
            if pos == (0, 0) || pos == self.exit_pos { continue; }
            let h = self.pos_hash(pos);
            let tier = Self::tier_at(pos);

            // NPC ~12%
            if (h >> 8) % 100 < 12 {
                if let Some(room) = self.rooms.get_mut(&pos) {
                    room.npc = Some(npc::random_npc(rng));
                }
                continue; // NPC rooms don't also get enemies
            }
            // Enemy ~40%
            if (h >> 16) % 100 < 40 {
                if let Some(room) = self.rooms.get_mut(&pos) {
                    room.enemy = Some(enemy::random_enemy(tier, rng));
                }
            }
            // Items ~25%
            if (h >> 24) % 100 < 25 {
                if let Some(room) = self.rooms.get_mut(&pos) {
                    room.items.push(items::random_ground_loot(rng));
                }
            }
        }
    }

    /// Reveal (mark visited) all rooms within `radius` of `center`.
    pub fn reveal_area(&mut self, center: Pos, radius: u32) {
        let r = radius as i32;
        for dr in -r..=r {
            for dc in -r..=r {
                let pos = (center.0 + dr, center.1 + dc);
                if let Some(room) = self.rooms.get_mut(&pos) {
                    room.visited = true;
                }
            }
        }
    }

    pub fn neighbors(&self, pos: Pos) -> Vec<(&'static str, Pos)> {
        let mut result = Vec::new();
        let (r, c) = pos;
        let dirs: [(&str, Pos); 4] = [
            ("north", (r - 1, c)),
            ("south", (r + 1, c)),
            ("west",  (r, c - 1)),
            ("east",  (r, c + 1)),
        ];
        for (name, npos) in dirs {
            if self.rooms.contains_key(&npos) {
                result.push((name, npos));
            }
        }
        result
    }

    pub fn exits(&self, pos: Pos) -> Vec<&'static str> {
        self.neighbors(pos).into_iter().map(|(name, _)| name).collect()
    }

    pub fn move_dir(&self, pos: Pos, dir: &str) -> Option<Pos> {
        let (r, c) = pos;
        let target = match dir {
            "north" | "n" => (r - 1, c),
            "south" | "s" => (r + 1, c),
            "west"  | "w" => (r, c - 1),
            "east"  | "e" => (r, c + 1),
            _ => return None,
        };
        if self.rooms.contains_key(&target) {
            Some(target)
        } else {
            None
        }
    }
}

// ── Room Descriptions ───────────────────────────────────────────────────

const ROOM_DESCS: &[&str] = &[
    "A damp stone chamber. Water drips from the ceiling.",
    "A dusty room with cobwebs in every corner.",
    "A narrow passage with deep scratch marks on the walls.",
    "A cold room. Your breath fogs in the air.",
    "A musty chamber with broken furniture scattered about.",
    "Glowing mushrooms light this cavern with eerie blue light.",
    "An old storage room with empty, rotting barrels.",
    "The walls are covered in strange, glowing runes.",
    "A crossroads of crumbling stone passages.",
    "A quiet alcove with a mossy floor and dripping stalactites.",
    "Torch brackets line the walls, but only darkness remains.",
    "A wide chamber with pillars supporting a vaulted ceiling.",
    "The air is thick with the smell of sulfur.",
    "Bones litter the floor of this grim chamber.",
    "A once-grand hall, now fallen to ruin.",
    "Crystal formations on the walls cast prismatic reflections.",
    "The ground slopes downward here. You feel a cold draft.",
    "Old mining equipment rusts in the corners of this cave.",
];

// ── Dungeon Generation ──────────────────────────────────────────────────

pub fn generate_dungeon(rng: &mut impl Rng) -> Map {
    let seed: u64 = rng.gen();

    // Place exit at Manhattan distance ~25-30 in a random direction
    let exit_dist = rng.gen_range(25..=30) as i32;
    let exit_r = rng.gen_range(-exit_dist..=exit_dist);
    let remaining = exit_dist - exit_r.abs();
    let exit_c = if rng.gen_bool(0.5) { remaining } else { -remaining };
    let exit_pos: Pos = (exit_r, exit_c);

    let mut map = Map {
        rooms: HashMap::new(),
        decided: HashSet::new(),
        seed,
        exit_pos,
    };

    // Pre-generate starting area (radius 4 around origin)
    map.ensure_generated((0, 0), 4, rng);

    // Set up start room
    if let Some(start) = map.rooms.get_mut(&(0, 0)) {
        start.description = "The dungeon entrance. Faint light filters in from behind you.".into();
        start.visited = true;
        start.enemy = None;
        start.npc = None;
        start.items = vec![items::rusty_dagger(), items::health_potion()];
    }

    // Pre-generate and set up exit room
    map.ensure_generated(exit_pos, 1, rng);
    if let Some(exit) = map.rooms.get_mut(&exit_pos) {
        exit.description = "A vast cavern. A DRAGON guards a massive golden door — the EXIT!".into();
        exit.enemy = Some(enemy::dragon());
        exit.npc = None;
    }

    map
}

// ── Map Display ─────────────────────────────────────────────────────────

/// Draw the map as ASCII art showing only rooms within `view_dist` of the player.
pub fn draw_map(map: &Map, player_pos: Pos, view_dist: i32) -> String {
    let mut lines = Vec::new();

    let min_r = player_pos.0 - view_dist;
    let max_r = player_pos.0 + view_dist;
    let min_c = player_pos.1 - view_dist;
    let max_c = player_pos.1 + view_dist;

    let width = (max_c - min_c + 1) as usize;

    // Top border
    let mut top = "+".to_string();
    for _ in 0..width {
        top += "---+";
    }
    lines.push(top);

    for r in min_r..=max_r {
        let mut row_mid = "|".to_string();
        let mut row_bot = "+".to_string();

        for c in min_c..=max_c {
            let pos: Pos = (r, c);
            // Cell content
            let cell = if pos == player_pos {
                " @ "
            } else if let Some(room) = map.rooms.get(&pos) {
                if !room.visited {
                    " # "
                } else if pos == map.exit_pos {
                    " X "
                } else if room.enemy.is_some() {
                    " E "
                } else if room.npc.is_some() {
                    " N "
                } else if !room.items.is_empty() {
                    " ? "
                } else {
                    "   "
                }
            } else {
                "///".into()
            };

            // Right wall: open passage if both cells have rooms
            let right = if c < max_c
                && map.rooms.contains_key(&pos)
                && map.rooms.contains_key(&(r, c + 1))
            {
                " "
            } else {
                "|"
            };
            row_mid += cell;
            row_mid += right;

            // Bottom wall
            let bot = if r < max_r
                && map.rooms.contains_key(&pos)
                && map.rooms.contains_key(&(r + 1, c))
            {
                "   "
            } else {
                "---"
            };
            row_bot += bot;
            row_bot += "+";
        }

        lines.push(row_mid);
        lines.push(row_bot);
    }

    lines.push(String::new());
    lines.push("@ You  X Exit  E Enemy  N NPC  ? Item  # Unexplored  /// Wall".into());
    lines.join("\n")
}
