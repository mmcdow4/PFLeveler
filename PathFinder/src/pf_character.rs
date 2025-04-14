use std::collections::{HashSet, HashMap};
use crate::{
    ability_scores,
    race,
    skill,
    feat,
    pf_table,
    error,
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
    pub languages_known: Vec<String>,
    pub languages_available: Vec<String>,
    pub favored_classes: HashSet<u32>,
    pub hit_points: u32,
    pub favored_class_hp_bonus: u32,
    pub favored_class_skill_bonus: u32,
    pub remaining_bonus_languages: u32,
    pub class_levels: HashMap<u32, u32>,
    pub feats: Vec<feat::Feat>,
    pub skills: Vec<skill::Skill>,
    pub ability_scores: HashMap<ability_scores::AbilityScore, i32>
}

impl PFCharacter {
    pub fn new() -> PFCharacter {
        let mut skills_vec: Vec<skill::Skill> = pf_table::get_pf_table()
            .get_skills()
            .values()
            .cloned()
            .collect();

        skills_vec.sort_by(|a, b| a.name.cmp(&b.name));
        let mut ability_scores = HashMap::new();
        for ability_idx in 0..ability_scores::NUMBER_ABILITY_SCORES {
            let ability = ability_scores::index_to_ability_score(Some(ability_idx)).unwrap();
            ability_scores.insert(ability, 0);
        }
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
            languages_known: Vec::new(),
            languages_available: Vec::new(),
            favored_classes: HashSet::new(),
            hit_points: 0,
            favored_class_hp_bonus: 0,
            favored_class_skill_bonus: 0,
            remaining_bonus_languages: 0,
            class_levels: HashMap::new(),
            feats: Vec::new(),
            skills: skills_vec,
            ability_scores: ability_scores,
        }
    }

    pub fn get_languages_known(&self) -> String {
        let mut language_string = String::from("");
        for language in &self.languages_known {
            if !language_string.is_empty() {
                language_string += ", ";
            }
            language_string += &language.clone();
        }

        language_string
    }
    
    pub fn get_languages_available(&self) -> String {
        let mut language_string = String::from("");
        for language in &self.languages_available {
            if !language_string.is_empty() {
                language_string += ", ";
            }
            language_string += &language.clone();
        }

        language_string
    }

    pub fn get_ability_mod(&self, ability: &ability_scores::AbilityScore) -> i32 {
        ability_scores::ability_score_to_mod(self.ability_scores[ability])
    }

    pub fn get_effective_skill_rank(&self, index: usize) -> Result<i32, error::PathFinderError> {
        if index >= self.skills.len() {
            Err(
                error::PathFinderError::InvalidArgument(
                    format!(
                        "Illegal string index {} for character with only {} skills",
                        index,
                        self.skills.len()
                    )
                )
            )
        } else {
            let skill_ref = &self.skills[index];
            let value =
                skill_ref.rank +
                self.get_ability_mod(&skill_ref.base_ability) +
                match skill_ref.is_favored && skill_ref.rank > 0 {
                    true => 3,
                    false => 0,
                };
            Ok(value)
        }
    }
}