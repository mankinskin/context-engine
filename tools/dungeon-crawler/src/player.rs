use crate::items::{Item, ItemKind, PotionEffect, BookEffect, Stat, Spell};

// ── Stats ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Stats {
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub wisdom: i32,
}

impl Stats {
    pub fn new() -> Self {
        Stats { strength: 5, dexterity: 5, intelligence: 5, wisdom: 5 }
    }

    pub fn get(&self, stat: &Stat) -> i32 {
        match stat {
            Stat::Strength => self.strength,
            Stat::Dexterity => self.dexterity,
            Stat::Intelligence => self.intelligence,
            Stat::Wisdom => self.wisdom,
        }
    }

    pub fn add(&mut self, stat: &Stat, amount: i32) {
        match stat {
            Stat::Strength => self.strength += amount,
            Stat::Dexterity => self.dexterity += amount,
            Stat::Intelligence => self.intelligence += amount,
            Stat::Wisdom => self.wisdom += amount,
        }
    }
}

// ── Buff ────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Buff {
    pub name: String,
    pub stat: Stat,
    pub amount: i32,
    pub turns_remaining: u32,
}

// ── Inventory ───────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub weapon: Option<Item>,
    pub armor: Option<Item>,
    pub backpack: Option<Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory { items: Vec::new(), weapon: None, armor: None, backpack: None }
    }

    pub fn max_slots(&self) -> usize {
        let base = 8;
        let bonus = match &self.backpack {
            Some(item) => match &item.kind {
                ItemKind::Backpack { extra_slots, .. } => *extra_slots,
                _ => 0,
            },
            None => 0,
        };
        base + bonus
    }

    pub fn used_slots(&self) -> usize {
        self.items.len()
    }

    pub fn total_weight(&self) -> u32 {
        let raw: u32 = self.items.iter().map(|i| i.weight).sum::<u32>()
            + self.weapon.as_ref().map_or(0, |i| i.weight)
            + self.armor.as_ref().map_or(0, |i| i.weight)
            + self.backpack.as_ref().map_or(0, |i| i.weight);
        let reduction = match &self.backpack {
            Some(item) => match &item.kind {
                ItemKind::Backpack { weight_reduction_pct, .. } => *weight_reduction_pct,
                _ => 0,
            },
            None => 0,
        };
        raw * (100 - reduction) / 100
    }

    pub fn max_weight(&self, strength: i32) -> u32 {
        (20 + strength * 3).max(0) as u32
    }

    pub fn can_add(&self, item: &Item, strength: i32) -> Result<(), &'static str> {
        if self.used_slots() >= self.max_slots() {
            return Err("Inventory full (no free slots)");
        }
        // Estimate weight after adding
        let new_weight = self.total_weight() + item.weight;
        if new_weight > self.max_weight(strength) {
            return Err("Too heavy to carry");
        }
        Ok(())
    }

    pub fn find_by_name(&self, name: &str) -> Option<usize> {
        let name_lower = name.to_lowercase();
        self.items.iter().position(|item| item.name.to_lowercase().contains(&name_lower))
    }
}

// ── Player ──────────────────────────────────────────────────────────────

