use rand::Rng;
use crate::items::{Spell, Stat};
use crate::player::Player;
use crate::enemy::Enemy;

/// Result of a combat action
pub enum CombatResult {
    Continue(String),
    EnemyDied { msg: String, xp: u32, coins: u32 },
    PlayerDied(String),
    Fled(String),
}

/// Player attacks enemy (melee)
pub fn player_attack(player: &mut Player, enemy: &mut Enemy, rng: &mut impl Rng) -> CombatResult {
    let raw_dmg = player.attack_damage(rng);
    let dmg = (raw_dmg - enemy.defense).max(1);
    enemy.hp -= dmg;

    let weapon_name = player.inventory.weapon.as_ref()
        .map(|w| w.name.as_str())
        .unwrap_or("fists");

    let mut msg = format!("  You strike with {} for {} dmg! (Enemy HP: {})",
        weapon_name, dmg, enemy.hp.max(0));

    if enemy.hp <= 0 {
        let coins = enemy.coin_drop(rng);
        msg += &format!("\n  {} is slain! (+{} XP, +{} coins)", enemy.name, enemy.xp, coins);
        return CombatResult::EnemyDied { msg, xp: enemy.xp, coins };
    }

    // Enemy retaliates
    msg += &enemy_turn(player, enemy, rng);

    if player.hp <= 0 {
        msg += "\n  You have fallen... GAME OVER.";
        return CombatResult::PlayerDied(msg);
    }
    player.tick_buffs();
    CombatResult::Continue(msg)
}

/// Player casts a spell in combat
pub fn player_cast(player: &mut Player, enemy: &mut Enemy, spell: &Spell, rng: &mut impl Rng) -> CombatResult {
    if player.mana < spell.mana_cost() {
        return CombatResult::Continue(format!("  Not enough mana! (Need {}, have {})", spell.mana_cost(), player.mana));
    }
    if !player.known_spells.contains(spell) {
        return CombatResult::Continue("  You don't know that spell!".into());
    }

    player.mana -= spell.mana_cost();
    let int = player.effective_stat(&Stat::Intelligence);
    let wis = player.effective_stat(&Stat::Wisdom);

    let mut msg = match spell {
        Spell::Fireball => {
            let dmg = rng.gen_range(8 + int..=15 + int * 2).max(1);
            let actual = (dmg - enemy.defense).max(1);
            enemy.hp -= actual;
            format!("  You cast Fireball for {} dmg! (Enemy HP: {})", actual, enemy.hp.max(0))
        }
        Spell::Lightning => {
            let dmg = rng.gen_range(5 + int..=12 + int * 2).max(1);
            let actual = (dmg - enemy.defense).max(1);
            enemy.hp -= actual;
            format!("  Lightning strikes for {} dmg! (Enemy HP: {})", actual, enemy.hp.max(0))
        }
        Spell::ArcaneMissile => {
            let dmg = rng.gen_range(4 + int / 2..=8 + int).max(1);
            // Arcane missile ignores defense
            enemy.hp -= dmg;
            format!("  Arcane Missile hits for {} dmg (ignores armor)! (Enemy HP: {})", dmg, enemy.hp.max(0))
        }
        Spell::Heal => {
            let heal = rng.gen_range(10 + wis..=20 + wis * 2);
            let actual = heal.min(player.max_hp - player.hp);
            player.hp += actual;
            format!("  You heal for {} HP! (HP: {}/{})", actual, player.hp, player.max_hp)
        }
        Spell::FrostShield => {
            let def = 3 + wis / 2;
            player.defense_buff = def;
            player.defense_buff_turns = 3;
            format!("  Frost Shield grants +{} defense for 3 turns!", def)
        }
    };

    if enemy.hp <= 0 {
        let coins = enemy.coin_drop(rng);
        msg += &format!("\n  {} is slain! (+{} XP, +{} coins)", enemy.name, enemy.xp, coins);
        return CombatResult::EnemyDied { msg, xp: enemy.xp, coins };
    }

    // Enemy retaliates (except if we just healed/shielded, they still attack)
    msg += &enemy_turn(player, enemy, rng);

    if player.hp <= 0 {
        msg += "\n  You have fallen... GAME OVER.";
        return CombatResult::PlayerDied(msg);
    }
    player.tick_buffs();
    CombatResult::Continue(msg)
}

/// Try to flee from combat
pub fn try_flee(player: &Player, enemy: &Enemy, rng: &mut impl Rng) -> CombatResult {
    let dex = player.effective_stat(&Stat::Dexterity);
    let chance = 0.3 + (dex as f32 * 0.03); // 30% base + 3% per DEX
    if rng.gen::<f32>() < chance {
        CombatResult::Fled("  You successfully flee from combat!".into())
    } else {
        let dmg = (enemy.attack_damage(rng) - player.total_defense()).max(1);
        CombatResult::Continue(format!(
            "  Failed to flee! {} hits you for {} dmg as you try to run! (HP: {})",
            enemy.name, dmg, (player.hp - dmg).max(0)
        ))
    }
}

/// Enemy takes their turn
fn enemy_turn(player: &mut Player, enemy: &Enemy, rng: &mut impl Rng) -> String {
    // Check dodge
    if rng.gen::<f32>() < player.dodge_chance() {
        return format!("\n  {} attacks but you dodge!", enemy.name);
    }

    let raw_dmg = enemy.attack_damage(rng);

    // Dark Mage special: sometimes casts a spell instead of melee
    if enemy.casts_spells && rng.gen_bool(0.4) {
        let spell_dmg = rng.gen_range(raw_dmg..=raw_dmg + 5);
        // Spells partially bypass defense
        let def = player.total_defense() / 2;
        let actual = (spell_dmg - def).max(1);
        player.hp -= actual;
        return format!("\n  {} casts dark magic for {} dmg! (HP: {})", enemy.name, actual, player.hp.max(0));
    }

    let actual = (raw_dmg - player.total_defense()).max(1);
    player.hp -= actual;
    format!("\n  {} hits you for {} dmg! (HP: {})", enemy.name, actual, player.hp.max(0))
}
