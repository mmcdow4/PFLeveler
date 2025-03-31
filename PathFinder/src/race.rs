use crate::{error::PathFinderError, ability_scores};
use std::{fmt, collections::HashSet};

#[derive(Debug)]
pub struct Race {
    pub id: u32,
    pub name: String,
    pub size: CharacterSize,
    pub speed: usize,
    pub languages: HashSet<String>,
    pub available_languages: HashSet<String>,
    pub racials: Vec<RacialAbility>,
    pub ability_score_offsets: [i8; ability_scores::NUMBER_ABILITY_SCORES + 1],
    pub bonus_feat: bool,
    pub bonus_skill: bool,
    pub num_favored_classes: usize,
}

impl Race {
    pub fn new(
        id: u32,
        name: &String,
        size: &String,
        speed: usize,
        languages: &HashSet<String>,
        available_languages: &HashSet<String>,
        racials: &Vec<RacialAbility>,
        ability_score_offsets: [i8; ability_scores::NUMBER_ABILITY_SCORES + 1],
        bonus_feat: bool,
        bonus_skill: bool,
        num_favored_classes: usize) -> Race
    {
        Race {
            id,
            name: String::from(name),
            size: CharacterSize::build(size).unwrap(),
            speed,
            languages: languages.clone(),
            available_languages: available_languages.clone(),
            racials: racials.clone(),
            ability_score_offsets,
            bonus_feat,
            bonus_skill,
            num_favored_classes,
        }
    }

    pub fn languages_str(&self) -> String {
        let mut language_list = String::new();

        for lang in &self.languages {
            language_list += lang;
            language_list += ";";
        }

        language_list.pop();
        language_list
    }

    pub fn available_languages_str(&self) -> String {
        let mut language_list = String::new();

        for lang in &self.available_languages {
            language_list += lang;
            language_list += ";";
        }

        language_list.pop();
        language_list
    }

    pub fn ability_score_offset(&self, ability_score: Option<ability_scores::AbilityScore>) -> i8 {
        self.ability_score_offsets[ability_scores::ability_score_to_index(ability_score)]
    }

}

#[derive(Clone)]
pub struct RacialAbility {
    pub id: u32,
    pub race_id: u32,
    pub name: String,
    pub description: String,
}

impl RacialAbility {
    pub fn new(id: u32, race_id: u32, name: &String, description: &String) -> RacialAbility {
        RacialAbility {
            id,
            race_id,
            name: String::from(name),
            description: String::from(description),
        }
    }
}

pub struct CharacterSize {
    pub text: String,
    pub combat_mod: i8,
    pub carry_capacity_mod: f64,
    pub item_price_mod: f64,
    pub item_weight_mod: f64,
    pub armor_bonus_mod: f64,
}

impl CharacterSize {
    pub fn build(text: &str) -> Result<CharacterSize, PathFinderError> {
        match text {
            "Fine" => {
                Ok(
                    CharacterSize {
                        text: String::from("Fine"),
                        combat_mod: -8,
                        carry_capacity_mod: 0.125,
                        item_price_mod: 0.5,
                        item_weight_mod: 0.1,
                        armor_bonus_mod: 0.5,
                    }
                )
            },
            "Diminutive" => {
                Ok(
                    CharacterSize {
                        text: String::from("Diminutive"),
                        combat_mod: -4,
                        carry_capacity_mod: 0.25,
                        item_price_mod: 0.5,
                        item_weight_mod: 0.1,
                        armor_bonus_mod: 0.5,
                    }
                )
            },
            "Tiny" => {
                Ok(
                    CharacterSize {
                        text: String::from("Tiny"),
                        combat_mod: -2,
                        carry_capacity_mod: 0.5,
                        item_price_mod: 0.5,
                        item_weight_mod: 0.1,
                        armor_bonus_mod: 0.5,
                    }
                )
            },
            "Small" => {
                Ok(
                    CharacterSize {
                        text: String::from("Small"),
                        combat_mod: -1,
                        carry_capacity_mod: 0.75,
                        item_price_mod: 1.0,
                        item_weight_mod: 0.5,
                        armor_bonus_mod: 1.0,
                    }
                )
            },
            "Medium" => {
                Ok(
                    CharacterSize {
                        text: String::from("Medium"),
                        combat_mod: 0,
                        carry_capacity_mod: 1.0,
                        item_price_mod: 1.0,
                        item_weight_mod: 1.0,
                        armor_bonus_mod: 1.0,
                    }
                )
            },
            "Large" => {
                Ok(
                    CharacterSize {
                        text: String::from("Large"),
                        combat_mod: 1,
                        carry_capacity_mod: 2.0,
                        item_price_mod: 2.0,
                        item_weight_mod: 2.0,
                        armor_bonus_mod: 1.0,
                    }
                )},
            "Huge" => {
                Ok(
                    CharacterSize {
                        text: String::from("Huge"),
                        combat_mod: 2,
                        carry_capacity_mod: 4.0,
                        item_price_mod: 4.0,
                        item_weight_mod: 5.0,
                        armor_bonus_mod: 1.0,
                    }
                )},
            "Gargantuan" => {
                Ok(
                    CharacterSize {
                        text: String::from("Gargantuan"),
                        combat_mod: 4,
                        carry_capacity_mod: 8.0,
                        item_price_mod: 8.0,
                        item_weight_mod: 8.0,
                        armor_bonus_mod: 1.0,
                    }
                )},
            "Colossal" => {
                Ok(
                    CharacterSize {
                        text: String::from("Colossal"),
                        combat_mod: 8,
                        carry_capacity_mod: 16.0,
                        item_price_mod: 16.0,
                        item_weight_mod: 12.0,
                        armor_bonus_mod: 1.0,
                    }
                )},
            _ => Err(PathFinderError::InvalidArgument(format!("Unrecognized character size {text}"))),
        }
    }
}

impl fmt::Debug for CharacterSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Size [{}]", self.text)
    }
}

impl fmt::Debug for RacialAbility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Racial id[{}] name[{}]", self.id, self.name)
    }
}