pub struct Player {
    pub hp: i32,
    pub max_hp: i32,
    pub mana: i32,
    pub max_mana: i32,
    pub level: u32,
    pub xp: u32,
    pub skill_points: u32,
    pub coins: u32,
    pub stats: Stats,
    pub inventory: Inventory,
    pub known_spells: Vec<Spell>,
    pub buffs: Vec<Buff>,
    pub pos: (i32, i32),
    pub enemies_revealed: bool,
    pub defense_buff: i32, // temporary defense from Frost Shield
    pub defense_buff_turns: u32,
    // Survival mechanics
    pub stamina: i32,
    pub max_stamina: i32,
    pub view_distance: i32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            hp: 20,
            max_hp: 20,
            mana: 10,
            max_mana: 10,
            level: 1,
            xp: 0,
            skill_points: 0,
            coins: 15,
            stats: Stats::new(),
            inventory: Inventory::new(),
            known_spells: Vec::new(),
            buffs: Vec::new(),
            pos: (0, 0),
            enemies_revealed: false,
            defense_buff: 0,
            defense_buff_turns: 0,
            stamina: 5,
            max_stamina: 5,
            view_distance: 3,
        }
    }

    pub fn xp_to_next_level(&self) -> u32 {
        self.level * 100
    }

    pub fn check_level_up(&mut self) -> bool {
        if self.xp >= self.xp_to_next_level() {
            self.xp -= self.xp_to_next_level();
            self.level += 1;
            self.max_hp += 5;
            self.hp = self.max_hp;
            self.max_mana += 3;
            self.mana = self.max_mana;
            self.max_stamina += 1;
            self.stamina = self.max_stamina;
            self.skill_points += 3;
            true
        } else {
            false
        }
    }

    pub fn effective_stat(&self, stat: &Stat) -> i32 {
        let base = self.stats.get(stat);
        let buff_bonus: i32 = self.buffs.iter()
            .filter(|b| &b.stat == stat)
            .map(|b| b.amount)
            .sum();
        base + buff_bonus
    }

    pub fn attack_damage(&self, rng: &mut impl rand::Rng) -> i32 {
        let (min_dmg, max_dmg, scaling) = match &self.inventory.weapon {
            Some(item) => match &item.kind {
                ItemKind::Weapon { min_dmg, max_dmg, scaling } => (*min_dmg, *max_dmg, scaling.clone()),
                _ => (1, 2, Stat::Strength),
            },
            None => (1, 2, Stat::Strength),
        };
        let base = rng.gen_range(min_dmg..=max_dmg);
        let stat_bonus = self.effective_stat(&scaling) / 2;
        (base + stat_bonus).max(1)
    }

    pub fn total_defense(&self) -> i32 {
        let armor_def = match &self.inventory.armor {
            Some(item) => match &item.kind {
                ItemKind::Armor { defense, .. } => *defense,
                _ => 0,
            },
            None => 0,
        };
        armor_def + self.defense_buff
    }

    pub fn dodge_chance(&self) -> f32 {
        let dex = self.effective_stat(&Stat::Dexterity);
        (dex as f32 * 2.0).min(30.0) / 100.0
    }

    pub fn tick_buffs(&mut self) {
        for buff in &mut self.buffs {
            if buff.turns_remaining > 0 {
                buff.turns_remaining -= 1;
            }
        }
        self.buffs.retain(|b| b.turns_remaining > 0);
        if self.defense_buff_turns > 0 {
            self.defense_buff_turns -= 1;
            if self.defense_buff_turns == 0 {
                self.defense_buff = 0;
            }
        }
    }

    pub fn use_potion(&mut self, index: usize) -> Option<String> {
        if index >= self.inventory.items.len() {
            return None;
        }
        let item = &self.inventory.items[index];
        match &item.kind {
            ItemKind::Potion(effect) => {
                let msg = match effect {
                    PotionEffect::Health(amount) => {
                        let heal = (*amount).min(self.max_hp - self.hp);
                        self.hp += heal;
                        format!("Restored {} HP! (HP: {}/{})", heal, self.hp, self.max_hp)
                    }
                    PotionEffect::Mana(amount) => {
                        let restore = (*amount).min(self.max_mana - self.mana);
                        self.mana += restore;
                        format!("Restored {} mana! (Mana: {}/{})", restore, self.mana, self.max_mana)
                    }
                    PotionEffect::StrengthBuff { amount, turns } => {
                        self.buffs.push(Buff {
                            name: "Strength Elixir".into(),
                            stat: Stat::Strength,
                            amount: *amount,
                            turns_remaining: *turns,
                        });
                        format!("+{} STR for {} turns!", amount, turns)
                    }
                    PotionEffect::SwiftnessBuff { amount, turns } => {
                        self.buffs.push(Buff {
                            name: "Swiftness".into(),
                            stat: Stat::Dexterity,
                            amount: *amount,
                            turns_remaining: *turns,
                        });
                        format!("+{} DEX for {} turns!", amount, turns)
                    }
                };
                self.inventory.items.remove(index);
                Some(msg)
            }
            _ => None,
        }
    }

    pub fn use_book(&mut self, index: usize) -> Option<String> {
        if index >= self.inventory.items.len() {
            return None;
        }
        let item = &self.inventory.items[index];
        match &item.kind {
            ItemKind::Book(effect) => {
                let msg = match effect {
                    BookEffect::LearnSpell(spell) => {
                        if self.known_spells.contains(spell) {
                            return Some("You already know that spell.".into());
                        }
                        let name = spell.name().to_string();
                        self.known_spells.push(spell.clone());
                        format!("You learned {}!", name)
                    }
                    BookEffect::RevealArea(radius) => {
                        format!("The map reveals rooms within {} tiles!", radius) // handled by game.rs
                    }
                    BookEffect::SkillPoints(pts) => {
                        self.skill_points += pts;
                        format!("Gained {} skill points! (Total: {})", pts, self.skill_points)
                    }
                    BookEffect::RevealEnemies => {
                        self.enemies_revealed = true;
                        "You can now see enemy stats before fighting!".into()
                    }
                };
                self.inventory.items.remove(index);
                Some(msg)
            }
            _ => None,
        }
    }
}
