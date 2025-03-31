use crate::ability_scores;

//New feat every 2 levels
pub const FEAT_LEVEL_MODULUS: usize = 2;

//New ability score every 4 levels
pub const ABILITY_SCORE_MODULUS: usize = 4;

//Additional attack every +5 BAB
pub const EXTRA_ATTACK_BAB_MODULUS: usize = 5;

pub const MAX_CHARACTER_LEVEL: usize = 20;
pub const NUMBER_BASE_CLASSES: usize = 11;

#[derive(Debug)]
pub struct CharacterClass {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub hit_die: u32,
    pub skills_per_level: u32,
    pub alignment_req: String,
    pub class_skill_list: String,
    pub caster_ability: Option<ability_scores::AbilityScore>,
    pub starting_wealth_d6: u32,
    pub features: Vec<ClassFeature>,
    pub abilities: Vec<ClassAbility>,
    pub choices: Vec<ClassChoice>,
    pub level_up_info: [ClassLevelUpInfo; MAX_CHARACTER_LEVEL],
}

impl CharacterClass {
    pub fn new(
        id: u32,
        name: &String,
        description: &String,
        hit_die: u32,
        skills_per_level: u32,
        alignment_req: &String,
        class_skill_list: &String,
        caster_ability: Option<ability_scores::AbilityScore>,
        starting_wealth_d6: u32,
        features: &Vec<ClassFeature>,
        abilities: &Vec<ClassAbility>,
        choices: &Vec<ClassChoice>,
        level_up_info: &[ClassLevelUpInfo; MAX_CHARACTER_LEVEL],
    ) -> CharacterClass {
        CharacterClass { 
            id,
            name: String::from(name),
            description: String::from(description),
            hit_die,
            skills_per_level,
            alignment_req: String::from(alignment_req),
            class_skill_list: String::from(class_skill_list),
            caster_ability,
            starting_wealth_d6,
            features: features.clone(),
            abilities: abilities.clone(),
            choices: choices.clone(),
            level_up_info: level_up_info.clone(),
        }
    }

    pub fn is_class_skill(&self, skill_name: &String) -> bool {
        self.class_skill_list.contains(skill_name)
    }
}


#[derive(Debug, Clone)]
pub struct ClassFeature {
    pub id: u32,
    pub class_id: u32,
    pub category_id: u32,
    pub num_choices: u32,
    pub level_req: u32,
    pub name: String,
    pub description: String,
    pub optional: bool,
}

impl ClassFeature {
    pub fn new(
        id: u32,
        class_id: u32,
        category_id: u32,
        num_choices: u32,
        level_req: u32,
        name: &String,
        description: &String,
        optional: bool,
    ) -> ClassFeature {
        ClassFeature {
            id,
            class_id,
            category_id,
            num_choices,
            level_req,
            name: String::from(name),
            description: String::from(description),
            optional,
        }
    }

    pub fn copy(&self) -> ClassFeature {
        ClassFeature::new(
            self.id,
            self.class_id,
            self.category_id,
            self.num_choices,
            self.level_req,
            &self.name,
            &self.description,
            self.optional,
        )
    }
}

#[derive(Debug, Clone)]
pub struct ClassAbility {
    pub id: u32,
    pub class_id: u32,
    pub choice_prereq_id: Option<u32>,
    pub name: String,
    pub description: String,
    pub level_req: u32,
    pub spell_id: Option<u32>,
    pub feat_id: Option<u32>,
}

impl ClassAbility {
    pub fn new(
        id: u32,
        class_id: u32,
        choice_prereq_id: Option<u32>,
        name: &String,
        description: &String,
        level_req: u32,
        spell_id: Option<u32>,
        feat_id: Option<u32>,
    ) -> ClassAbility {
        ClassAbility {
            id,
            class_id,
            choice_prereq_id,
            name: String::from(name),
            description: String::from(description),
            level_req,
            spell_id,
            feat_id,
        }
    }

