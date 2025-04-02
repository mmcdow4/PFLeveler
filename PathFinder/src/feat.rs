
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FeatChoiceType {
    FeatListChoice = 0,
    FeatWeaponChoice = 1,
    FeatSkillChoice = 2,
    FeatSpellChoice = 3,
    FeatMercyChoice = 4,
}

pub fn index_to_feat_choice_type(choice: u32) -> Option<FeatChoiceType> {
    match choice {
        0 => Some(FeatChoiceType::FeatListChoice),
        1 => Some(FeatChoiceType::FeatWeaponChoice),
        2 => Some(FeatChoiceType::FeatSkillChoice),
        3 => Some(FeatChoiceType::FeatSpellChoice),
        4 => Some(FeatChoiceType::FeatMercyChoice),
        _ => None,
    }
}

pub fn feat_choice_type_to_index(choice: FeatChoiceType) -> u32 {
    match choice {
        FeatChoiceType::FeatListChoice => 0,
        FeatChoiceType::FeatWeaponChoice => 1,
        FeatChoiceType::FeatSkillChoice => 2,
        FeatChoiceType::FeatSpellChoice => 3,
        FeatChoiceType::FeatMercyChoice => 4,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Feat {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub description: String,
    pub prerequisites: String,
    pub prerequisite_feats: String,
    pub benefit: String,
    pub normal: String,
    pub special: String,
    pub source: String,
    pub teamwork: bool,
    pub critical: bool,
    pub grit: bool,
    pub style: bool,
    pub performance: bool,
    pub racial: bool,
    pub companion_familiar: bool,
    pub race_name: String,
    pub note: String,
    pub goal: String,
    pub completion_benefit: String,
    pub multiple: bool,
    pub suggested_traits: String,
    pub choice: Option<FeatChoiceType>,
    pub choice_string: String,
}

impl Feat {
    pub fn new(
        id: u32,
        name: &String,
        category: &String,
        description: &String,
        prerequisites: &String,
        prerequisite_feats: &String,
        benefit: &String,
        normal: &String,
        special: &String,
        source: &String,
        teamwork: bool,
        critical: bool,
        grit: bool,
        style: bool,
        performance: bool,
        racial: bool,
        companion_familiar: bool,
        race_name: &String,
        note: &String,
        goal: &String,
        completion_benefit: &String,
        multiple: bool,
        suggested_traits: &String,
        choice: Option<FeatChoiceType>
    ) -> Feat {
        Feat {
            id,
            name: name.clone(),
            category: category.clone(),
            description: description.clone(),
            prerequisites: prerequisites.clone(),
            prerequisite_feats: prerequisite_feats.clone(),
            benefit: benefit.clone(),
            normal: normal.clone(),
            special: special.clone(),
            source: source.clone(),
            teamwork,
            critical,
            grit,
            style,
            performance,
            racial,
            companion_familiar,
            race_name: race_name.clone(),
            note: note.clone(),
            goal: goal.clone(),
            completion_benefit: completion_benefit.clone(),
            multiple,
            suggested_traits: suggested_traits.clone(),
            choice,
            choice_string: "".to_string(),
        }
    }

    pub fn choice_str(&self) -> String {
        match self.choice {
            Some(val) => feat_choice_type_to_index(val).to_string(),
            None => "NULL".to_string(),
        }
    }
}