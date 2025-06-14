use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    pub strg: i8,
    pub dext: i8,
    pub cnst: i8,
    pub intl: i8,
    pub wisd: i8,
    pub chrs: i8,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub enum LCAxis {
    Lawful,
    Neutral,
    Chaotic,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GEAxis {
    Good,
    Neutral,
    Evil,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Appearance {
    pub age: i32,
    pub height: i32,
    pub weight: i32,
    pub more: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharData {
    pub id: i32,
    pub img_path: Option<String>,
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
/*
impl Stats {
    pub fn new() -> Self {
        Stats {
            strg: 0,
            dext: 0,
            cnst: 0,
            intl: 0,
            wisd: 0,
            chrs: 0,
        }
    }
}
*/

pub fn dummy_data() -> CharData {
    CharData {
        id: 0,
        img_path: None,
        name: "Hell Boy".to_string(),
        stats: Stats {
            strg: 0,
            intl: 0,
            dext: 0,
            cnst: 0,
            wisd: 0,
            chrs: 0,
        },
        class: Classes::Artificer,
        race: Races::Orc,
        lvl: 9,
        alignment: (LCAxis::Chaotic, GEAxis::Good),
        bag: Vec::new(),
        balance: 0,
        capabilities: Vec::new(),
        appearance: Appearance {
            age: 25,
            height: 189,
            weight: 156,
            more: "Red skin, hellish look".to_string(),
        },
        notes: "Will try to kill anything that goes over world's fate".to_string(),
    }
}
