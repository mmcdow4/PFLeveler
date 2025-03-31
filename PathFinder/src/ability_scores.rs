pub const NUMBER_ABILITY_SCORES: usize = 6;

#[derive(Debug, Clone, Copy)]
pub enum AbilityScore {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
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