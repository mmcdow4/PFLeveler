use std::collections::{HashSet, HashMap};
use crate::{
    ability_scores,
    race,
    skill,
    feat
};

pub struct PFCharacter {
    pub name: String,
    pub player_name: String,
    pub height: String,
    pub weight: String,
    pub hair: String,
    pub eyes: String,
    pub deity: String,
    pub homeland: String,
    pub gender: String,
    pub age: String,
    pub alignment: String,
    pub race: Option<race::Race>,
    pub languages_known: HashSet<String>,
    pub languages_avail: HashSet<String>,
    pub favored_classes: HashSet<u32>,
    pub hit_points: u32,
    pub favored_class_hp_bonus: u32,
    pub favored_class_skill_bonus: u32,
    pub remaining_bonus_languages: u32,
    pub class_levels: HashMap<u32, u32>,
    pub feats: Vec<feat::Feat>,
    pub skills: HashMap<u32, skill::Skill>,
    pub unused_skill_points: u32,
    pub ability_scores: HashMap<ability_scores::AbilityScore, u32>
}

impl PFCharacter {
    pub fn new() -> PFCharacter {
        PFCharacter {
            name: String::from(""),
            player_name: String::from(""),
            height: String::from(""),
            weight: String::from(""),
            hair: String::from(""),
            eyes: String::from(""),
            deity: String::from(""),
            homeland: String::from(""),
            gender: String::from(""),
            age: String::from(""),
            alignment: String::from(""),
            race: None,
            languages_known: HashSet::new(),
            languages_avail: HashSet::new(),
            favored_classes: HashSet::new(),
            hit_points: 0,
            favored_class_hp_bonus: 0,
            favored_class_skill_bonus: 0,
            remaining_bonus_languages: 0,
            class_levels: HashMap::new(),
            feats: Vec::new(),
            skills: HashMap::new(),
            unused_skill_points: 0,
            ability_scores: HashMap::new(),
        }
    }

    pub fn get_languages_known(&self) -> String {
        let mut language_string = String::from("");
        for language in &self.languages_known {
            if language_string.is_empty() {
                language_string += &language.clone();
            } else {
                language_string += ", ";
                language_string += &language.clone();
            }
        }

        language_string
    }
    
    pub fn get_languages_available(&self) -> String {
        let mut language_string = String::from("");
        for language in &self.languages_avail {
            if language_string.is_empty() {
                language_string += &language.clone();
            } else {
                language_string += ", ";
                language_string += &language.clone();
            }
        }

        language_string
    }
}