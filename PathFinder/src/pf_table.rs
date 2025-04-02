use std::{collections::HashMap, sync::OnceLock, fmt};
use rusqlite;
use crate::{
    character_class,
    equipment,
    feat,
    race,
    skill,
    spell,
    utilities,
    error,
};

#[derive(PartialEq)]
pub struct PfTable {
    spells: HashMap<u32, spell::Spell>,
    skills: HashMap<u32, skill::Skill>,
    feats: HashMap<u32, feat::Feat>,
    races: HashMap<u32, race::Race>,
    classes: HashMap<u32, character_class::CharacterClass>,
    class_features: HashMap<u32, character_class::ClassFeature>,
    class_choices: HashMap<u32, character_class::ClassChoice>,
    class_abilities: HashMap<u32, character_class::ClassAbility>,
    general_items: HashMap<u32, equipment::GeneralItem>,
    weapons: HashMap<u32, equipment::Weapon>,
    armor: HashMap<u32, equipment::Armor>,
}

impl PfTable {
    pub fn new() -> PfTable {
        PfTable {
            spells: HashMap::new(),
            skills: HashMap::new(),
            feats: HashMap::new(),
            races: HashMap::new(),
            classes: HashMap::new(),
            class_features: HashMap::new(),
            class_choices: HashMap::new(),
            class_abilities: HashMap::new(),
            general_items: HashMap::new(),
            weapons: HashMap::new(),
            armor: HashMap::new(),
        }
    }

    pub fn read_file(&mut self, filename: &String) {
        let db_connection = rusqlite::Connection::open(filename).unwrap();
        utilities::read_spells_from_db(&db_connection, &mut self.spells);
        utilities::read_skills_from_db(&db_connection, &mut self.skills);
        utilities::read_races_from_db(&db_connection, &mut self.races);
        utilities::read_classes_from_db(&db_connection,
            &mut self.classes,
            &mut self.class_choices,
            &mut self.class_abilities,
            &mut self.class_features
        );
        utilities::read_feats_from_db(&db_connection, &mut self.feats);
        utilities::read_general_equipment_from_db(&db_connection, &mut self.general_items);
        utilities::read_weapons_from_db(&db_connection, &mut self.weapons);
        utilities::read_armor_from_db(&db_connection, &mut self.armor);
    }

    pub fn write_file(&self, filename: &String) {
        let db_connection = rusqlite::Connection::open(filename).unwrap();
        utilities::init_sqlit_db(&db_connection).expect("Failed to initialize database copy");
        utilities::write_spells_to_db(&db_connection, &self.spells.values().cloned().collect());
        utilities::write_feats_to_db(&db_connection, &self.feats.values().cloned().collect());
        utilities::write_races_to_db(&db_connection, &self.races.values().cloned().collect());
        for (_, r) in &self.races {
            if !r.racials.is_empty() {
                utilities::write_racials_to_db(&db_connection, &r.racials);
            }
        }

        utilities::write_classes_to_db(&db_connection, &self.classes.values().cloned().collect());
        for (_, c) in &self.classes {
            if !c.abilities.is_empty() {
                utilities::write_class_abilities_to_db(&db_connection, &c.abilities);
            }
            if !c.choices.is_empty() {
                utilities::write_class_choices_to_db(&db_connection, &c.choices);
            }
            if !c.features.is_empty() {
                utilities::write_class_features_to_db(&db_connection, &c.features);
            }
        }

        let mut skills_vec = Vec::new();
        for skill_id in 0..self.skills.len() {
            skills_vec.push(self.skills.get(&(skill_id as u32)).unwrap().clone());
        }
        utilities::write_skills_to_db(&db_connection, &skills_vec);

        utilities::write_general_equipment_to_db(&db_connection, &self.general_items.values().cloned().collect());
        utilities::write_weapons_to_db(&db_connection, &self.weapons.values().cloned().collect());
        utilities::write_armor_to_db(&db_connection, &self.armor.values().cloned().collect());
    }

    pub fn eq(&self, other: &Self) -> bool {
        (self.spells == other.spells) &&
        (self.feats == other.feats) &&
        (self.races == other.races) &&
        (self.classes == other.classes) &&
        (self.class_abilities == other.class_abilities) &&
        (self.class_choices == other.class_choices) &&
        (self.class_features == other.class_features) &&
        (self.general_items == other.general_items) &&
        (self.weapons == other.weapons) &&
        (self.armor == other.armor)
    }


    // Spell Getters
    pub fn get_spell(&self, id: u32) -> Result<&spell::Spell, error::PathFinderError> {
        match self.spells.get(&id) {
            Some(x) => Ok(x),
            None => Err(error::PathFinderError::InvalidArgument(format!("Key {id} does not exist in the spells table")))
        }
    }

    pub fn get_spells(&self) -> &HashMap<u32, spell::Spell> {
        &self.spells
    }

    // Skill Getters
    pub fn get_skill(&self, id: u32) -> Result<&skill::Skill, error::PathFinderError> {
        match self.skills.get(&id) {
            Some(x) => Ok(x),
            None => Err(error::PathFinderError::InvalidArgument(format!("Key {id} does not exist in the skills table")))
        }
    }

