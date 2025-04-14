use crate::ability_scores;

pub const NUMBER_SKILLS: usize = 39;

#[derive(Debug, Clone, PartialEq)]
pub struct Skill {
    pub name: String,
    pub rank: i32,
    pub is_favored: bool,
    pub trained_only: bool,
    pub ac_penalty_applies: bool,
    pub base_ability: ability_scores::AbilityScore,
}

impl Skill {
    pub fn new(
        name: &String,
        rank: i32,
        is_favored: bool,
        trained_only: bool,
        ac_penalty_applies: bool,
        base_ability: ability_scores::AbilityScore
    ) -> Skill {
        Skill {
            name: String::from(name),
            rank,
            is_favored,
            trained_only,
            ac_penalty_applies,
            base_ability,
        }
    }
}