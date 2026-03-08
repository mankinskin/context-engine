use std::fmt;

// ── Core Types ──────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stat {
    Strength,
    Dexterity,
    Intelligence,
    Wisdom,
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stat::Strength => write!(f, "STR"),
            Stat::Dexterity => write!(f, "DEX"),
            Stat::Intelligence => write!(f, "INT"),
            Stat::Wisdom => write!(f, "WIS"),
        }
    }
}

impl Stat {
    pub fn from_str(s: &str) -> Option<Stat> {
        match s {
            "str" | "strength" => Some(Stat::Strength),
            "dex" | "dexterity" => Some(Stat::Dexterity),
            "int" | "intelligence" => Some(Stat::Intelligence),
            "wis" | "wisdom" => Some(Stat::Wisdom),
            _ => None,
        }
    }
}

// ── Spells ──────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Spell {
    Fireball,
    Heal,
    Lightning,
    FrostShield,
    ArcaneMissile,
}

impl Spell {
    pub fn name(&self) -> &str {
        match self {
            Spell::Fireball => "Fireball",
            Spell::Heal => "Heal",
            Spell::Lightning => "Lightning",
            Spell::FrostShield => "Frost Shield",
            Spell::ArcaneMissile => "Arcane Missile",
        }
    }

    pub fn mana_cost(&self) -> i32 {
        match self {
            Spell::Fireball => 8,
            Spell::Heal => 6,
            Spell::Lightning => 10,
            Spell::FrostShield => 5,
            Spell::ArcaneMissile => 4,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Spell::Fireball => "Fire damage (INT scaling), 8 mana",
            Spell::Heal => "Restore HP (WIS scaling), 6 mana",
            Spell::Lightning => "Lightning damage (INT scaling), 10 mana",
            Spell::FrostShield => "+defense for 3 turns, 5 mana",
            Spell::ArcaneMissile => "Magic damage, never misses, 4 mana",
        }
    }

    pub fn from_str(s: &str) -> Option<Spell> {
        match s {
            "fireball" => Some(Spell::Fireball),
            "heal" => Some(Spell::Heal),
            "lightning" => Some(Spell::Lightning),
            "frost shield" | "frostshield" | "frost" => Some(Spell::FrostShield),
            "arcane missile" | "arcanemissile" | "missile" => Some(Spell::ArcaneMissile),
            _ => None,
        }
    }
}

impl fmt::Display for Spell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

// ── Item Effects ────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub enum PotionEffect {
    Health(i32),
    Mana(i32),
    StrengthBuff { amount: i32, turns: u32 },
    SwiftnessBuff { amount: i32, turns: u32 },
}

#[derive(Clone, Debug)]
pub enum BookEffect {
    LearnSpell(Spell),
    RevealArea(u32),
    SkillPoints(u32),
    RevealEnemies,
}

#[derive(Clone, Debug)]
pub enum ItemKind {
    Weapon { min_dmg: i32, max_dmg: i32, scaling: Stat },
    Armor { defense: i32, mana_bonus: i32 },
    Potion(PotionEffect),
    Book(BookEffect),
    Backpack { extra_slots: usize, weight_reduction_pct: u32 },
}

// ── Item ────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub weight: u32,
    pub value: u32,
    pub kind: ItemKind,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Item {
    pub fn short_desc(&self) -> String {
        match &self.kind {
            ItemKind::Weapon { min_dmg, max_dmg, scaling, .. } => {
                format!("{}-{} dmg ({}), wt:{}, {}c", min_dmg, max_dmg, scaling, self.weight, self.value)
            }
            ItemKind::Armor { defense, mana_bonus } => {
                let mut s = format!("def +{}", defense);
                if *mana_bonus != 0 {
                    s += &format!(", mana {:+}", mana_bonus);
                }
                format!("{}, wt:{}, {}c", s, self.weight, self.value)
            }
            ItemKind::Potion(_) => {
                format!("{}, wt:{}, {}c", self.description, self.weight, self.value)
            }
            ItemKind::Book(_) => {
                format!("{}, wt:{}, {}c", self.description, self.weight, self.value)
            }
            ItemKind::Backpack { extra_slots, weight_reduction_pct } => {
                format!("+{} slots, -{}% weight, wt:{}, {}c", extra_slots, weight_reduction_pct, self.weight, self.value)
            }
        }
    }

    pub fn is_equippable_weapon(&self) -> bool {
        matches!(self.kind, ItemKind::Weapon { .. })
    }

    pub fn is_equippable_armor(&self) -> bool {
        matches!(self.kind, ItemKind::Armor { .. })
    }

    pub fn is_backpack(&self) -> bool {
        matches!(self.kind, ItemKind::Backpack { .. })
    }
}

