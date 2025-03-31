use crate::race;
use regex::Regex;

pub const SP_TO_CP: u32 = 10;
pub const GP_TO_CP: u32 = 100;
pub const PP_TO_CP: u32 = 1000;

#[derive(Clone, Copy)]
pub enum EquipmentCategoryMarker {
    AllEquipment = 0,
    OtherEquipment = 1,
    AdventuringGear = 2,
    ToolsAndSkillKits = 3,
    Clothing = 4,
    MountsEtc = 5,
    Transport = 6,
    FoodDrink = 7,
    TradeGoods = 8,
    SpecialSubstances = 9,
    Services = 10,
    Weapons = 11,
    Armor = 12,
}

pub fn currency_to_string(value: u32) -> String {
    let mut out_str = String::new();
    let mut mut_val = value;
    if mut_val > PP_TO_CP {
        out_str.push_str(&format!("{} pp", value / PP_TO_CP));
    }
    mut_val = mut_val % PP_TO_CP;
    if mut_val > GP_TO_CP {
        if out_str.is_empty() {
            out_str.push_str(&format!("{} gp", mut_val / GP_TO_CP));
        } else {
            out_str.push_str(&format!(", {} gp", mut_val / GP_TO_CP));
        }
    }
    mut_val = mut_val % GP_TO_CP;
    if mut_val > SP_TO_CP {
        if out_str.is_empty() {
            out_str.push_str(&format!("{} sp", mut_val / SP_TO_CP));
        } else {
            out_str.push_str(&format!(", {} sp", mut_val / SP_TO_CP));
        }
    }
    mut_val = mut_val % SP_TO_CP;
    if mut_val > 0 {
        if out_str.is_empty() {
            out_str.push_str(&format!("{} cp", mut_val));
        } else {
            out_str.push_str(&format!(", {} cp", mut_val));
        }
    }

    out_str
}

pub fn string_to_currency(input_string: &String) -> u32 {
    let re = Regex::new(r"(?:(\d+) pp)?(?:,?\s*(\d+) gp)?(?:,?\s*(\d+) sp)?(?:,?\s*(\d+) cp)?").unwrap();

    if let Some(caps) = re.captures(input_string) {
        let pp = caps.get(1).map(|m| m.as_str()).unwrap_or("0").parse::<u32>().unwrap();
        let gp = caps.get(2).map(|m| m.as_str()).unwrap_or("0").parse::<u32>().unwrap();
        let sp = caps.get(3).map(|m| m.as_str()).unwrap_or("0").parse::<u32>().unwrap();
        let cp = caps.get(4).map(|m| m.as_str()).unwrap_or("0").parse::<u32>().unwrap();

        pp * PP_TO_CP + gp * GP_TO_CP + sp * SP_TO_CP + cp
    } else {
        0
    }
}
pub fn index_to_eq_category_id(category_id: u32) -> EquipmentCategoryMarker {
    match category_id {
        0 => EquipmentCategoryMarker::AllEquipment,
        1 => EquipmentCategoryMarker::OtherEquipment,
        2 => EquipmentCategoryMarker::AdventuringGear,
        3 => EquipmentCategoryMarker::ToolsAndSkillKits,
        4 => EquipmentCategoryMarker::Clothing,
        5 => EquipmentCategoryMarker::MountsEtc,
        6 => EquipmentCategoryMarker::Transport,
        7 => EquipmentCategoryMarker::FoodDrink,
        8 => EquipmentCategoryMarker::TradeGoods,
        9 => EquipmentCategoryMarker::SpecialSubstances,
        10 => EquipmentCategoryMarker::Services,
        11 => EquipmentCategoryMarker::Weapons,
        12 => EquipmentCategoryMarker::Armor,
        _ => unreachable!("Unexpected Equipment category ID {category_id}"),
    }
}

pub fn eq_category_id_to_str(category_id: &EquipmentCategoryMarker) -> String {
    match category_id {
        EquipmentCategoryMarker::AllEquipment => "0".to_string(),
        EquipmentCategoryMarker::OtherEquipment => "1".to_string(),
        EquipmentCategoryMarker::AdventuringGear => "2".to_string(),
        EquipmentCategoryMarker::ToolsAndSkillKits => "3".to_string(),
        EquipmentCategoryMarker::Clothing => "4".to_string(),
        EquipmentCategoryMarker::MountsEtc => "5".to_string(),
        EquipmentCategoryMarker::Transport => "6".to_string(),
        EquipmentCategoryMarker::FoodDrink => "7".to_string(),
        EquipmentCategoryMarker::TradeGoods => "8".to_string(),
        EquipmentCategoryMarker::SpecialSubstances => "9".to_string(),
        EquipmentCategoryMarker::Services => "10".to_string(),
        EquipmentCategoryMarker::Weapons => "11".to_string(),
        EquipmentCategoryMarker::Armor => "12".to_string(),
    }
}
#[derive(Clone, Copy)]
pub enum WeaponCategoryMarker {
    LightMeleeWeapon,
    OneHandedMelee,
    TwoHandedMelee,
    Ranged,
    Ammunition
}

