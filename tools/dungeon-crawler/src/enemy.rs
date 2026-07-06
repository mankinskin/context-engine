use rand::Rng;

#[derive(Clone, Debug)]
pub struct Enemy {
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub min_dmg: i32,
    pub max_dmg: i32,
    pub defense: i32,
    pub xp: u32,
    pub min_coins: u32,
    pub max_coins: u32,
    pub is_boss: bool,
    pub casts_spells: bool,
}

impl Enemy {
    pub fn attack_damage(
        &self,
        rng: &mut impl Rng,
    ) -> i32 {
        rng.gen_range(self.min_dmg..=self.max_dmg)
    }

    pub fn coin_drop(
        &self,
        rng: &mut impl Rng,
    ) -> u32 {
        rng.gen_range(self.min_coins..=self.max_coins)
    }
}

fn enemy(
    name: &str,
    hp: i32,
    min: i32,
    max: i32,
    def: i32,
    xp: u32,
    c1: u32,
    c2: u32,
) -> Enemy {
    Enemy {
        name: name.into(),
        hp,
        max_hp: hp,
        min_dmg: min,
        max_dmg: max,
        defense: def,
        xp,
        min_coins: c1,
        max_coins: c2,
        is_boss: false,
        casts_spells: false,
    }
}

pub fn giant_rat() -> Enemy {
    enemy("Giant Rat", 5, 1, 3, 0, 10, 1, 4)
}
pub fn goblin() -> Enemy {
    enemy("Goblin", 10, 2, 5, 1, 20, 4, 10)
}
pub fn skeleton() -> Enemy {
    enemy("Skeleton", 15, 3, 7, 2, 35, 7, 15)
}
pub fn orc() -> Enemy {
    enemy("Orc", 22, 4, 9, 3, 55, 12, 22)
}
pub fn troll() -> Enemy {
    enemy("Troll", 30, 5, 11, 3, 75, 18, 30)
}
pub fn wraith() -> Enemy {
    enemy("Wraith", 25, 7, 13, 2, 90, 20, 35)
}

pub fn dark_mage() -> Enemy {
    let mut e = enemy("Dark Mage", 18, 9, 15, 1, 100, 25, 45);
    e.casts_spells = true;
    e
}

pub fn dragon() -> Enemy {
    let mut e = enemy("Ancient Dragon", 60, 10, 22, 5, 250, 80, 150);
    e.is_boss = true;
    e
}

type EnemyFactory = fn() -> Enemy;

fn pick_from_pool(
    rng: &mut impl Rng,
    pool: [EnemyFactory; 3],
) -> Enemy {
    let idx = rng.gen_range(0..pool.len());
    pool[idx]()
}

fn random_tier_one(rng: &mut impl Rng) -> Enemy {
    if rng.gen_bool(0.5) {
        giant_rat()
    } else {
        goblin()
    }
}

fn random_mid_tier_enemy(
    tier: u32,
    rng: &mut impl Rng,
) -> Enemy {
    match tier {
        2 => pick_from_pool(rng, [goblin, skeleton, giant_rat]),
        3 => pick_from_pool(rng, [skeleton, orc, goblin]),
        4 => pick_from_pool(rng, [orc, troll, skeleton]),
        5 => pick_from_pool(rng, [troll, wraith, orc]),
        6 => pick_from_pool(rng, [wraith, dark_mage, troll]),
        _ => dark_mage(),
    }
}

/// Random enemy based on difficulty tier (0-7)
pub fn random_enemy(
    tier: u32,
    rng: &mut impl Rng,
) -> Enemy {
    match tier {
        0 => giant_rat(),
        1 => random_tier_one(rng),
        _ => random_mid_tier_enemy(tier, rng),
    }
}