// ── Item Templates ──────────────────────────────────────────────────────

fn weapon(name: &str, desc: &str, min: i32, max: i32, sc: Stat, w: u32, v: u32) -> Item {
    Item { name: name.into(), description: desc.into(), weight: w, value: v,
           kind: ItemKind::Weapon { min_dmg: min, max_dmg: max, scaling: sc } }
}

fn armor(name: &str, desc: &str, def: i32, mana: i32, w: u32, v: u32) -> Item {
    Item { name: name.into(), description: desc.into(), weight: w, value: v,
           kind: ItemKind::Armor { defense: def, mana_bonus: mana } }
}

fn potion(name: &str, desc: &str, effect: PotionEffect, v: u32) -> Item {
    Item { name: name.into(), description: desc.into(), weight: 1, value: v,
           kind: ItemKind::Potion(effect) }
}

fn book(name: &str, desc: &str, effect: BookEffect, v: u32) -> Item {
    Item { name: name.into(), description: desc.into(), weight: 1, value: v,
           kind: ItemKind::Book(effect) }
}

fn backpack(name: &str, desc: &str, slots: usize, pct: u32, w: u32, v: u32) -> Item {
    Item { name: name.into(), description: desc.into(), weight: w, value: v,
           kind: ItemKind::Backpack { extra_slots: slots, weight_reduction_pct: pct } }
}

// Weapons
pub fn rusty_dagger() -> Item { weapon("Rusty Dagger", "A dull but functional dagger", 1, 3, Stat::Strength, 2, 5) }
pub fn iron_sword() -> Item { weapon("Iron Sword", "A reliable iron blade", 3, 6, Stat::Strength, 4, 30) }
pub fn steel_greatsword() -> Item { weapon("Steel Greatsword", "A heavy two-handed sword", 5, 10, Stat::Strength, 6, 60) }
pub fn magic_staff() -> Item { weapon("Magic Staff", "A staff pulsing with energy", 2, 5, Stat::Intelligence, 3, 40) }
pub fn hunters_bow() -> Item { weapon("Hunter's Bow", "A well-crafted longbow", 3, 7, Stat::Dexterity, 3, 35) }
pub fn enchanted_blade() -> Item { weapon("Enchanted Blade", "A blade wreathed in blue flame", 4, 8, Stat::Intelligence, 3, 80) }
pub fn war_axe() -> Item { weapon("War Axe", "A brutal double-headed axe", 4, 9, Stat::Strength, 5, 50) }

// Armor
pub fn leather_armor() -> Item { armor("Leather Armor", "Tough but flexible leather", 2, 0, 4, 25) }
pub fn chain_mail() -> Item { armor("Chain Mail", "Interlocking iron rings", 4, 0, 7, 50) }
pub fn mystic_robe() -> Item { armor("Mystic Robe", "Shimmering robes that boost mana", 1, 15, 2, 45) }
pub fn plate_armor() -> Item { armor("Plate Armor", "Heavy full-body plate", 6, -5, 10, 90) }

