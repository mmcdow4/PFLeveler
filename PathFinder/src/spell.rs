#[derive(Debug, Clone, PartialEq)]
pub struct Spell {
    pub id: u32,
    pub name: String,
    pub spell_school: String, //TODO: enum this?
    pub spell_sub_school: String, //TODO
    pub description: String,
    pub short_description: String,
    pub cast_time: String,
    pub range: String,
    pub components: String,
    pub duration: String,
    pub saving_throw: String,
    pub spell_resistance: String,
    pub sorc_level: Option<u8>,
    pub wiz_level: Option<u8>,
    pub cleric_level: Option<u8>,
    pub druid_level: Option<u8>,
    pub ranger_level: Option<u8>,
    pub bard_level: Option<u8>,
    pub pal_level: Option<u8>,
    pub sla_level: Option<u8>,
    pub element_mask: u32,
}

impl Spell {
    pub fn new(id: u32,
        name: &String,
        spell_school: &String,
        spell_sub_school: &String,
        description: &String,
        short_description: &String,
        cast_time: &String,
        range: &String,
        components: &String,
        duration: &String,
        saving_throw: &String,
        spell_resistance: &String,
        sorc_level: Option<u8>,
        wiz_level: Option<u8>,
        cleric_level: Option<u8>,
        druid_level: Option<u8>,
        ranger_level: Option<u8>,
        bard_level: Option<u8>,
        pal_level: Option<u8>,
        sla_level: Option<u8>,
        element_mask: u32) -> Spell {
            let name = String::from(name);
            let spell_school = String::from(spell_school);
            let spell_sub_school = String::from(spell_sub_school);
            let description = String::from(description);
            let short_description = String::from(short_description);
            let cast_time = String::from(cast_time);
            let range = String::from(range);
            let components = String::from(components);
            let duration = String::from(duration);
            let saving_throw = String::from(saving_throw);
            let spell_resistance = String::from(spell_resistance);
            Spell {
                id,
                name,
                spell_school,
                spell_sub_school,
                description,
                short_description,
                cast_time,
                range,
                components,
                duration,
                saving_throw,
                spell_resistance,
                sorc_level,
                wiz_level,
                cleric_level,
                druid_level,
                ranger_level,
                bard_level,
                pal_level,
                sla_level,
                element_mask
            }
    }

    pub fn sorc_level_str(&self) -> String {
        match self.sorc_level {
            Some(lvl) => lvl.to_string(),
            None => "NULL".to_string(),
        }
    }

    pub fn wiz_level_str(&self) -> String {
        match self.wiz_level {
            Some(lvl) => lvl.to_string(),
            None => "NULL".to_string(),
        }
    }

    pub fn bard_level_str(&self) -> String {
        match self.bard_level {
            Some(lvl) => lvl.to_string(),
            None => "NULL".to_string(),
        }
    }

    pub fn cleric_level_str(&self) -> String {
        match self.cleric_level {
            Some(lvl) => lvl.to_string(),
            None => "NULL".to_string(),
        }
    }

    pub fn ranger_level_str(&self) -> String {
        match self.ranger_level {
            Some(lvl) => lvl.to_string(),
            None => "NULL".to_string(),
        }
    }

    pub fn druid_level_str(&self) -> String {
        match self.druid_level {
            Some(lvl) => lvl.to_string(),
            None => "NULL".to_string(),
        }
    }

    pub fn pal_level_str(&self) -> String {
        match self.pal_level {
            Some(lvl) => lvl.to_string(),
            None => "NULL".to_string(),
        }
    }

    pub fn sla_level_str(&self) -> String {
        match self.sla_level {
            Some(lvl) => lvl.to_string(),
            None => "NULL".to_string(),
        }
    }
    
    pub fn is_acid(&self) -> bool {
        self.element_mask & 0x1 > 0
    }

    pub fn is_air(&self) -> bool {
        self.element_mask & 0x2 > 0
    }

    pub fn is_chaotic(&self) -> bool {
        self.element_mask & 0x4 > 0
    }

    pub fn is_cold(&self) -> bool {
        self.element_mask & 0x8 > 0
    }

    pub fn is_curse(&self) -> bool {
        self.element_mask & 0x10 > 0
    }

    pub fn is_darkness(&self) -> bool {
        self.element_mask & 0x20 > 0
    }

    pub fn is_death(&self) -> bool {
        self.element_mask & 0x40 > 0
    }

    pub fn is_disease(&self) -> bool {
        self.element_mask & 0x80 > 0
    }

    pub fn is_earth(&self) -> bool {
        self.element_mask & 0x100 > 0
    }

    pub fn is_electricity(&self) -> bool {
        self.element_mask & 0x200 > 0
    }

    pub fn is_emotion(&self) -> bool {
        self.element_mask & 0x400 > 0
    }

    pub fn is_evil(&self) -> bool {
        self.element_mask & 0x800 > 0
    }

    pub fn is_fear(&self) -> bool {
        self.element_mask & 0x1000 > 0
    }

    pub fn is_fire(&self) -> bool {
        self.element_mask & 0x2000 > 0
    }

    pub fn is_force(&self) -> bool {
        self.element_mask & 0x4000 > 0
    }

    pub fn is_good(&self) -> bool {
        self.element_mask & 0x8000 > 0
    }

    pub fn is_lawful(&self) -> bool {
        self.element_mask & 0x10000 > 0
    }

    pub fn is_light(&self) -> bool {
        self.element_mask & 0x20000 > 0
    }

    pub fn is_mind_affecting(&self) -> bool {
        self.element_mask & 0x40000 > 0
    }

    pub fn is_pain(&self) -> bool {
        self.element_mask & 0x80000 > 0
    }

    pub fn is_poison(&self) -> bool {
        self.element_mask & 0x100000 > 0
    }

    pub fn is_shadow(&self) -> bool {
        self.element_mask & 0x200000 > 0
    }

    pub fn is_sonic(&self) -> bool {
        self.element_mask & 0x400000 > 0
    }

    pub fn is_water(&self) -> bool {
        self.element_mask & 0x800000 > 0
    }
}