pub fn index_to_weapon_category(weapon_category: u32) -> Option<WeaponCategoryMarker> {
    match weapon_category {
        0 => Some(WeaponCategoryMarker::LightMeleeWeapon),
        1 => Some(WeaponCategoryMarker::OneHandedMelee),
        2 => Some(WeaponCategoryMarker::TwoHandedMelee),
        3 => Some(WeaponCategoryMarker::Ranged),
        4 => Some(WeaponCategoryMarker::Ammunition),
        _ => None,
    }
}

pub fn weapon_category_to_index(weapon_category: &WeaponCategoryMarker) -> u32 {
    match weapon_category {
        WeaponCategoryMarker::LightMeleeWeapon => 0,
        WeaponCategoryMarker::OneHandedMelee => 1,
        WeaponCategoryMarker::TwoHandedMelee => 2,
        WeaponCategoryMarker::Ranged => 3,
        WeaponCategoryMarker::Ammunition => 4,
    }
}

#[derive(Clone, Copy)]
pub enum WeaponProficiencyMarker {
    MartialWeapon,
    SimpleWeapon,
    ExoticWeapon,
}

pub fn index_to_proficiency(proficiency: u32) -> Option<WeaponProficiencyMarker> {
    match proficiency {
        0 => Some(WeaponProficiencyMarker::MartialWeapon),
        1 => Some(WeaponProficiencyMarker::SimpleWeapon),
        2 => Some(WeaponProficiencyMarker::ExoticWeapon),
        _ => None,
    }
}

pub fn proficiency_to_index(proficiency: &WeaponProficiencyMarker) -> u32 {
    match proficiency {
        WeaponProficiencyMarker::MartialWeapon => 0,
        WeaponProficiencyMarker::SimpleWeapon => 1,
        WeaponProficiencyMarker::ExoticWeapon => 2,
    }
}

#[derive(Clone, Copy)]
pub enum ArmorCategoryMarker {
    LightArmor,
    MediumArmor,
    HeavyArmor,
    Shield,
    Extra,
}

pub fn index_to_armor_category(armor_category: u32) -> Option<ArmorCategoryMarker> {
    match armor_category {
        0 => Some(ArmorCategoryMarker::LightArmor),
        1 => Some(ArmorCategoryMarker::MediumArmor),
        2 => Some(ArmorCategoryMarker::HeavyArmor),
        3 => Some(ArmorCategoryMarker::Shield),
        4 => Some(ArmorCategoryMarker::Extra),
        _ => None,
    }
}

pub fn armor_category_to_index(armor_category: &ArmorCategoryMarker) -> u32 {
    match armor_category {
        ArmorCategoryMarker::LightArmor => 0,
        ArmorCategoryMarker::MediumArmor => 1,
        ArmorCategoryMarker::HeavyArmor => 2,
        ArmorCategoryMarker::Shield => 3,
        ArmorCategoryMarker::Extra => 4,
    }
}

pub trait Equipment {
    fn get_id(&self) -> u32;

    fn get_name(&self, size: &race::CharacterSize, quality_override: bool) -> String;

    fn get_description(&self) -> String;

    fn get_category(&self) -> EquipmentCategoryMarker;

    fn get_price(&self, size: &race::CharacterSize, quality_override: bool) -> u32;

    fn get_weight(&self, size: &race::CharacterSize) -> f64;
}

#[derive(Clone)]
pub struct GeneralItem {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub item_type: EquipmentCategoryMarker,
    pub price: u32,
    pub weight: f64,
    pub size_mod: bool,
}

impl GeneralItem {
    pub fn new(
        id: u32,
        name: &String,
        description: &String,
        item_type: EquipmentCategoryMarker,
        price: u32,
        weight: f64,
        size_mod: bool) -> GeneralItem {
            GeneralItem {
                id,
                name: String::from(name),
                description: String::from(description),
                item_type,
                price,
                weight,
                size_mod,
            }
    }
}
impl Equipment for &GeneralItem {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_name(&self, size: &race::CharacterSize, _quality_override: bool) -> String {
        let mut outstring = String::new();

        if size.text != "Medium" {
            outstring = format!("{} ", size.text.clone());
        }

        outstring += &self.name.clone();

        outstring
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_category(&self) -> EquipmentCategoryMarker {
        self.item_type
    }

    fn get_price(&self, _size: &race::CharacterSize, _quality_override: bool) -> u32 {
        self.price
    }