// Potions
pub fn health_potion() -> Item { potion("Health Potion", "Restore 15 HP", PotionEffect::Health(15), 10) }
pub fn greater_health_potion() -> Item { potion("Greater Health Potion", "Restore 30 HP", PotionEffect::Health(30), 25) }
pub fn mana_potion() -> Item { potion("Mana Potion", "Restore 12 mana", PotionEffect::Mana(12), 12) }
pub fn strength_elixir() -> Item { potion("Strength Elixir", "+3 STR for 5 turns", PotionEffect::StrengthBuff { amount: 3, turns: 5 }, 20) }
pub fn swiftness_potion() -> Item { potion("Swiftness Potion", "+3 DEX for 5 turns", PotionEffect::SwiftnessBuff { amount: 3, turns: 5 }, 20) }

// Books
pub fn tome_fireball() -> Item { book("Spell Tome: Fireball", "Learn the Fireball spell", BookEffect::LearnSpell(Spell::Fireball), 50) }
pub fn tome_heal() -> Item { book("Spell Tome: Heal", "Learn the Heal spell", BookEffect::LearnSpell(Spell::Heal), 40) }
pub fn tome_lightning() -> Item { book("Spell Tome: Lightning", "Learn the Lightning spell", BookEffect::LearnSpell(Spell::Lightning), 55) }
pub fn tome_frost_shield() -> Item { book("Spell Tome: Frost Shield", "Learn the Frost Shield spell", BookEffect::LearnSpell(Spell::FrostShield), 35) }
pub fn tome_arcane_missile() -> Item { book("Spell Tome: Arcane Missile", "Learn the Arcane Missile spell", BookEffect::LearnSpell(Spell::ArcaneMissile), 30) }
pub fn map_fragment() -> Item { book("Map Fragment", "Reveals rooms within 5 tiles", BookEffect::RevealArea(5), 15) }
pub fn ancient_text() -> Item { book("Ancient Text", "Gain 2 skill points", BookEffect::SkillPoints(2), 40) }
pub fn bestiary() -> Item { book("Bestiary", "Reveals enemy stats in combat", BookEffect::RevealEnemies, 20) }

// Backpacks
pub fn leather_satchel() -> Item { backpack("Leather Satchel", "A small but sturdy bag", 3, 15, 1, 20) }
pub fn explorers_pack() -> Item { backpack("Explorer's Pack", "A spacious adventurer's pack", 5, 25, 2, 50) }
pub fn bag_of_holding() -> Item { backpack("Bag of Holding", "Magically larger on the inside", 8, 50, 1, 150) }

/// Items a merchant might sell
pub fn merchant_stock() -> Vec<Item> {
    vec![
        iron_sword(), hunters_bow(), magic_staff(), war_axe(),
        leather_armor(), chain_mail(), mystic_robe(),
        health_potion(), health_potion(), mana_potion(),
        strength_elixir(), leather_satchel(), explorers_pack(),
    ]
}

/// Items a sage might sell
pub fn sage_stock() -> Vec<Item> {
    vec![
        tome_fireball(), tome_heal(), tome_lightning(),
        tome_frost_shield(), tome_arcane_missile(),
        map_fragment(), ancient_text(), bestiary(),
        mana_potion(), mana_potion(),
    ]
}

/// Random ground loot for rooms
pub fn random_ground_loot(rng: &mut impl rand::Rng) -> Item {
    let items = vec![
        rusty_dagger(), health_potion(), health_potion(), mana_potion(),
        leather_armor(), ancient_text(), map_fragment(), bestiary(),
        swiftness_potion(), strength_elixir(), leather_satchel(),
        tome_heal(), tome_arcane_missile(), hunters_bow(),
    ];
    items[rng.gen_range(0..items.len())].clone()
}

/// Boss drop
pub fn dragon_hoard(rng: &mut impl rand::Rng) -> Vec<Item> {
    let mut loot = vec![greater_health_potion()];
    let rares = vec![enchanted_blade(), steel_greatsword(), plate_armor(), bag_of_holding()];
    loot.push(rares[rng.gen_range(0..rares.len())].clone());
    loot
}
