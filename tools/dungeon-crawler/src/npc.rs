use crate::items::{self, Item};
use rand::Rng;
use rand::seq::SliceRandom;

#[derive(Clone, Debug, PartialEq)]
pub enum NpcKind {
    Merchant,
    Sage,
    Healer,
    Blacksmith,
    Hermit,
}

#[derive(Clone, Debug)]
pub struct Npc {
    pub name: String,
    pub kind: NpcKind,
    pub dialogue: Vec<String>,
    pub shop: Vec<Item>,
    pub talked: bool,
    pub gave_gift: bool,
}

impl Npc {
    pub fn greeting(&self) -> &str {
        if self.talked {
            self.dialogue.last().map(|s| s.as_str()).unwrap_or("...")
        } else {
            self.dialogue.first().map(|s| s.as_str()).unwrap_or("Hello, traveler.")
        }
    }
}

const MERCHANT_NAMES: &[&str] = &["Gorbin", "Thessa", "Brunk", "Mira"];
const SAGE_NAMES: &[&str] = &["Eldrin", "Sybil", "Thalor", "Lunara"];
const HEALER_NAMES: &[&str] = &["Sister Maren", "Brother Aldric", "Sage Liora"];
const SMITH_NAMES: &[&str] = &["Forge-Master Kael", "Ironhide Durga", "Smithy Bren"];
const HERMIT_NAMES: &[&str] = &["Old Bones", "The Whisperer", "Moss-Beard", "Pale Elara"];

pub fn make_merchant(rng: &mut impl Rng) -> Npc {
    let name = MERCHANT_NAMES.choose(rng).unwrap().to_string();
    let stock = {
        let mut s = items::merchant_stock();
        s.shuffle(rng);
        s.truncate(6 + rng.gen_range(0..3));
        s
    };
    Npc {
        name: name.clone(),
        kind: NpcKind::Merchant,
        dialogue: vec![
            format!("Welcome! {} has the finest wares in the dungeon.", name),
            "Buy or sell — I deal in everything.".into(),
            "Come back anytime, friend.".into(),
        ],
        shop: stock,
        talked: false,
        gave_gift: false,
    }
}

pub fn make_sage(rng: &mut impl Rng) -> Npc {
    let name = SAGE_NAMES.choose(rng).unwrap().to_string();
    let stock = {
        let mut s = items::sage_stock();
        s.shuffle(rng);
        s.truncate(5 + rng.gen_range(0..3));
        s
    };
    Npc {
        name: name.clone(),
        kind: NpcKind::Sage,
        dialogue: vec![
            format!("{} peers at you from beneath a hooded cloak. 'Knowledge is power.'", name),
            "The dragon guards the exit. You'll need spells and steel.".into(),
            "Wisdom and intelligence can be mightier than brute force.".into(),
        ],
        shop: stock,
        talked: false,
        gave_gift: false,
    }
}

pub fn make_healer(rng: &mut impl Rng) -> Npc {
    let name = HEALER_NAMES.choose(rng).unwrap().to_string();
    Npc {
        name: name.clone(),
        kind: NpcKind::Healer,
        dialogue: vec![
            format!("{} smiles gently. 'Let me tend your wounds.'", name),
            "5 coins for a quick heal, 15 for full restoration.".into(),
            "Be careful out there. The deeper rooms hold terrible things.".into(),
        ],
        shop: Vec::new(),
        talked: false,
        gave_gift: false,
    }
}

pub fn make_blacksmith(rng: &mut impl Rng) -> Npc {
    let name = SMITH_NAMES.choose(rng).unwrap().to_string();
    Npc {
        name: name.clone(),
        kind: NpcKind::Blacksmith,
        dialogue: vec![
            format!("{} pounds metal on an anvil. 'Need a weapon sharpened?'", name),
            "Bring me your weapon and some coin. I'll make it stronger.".into(),
            "A good blade is worth more than a heavy one.".into(),
        ],
        shop: Vec::new(),
        talked: false,
        gave_gift: false,
    }
}

pub fn make_hermit(rng: &mut impl Rng) -> Npc {
    let name = HERMIT_NAMES.choose(rng).unwrap().to_string();
    Npc {
        name: name.clone(),
        kind: NpcKind::Hermit,
        dialogue: vec![
            format!("{} cackles. 'Heh heh... another adventurer. Take this, you'll need it.'", name),
            "I've been down here longer than I can remember...".into(),
            "The walls whisper secrets if you listen closely.".into(),
            "Beware the shadows. Not everything dead stays dead.".into(),
        ],
        shop: Vec::new(),
        talked: false,
        gave_gift: false,
    }
}

/// Generate a random NPC appropriate for the map
pub fn random_npc(rng: &mut impl Rng) -> Npc {
    let roll: u32 = rng.gen_range(0..10);
    match roll {
        0..=2 => make_merchant(rng),
        3..=4 => make_sage(rng),
        5..=6 => make_healer(rng),
        7..=8 => make_blacksmith(rng),
        _ => make_hermit(rng),
    }
}