    fn get_weight(&self, size: &race::CharacterSize) -> f64 {
        if self.size_mod && size.text == "Small" {
            self.weight / 4.0
        } else {
            self.weight
        }
    }
}

#[derive(Clone)]
pub struct Material {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub ammunition_price_mod: f64,
    pub light_armor_price_mod: f64,
    pub medium_armor_price_mod: f64,
    pub heavy_armor_price_mod: f64,
    pub weapon_price_mod: f64,
}

#[derive(Clone)]
pub struct Weapon {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub price: u32,
    pub weight: f64,
    pub size_mod: bool,
    pub masterwork: bool,
    pub damage_type: String,
    pub weapon_type: WeaponCategoryMarker,
    pub proficiency: WeaponProficiencyMarker,
    pub material: Option<Material>,
    pub dmg_s: String,
    pub dmg_m: String,
    pub crit: String,
    pub range: String,
    pub special: String,
    pub ammo_count: u32,
}

impl Weapon {
    pub fn new(
        id: u32,
        name: &String,
        description: &String,
        price: u32,
        weight: f64,
        size_mod: bool,
        masterwork: bool,
        damage_type: &String,
        weapon_type: WeaponCategoryMarker,
        proficiency: WeaponProficiencyMarker,
        material: Option<Material>,
        dmg_s: &String,
        dmg_m: &String,
        crit: &String,
        range: &String,
        special: &String,
        ammo_count: u32) -> Weapon {
        
        Weapon {
            id,
            name: String::from(name),
            description: String::from(description),
            price,
            weight,
            size_mod,
            masterwork,
            damage_type: String::from(damage_type),
            weapon_type,
            proficiency,
            material,
            dmg_s: String::from(dmg_s),
            dmg_m: String::from(dmg_m),
            crit: String::from(crit),
            range: String::from(range),
            special: String::from(special),
            ammo_count,
        }
    }
}

impl Equipment for &Weapon {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_name(&self, size: &race::CharacterSize, quality_override: bool) -> String {
        let mut outstring = String::new();

        if size.text != "Medium" {
            outstring = format!("{} ", size.text.clone());
        }

        if quality_override || self.masterwork {
            outstring += "Master Work ";
        }

        if self.material.is_some() {
            outstring += &format!("{} ", self.material.as_ref().unwrap().name.clone());
        }
        outstring += &self.name.clone();

        outstring
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_category(&self) -> EquipmentCategoryMarker {
        EquipmentCategoryMarker::Weapons
    }

    fn get_price(&self, size: &race::CharacterSize, quality_override: bool) -> u32 {
        let mut masterwork_mod: u32 = 0;
        if self.masterwork || quality_override {
            masterwork_mod = match self.weapon_type {
                WeaponCategoryMarker::Ammunition => 6 * self.ammo_count * GP_TO_CP,
                _ => 300* GP_TO_CP,
            };
        }

        // TODO: Material price change here
        ((self.price as f64) * size.item_price_mod) as u32 + masterwork_mod
    }

    fn get_weight(&self, size: &race::CharacterSize) -> f64 {
        self.weight * size.item_weight_mod
    }
}

#[derive(Clone)]
pub struct Armor {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub price: u32,
    pub weight: f64,
    pub size_mod: bool,
    pub masterwork: bool,
    pub armor_type: ArmorCategoryMarker,
    pub material: Option<Material>,
    pub armor_bonus: u32,
    pub max_dex_bonus: Option<u32>,
    pub armor_check_penalty: i32,
    pub spell_fail_chance: u32,
    pub speed_30: Option<u32>,
    pub speed_20: Option<u32>,
}

impl Armor {
    pub fn new(
        id: u32,
        name: &String,
        description: &String,
        price: u32,
        weight: f64,
        size_mod: bool,
        masterwork: bool,
        armor_type: ArmorCategoryMarker,
        material: Option<Material>,
        armor_bonus: u32,
        max_dex_bonus: Option<u32>,
        armor_check_penalty: i32,
        spell_fail_chance: u32,
        speed_30: Option<u32>,
        speed_20: Option<u32>) -> Armor {
            Armor {
                id,
                name: String::from(name),
                description: String::from(description),
                price,
                weight,
                size_mod,
                masterwork,
                armor_type,
                material,
                armor_bonus,
                max_dex_bonus,
                armor_check_penalty,
                spell_fail_chance,
                speed_30,
                speed_20,
            }
        }
}
impl Equipment for &Armor {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_name(&self, size: &race::CharacterSize, quality_override: bool) -> String {
        let mut outstring = String::new();

        if size.text != "Medium" {
            outstring = format!("{} ", size.text.clone());
        }

        if quality_override || self.masterwork {
            outstring += "Master Work ";
        }

        if self.material.is_some() {
            outstring += &format!("{} ", self.material.as_ref().unwrap().name.clone());
        }
        outstring += &self.name.clone();

        outstring
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_category(&self) -> EquipmentCategoryMarker {
        EquipmentCategoryMarker::Armor
    }

    fn get_price(&self, size: &race::CharacterSize, quality_override: bool) -> u32 {
        let mut masterwork_mod: u32 = 0;
        if self.masterwork || quality_override {
            masterwork_mod = 150 * GP_TO_CP;
        }

        // TODO: Material price change here
        ((self.price as f64) * size.item_price_mod) as u32 + masterwork_mod
    }

    fn get_weight(&self, size: &race::CharacterSize) -> f64 {
        self.weight * size.item_weight_mod
    }
}