    pub fn get_skills(&self) -> &HashMap<u32, skill::Skill> {
        &self.skills
    }

    // Feat Getters
    pub fn get_feat(&self, id: u32) -> Result<&feat::Feat, error::PathFinderError> {
        match self.feats.get(&id) {
            Some(x) => Ok(x),
            None => Err(error::PathFinderError::InvalidArgument(format!("Key {id} does not exist in the feats table")))
        }
    }

    pub fn get_feats(&self) -> &HashMap<u32, feat::Feat> {
        &self.feats
    }

    // Race Getters
    pub fn get_race(&self, id: u32) -> Result<&race::Race, error::PathFinderError> {
        match self.races.get(&id) {
            Some(x) => Ok(x),
            None => Err(error::PathFinderError::InvalidArgument(format!("Key {id} does not exist in the races table")))
        }
    }

    pub fn get_races(&self) -> &HashMap<u32, race::Race> {
        &self.races
    }

    // Class Getters
    pub fn get_class(&self, id: u32) -> Result<&character_class::CharacterClass, error::PathFinderError> {
        match self.classes.get(&id) {
            Some(x) => Ok(x),
            None => Err(error::PathFinderError::InvalidArgument(format!("Key {id} does not exist in the classes table")))
        }
    }

    pub fn get_classes(&self) -> &HashMap<u32, character_class::CharacterClass> {
        &self.classes
    }

    // Class Features Getters
    pub fn get_class_feature(&self, id: u32) -> Result<&character_class::ClassFeature, error::PathFinderError> {
        match self.class_features.get(&id) {
            Some(x) => Ok(x),
            None => Err(error::PathFinderError::InvalidArgument(format!("Key {id} does not exist in the classes table")))
        }
    }

    pub fn get_class_features(&self) -> &HashMap<u32, character_class::ClassFeature> {
        &self.class_features
    }

    // Class Choices Getters
    pub fn get_class_choice(&self, id: u32) -> Result<&character_class::ClassChoice, error::PathFinderError> {
        match self.class_choices.get(&id) {
            Some(x) => Ok(x),
            None => Err(error::PathFinderError::InvalidArgument(format!("Key {id} does not exist in the classes table")))
        }
    }

    pub fn get_class_choices(&self) -> &HashMap<u32, character_class::ClassChoice> {
        &self.class_choices
    }

    // Class Abilities Getters
    pub fn get_class_ablity(&self, id: u32) -> Result<&character_class::ClassAbility, error::PathFinderError> {
        match self.class_abilities.get(&id) {
            Some(x) => Ok(x),
            None => Err(error::PathFinderError::InvalidArgument(format!("Key {id} does not exist in the classes table")))
        }
    }

    pub fn get_class_abilities(&self) -> &HashMap<u32, character_class::ClassAbility> {
        &self.class_abilities
    }

    // Class Level Ups Getters
    pub fn get_class_level_up(&self, class_id: u32, level: usize) -> Result<&character_class::ClassLevelUpInfo, error::PathFinderError> {
        if level < character_class::MAX_CHARACTER_LEVEL {
            match self.classes.get(&class_id) {
                Some(x) => Ok(&x.level_up_info[level]),
                None => Err(error::PathFinderError::InvalidArgument(format!("Key {class_id} does not exist in the classes table")))
            }
        } else {
            Err(error::PathFinderError::InvalidArgument(format!("Level {level} is beyond the maximum class level")))
        }
    }

    pub fn get_class_level_ups(&self, class_id: u32) -> Result<&[character_class::ClassLevelUpInfo; character_class::MAX_CHARACTER_LEVEL], error::PathFinderError> {
        match self.classes.get(&class_id) {
            Some(x) => Ok(&x.level_up_info),
            None => Err(error::PathFinderError::InvalidArgument(format!("Key {class_id} does not exist in the classes table")))
        }
    }
    
    // Equipment Getters
    pub fn get_item(&self, id: u32) -> Result<Box<dyn equipment::Equipment + '_>, error::PathFinderError> {
        if self.general_items.contains_key(&id) {
            Ok(Box::new(self.general_items.get(&id).unwrap()))
        } else if self.weapons.contains_key(&id) {
            Ok(Box::new(self.weapons.get(&id).unwrap()))
        } else if self.armor.contains_key(&id) {
            Ok(Box::new(self.armor.get(&id).unwrap()))
        } else {
            Err(error::PathFinderError::InvalidArgument(format!("Key {id} does not exist in the classes table")))
        }
    }

    pub fn get_general_items(&self) -> &HashMap<u32, equipment::GeneralItem> {
        &self.general_items
    }

    pub fn get_weapons(&self) -> &HashMap<u32, equipment::Weapon> {
        &self.weapons
    }

    pub fn get_armor(&self) -> &HashMap<u32, equipment::Armor> {
        &self.armor
    }
}

impl fmt::Debug for PfTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Blah")
    }
}

static mut PF_TABLE: OnceLock<PfTable> = OnceLock::new();

pub fn init_pf_table(filename: &String) -> &'static PfTable {
    unsafe { 
        PF_TABLE.get_or_init(|| {
            let mut table = PfTable::new();
            table.read_file(filename);
            table
        })
    }
}