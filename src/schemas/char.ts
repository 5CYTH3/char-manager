type Stats = {
    strg: number;
    dext: number;
    cnst: number;
    intl: number;
    wisd: number;
    chrs: number;
};

export enum Classes {
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

export enum Races {
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

export enum LCAxis {
    Lawful,
    Neutral,
    Chaotic,
}

export enum GEAxis {
    Good,
    Neutral,
    Evil,
}

type Appearance = {
    age: number;
    height: number;
    weight: number;
    more: string;
};

export type CharData = {
    id: number;
    img_path?: string;
    name: string;
    stats: Stats;
    class: Classes;
    lvl: number;
    race: Races;
    alignment: [LCAxis, GEAxis];
    bag: string[];
    balance: number;
    capabilities: string[];
    appearance: Appearance;
    notes: string;
};
