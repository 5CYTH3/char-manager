use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub strg: i8,
    pub dext: i8,
    pub cnst: i8,
    pub intl: i8,
    pub wisd: i8,
    pub chrs: i8,
}

#[derive(Serialize, Deserialize)]
pub enum Classes {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Monk,
    Fighter,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
    Artificer,
}

#[derive(Serialize, Deserialize)]
pub enum Races {
    Dward,
    Elf,
    Halfling,
    Human,
    Aasimar,
    Dragonborn,
    Gnome,
    Goliath,
    Orc,
    Tiefling,
    Githyanki,
    Goblin,
    Kobold,
}

#[derive(Serialize, Deserialize)]
enum LCAxis {
    Lawful,
    Neutral,
    Chaotic,
}

#[derive(Serialize, Deserialize)]
pub enum GEAxis {
    Good,
    Neutral,
    Evil,
}

#[derive(Serialize, Deserialize)]
struct Appearance {
    pub age: i32,
    pub height: i32,
    pub weight: i32,
    pub more: String,
}

#[derive(Serialize, Deserialize)]
pub struct CharData {
    pub id: i32,
    pub img_path: String,
    pub name: String,
    pub stats: Stats,
    pub class: Classes,
    pub lvl: i32,
    pub race: Races,
    pub alignment: (LCAxis, GEAxis),
    pub bag: Vec<String>,
    pub balance: i32,
    pub capabilities: Vec<String>,
    pub appearance: Appearance,
    pub notes: String,
}
