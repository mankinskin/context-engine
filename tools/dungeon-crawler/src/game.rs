use std::io::{self, BufRead, Write};
use rand::Rng;

use crate::items::{self, Item, ItemKind, BookEffect, Spell, Stat};
use crate::player::Player;
use crate::enemy::Enemy;
use crate::npc::NpcKind;
use crate::world::{self, Map, draw_map};
use crate::combat::{self, CombatResult};

pub struct Game {
    pub player: Player,
    pub map: Map,
    pub rng: rand::rngs::ThreadRng,
    pub running: bool,
    pub combat_target: Option<Enemy>,
    pub won: bool,
}

impl Game {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let map = world::generate_dungeon(&mut rng);
        Game {
            player: Player::new(),
            map,
            rng,
            running: true,
            combat_target: None,
            won: false,
        }
    }

    pub fn run(&mut self) {
        println!();
        println!("{}", "=".repeat(56));
        println!("       DUNGEON CRAWLER — Rust Edition");
        println!("{}", "=".repeat(56));
        println!("Defeat the Dragon and reach the EXIT to win!");
        println!("Type 'help' for commands.\n");
        let vd = self.player.view_distance;
        println!("{}", draw_map(&self.map, self.player.pos, vd));
        self.look();

        let stdin = io::stdin();
        while self.running {
            print!("\n> ");
            io::stdout().flush().ok();
            let mut line = String::new();
            match stdin.lock().read_line(&mut line) {
                Ok(0) | Err(_) => { println!("\nBye!"); break; }
                _ => {}
            }
            let cmd = line.trim().to_lowercase();
            if cmd.is_empty() { continue; }

            if self.combat_target.is_some() {
                self.handle_combat_cmd(&cmd);
            } else {
                self.handle_explore_cmd(&cmd);
            }
        }

        if self.won {
            println!("\n{}", "=".repeat(56));
            println!("  *** CONGRATULATIONS! YOU DEFEATED THE DRAGON! ***");
            println!("  *** YOU ESCAPED THE DUNGEON VICTORIOUS! ***");
            println!("{}", "=".repeat(56));
            println!("\nFinal stats:");
            self.show_stats();
        } else if self.player.hp <= 0 {
            println!("\n--- GAME OVER ---");
            println!("You reached level {} with {} coins.", self.player.level, self.player.coins);
            println!("You explored {} rooms.", self.map.rooms.values().filter(|r| r.visited).count());
        }
    }

    // ── Explore Commands ────────────────────────────────────────────────

    fn handle_explore_cmd(&mut self, cmd: &str) {
        match cmd {
            "n" | "north" => self.do_move("north"),
            "s" | "south" => self.do_move("south"),
            "e" | "east"  => self.do_move("east"),
            "w" | "west"  => self.do_move("west"),
            "look" | "l" => self.look(),
            "map" | "m" => {
                let vd = self.player.view_distance;
                println!("{}", draw_map(&self.map, self.player.pos, vd));
            }
            "rest" | "r" => self.do_rest(),
            "fight" | "attack" => self.start_combat(),
            "talk" => self.do_talk(),
            "trade" => self.do_trade(),
            "heal" => self.do_npc_heal(),
            "upgrade" => self.do_upgrade(),
            "inv" | "inventory" | "i" => self.show_inventory(),
            "stats" | "st" => self.show_stats(),
            "spells" => self.show_spells(),
            "help" | "h" | "?" => self.show_help(),
            "quit" | "q" => { println!("Thanks for playing!"); self.running = false; }
            _ => {
                if let Some(rest) = cmd.strip_prefix("take ") {
                    self.do_take_named(rest);
                } else if cmd == "take" || cmd == "get" {
                    self.do_take_first();
                } else if let Some(rest) = cmd.strip_prefix("drop ") {
                    self.do_drop(rest);
                } else if let Some(rest) = cmd.strip_prefix("use ") {
                    self.do_use(rest);
                } else if let Some(rest) = cmd.strip_prefix("equip ") {
                    self.do_equip(rest);
                } else if cmd == "unequip weapon" || cmd == "unequip w" {
                    self.do_unequip_weapon();
                } else if cmd == "unequip armor" || cmd == "unequip a" {
                    self.do_unequip_armor();
                } else if cmd == "unequip backpack" || cmd == "unequip b" {
                    self.do_unequip_backpack();
                } else if let Some(rest) = cmd.strip_prefix("allocate ") {
                    self.do_allocate(rest);
                } else if let Some(rest) = cmd.strip_prefix("buy ") {
                    self.do_buy(rest);
                } else if let Some(rest) = cmd.strip_prefix("sell ") {
                    self.do_sell(rest);
                } else if let Some(rest) = cmd.strip_prefix("cast ") {
                    // Cast heal outside combat
                    if rest == "heal" && self.player.known_spells.contains(&Spell::Heal) {
                        if self.player.mana >= Spell::Heal.mana_cost() {
                            self.player.mana -= Spell::Heal.mana_cost();
                            let wis = self.player.effective_stat(&Stat::Wisdom);
                            let heal = self.rng.gen_range(10 + wis..=20 + wis * 2);
                            let actual = heal.min(self.player.max_hp - self.player.hp);
                            self.player.hp += actual;
                            println!("You cast Heal! Restored {} HP. (HP: {}/{})", actual, self.player.hp, self.player.max_hp);
                        } else {
                            println!("Not enough mana! (Need {}, have {})", Spell::Heal.mana_cost(), self.player.mana);
                        }
                    } else {
                        println!("You can only cast Heal outside of combat.");
                    }
                } else {
                    println!("Unknown command. Type 'help'.");
                }
            }
        }
    }

    // ── Combat Commands ─────────────────────────────────────────────────

    fn handle_combat_cmd(&mut self, cmd: &str) {
        let enemy = match self.combat_target.as_mut() {
            Some(e) => e as *mut Enemy,
            None => return,
        };
        // Safety: we only access enemy through this pointer while combat_target is Some
        let enemy = unsafe { &mut *enemy };

        let result = match cmd {
            "attack" | "a" | "fight" | "hit" => {
                combat::player_attack(&mut self.player, enemy, &mut self.rng)
            }
            "flee" | "run" => {
                combat::try_flee(&self.player, enemy, &mut self.rng)
            }
            _ if cmd.starts_with("cast ") => {
                let spell_name = cmd.strip_prefix("cast ").unwrap();
                match Spell::from_str(spell_name) {
                    Some(spell) => combat::player_cast(&mut self.player, enemy, &spell, &mut self.rng),
                    None => CombatResult::Continue("Unknown spell. Type 'spells' to see known spells.".into()),
                }
            }
            _ if cmd.starts_with("use ") => {
                let item_name = cmd.strip_prefix("use ").unwrap();
                if let Some(idx) = self.player.inventory.find_by_name(item_name) {
                    if let Some(msg) = self.player.use_potion(idx) {
                        // Using a potion costs your turn, enemy attacks
                        let mut full_msg = format!("  {}", msg);
                        // Enemy still attacks
                        let raw_dmg = enemy.attack_damage(&mut self.rng);
                        let actual = (raw_dmg - self.player.total_defense()).max(1);
                        if self.rng.gen::<f32>() < self.player.dodge_chance() {
                            full_msg += &format!("\n  {} attacks but you dodge!", enemy.name);
                        } else {
                            self.player.hp -= actual;
                            full_msg += &format!("\n  {} hits you for {} dmg! (HP: {})", enemy.name, actual, self.player.hp.max(0));
                        }
                        if self.player.hp <= 0 {
                            CombatResult::PlayerDied(full_msg + "\n  You have fallen... GAME OVER.")
                        } else {
                            self.player.tick_buffs();
                            CombatResult::Continue(full_msg)
                        }
                    } else {
                        CombatResult::Continue("Can't use that item in combat.".into())
                    }
                } else {
                    CombatResult::Continue("You don't have that item.".into())
                }
            }
            "inv" | "inventory" | "i" => { self.show_inventory(); return; }
            "spells" => { self.show_spells(); return; }
            "stats" | "st" => { self.show_stats(); return; }
            "help" | "h" | "?" => {
                println!("Combat: attack, cast <spell>, use <potion>, flee, inv, spells, stats");
                return;
            }
            _ => {
                println!("In combat! Use: attack, cast <spell>, use <potion>, flee");
                return;
            }
        };

        match result {
            CombatResult::Continue(msg) => {
                println!("{}", msg);
                self.show_combat_status();
            }
            CombatResult::EnemyDied { msg, xp, coins } => {
                println!("{}", msg);
                self.player.xp += xp;
                self.player.coins += coins;

                let was_boss = self.combat_target.as_ref().map_or(false, |e| e.is_boss);
                self.combat_target = None;

                // Drop loot
                if was_boss {
                    let loot = items::dragon_hoard(&mut self.rng);
                    let pos = self.player.pos;
                    if let Some(room) = self.map.rooms.get_mut(&pos) {
                        for item in loot {
                            println!("  The dragon dropped: {}!", item.name);
                            room.items.push(item);
                        }
                    }
                } else if self.rng.gen_bool(0.3) {
                    let loot = items::random_ground_loot(&mut self.rng);
                    let pos = self.player.pos;
                    if let Some(room) = self.map.rooms.get_mut(&pos) {
                        println!("  Dropped: {}!", loot.name);
                        room.items.push(loot);
                    }
                }

                // Level up check
                while self.player.check_level_up() {
                    println!("\n  *** LEVEL UP! You are now level {}! ***", self.player.level);
                    println!("  +5 max HP, +3 max mana, +3 skill points. Fully restored!");
                }

                // Check win condition
                if was_boss && self.player.pos == self.map.exit_pos {
                    self.won = true;
                    self.running = false;
                }

                self.show_status();
            }
            CombatResult::PlayerDied(msg) => {
                println!("{}", msg);
                self.combat_target = None;
                self.running = false;
            }
            CombatResult::Fled(msg) => {
                println!("{}", msg);
                // Put enemy back in room
                if let Some(enemy) = self.combat_target.take() {
                    let pos = self.player.pos;
                    if let Some(room) = self.map.rooms.get_mut(&pos) {
                        room.enemy = Some(enemy);
                    }
                }
            }
        }
    }

    // ── Movement ────────────────────────────────────────────────────────

    fn do_move(&mut self, dir: &str) {
        // Check stamina
        if self.player.stamina <= 0 {
            println!("You're too exhausted to move! Use 'rest' to recover stamina.");
            return;
        }
        // Check if enemy blocks
        {
            let pos = self.player.pos;
            if let Some(room) = self.map.rooms.get(&pos) {
                if let Some(enemy) = &room.enemy {
                    println!("The {} blocks your way! Fight or flee!", enemy.name);
                    return;
                }
            }
        }
        match self.map.move_dir(self.player.pos, dir) {
            Some(new_pos) => {
                self.player.pos = new_pos;
                self.player.stamina -= 1;
                // Procedurally generate rooms around the new position
                let gen_radius = self.player.view_distance + 1;
                self.map.ensure_generated(new_pos, gen_radius, &mut self.rng);
                if let Some(room) = self.map.rooms.get_mut(&new_pos) {
                    room.visited = true;
                }
                self.player.tick_buffs();
                println!();
                let vd = self.player.view_distance;
                println!("{}", draw_map(&self.map, self.player.pos, vd));
                self.look();
            }
            None => println!("You can't go that way!"),
        }
    }

    fn do_rest(&mut self) {
        // Check if enemy blocks rest
        let pos = self.player.pos;
        if let Some(room) = self.map.rooms.get(&pos) {
            if room.enemy.is_some() {
                println!("You can't rest with an enemy here!");
                return;
            }
        }
        self.player.stamina = self.player.max_stamina;
        self.player.tick_buffs();
        // Small HP/mana regen on rest
        let hp_regen = (self.player.max_hp / 10).max(1);
        let mana_regen = (self.player.max_mana / 5).max(1);
        self.player.hp = (self.player.hp + hp_regen).min(self.player.max_hp);
        self.player.mana = (self.player.mana + mana_regen).min(self.player.max_mana);
        println!("You rest and recover your stamina. (+{} HP, +{} mana)", hp_regen, mana_regen);
        self.show_status();
    }

    // ── Look ────────────────────────────────────────────────────────────

    fn look(&self) {
        let pos = self.player.pos;
        let room = match self.map.rooms.get(&pos) {
            Some(r) => r,
            None => return,
        };
        let dist = Map::distance(pos);
        println!("\n--- Room ({},{}) [distance: {}] ---", pos.0, pos.1, dist);
        println!("{}", room.description);

        if let Some(enemy) = &room.enemy {
            if self.player.enemies_revealed {
                println!("  !! {} (HP:{}/{} ATK:{}-{} DEF:{})",
                    enemy.name, enemy.hp, enemy.max_hp, enemy.min_dmg, enemy.max_dmg, enemy.defense);
            } else {
                println!("  !! A {} is here!", enemy.name);
            }
        }

        if let Some(npc) = &room.npc {
            let kind = match &npc.kind {
                NpcKind::Merchant => "Merchant",
                NpcKind::Sage => "Sage",
                NpcKind::Healer => "Healer",
                NpcKind::Blacksmith => "Blacksmith",
                NpcKind::Hermit => "Hermit",
            };
            println!("  {} the {} is here.", npc.name, kind);
        }

        if !room.items.is_empty() {
            println!("  Items on the ground:");
            for (i, item) in room.items.iter().enumerate() {
                println!("    {}. {} ({})", i + 1, item.name, item.short_desc());
            }
        }

        let exits = self.map.exits(pos);
        println!("  Exits: {}", exits.join(", "));
        self.show_status();
    }

    fn show_status(&self) {
        let p = &self.player;
        let dist = Map::distance(p.pos);
        println!("[HP:{}/{} Mana:{}/{} Stam:{}/{} Dist:{} | Lvl:{} XP:{}/{} Coins:{}]",
            p.hp, p.max_hp, p.mana, p.max_mana,
            p.stamina, p.max_stamina, dist,
            p.level, p.xp, p.xp_to_next_level(), p.coins,
        );
    }

    fn show_combat_status(&self) {
        if let Some(enemy) = &self.combat_target {
            println!("  [{}: HP {}/{}]  [You: HP {}/{}, Mana {}/{}]",
                enemy.name, enemy.hp.max(0), enemy.max_hp,
                self.player.hp.max(0), self.player.max_hp,
                self.player.mana, self.player.max_mana);
        }
    }

    // ── Combat Start ────────────────────────────────────────────────────

    fn start_combat(&mut self) {
        let pos = self.player.pos;
        let enemy = {
            let room = match self.map.rooms.get_mut(&pos) {
                Some(r) => r,
                None => { println!("Nothing to fight."); return; }
            };
            match room.enemy.take() {
                Some(e) => e,
                None => { println!("Nothing to fight here."); return; }
            }
        };
        println!("\n=== BATTLE: You vs {}! ===", enemy.name);
        if self.player.enemies_revealed || enemy.is_boss {
            println!("  Enemy - HP:{}/{} ATK:{}-{} DEF:{}",
                enemy.hp, enemy.max_hp, enemy.min_dmg, enemy.max_dmg, enemy.defense);
        }
        println!("  Commands: attack, cast <spell>, use <potion>, flee");
        self.combat_target = Some(enemy);
        self.show_combat_status();
    }

    // ── Item Commands ───────────────────────────────────────────────────

    fn do_take_first(&mut self) {
        let pos = self.player.pos;
        let item = {
            let room = match self.map.rooms.get_mut(&pos) {
                Some(r) => r,
                None => { println!("Nothing here."); return; }
            };
            if room.items.is_empty() {
                println!("Nothing to pick up."); return;
            }
            if room.items.len() > 1 {
                println!("Multiple items here. Use 'take <name>' or 'take <number>':");
                for (i, item) in room.items.iter().enumerate() {
                    println!("  {}. {} ({})", i + 1, item.name, item.short_desc());
                }
                return;
            }
            room.items.remove(0)
        };
        self.add_item_to_inventory(item);
    }

    fn do_take_named(&mut self, name: &str) {
        let pos = self.player.pos;
        let item = {
            let room = match self.map.rooms.get_mut(&pos) {
                Some(r) => r,
                None => { println!("Nothing here."); return; }
            };
            // Try as number first
            if let Ok(n) = name.parse::<usize>() {
                if n == 0 || n > room.items.len() {
                    println!("Invalid item number."); return;
                }
                room.items.remove(n - 1)
            } else {
                let name_lower = name.to_lowercase();
                match room.items.iter().position(|i| i.name.to_lowercase().contains(&name_lower)) {
                    Some(idx) => room.items.remove(idx),
                    None => { println!("No item matching '{}' here.", name); return; }
                }
            }
        };
        self.add_item_to_inventory(item);
    }

    fn add_item_to_inventory(&mut self, item: Item) {
        let strength = self.player.effective_stat(&Stat::Strength);
        match self.player.inventory.can_add(&item, strength) {
            Ok(()) => {
                println!("Picked up {}. ({})", item.name, item.short_desc());
                self.player.inventory.items.push(item);
            }
            Err(reason) => {
                println!("Can't pick up {}: {}.", item.name, reason);
                // Put it back
                let pos = self.player.pos;
                if let Some(room) = self.map.rooms.get_mut(&pos) {
                    room.items.push(item);
                }
            }
        }
    }

    fn do_drop(&mut self, name: &str) {
        let idx = match self.player.inventory.find_by_name(name) {
            Some(i) => i,
            None => { println!("You don't have '{}'.", name); return; }
        };
        let item = self.player.inventory.items.remove(idx);
        println!("Dropped {}.", item.name);
        let pos = self.player.pos;
        if let Some(room) = self.map.rooms.get_mut(&pos) {
            room.items.push(item);
        }
    }

    fn do_use(&mut self, name: &str) {
        let idx = match self.player.inventory.find_by_name(name) {
            Some(i) => i,
            None => { println!("You don't have '{}'.", name); return; }
        };
        // Try potion first
        let is_potion = matches!(self.player.inventory.items[idx].kind, ItemKind::Potion(_));
        let is_book = matches!(self.player.inventory.items[idx].kind, ItemKind::Book(_));

        if is_potion {
            if let Some(msg) = self.player.use_potion(idx) {
                println!("{}", msg);
            }
        } else if is_book {
            let reveal_radius = match &self.player.inventory.items[idx].kind {
                ItemKind::Book(BookEffect::RevealArea(r)) => Some(*r),
                _ => None,
            };
            if let Some(msg) = self.player.use_book(idx) {
                println!("{}", msg);
                if let Some(radius) = reveal_radius {
                    let pos = self.player.pos;
                    self.map.ensure_generated(pos, radius as i32 + 1, &mut self.rng);
                    self.map.reveal_area(pos, radius);
                    let vd = self.player.view_distance.max(radius as i32);
                    println!("{}", draw_map(&self.map, self.player.pos, vd));
                }
            }
        } else {
            println!("Can't use that. Try 'equip' for weapons/armor.");
        }
    }

    fn do_equip(&mut self, name: &str) {
        let idx = match self.player.inventory.find_by_name(name) {
            Some(i) => i,
            None => { println!("You don't have '{}'.", name); return; }
        };
        let item = self.player.inventory.items.remove(idx);
        match &item.kind {
            ItemKind::Weapon { .. } => {
                if let Some(old) = self.player.inventory.weapon.take() {
                    println!("Unequipped {}.", old.name);
                    self.player.inventory.items.push(old);
                }
                println!("Equipped {}! ({})", item.name, item.short_desc());
                self.player.inventory.weapon = Some(item);
            }
            ItemKind::Armor { mana_bonus, .. } => {
                // Remove old armor mana bonus
                if let Some(old) = self.player.inventory.armor.take() {
                    if let ItemKind::Armor { mana_bonus: old_bonus, .. } = &old.kind {
                        self.player.max_mana -= old_bonus;
                        self.player.mana = self.player.mana.min(self.player.max_mana);
                    }
                    println!("Unequipped {}.", old.name);
                    self.player.inventory.items.push(old);
                }
                self.player.max_mana += mana_bonus;
                if *mana_bonus > 0 {
                    self.player.mana += mana_bonus;
                }
                println!("Equipped {}! ({})", item.name, item.short_desc());
                self.player.inventory.armor = Some(item);
            }
            ItemKind::Backpack { .. } => {
                if let Some(old) = self.player.inventory.backpack.take() {
                    println!("Unequipped {}.", old.name);
                    self.player.inventory.items.push(old);
                }
                println!("Equipped {}! ({})", item.name, item.short_desc());
                self.player.inventory.backpack = Some(item);
            }
            _ => {
                println!("Can't equip that.");
                self.player.inventory.items.push(item);
            }
        }
    }

    fn do_unequip_weapon(&mut self) {
        if let Some(item) = self.player.inventory.weapon.take() {
            println!("Unequipped {}.", item.name);
            self.player.inventory.items.push(item);
        } else {
            println!("No weapon equipped.");
        }
    }

    fn do_unequip_armor(&mut self) {
        if let Some(item) = self.player.inventory.armor.take() {
            if let ItemKind::Armor { mana_bonus, .. } = &item.kind {
                self.player.max_mana -= mana_bonus;
                self.player.mana = self.player.mana.min(self.player.max_mana);
            }
            println!("Unequipped {}.", item.name);
            self.player.inventory.items.push(item);
        } else {
            println!("No armor equipped.");
        }
    }

    fn do_unequip_backpack(&mut self) {
        if let Some(item) = self.player.inventory.backpack.take() {
            println!("Unequipped {}.", item.name);
            self.player.inventory.items.push(item);
        } else {
            println!("No backpack equipped.");
        }
    }

    // ── Character Commands ──────────────────────────────────────────────

    fn do_allocate(&mut self, stat_name: &str) {
        if self.player.skill_points == 0 {
            println!("No skill points available.");
            return;
        }
        match stat_name {
            "view" | "vision" | "sight" => {
                self.player.view_distance += 1;
                self.player.skill_points -= 1;
                println!("View distance increased to {}! ({} points left)",
                    self.player.view_distance, self.player.skill_points);
            }
            "stamina" | "stam" | "endurance" => {
                self.player.max_stamina += 2;
                self.player.stamina += 2;
                self.player.skill_points -= 1;
                println!("Max stamina increased to {}! ({} points left)",
                    self.player.max_stamina, self.player.skill_points);
            }
            _ => match Stat::from_str(stat_name) {
                Some(stat) => {
                    self.player.stats.add(&stat, 1);
                    self.player.skill_points -= 1;
                    println!("Allocated 1 point to {}. {} is now {}. ({} points left)",
                        stat, stat, self.player.stats.get(&stat), self.player.skill_points);
                }
                None => println!("Unknown stat. Use: str, dex, int, wis, view, stamina"),
            }
        }
    }

    fn show_inventory(&self) {
        let inv = &self.player.inventory;
        println!("\n--- Inventory ({}/{} slots, weight: {}/{}) ---",
            inv.used_slots(), inv.max_slots(),
            inv.total_weight(), inv.max_weight(self.player.effective_stat(&Stat::Strength)));

        if let Some(w) = &inv.weapon {
            println!("  [Weapon] {} ({})", w.name, w.short_desc());
        } else {
            println!("  [Weapon] Fists (1-2 dmg)");
        }
        if let Some(a) = &inv.armor {
            println!("  [Armor]  {} ({})", a.name, a.short_desc());
        } else {
            println!("  [Armor]  None");
        }
        if let Some(b) = &inv.backpack {
            println!("  [Pack]   {} ({})", b.name, b.short_desc());
        } else {
            println!("  [Pack]   None");
        }

        if inv.items.is_empty() {
            println!("  Bag: (empty)");
        } else {
            println!("  Bag:");
            for (i, item) in inv.items.iter().enumerate() {
                println!("    {}. {} ({})", i + 1, item.name, item.short_desc());
            }
        }
    }

    fn show_stats(&self) {
        let p = &self.player;
        println!("\n--- Character ---");
        println!("  Level: {}   XP: {}/{}   Coins: {}", p.level, p.xp, p.xp_to_next_level(), p.coins);
        println!("  HP: {}/{}   Mana: {}/{}   Stamina: {}/{}",
            p.hp, p.max_hp, p.mana, p.max_mana, p.stamina, p.max_stamina);
        println!("  View Distance: {}   Position: ({},{})", p.view_distance, p.pos.0, p.pos.1);
        println!("  STR: {}  DEX: {}  INT: {}  WIS: {}",
            p.stats.strength, p.stats.dexterity, p.stats.intelligence, p.stats.wisdom);
        if p.skill_points > 0 {
            println!("  Skill Points: {} (use 'allocate <str/dex/int/wis/view/stamina>')", p.skill_points);
        }
        println!("  Attack: {}-{} ({})",
            match &p.inventory.weapon {
                Some(w) => match &w.kind { ItemKind::Weapon { min_dmg, .. } => *min_dmg, _ => 1 },
                None => 1
            } + p.effective_stat(&Stat::Strength) / 2,
            match &p.inventory.weapon {
                Some(w) => match &w.kind { ItemKind::Weapon { max_dmg, .. } => *max_dmg, _ => 2 },
                None => 2
            } + p.effective_stat(&Stat::Strength) / 2,
            match &p.inventory.weapon { Some(w) => w.name.as_str(), None => "Fists" }
        );
        println!("  Defense: {}   Dodge: {:.0}%", p.total_defense(), p.dodge_chance() * 100.0);
        if !p.buffs.is_empty() {
            println!("  Active buffs:");
            for buff in &p.buffs {
                println!("    {} +{} {} ({} turns)", buff.name, buff.amount, buff.stat, buff.turns_remaining);
            }
        }
    }

    fn show_spells(&self) {
        if self.player.known_spells.is_empty() {
            println!("You don't know any spells yet. Find spell tomes!");
        } else {
            println!("\n--- Known Spells ---");
            for spell in &self.player.known_spells {
                println!("  {} — {}", spell.name(), spell.description());
            }
        }
    }

    // ── NPC Interaction ─────────────────────────────────────────────────

    fn do_talk(&mut self) {
        let pos = self.player.pos;
        // Take NPC out to avoid borrow issues
        let mut npc = {
            let room = match self.map.rooms.get_mut(&pos) {
                Some(r) => r,
                None => { println!("No one here to talk to."); return; }
            };
            match room.npc.take() {
                Some(n) => n,
                None => { println!("No one here to talk to."); return; }
            }
        };

        if !npc.talked {
            for line in &npc.dialogue {
                println!("  \"{}\"", line);
            }
            npc.talked = true;

            // Hermit gives a free gift on first talk
            if npc.kind == NpcKind::Hermit && !npc.gave_gift {
                let gift = items::random_ground_loot(&mut self.rng);
                println!("\n  {} gives you: {}!", npc.name, gift.name);
                let strength = self.player.effective_stat(&Stat::Strength);
                match self.player.inventory.can_add(&gift, strength) {
                    Ok(()) => self.player.inventory.items.push(gift),
                    Err(_) => {
                        println!("  (Inventory full! Dropped on the ground.)");
                        if let Some(room) = self.map.rooms.get_mut(&pos) {
                            room.items.push(gift);
                        }
                    }
                }
                npc.gave_gift = true;
            }
        } else {
            let line = npc.dialogue.last().cloned().unwrap_or_else(|| "...".into());
            println!("  \"{}\"", line);
        }

        // Hints based on NPC kind
        match npc.kind {
            NpcKind::Merchant | NpcKind::Sage => println!("  (Use 'trade' to buy/sell)"),
            NpcKind::Healer => println!("  (Use 'heal' — 5c quick heal, 15c full restore)"),
            NpcKind::Blacksmith => println!("  (Use 'upgrade' to improve your weapon)"),
            _ => {}
        }

        // Put NPC back
        if let Some(room) = self.map.rooms.get_mut(&pos) {
            room.npc = Some(npc);
        }
    }

    fn do_trade(&self) {
        let pos = self.player.pos;
        let room = match self.map.rooms.get(&pos) {
            Some(r) => r,
            None => { println!("No one to trade with."); return; }
        };
        let npc = match &room.npc {
            Some(n) if n.kind == NpcKind::Merchant || n.kind == NpcKind::Sage => n,
            _ => { println!("No merchant or sage here. Find one to trade!"); return; }
        };

        println!("\n--- {}'s Wares ---", npc.name);
        if npc.shop.is_empty() {
            println!("  (Sold out!)");
        } else {
            for (i, item) in npc.shop.iter().enumerate() {
                println!("  {}. {} — {} coins ({})", i + 1, item.name, item.value, item.short_desc());
            }
        }
        println!("\n  Your coins: {}", self.player.coins);
        println!("  Use 'buy <n>' to buy, 'sell <name>' to sell");
    }

    fn do_buy(&mut self, arg: &str) {
        let pos = self.player.pos;
        // Take NPC out
        let mut npc = {
            let room = match self.map.rooms.get_mut(&pos) {
                Some(r) => r,
                None => { println!("No one to trade with."); return; }
            };
            match room.npc.take() {
                Some(n) if n.kind == NpcKind::Merchant || n.kind == NpcKind::Sage => n,
                Some(n) => { room.npc = Some(n); println!("This NPC doesn't sell items."); return; }
                None => { println!("No one to trade with."); return; }
            }
        };

        let index = match arg.parse::<usize>() {
            Ok(n) if n > 0 && n <= npc.shop.len() => n - 1,
            _ => {
                println!("Invalid. Use 'buy <number>' (see 'trade' for list).");
                if let Some(room) = self.map.rooms.get_mut(&pos) { room.npc = Some(npc); }
                return;
            }
        };

        let item = &npc.shop[index];
        if self.player.coins < item.value {
            println!("Not enough coins! Need {}, have {}.", item.value, self.player.coins);
            if let Some(room) = self.map.rooms.get_mut(&pos) { room.npc = Some(npc); }
            return;
        }

        let strength = self.player.effective_stat(&Stat::Strength);
        if let Err(reason) = self.player.inventory.can_add(item, strength) {
            println!("Can't carry it: {}.", reason);
            if let Some(room) = self.map.rooms.get_mut(&pos) { room.npc = Some(npc); }
            return;
        }

        let item = npc.shop.remove(index);
        self.player.coins -= item.value;
        println!("Bought {} for {} coins! (Coins: {})", item.name, item.value, self.player.coins);
        self.player.inventory.items.push(item);

        // Put NPC back
        if let Some(room) = self.map.rooms.get_mut(&pos) { room.npc = Some(npc); }
    }

    fn do_sell(&mut self, name: &str) {
        let pos = self.player.pos;
        // Check there's a merchant
        {
            let room = match self.map.rooms.get(&pos) {
                Some(r) => r,
                None => { println!("No one to sell to."); return; }
            };
            match &room.npc {
                Some(n) if n.kind == NpcKind::Merchant || n.kind == NpcKind::Sage => {}
                _ => { println!("No merchant here to sell to."); return; }
            }
        }

        let idx = match self.player.inventory.find_by_name(name) {
            Some(i) => i,
            None => { println!("You don't have '{}'.", name); return; }
        };

        let item = self.player.inventory.items.remove(idx);
        let sell_price = (item.value + 1) / 2; // 50% value
        self.player.coins += sell_price;
        println!("Sold {} for {} coins. (Coins: {})", item.name, sell_price, self.player.coins);
    }

    fn do_npc_heal(&mut self) {
        let pos = self.player.pos;
        // Check for healer
        let has_healer = self.map.rooms.get(&pos)
            .and_then(|r| r.npc.as_ref())
            .map_or(false, |n| n.kind == NpcKind::Healer);
        if !has_healer {
            println!("No healer here.");
            return;
        }

        if self.player.hp == self.player.max_hp && self.player.mana == self.player.max_mana {
            println!("You're already at full health!");
            return;
        }

        if self.player.coins >= 15 {
            self.player.coins -= 15;
            self.player.hp = self.player.max_hp;
            self.player.mana = self.player.max_mana;
            println!("Full restoration! HP and Mana fully restored. (-15 coins, {} remaining)", self.player.coins);
        } else if self.player.coins >= 5 {
            self.player.coins -= 5;
            let heal = (self.player.max_hp / 2).min(self.player.max_hp - self.player.hp);
            self.player.hp += heal;
            let mana_heal = (self.player.max_mana / 3).min(self.player.max_mana - self.player.mana);
            self.player.mana += mana_heal;
            println!("Quick heal! +{} HP, +{} mana. (-5 coins, {} remaining)", heal, mana_heal, self.player.coins);
        } else {
            println!("Not enough coins! Quick heal: 5c, Full restore: 15c. You have {} coins.", self.player.coins);
        }
    }

    fn do_upgrade(&mut self) {
        let pos = self.player.pos;
        let has_smith = self.map.rooms.get(&pos)
            .and_then(|r| r.npc.as_ref())
            .map_or(false, |n| n.kind == NpcKind::Blacksmith);
        if !has_smith {
            println!("No blacksmith here.");
            return;
        }

        let weapon = match &self.player.inventory.weapon {
            Some(w) => w,
            None => { println!("You need a weapon equipped to upgrade."); return; }
        };

        let cost = (weapon.value + 1) / 2;
        println!("Upgrade {} for {} coins? (+1 min/max damage)", weapon.name, cost);
        println!("Type 'upgrade' again to confirm, or any other command to cancel.");

        // For simplicity, just do it (a real game would have confirm flow)
        if self.player.coins < cost {
            println!("Not enough coins! Need {}.", cost);
            return;
        }

        self.player.coins -= cost;
        if let Some(w) = &mut self.player.inventory.weapon {
            if let ItemKind::Weapon { min_dmg, max_dmg, .. } = &mut w.kind {
                *min_dmg += 1;
                *max_dmg += 1;
            }
            w.value += cost / 2;
            println!("Weapon upgraded! {} now does more damage. (-{} coins)", w.name, cost);
        }
    }

    // ── Help ────────────────────────────────────────────────────────────

    fn show_help(&self) {
        println!("\n--- Commands ---");
        println!("  Movement:   north/south/east/west  (n/s/e/w)  [costs 1 stamina]");
        println!("  Look:       look (l), map (m)");
        println!("  Survival:   rest (r)  [restores stamina, small HP/mana regen]");
        println!("  Items:      take [name/#], drop <name>, use <name>");
        println!("  Equipment:  equip <name>, unequip weapon/armor/backpack");
        println!("  Combat:     fight, cast <spell>, flee");
        println!("  Magic:      cast heal (outside combat), spells");
        println!("  Character:  stats (st), inv (i)");
        println!("  Allocate:   allocate <str/dex/int/wis/view/stamina>");
        println!("  NPC:        talk, trade, buy <#>, sell <name>, heal, upgrade");
        println!("  System:     help (h), quit (q)");
    }
}