    pub fn copy(&self) -> ClassAbility {
        ClassAbility::new(
            self.id,
            self.class_id,
            self.choice_prereq_id,
            &self.name,
            &self.description,
            self.level_req,
            self.spell_id,
            self.feat_id,
        )
    }

    pub fn choice_prereq_id_str(&self) -> String {
        match self.choice_prereq_id {
            Some(val) => val.to_string(),
            None => "NULL".to_string(),
        }
    }

    pub fn spell_id_str(&self) -> String {
        match self.spell_id {
            Some(val) => val.to_string(),
            None => "NULL".to_string(),
        }
    }

    pub fn feat_id_str(&self) -> String {
        match self.feat_id {
            Some(val) => val.to_string(),
            None => "NULL".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClassChoice {
    pub id: u32,
    pub class_id: u32,
    pub category_id: u32,
    pub level_req: u32,
    pub name: String,
    pub description: String,
    pub max_num_selections: u32,
    pub num_subsequent_choices: u32,
    pub subsequent_choice_category: Option<u32>,
    pub prereq_id: Option<u32>,
    pub feat_id: Option<u32>,
    pub extra: String,
}

impl ClassChoice {
    pub fn new(
        id: u32,
        class_id: u32,
        category_id: u32,
        level_req: u32,
        name: &str,
        description: &str,
        max_num_selections: u32,
        num_subsequent_choices: u32,
        subsequent_choice_category: Option<u32>,
        prereq_id: Option<u32>,
        feat_id: Option<u32>,
        extra: &str,
    ) -> ClassChoice {
        ClassChoice {
            id,
            class_id,
            category_id,
            level_req,
            name: String::from(name),
            description: String::from(description),
            max_num_selections,
            num_subsequent_choices,
            subsequent_choice_category,
            prereq_id,
            feat_id,
            extra: String::from(extra),
        }
    }

    pub fn copy(&self) -> ClassChoice {
        ClassChoice::new(
            self.id,
            self.class_id,
            self.category_id,
            self.level_req,
            &self.name,
            &self.description,
            self.max_num_selections,
            self.num_subsequent_choices,
            self.subsequent_choice_category,
            self.prereq_id,
            self.feat_id,
            &self.extra,
        )
    }
    pub fn subsequent_category_str(&self) -> String {
        match self.subsequent_choice_category {
            Some(value) => value.to_string(),
            None => "NULL".to_string(),
        }
    }

    pub fn prereq_id_str(&self) -> String {
        match self.prereq_id {
            Some(value) => value.to_string(),
            None => "NULL".to_string(),
        }
    }

    pub fn feat_id_str(&self) -> String {
        match self.feat_id {
            Some(value) => value.to_string(),
            None => "NULL".to_string(),
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct ClassLevelUpInfo {
    pub base_attack_bonus: u8,
    pub base_fort_save: u8,
    pub base_reflex_save: u8,
    pub base_will_save: u8,
    pub spells_per_day_0: Option<u8>,
    pub spells_per_day_1: Option<u8>,
    pub spells_per_day_2: Option<u8>,
    pub spells_per_day_3: Option<u8>,
    pub spells_per_day_4: Option<u8>,
    pub spells_per_day_5: Option<u8>,
    pub spells_per_day_6: Option<u8>,
    pub spells_per_day_7: Option<u8>,
    pub spells_per_day_8: Option<u8>,
    pub spells_per_day_9: Option<u8>,
    pub spells_known_0: Option<u8>,
    pub spells_known_1: Option<u8>,
    pub spells_known_2: Option<u8>,
    pub spells_known_3: Option<u8>,
    pub spells_known_4: Option<u8>,
    pub spells_known_5: Option<u8>,
    pub spells_known_6: Option<u8>,
    pub spells_known_7: Option<u8>,
    pub spells_known_8: Option<u8>,
    pub spells_known_9: Option<u8>,
}

impl ClassLevelUpInfo {
    pub fn new(
        base_attack_bonus: u8,
        base_fort_save: u8,
        base_reflex_save: u8,
        base_will_save: u8,
        spells_per_day_0: Option<u8>,
        spells_per_day_1: Option<u8>,
        spells_per_day_2: Option<u8>,
        spells_per_day_3: Option<u8>,
        spells_per_day_4: Option<u8>,
        spells_per_day_5: Option<u8>,
        spells_per_day_6: Option<u8>,
        spells_per_day_7: Option<u8>,
        spells_per_day_8: Option<u8>,
        spells_per_day_9: Option<u8>,
        spells_known_0: Option<u8>,
        spells_known_1: Option<u8>,
        spells_known_2: Option<u8>,
        spells_known_3: Option<u8>,
        spells_known_4: Option<u8>,
        spells_known_5: Option<u8>,
        spells_known_6: Option<u8>,
        spells_known_7: Option<u8>,
        spells_known_8: Option<u8>,
        spells_known_9: Option<u8>,
    ) -> ClassLevelUpInfo {
        ClassLevelUpInfo {
            base_attack_bonus,
            base_fort_save,
            base_reflex_save,
            base_will_save,
            spells_per_day_0,
            spells_per_day_1,
            spells_per_day_2,
            spells_per_day_3,
            spells_per_day_4,
            spells_per_day_5,
            spells_per_day_6,
            spells_per_day_7,
            spells_per_day_8,
            spells_per_day_9,
            spells_known_0,
            spells_known_1,
            spells_known_2,
            spells_known_3,
            spells_known_4,
            spells_known_5,
            spells_known_6,
            spells_known_7,
            spells_known_8,
            spells_known_9,
        }
    }

    pub fn new_blank() -> ClassLevelUpInfo {
        ClassLevelUpInfo {
            base_attack_bonus: 0,
            base_fort_save: 0,
            base_reflex_save: 0,
            base_will_save: 0,
            spells_per_day_0: None,
            spells_per_day_1: None,
            spells_per_day_2: None,
            spells_per_day_3: None,
            spells_per_day_4: None,
            spells_per_day_5: None,
            spells_per_day_6: None,
            spells_per_day_7: None,
            spells_per_day_8: None,
            spells_per_day_9: None,
            spells_known_0: None,
            spells_known_1: None,
            spells_known_2: None,
            spells_known_3: None,
            spells_known_4: None,
            spells_known_5: None,
            spells_known_6: None,
            spells_known_7: None,
            spells_known_8: None,
            spells_known_9: None,
        }
    }

    pub fn spells_per_day(&self, level: usize) -> Option<u8> {
        match level {
            0 => self.spells_per_day_0,
            1 => self.spells_per_day_1,
            2 => self.spells_per_day_2,
            3 => self.spells_per_day_3,
            4 => self.spells_per_day_4,
            5 => self.spells_per_day_5,
            6 => self.spells_per_day_6,
            7 => self.spells_per_day_7,
            8 => self.spells_per_day_8,
            9 => self.spells_per_day_9,
            _ => None,
        }
    }

    pub fn spells_per_day_str(&self, level: usize) -> String {
        match self.spells_per_day(level) {
            Some(x) => x.to_string(),
            None => "NULL".to_string(),
        }
    }

    pub fn spells_known(&self, level: usize) -> Option<u8> {
        match level {
            0 => self.spells_known_0,
            1 => self.spells_known_1,
            2 => self.spells_known_2,
            3 => self.spells_known_3,
            4 => self.spells_known_4,
            5 => self.spells_known_5,
            6 => self.spells_known_6,
            7 => self.spells_known_7,
            8 => self.spells_known_8,
            9 => self.spells_known_9,
            _ => None,
        }
    }

    pub fn spells_known_str(&self, level: usize) -> String {
        match self.spells_known(level) {
            Some(x) => x.to_string(),
            None => "NULL".to_string(),
        }
    }
}