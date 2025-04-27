use std::hash::{Hash, Hasher};

pub const NUMBER_ABILITY_SCORES: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityScore {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl Hash for AbilityScore {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ability_score_to_index(Some(*self)).hash(state);
    }
}

pub fn ability_score_to_index(ability_score: Option<AbilityScore>) -> usize {
    match ability_score {
        Some(AbilityScore::Strength) => 0,
        Some(AbilityScore::Dexterity) => 1,
        Some(AbilityScore::Constitution) => 2,
        Some(AbilityScore::Intelligence) => 3,
        Some(AbilityScore::Wisdom) => 4,
        Some(AbilityScore::Charisma) => 5,
        None => 6,
    }
}

pub fn index_to_ability_score(index: Option<usize>) -> Option<AbilityScore> {
    if index.is_none() {
        None
    } else {
        match index.unwrap() {
            0 => Some(AbilityScore::Strength),
            1 => Some(AbilityScore::Dexterity),
            2 => Some(AbilityScore::Constitution),
            3 => Some(AbilityScore::Intelligence),
            4 => Some(AbilityScore::Wisdom),
            5 => Some(AbilityScore::Charisma),
            _ => None,
        }
    }
}

pub fn index_to_ability_score_string(index: Option<usize>) -> String {
    ability_score_to_string(index_to_ability_score(index))
}

pub fn ability_score_to_string(ability_score: Option<AbilityScore>) -> String {
    match ability_score {
        Some(AbilityScore::Strength) => String::from("Strength"),
        Some(AbilityScore::Dexterity) => String::from("Dexterity"),
        Some(AbilityScore::Constitution) => String::from("Constitution"),
        Some(AbilityScore::Intelligence) => String::from("Intelligence"),
        Some(AbilityScore::Wisdom) => String::from("Wisdom"),
        Some(AbilityScore::Charisma) => String::from("Charisma"),
        _ => String::from("")
    }
}

pub fn ability_score_to_abbrev(ability_score: Option<AbilityScore>) -> String {
    match ability_score {
        Some(AbilityScore::Strength) => String::from("STR"),
        Some(AbilityScore::Dexterity) => String::from("DEX"),
        Some(AbilityScore::Constitution) => String::from("CON"),
        Some(AbilityScore::Intelligence) => String::from("INT"),
        Some(AbilityScore::Wisdom) => String::from("WIS"),
        Some(AbilityScore::Charisma) => String::from("CHA"),
        _ => String::from("")
    }
}

pub fn string_to_ability_score(str: &String) -> Option<AbilityScore> {
    match str.as_str() {
        "Strength" => Some(AbilityScore::Strength),
        "Dexterity" => Some(AbilityScore::Dexterity),
        "Constitution" => Some(AbilityScore::Constitution),
        "Intelligence" => Some(AbilityScore::Intelligence),
        "Wisdom" => Some(AbilityScore::Wisdom),
        "Charisma" => Some(AbilityScore::Charisma),
        _ => None
    }
}

pub fn value_to_modifier(value: i32) -> i32 {
    (value - 10) / 2
}

pub fn number_bonus_spell_slots(caster_ability_value: i32, spell_level: i32) -> i32 {
    let modifier = value_to_modifier(caster_ability_value);
    if spell_level == 0 || modifier < spell_level {
        0
    } else {
        ((modifier - spell_level) / 4) + 1
    }
}