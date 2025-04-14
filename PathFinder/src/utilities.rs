use std::collections::HashMap;
use rusqlite;
use crate::{
  spell,
  ability_scores,
  race,
  skill,
  character_class,
  feat,
  equipment
};

pub fn init_sqlit_db(database: &rusqlite::Connection) -> rusqlite::Result<()> {
    //Create SPELLS table
    database.execute(
        "CREATE TABLE SPELLS( \
      ID INTEGER PRIMARY KEY   NOT NULL, \
      NAME  TEXT NOT NULL, \
      SCHOOL  TEXT NOT NULL, \
      SUBSCHOOL TEXT NOT NULL, \
      CASTING_TIME TEXT NOT NULL, \
      COMPONENTS TEXT NOT NULL, \
      RANGE TEXT, \
      DURATION TEXT NOT NULL, \
      SAVING_THROW TEXT, \
      SPELL_RESISTANCE TEXT, \
      SORCERER_LVL  INTEGER, \
      WIZARD_LVL    INTEGER, \
      CLERIC_LVL    INTEGER, \
      DRUID_LVL     INTEGER, \
      RANGER_LVL    INTEGER, \
      BARD_LVL      INTEGER, \
      PALADIN_LVL   INTEGER, \
      SLA_LVL       INTEGER, \
      ELEMENT_MASK  INTEGER, \
      SHORT_DESC TEXT NOT NULL, \
      DESC  TEXT NOT NULL);",
        (),
    )?;

    //Create SKILLS table
    database.execute(
      "CREATE TABLE SKILLS( \
    ID INTEGER PRIMARY KEY   NOT NULL, \
    NAME  TEXT NOT NULL, \
    BASE_ABILITY  TEXT NOT NULL, \
    TRAINED_ONLY INTEGER NOT NULL, \
    AC_PENALTY_APPLIES INTEGER NOT NULL);",
      (),
  )?;

    //Create FEATS table
    database.execute(
        "CREATE TABLE FEATS( \
      ID INTEGER PRIMARY KEY   NOT NULL, \
      NAME  TEXT NOT NULL, \
      TYPE  TEXT NOT NULL, \
      DESC  TEXT NOT NULL, \
      PREREQUISITES TEXT NOT NULL, \
      PREREQUISITE_FEATS TEXT NOT NULL, \
      BENEFITS TEXT NOT NULL, \
      NORMAL TEXT NOT NULL, \
      SPECIAL TEXT NOT NULL, \
      SOURCE TEXT NOT NULL, \
      TEAMWORK INTEGER NOT NULL, \
      CRITICAL INTEGER NOT NULL, \
      GRIT  INTIGER NOT NULL, \
      STYLE INTEGER NOT NULL, \
      PERFORMANCE INTEGER NOT NULL, \
      RACIAL INTEGER NOT NULL, \
      COMPANIONS_FAMILIAR INTEGER NOT NULL, \
      RACE_NAME TEXT, \
      NOTE TEXT, \
      GOAL TEXT, \
      COMPLETION_BENEFIT TEXT, \
      MULTIPLE INTEGER NOT NULL, \
      SUGGESTED_TRAITS TEXT, \
      CHOICE INTEGER);",
        (),
    )?;

    //Create RACES table
    database.execute(
        "CREATE TABLE RACES( \
      ID INTEGER PRIMARY KEY   NOT NULL,  \
      NAME  TEXT NOT NULL, \
      SIZE  TEXT NOT NULL, \
      SPEED INTEGER NOT NULL, \
      BONUS_STR INTEGER, \
      BONUS_DEX INTEGER, \
      BONUS_CON INTEGER, \
      BONUS_INT INTEGER, \
      BONUS_WIS INTEGER, \
      BONUS_CHA INTEGER, \
      BONUS_FLX INTEGER, \
      BONUS_FEAT INTEGER, \
      BONUS_SKILL INTEGER, \
      NUM_FAVCLASS INTEGER, \
      LANG_KNOWN TEXT NOT NULL, \
      LANG_AVAIL TEXT NOT NULL);",
        (),
    )?;

    //Create RACIAL_ABILITIES table
    database.execute(
        "CREATE TABLE RACIAL_ABILITIES (  \
      ID INTEGER PRIMARY KEY   NOT NULL,  \
      RACE_ID INTEGER NOT NULL,  \
      NAME  TEXT NOT NULL,  \
      DESCRIPTION  TEXT NOT NULL, \
      FOREIGN KEY (RACE_ID) REFERENCES RACES (ID) );",
        (),
    )?;

    //Create CLASSES table
    database.execute(
        "CREATE TABLE CLASSES (  \
      ID INTEGER PRIMARY KEY   NOT NULL,  \
      NAME  TEXT NOT NULL,  \
      DESCRIPTION  TEXT NOT NULL, \
      HIT_DIE INTEGER, \
      SKILLS_PER_LEVEL INTEGER, \
      ALIGNMENT_REQS TEXT, \
      CASTER_ABILITY INTEGER, \
      STARTING_WEALTH_D6 INTEGER, \
      CLASS_SKILLS TEXT NOT NULL);",
        (),
    )?;

    //Create CLASS_FEATURES table
    database.execute(
        "CREATE TABLE CLASS_FEATURES (  \
      ID INTEGER PRIMARY KEY   NOT NULL,  \
      CLASS_ID INTEGER NOT NULL, \
      CATEGORY_ID INTEGER NOT NULL, \
      NUM_CHOICES INTEGER, \
      LEVEL_REQ INTEGER NOT NULL, \
      OPTIONAL INTEGER NOT NULL, \
      NAME  TEXT NOT NULL,  \
      DESCRIPTION  TEXT NOT NULL, \
      FOREIGN KEY (CLASS_ID) REFERENCES CLASSES (ID));",
        (),
    )?;

    //Create CLASS_CHOICES table
    database.execute(
        "CREATE TABLE CLASS_CHOICES (  \
      ID INTEGER PRIMARY KEY   NOT NULL,  \
      CLASS_ID INTEGER NOT NULL, \
      CATEGORY_ID INTEGER NOT NULL, \
      LEVEL_REQ INTEGER NOT NULL, \
      NAME  TEXT NOT NULL,  \
      DESCRIPTION  TEXT NOT NULL, \
      MAX_NUM_SELECTIONS INTEGER NOT NULL, \
      NUM_SUBSEQUENT_CHOICES INTEGER NOT NULL, \
      SUBSEQUENT_CHOICE_CATEGORY INTEGER, \
      PREREQ_ID INTEGER, \
      FEAT_ID INTEGER, \
      FOREIGN KEY (CLASS_ID) REFERENCES CLASSES (ID));",
        (),
    )?;

    //Create CLASS_ABILITIES table
    database.execute(
        "CREATE TABLE CLASS_ABILITIES (  \
      ID INTEGER PRIMARY KEY   NOT NULL,  \
      CLASS_ID INTEGER NOT NULL, \
      CHOICE_PREREQ_ID INTEGER, \
      NAME  TEXT NOT NULL,  \
      DESCRIPTION  TEXT NOT NULL, \
      LEVEL_REQ INTEGER NOT NULL, \
      SPELL_ID INTEGER, \
      FEAT_ID INTEGER, \
      FOREIGN KEY (CLASS_ID) REFERENCES CLASSES (ID));",
        (),
    )?;

    //Create CLASS_LEVEL_UPS table
    database.execute(
        "CREATE TABLE CLASS_LEVEL_UPS (  \
      ID INTEGER PRIMARY KEY   NOT NULL,  \
      CLASS_ID INTEGER NOT NULL, \
      LEVEL INTEGER NOT NULL, \
      BASE_ATTACK_BONUS INTEGER NOT NULL, \
      BASE_FORTITUDE_SAVE INTEGER NOT NULL, \
      BASE_REFLEX_SAVE INTEGER NOT NULL, \
      BASE_WILL_SAVE INTEGER NOT NULL, \
      SPELLS_PER_DAY_0 INTEGER, \
      SPELLS_PER_DAY_1 INTEGER, \
      SPELLS_PER_DAY_2 INTEGER, \
      SPELLS_PER_DAY_3 INTEGER, \
      SPELLS_PER_DAY_4 INTEGER, \
      SPELLS_PER_DAY_5 INTEGER, \
      SPELLS_PER_DAY_6 INTEGER, \
      SPELLS_PER_DAY_7 INTEGER, \
      SPELLS_PER_DAY_8 INTEGER, \
      SPELLS_PER_DAY_9 INTEGER, \
      SPELLS_KNOWN_0 INTEGER, \
      SPELLS_KNOWN_1 INTEGER, \
      SPELLS_KNOWN_2 INTEGER, \
      SPELLS_KNOWN_3 INTEGER, \
      SPELLS_KNOWN_4 INTEGER, \
      SPELLS_KNOWN_5 INTEGER, \
      SPELLS_KNOWN_6 INTEGER, \
      SPELLS_KNOWN_7 INTEGER, \
      SPELLS_KNOWN_8 INTEGER, \
      SPELLS_KNOWN_9 INTEGER, \
      FOREIGN KEY (CLASS_ID) REFERENCES CLASSES (ID));",
        (),
    )?;

    //Create GENERAL_EQUIPMENT table
    database.execute(
        "CREATE TABLE GENERAL_EQUIPMENT (  \
      ID INTEGER PRIMARY KEY   NOT NULL,  \
      CATEGORY_ID INTEGER NOT NULL, \
      PRICE INTEGER NOT NULL, \
      WEIGHT_LB REAL NOT NULL, \
      SIZE_MOD INTEGER NOT NULL, \
      NAME TEXT NOT NULL, \
      DESCRIPTION TEXT NOT NULL);",
        (),
    )?;

    //Create WEAPONS table
    database.execute(
        "CREATE TABLE WEAPONS (  \
      ID INTEGER PRIMARY KEY   NOT NULL,  \
      CATEGORY_ID INTEGER NOT NULL, \
      PROFICIENCY_ID INTEGER NOT NULL, \
      PRICE INTEGER NOT NULL, \
      WEIGHT_LB REAL NOT NULL, \
      DMG_S TEXT NOT NULL, \
      DMG_M TEXT NOT NULL, \
      CRIT TEXT NOT NULL, \
      RANGE TEXT NOT NULL, \
      DAMAGE_TYPE TEXT NOT NULL, \
      SPECIAL TEXT NOT NULL, \
      AMMO_COUNT INTEGER NOT NULL, \
      NAME TEXT NOT NULL, \
      DESCRIPTION TEXT NOT NULL);",
        (),
    )?;

    //Create ARMOR table
    database.execute(
        "CREATE TABLE ARMOR (  \
      ID INTEGER PRIMARY KEY   NOT NULL,  \
      CATEGORY_ID INTEGER NOT NULL, \
      PRICE INTEGER NOT NULL, \
      WEIGHT_LB REAL NOT NULL, \
      ARMOR_BONUS INTEGER NOT NULL, \
      MAX_DEX_BONUS INTEGER, \
      ARMOR_CHECK_PENALTY INTEGER NOT NULL, \
      SPELL_FAIL_CHANCE INTEGER NOT NULL, \
      SPEED_30FT INTEGER, \
      SPEED_20FT INTEGER, \
      NAME TEXT NOT NULL, \
      DESCRIPTION TEXT NOT NULL);",
        (),
    )?;

    Ok(())
}

pub fn write_spells_to_db(database: &rusqlite::Connection, spell_vec: &Vec<spell::Spell>) {
    let mut cmd_string = String::with_capacity(32768);

    cmd_string += "INSERT INTO SPELLS (ID, NAME, SCHOOL, SUBSCHOOL, CASTING_TIME, COMPONENTS, RANGE, DURATION, \
     SAVING_THROW, SPELL_RESISTANCE, SORCERER_LVL, WIZARD_LVL, CLERIC_LVL, DRUID_LVL, RANGER_LVL, BARD_LVL, PALADIN_LVL, SLA_LVL, ELEMENT_MASK, SHORT_DESC, DESC)\nVALUES\n";

    for spell in spell_vec {
        cmd_string += &format!(
            "({}, \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", {}, {}, {}, {}, {}, {}, {}, {}, {}, \"{}\", \"{}\"),",
            spell.id, spell.name, spell.spell_school, spell.spell_sub_school, spell.cast_time, spell.components, spell.range,
            spell.duration, spell.saving_throw, spell.spell_resistance, spell.sorc_level_str(), spell.wiz_level_str(), 
            spell.cleric_level_str(), spell.druid_level_str(), spell.ranger_level_str(), spell.bard_level_str(), spell.pal_level_str(), 
            spell.sla_level_str(), spell.element_mask, spell.short_description, spell.description,
        );
    }

    /* Remove ending comma, replace with a semicolon */
    cmd_string.pop();
    cmd_string += ";";

    /* Execute the command */
    database.execute(&cmd_string, ()).unwrap();
}

pub fn read_spells_from_db(database: &rusqlite::Connection, spell_map: &mut HashMap<u32, spell::Spell>) {
  let query_string = "SELECT ID, NAME, SCHOOL, SUBSCHOOL, DESC, SHORT_DESC,\
   CASTING_TIME, RANGE, COMPONENTS, DURATION, SAVING_THROW, SPELL_RESISTANCE,\
   SORCERER_LVL, WIZARD_LVL, CLERIC_LVL, DRUID_LVL, RANGER_LVL, BARD_LVL, \
   PALADIN_LVL, SLA_LVL, ELEMENT_MASK FROM SPELLS\n";

  let mut stmt = database.prepare(query_string).expect("Spell Table Query Failed");
  let mut rows = stmt.query([]).expect("Spell Table Query Failed");

  while let Some(row) = rows.next().unwrap() {
      let id = row.get::<usize, u32>(0).expect("Wrong number of Spell Elements");
      spell_map.entry(id).or_insert_with(|| spell::Spell::new(
        id,
        &row.get::<usize, String>(1).expect(&format!("Wrong number of Spell Elements for {}", id)),
        &row.get::<usize, String>(2).expect(&format!("Wrong number of Spell Elements for {}", id)),
        &row.get::<usize, String>(3).expect(&format!("Wrong number of Spell Elements for {}", id)),
        &row.get::<usize, String>(4).expect(&format!("Wrong number of Spell Elements for {}", id)),
        &row.get::<usize, String>(5).expect(&format!("Wrong number of Spell Elements for {}", id)),
        &row.get::<usize, String>(6).expect(&format!("Wrong number of Spell Elements for {}", id)),
        &row.get::<usize, String>(7).expect(&format!("Wrong number of Spell Elements for {}", id)),
        &row.get::<usize, String>(8).expect(&format!("Wrong number of Spell Elements for {}", id)),
        &row.get::<usize, String>(9).expect(&format!("Wrong number of Spell Elements for {}", id)),
        &row.get::<usize, String>(10).expect(&format!("Wrong number of Spell Elements for {}", id)),
        &row.get::<usize, String>(11).expect(&format!("Wrong number of Spell Elements for {}", id)),
        row.get::<usize, Option<u8>>(12).expect(&format!("Wrong number of Spell Elements for {}", id)),
        row.get::<usize, Option<u8>>(13).expect(&format!("Wrong number of Spell Elements for {}", id)),
        row.get::<usize, Option<u8>>(14).expect(&format!("Wrong number of Spell Elements for {}", id)),
        row.get::<usize, Option<u8>>(15).expect(&format!("Wrong number of Spell Elements for {}", id)),
        row.get::<usize, Option<u8>>(16).expect(&format!("Wrong number of Spell Elements for {}", id)),
        row.get::<usize, Option<u8>>(17).expect(&format!("Wrong number of Spell Elements for {}", id)),
        row.get::<usize, Option<u8>>(18).expect(&format!("Wrong number of Spell Elements for {}", id)),
        row.get::<usize, Option<u8>>(19).expect(&format!("Wrong number of Spell Elements for {}", id)),
        row.get::<usize, u32>(20).expect(&format!("Wrong number of Spell Elements for {}", id)),
      ));
    }
}

pub fn write_races_to_db(database: &rusqlite::Connection, race_vec: &Vec<race::Race>) {
  let mut cmd_string = String::with_capacity(32768);

  cmd_string += "INSERT INTO RACES (ID,NAME,SIZE,SPEED,BONUS_STR,BONUS_DEX,BONUS_CON,BONUS_INT,BONUS_WIS,BONUS_CHA, \
      BONUS_FLX,BONUS_FEAT,BONUS_SKILL,NUM_FAVCLASS,LANG_KNOWN,LANG_AVAIL)\nVALUES\n";

  for race in race_vec {
    cmd_string += &format!(
        "({}, \"{}\", \"{}\", {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, \"{}\", \"{}\"),",
        race.id, race.name, &race.size.text, race.speed,
        race.ability_score_offset(Some(ability_scores::AbilityScore::Strength)),
        race.ability_score_offset(Some(ability_scores::AbilityScore::Dexterity)),
        race.ability_score_offset(Some(ability_scores::AbilityScore::Constitution)),
        race.ability_score_offset(Some(ability_scores::AbilityScore::Intelligence)),
        race.ability_score_offset(Some(ability_scores::AbilityScore::Wisdom)),
        race.ability_score_offset(Some(ability_scores::AbilityScore::Charisma)),
        race.ability_score_offset(None),
        match race.bonus_feat {
          true => 1,
          false => 0,
        }, match race.bonus_skill {
          true => 1,
          false => 0,
        },
        race.num_favored_classes, race.languages_str(), race.languages_available_str(),
    );
  }

  /* Remove ending comma, replace with a semicolon */
  cmd_string.pop();
  cmd_string += ";";

  /* Execute the command */
  database.execute(&cmd_string, ()).unwrap();
}

pub fn read_races_from_db(database: &rusqlite::Connection, race_map: &mut HashMap<u32, race::Race>) {
  let query_string = "SELECT ID,NAME,SIZE,SPEED,BONUS_STR,BONUS_DEX,BONUS_CON,\
      BONUS_INT,BONUS_WIS,BONUS_CHA,BONUS_FLX,BONUS_FEAT,BONUS_SKILL,NUM_FAVCLASS,\
      LANG_KNOWN,LANG_AVAIL FROM RACES\n";

  let mut stmt = database.prepare(query_string).expect("Race Table Query Failed");
  let mut rows = stmt.query([]).expect("Race Table Query Failed");

  while let Some(row) = rows.next().unwrap() {
    let id = row.get::<usize, u32>(0).expect("Wrong number of Race Elements");
    let ability_score_offsets: [i8; ability_scores::NUMBER_ABILITY_SCORES + 1] = [
      row.get::<usize, i8>(4).expect("Wrong number of Race Elements"),
      row.get::<usize, i8>(5).expect("Wrong number of Race Elements"),
      row.get::<usize, i8>(6).expect("Wrong number of Race Elements"),
      row.get::<usize, i8>(7).expect("Wrong number of Race Elements"),
      row.get::<usize, i8>(8).expect("Wrong number of Race Elements"),
      row.get::<usize, i8>(9).expect("Wrong number of Race Elements"),
      row.get::<usize, i8>(10).expect("Wrong number of Race Elements"),
    ];

    let languages: Vec<String> = row.get::<usize, String>(14).expect("Wrong number of Race Elements").split(';').map(String::from).collect();
    let languages_available: Vec<String> = row.get::<usize, String>(15).expect("Wrong number of Race Elements").split(';').map(String::from).collect();

    let racial_query = format!("SELECT ID,NAME,DESCRIPTION FROM RACIAL_ABILITIES WHERE RACE_ID == {}\n", id);
    let mut racial_stmt = database.prepare(&racial_query).expect("Racial Table Query Failed");
    let racial_rows = racial_stmt.query_map([], |racial_row| {
      Ok(race::RacialAbility::new(
        racial_row.get::<usize, u32>(0).expect("Wrong number of Racial Ability elements"),
        id,
        &racial_row.get::<usize, String>(1).expect("Wrong number of Racial Ability elements"),
        &racial_row.get::<usize, String>(2).expect("Wrong number of Racial Ability elements")))
    }).unwrap();
    let mut racials: Vec<race::RacialAbility> = Vec::new();
    for racial in racial_rows {
      racials.push(racial.unwrap());
    }

    let bonus_feat = match row.get::<usize, u32>(11).expect("Wrong number of Race Elements") {
      1 => true,
      0 => false,
      _ => unreachable!("Unexpected value in BONUS_FEAT"),
    };
    let bonus_skill = match row.get::<usize, u32>(12).expect("Wrong number of Race Elements") {
      1 => true,
      0 => false,
      _ => unreachable!("Unexpected value in BONUS_FEAT"),
    };
    race_map.entry(id).or_insert_with(|| race::Race::new(
      id,
      &row.get::<usize, String>(1).expect("Wrong number of Race Elements"),
      &row.get::<usize, String>(2).expect("Wrong number of Race Elements"),
      row.get::<usize, usize>(3).expect("Wrong number of Race Elements"),
      &languages,
      &languages_available,
      &racials,
      ability_score_offsets,
      bonus_feat,
      bonus_skill,
      row.get::<usize, usize>(13).expect("Wrong number of Race Elements"),
    ));
  }
}

pub fn write_racials_to_db(database: &rusqlite::Connection, racial_vec: &Vec<race::RacialAbility>) {
  let mut cmd_string = String::with_capacity(32768);

  cmd_string += "INSERT INTO RACIAL_ABILITIES (ID,RACE_ID,NAME,DESCRIPTION)\nVALUES\n";

  for racial in racial_vec {
      cmd_string += &format!(
          "({}, {}, \"{}\", \"{}\"),",
          racial.id, racial.race_id, racial.name, racial.description,
      );
  }

  /* Remove ending comma, replace with a semicolon */
  cmd_string.pop();
  cmd_string += ";";

  /* Execute the command */
  database.execute(&cmd_string, ()).unwrap();
}

pub fn write_skills_to_db(database: &rusqlite::Connection, skills: &Vec<skill::Skill>/*[skill::Skill; skill::NUMBER_SKILLS]*/) {
  let mut cmd_string = String::with_capacity(32768);

  cmd_string += "INSERT INTO SKILLS (ID,NAME,BASE_ABILITY,TRAINED_ONLY,AC_PENALTY_APPLIES)\nVALUES\n";

  let mut count = 0;
  //for skillIdx in 0..skill::NUMBER_SKILLS {
  for skill in skills {
    let trained_value = match skill.trained_only {//skills[skillIdx].trained_only() {
      true => 1,
      false => 0,
    };
    let ac_penalty_value = match skill.ac_penalty_applies {//skills[skillIdx].ac_penalty_applies() {
      true => 1,
      false => 0,
    };
    cmd_string += &format!(
      "({}, \"{}\", \"{}\", {}, {}),",
      //skillIdx, skills[skillIdx].name(), ability_scores::ability_score_to_string(Some(skills[skillIdx].base_ability())), trained_value, ac_penalty_value,
      count, skill.name, ability_scores::ability_score_to_string(Some(skill.base_ability)), trained_value, ac_penalty_value,
    );
    count = count + 1;
  }

  /* Remove ending comma, replace with a semicolon */
  cmd_string.pop();
  cmd_string += ";";

  /* Execute the command */
  database.execute(&cmd_string, ()).unwrap();
}

pub fn read_skills_from_db(database: &rusqlite::Connection, skill_map: &mut HashMap<u32, skill::Skill>) {
  let query_string = "SELECT ID,NAME,TRAINED_ONLY,AC_PENALTY_APPLIES,BASE_ABILITY FROM SKILLS\n";

  let mut stmt = database.prepare(query_string).expect("Skill Table Query Failed");
  let mut rows = stmt.query([]).expect("Skill Table Query Failed");

  while let Some(row) = rows.next().unwrap() {
    skill_map.entry(row.get::<usize, u32>(0).expect("Wrong number of Skill Elements")).or_insert_with(|| skill::Skill::new(
        &row.get::<usize, String>(1).expect("Wrong number of Skill Elements"),
        0,
        false,
        match row.get::<usize, u32>(2).expect("Wrong number of Skill Elements") {
          0 => false,
          1 => true,
          _ => unreachable!("Unexpected trained only value in skill table"),
        },
        match row.get::<usize, u32>(3).expect("Wrong number of Skill Elements") {
          0 => false,
          1 => true,
          _ => unreachable!("Unexpected ac penalty value in skill table"),
        },
        ability_scores::string_to_ability_score(&row.get::<usize, String>(4).expect("Wrong number of Skill Elements")).unwrap(),
      ));
    }
}
pub fn write_classes_to_db(database: &rusqlite::Connection, classes: &Vec<character_class::CharacterClass>) {
  let mut cmd_string = String::with_capacity(32768);

  cmd_string += "INSERT INTO CLASSES (ID,NAME,DESCRIPTION,HIT_DIE,SKILLS_PER_LEVEL,ALIGNMENT_REQS,CASTER_ABILITY,STARTING_WEALTH_D6,CLASS_SKILLS)\nVALUES\n";

  for class in classes {
    cmd_string += &format!(
      "({}, \"{}\", \"{}\", {}, {},\"{}\",{},{},\"{}\"),",
      class.id, class.name, class.description, class.hit_die, class.skills_per_level, class.alignment_req,
       ability_scores::ability_score_to_index(class.caster_ability), class.starting_wealth_d6, class.class_skill_list,
    );
  }

  /* Remove ending comma, replace with a semicolon */
  cmd_string.pop();
  cmd_string += ";";

  /* Execute the command */
  database.execute(&cmd_string, ()).unwrap();
}

pub fn read_classes_from_db(database: &rusqlite::Connection,
  class_map: &mut HashMap<u32, character_class::CharacterClass>,
  class_choices: &mut HashMap<u32, character_class::ClassChoice>,
  class_abilities: &mut HashMap<u32, character_class::ClassAbility>,
  class_features: &mut HashMap<u32, character_class::ClassFeature>,) {
  let query_string = "SELECT ID,NAME,DESCRIPTION,HIT_DIE,SKILLS_PER_LEVEL,\
    ALIGNMENT_REQS,CLASS_SKILLS,CASTER_ABILITY,STARTING_WEALTH_D6 FROM CLASSES\n";

  let mut stmt = database.prepare(query_string).expect("Class Table Query Failed");
  let mut rows = stmt.query([]).expect("Class Table Query Failed");

  while let Some(row) = rows.next().unwrap() {
    let id = row.get::<usize, u32>(0).expect("Wrong number of Class Elements");
    
    let mut choices: Vec<character_class::ClassChoice> = vec![];
    let mut abilities: Vec<character_class::ClassAbility> = vec![];
    let mut features: Vec<character_class::ClassFeature> = vec![];

    let query = format!("SELECT ID,CATEGORY_ID,NUM_CHOICES,LEVEL_REQ,NAME,DESCRIPTION,OPTIONAL FROM CLASS_FEATURES WHERE CLASS_ID == {}\n", id);
    let mut sub_stmt = database.prepare(&query).expect("Class Feature Table Query Failed");
    let sub_rows = sub_stmt.query_map([], |sub_row| {
      Ok(character_class::ClassFeature::new(
        sub_row.get::<usize, u32>(0).expect("Wrong number of Class Feature elements"),
        id,
        sub_row.get::<usize, u32>(1).expect("Wrong number of Class Feature elements"),
        sub_row.get::<usize, u32>(2).expect("Wrong number of Class Feature elements"),
        sub_row.get::<usize, u32>(3).expect("Wrong number of Class Feature elements"),
        &sub_row.get::<usize, String>(4).expect("Wrong number of Racial Ability elements"),
        &sub_row.get::<usize, String>(5).expect("Wrong number of Racial Ability elements"),
        match sub_row.get::<usize, u32>(6).expect("Wrong number of Class Feature elements") {
          1 => true,
          0 => false,
          _ => unreachable!("Unexpected value for class feature: optional"),
        }))
    }).unwrap();
    for feature in sub_rows {
      let tmp = feature.unwrap();
      features.push(tmp.copy());
      class_features.insert(tmp.id, tmp);
    }

    let query = format!("SELECT ID,CHOICE_PREREQ_ID,NAME,DESCRIPTION,LEVEL_REQ,\
      SPELL_ID,FEAT_ID FROM CLASS_ABILITIES WHERE CLASS_ID == {}\n", id);
    let mut sub_stmt = database.prepare(&query).expect("Class Ability Table Query Failed");
    let sub_rows = sub_stmt.query_map([], |sub_row| {
      let choice_prereq_id = sub_row.get::<usize, Option<u32>>(1).expect("Wrong number of Class Ability elements");
      let spell_id = sub_row.get::<usize, Option<u32>>(5).expect("Wrong number of Class Ability elements");
      let feat_id = sub_row.get::<usize, Option<u32>>(6).expect("Wrong number of Class Ability elements");
      Ok(character_class::ClassAbility::new(
        sub_row.get::<usize, u32>(0).expect("Wrong number of Class Ability elements"),
        id,
        choice_prereq_id,
        &sub_row.get::<usize, String>(2).expect("Wrong number of Class Ability elements"),
        &sub_row.get::<usize, String>(3).expect("Wrong number of Class Ability elements"),
        sub_row.get::<usize, u32>(4).expect("Wrong number of Class Ability elements"),
        spell_id,
        feat_id,
      ))
    }).unwrap();
    for ability in sub_rows {
      let tmp = ability.unwrap();
      abilities.push(tmp.copy());
      class_abilities.insert(tmp.id, tmp);
    }

    
    let query = format!("SELECT ID,CATEGORY_ID,LEVEL_REQ,NAME,DESCRIPTION,\
      MAX_NUM_SELECTIONS,NUM_SUBSEQUENT_CHOICES,SUBSEQUENT_CHOICE_CATEGORY,\
      PREREQ_ID,FEAT_ID FROM CLASS_CHOICES WHERE CLASS_ID == {}\n", id);
    let mut sub_stmt = database.prepare(&query).expect("Class Choice Table Query Failed");
    let sub_rows = sub_stmt.query_map([], |sub_row| {
      let subsequent_choice_id = sub_row.get::<usize, Option<u32>>(1).expect("Wrong number of Class Choice elements");
      let prereq_id = sub_row.get::<usize, Option<u32>>(8).expect("Wrong number of Class Choice elements");
      let feat_id = sub_row.get::<usize, Option<u32>>(9).expect("Wrong number of Class Choice elements");
      Ok(character_class::ClassChoice::new(
        sub_row.get::<usize, u32>(0).expect("Wrong number of Class Choice elements"),
        id,
        sub_row.get::<usize, u32>(1).expect("Wrong number of Class Choice elements"),
        sub_row.get::<usize, u32>(2).expect("Wrong number of Class Choice elements"),
        &sub_row.get::<usize, String>(3).expect("Wrong number of Class Choice elements"),
        &sub_row.get::<usize, String>(4).expect("Wrong number of Class Choice elements"),
        sub_row.get::<usize, u32>(5).expect("Wrong number of Class Choice elements"),
        sub_row.get::<usize, u32>(6).expect("Wrong number of Class Choice elements"),
        subsequent_choice_id,
        prereq_id,
        feat_id,
        "",
      ))
    }).unwrap();
    for choice in sub_rows {
      let tmp = choice.unwrap();
      choices.push(tmp.copy());
      class_choices.insert(tmp.id, tmp);
    }

    let mut level_ups = [character_class::ClassLevelUpInfo::new_blank(); character_class::MAX_CHARACTER_LEVEL];
    let query = format!("SELECT ID,LEVEL,BASE_ATTACK_BONUS,BASE_FORTITUDE_SAVE,\
    BASE_REFLEX_SAVE,BASE_WILL_SAVE,SPELLS_PER_DAY_0,SPELLS_PER_DAY_1,SPELLS_PER_DAY_2,\
    SPELLS_PER_DAY_3,SPELLS_PER_DAY_4,SPELLS_PER_DAY_5,SPELLS_PER_DAY_6,SPELLS_PER_DAY_7,\
    SPELLS_PER_DAY_8,SPELLS_PER_DAY_9,SPELLS_KNOWN_0,SPELLS_KNOWN_1,SPELLS_KNOWN_2,\
    SPELLS_KNOWN_3,SPELLS_KNOWN_4,SPELLS_KNOWN_5,SPELLS_KNOWN_6,SPELLS_KNOWN_7,\
    SPELLS_KNOWN_8,SPELLS_KNOWN_9 FROM CLASS_LEVEL_UPS WHERE CLASS_ID == {}\n", id);
    let mut sub_stmt = database.prepare(&query).expect("Class Level Up Table Query Failed");
    let _ = sub_stmt.query_map([], |sub_row| {
      let level = sub_row.get::<usize, usize>(1).expect("Wrong number of Class Level Up elements");
      level_ups[level] = character_class::ClassLevelUpInfo::new(
        sub_row.get::<usize, u8>(0).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, u8>(1).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, u8>(2).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, u8>(3).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, Option<u8>>(4).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, Option<u8>>(5).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, Option<u8>>(6).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, Option<u8>>(7).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, Option<u8>>(8).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, Option<u8>>(9).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, Option<u8>>(10).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, Option<u8>>(11).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, Option<u8>>(12).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, Option<u8>>(13).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, Option<u8>>(14).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, Option<u8>>(15).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, Option<u8>>(16).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, Option<u8>>(17).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, Option<u8>>(18).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, Option<u8>>(19).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, Option<u8>>(20).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, Option<u8>>(21).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, Option<u8>>(22).expect("Wrong number of Class Level Up elements"),
        sub_row.get::<usize, Option<u8>>(23).expect("Wrong number of Class Level Up elements"),
      );
      Ok(level_ups[level].clone())
    }).unwrap();

    class_map.entry(id).or_insert_with(|| character_class::CharacterClass::new(
      id,
      &row.get::<usize, String>(1).expect("Wrong number of Class Elements"),
      &row.get::<usize, String>(2).expect("Wrong number of Class Elements"),
      row.get::<usize, u32>(3).expect("Wrong number of Class Elements"),
      row.get::<usize, u32>(4).expect("Wrong number of Class Elements"),
      &row.get::<usize, String>(5).expect("Wrong number of Class Elements"),
      &row.get::<usize, String>(6).expect("Wrong number of Class Elements"),
      ability_scores::string_to_ability_score(&row.get::<usize, String>(6).expect("Wrong number of Class Elements")),
      row.get::<usize, u32>(8).expect("Wrong number of Class Elements"),
      &features,
      &abilities,
      &choices,
      &level_ups,
    ));
  }
}

pub fn write_class_features_to_db(database: &rusqlite::Connection, features: &Vec<character_class::ClassFeature>) {
  let mut cmd_string = String::with_capacity(32768);

  cmd_string += "INSERT INTO CLASS_FEATURES (ID,CLASS_ID,CATEGORY_ID,NUM_CHOICES,LEVEL_REQ,OPTIONAL,NAME,DESCRIPTION)\nVALUES\n";

  for feature in features {
    cmd_string += &format!(
      "({}, {}, {}, {}, {}, {}, \"{}\", \"{}\"),",
        feature.id, feature.class_id, feature.category_id, feature.num_choices,
        feature.level_req, match feature.optional {
          true => 1,
          false => 0,
        }, feature.name, feature.description,
    );
  }

  /* Remove ending comma, replace with a semicolon */
  cmd_string.pop();
  cmd_string += ";";

  /* Execute the command */
  database.execute(&cmd_string, ()).unwrap();
}

pub fn write_class_abilities_to_db(database: &rusqlite::Connection, abilities: &Vec<character_class::ClassAbility>) {
  let mut cmd_string = String::with_capacity(32768);
  
  cmd_string += "INSERT INTO CLASS_ABILITIES (ID,CLASS_ID,CHOICE_PREREQ_ID,NAME,DESCRIPTION,LEVEL_REQ,SPELL_ID,FEAT_ID)\nVALUES\n";

  for ability in abilities {
    cmd_string += &format!(
      "({}, {}, {}, \"{}\", \"{}\", {}, {}, {}),",
      ability.id, ability.class_id, ability.choice_prereq_id_str(), &ability.name,
      ability.description, ability.level_req, ability.spell_id_str(), ability.feat_id_str(),
    );
  }

  /* Remove ending comma, replace with a semicolon */
  cmd_string.pop();
  cmd_string += ";";

  /* Execute the command */
  database.execute(&cmd_string, ()).unwrap();
}

pub fn write_class_choices_to_db(database: &rusqlite::Connection, choices: &Vec<character_class::ClassChoice>) {
  let mut cmd_string = String::with_capacity(32768);

  cmd_string += "INSERT INTO CLASS_CHOICES (ID,CLASS_ID,CATEGORY_ID,LEVEL_REQ,NAME,DESCRIPTION,MAX_NUM_SELECTIONS,NUM_SUBSEQUENT_CHOICES,SUBSEQUENT_CHOICE_CATEGORY,PREREQ_ID,FEAT_ID)\nVALUES\n";

  for choice in choices {
    cmd_string += &format!(
      "({}, {}, {}, {}, \"{}\", \"{}\", {}, {}, {}, {}, {}),",
      choice.id, choice.class_id, choice.category_id, choice.level_req, &choice.name,
      &choice.description, choice.max_num_selections, choice.num_subsequent_choices,
      &choice.subsequent_category_str(), &choice.prereq_id_str(), &choice.feat_id_str(),
    );
  }

  /* Remove ending comma, replace with a semicolon */
  cmd_string.pop();
  cmd_string += ";";

  /* Execute the command */
  database.execute(&cmd_string, ()).unwrap();
}

pub fn write_class_level_info_to_db(
  database: &rusqlite::Connection,
  class_level_array: &[[character_class::ClassLevelUpInfo; character_class::MAX_CHARACTER_LEVEL]; character_class::NUMBER_BASE_CLASSES]) {
  let mut cmd_string = String::with_capacity(32768);

  cmd_string += "INSERT INTO CLASS_LEVEL_UPS (ID,CLASS_ID,LEVEL,BASE_ATTACK_BONUS,BASE_FORTITUDE_SAVE,BASE_REFLEX_SAVE,\
    BASE_WILL_SAVE,SPELLS_PER_DAY_0,SPELLS_PER_DAY_1,SPELLS_PER_DAY_2,SPELLS_PER_DAY_3,SPELLS_PER_DAY_4,SPELLS_PER_DAY_5,\
    SPELLS_PER_DAY_6,SPELLS_PER_DAY_7,SPELLS_PER_DAY_8,SPELLS_PER_DAY_9,SPELLS_KNOWN_0,SPELLS_KNOWN_1,SPELLS_KNOWN_2,\
    SPELLS_KNOWN_3,SPELLS_KNOWN_4,SPELLS_KNOWN_5,SPELLS_KNOWN_6,SPELLS_KNOWN_7,SPELLS_KNOWN_8,SPELLS_KNOWN_9)\nVALUES\n";

  let mut counter = 0;
  for class in 0..character_class::NUMBER_BASE_CLASSES {
    for level in 0..character_class::MAX_CHARACTER_LEVEL {
      let level_up = &class_level_array[class][level];
      cmd_string += &format!(
        "({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}),",
        counter, class, level + 1, level_up.base_attack_bonus, level_up.base_fort_save,
        level_up.base_reflex_save, level_up.base_will_save, level_up.spells_per_day_str(0),
        level_up.spells_per_day_str(1), level_up.spells_per_day_str(2),
        level_up.spells_per_day_str(3), level_up.spells_per_day_str(4),
        level_up.spells_per_day_str(5), level_up.spells_per_day_str(6),
        level_up.spells_per_day_str(7), level_up.spells_per_day_str(8),
        level_up.spells_per_day_str(9), level_up.spells_known_str(0),
        level_up.spells_known_str(1), level_up.spells_known_str(2),
        level_up.spells_known_str(3), level_up.spells_known_str(4),
        level_up.spells_known_str(5), level_up.spells_known_str(6),
        level_up.spells_known_str(7), level_up.spells_known_str(8),
        level_up.spells_known_str(9),
      );
      counter += 1;
    }
  }

  /* Remove ending comma, replace with a semicolon */
  cmd_string.pop();
  cmd_string += ";";

  /* Execute the command */
  database.execute(&cmd_string, ()).unwrap();
}

pub fn write_feats_to_db(database: &rusqlite::Connection, feats: &Vec<feat::Feat>) {
  let mut cmd_string = String::with_capacity(32768);

  cmd_string += "INSERT INTO FEATS (ID,NAME,TYPE,DESC,PREREQUISITES, \
    PREREQUISITE_FEATS,BENEFITS,NORMAL,SPECIAL,SOURCE,TEAMWORK,CRITICAL,GRIT, \
    STYLE,PERFORMANCE,RACIAL,COMPANIONS_FAMILIAR,RACE_NAME,NOTE,GOAL, \
    COMPLETION_BENEFIT,MULTIPLE,SUGGESTED_TRAITS,CHOICE)\nVALUES\n";

  for feat in feats {
    cmd_string += &format!(
      "({}, \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \
      {}, {}, {}, {}, {}, {}, {}, \"{}\", \"{}\", \"{}\", \"{}\", {}, \"{}\", {}),",
      feat.id, &feat.name, &feat.category, &feat.description, &feat.prerequisites,
      &feat.prerequisite_feats, &feat.benefit, &feat.normal, &feat.special, 
      &feat.source, feat.teamwork, feat.critical, feat.grit, feat.style, feat.performance,
      feat.racial, feat.companion_familiar, &feat.race_name, &feat.note, &feat.goal,
      &feat.completion_benefit, feat.multiple, &feat.suggested_traits, feat.choice_str(),
    );
  }

  /* Remove ending comma, replace with a semicolon */
  cmd_string.pop();
  cmd_string += ";";

  /* Execute the command */
  database.execute(&cmd_string, ()).unwrap();
}

pub fn read_feats_from_db(database: &rusqlite::Connection, feat_map: &mut HashMap<u32, feat::Feat>) {
  let query_string = "SELECT ID,NAME,TYPE,DESC,PREREQUISITES, \
    PREREQUISITE_FEATS,BENEFITS,NORMAL,SPECIAL,SOURCE,TEAMWORK,CRITICAL,GRIT, \
    STYLE,PERFORMANCE,RACIAL,COMPANIONS_FAMILIAR,RACE_NAME,NOTE,GOAL, \
    COMPLETION_BENEFIT,MULTIPLE,SUGGESTED_TRAITS,CHOICE FROM FEATS\n";

  let mut stmt = database.prepare(query_string).expect("Feat Table Query Failed");
  let mut rows = stmt.query([]).expect("Feat Table Query Failed");

  while let Some(row) = rows.next().unwrap() {
    feat_map.entry(row.get::<usize, u32>(0).expect("Wrong number of Feat Elements")).or_insert_with(|| feat::Feat::new(
      row.get::<usize, u32>(0).expect("Wrong number of Feat Elements"),
      &row.get::<usize, String>(1).expect("Wrong number of Feat Elements"),
      &row.get::<usize, String>(2).expect("Wrong number of Feat Elements"),
      &row.get::<usize, String>(3).expect("Wrong number of Feat Elements"),
      &row.get::<usize, String>(4).expect("Wrong number of Feat Elements"),
      &row.get::<usize, String>(5).expect("Wrong number of Feat Elements"),
      &row.get::<usize, String>(6).expect("Wrong number of Feat Elements"),
      &row.get::<usize, String>(7).expect("Wrong number of Feat Elements"),
      &row.get::<usize, String>(8).expect("Wrong number of Feat Elements"),
      &row.get::<usize, String>(9).expect("Wrong number of Feat Elements"),
      match row.get::<usize, u32>(10).expect("Wrong number of Feat Elements") {
        0 => false,
        1 => true,
        _ => unreachable!("Bad Teamwork value in Feat Table"),
      },
      match row.get::<usize, u32>(11).expect("Wrong number of Feat Elements") {
        0 => false,
        1 => true,
        _ => unreachable!("Bad Critical value in Feat Table"),
      },
      match row.get::<usize, u32>(12).expect("Wrong number of Feat Elements") {
        0 => false,
        1 => true,
        _ => unreachable!("Bad Grit value in Feat Table"),
      },
      match row.get::<usize, u32>(13).expect("Wrong number of Feat Elements") {
        0 => false,
        1 => true,
        _ => unreachable!("Bad Style value in Feat Table"),
      },
      match row.get::<usize, u32>(14).expect("Wrong number of Feat Elements") {
        0 => false,
        1 => true,
        _ => unreachable!("Bad Performance value in Feat Table"),
      },
      match row.get::<usize, u32>(15).expect("Wrong number of Feat Elements") {
        0 => false,
        1 => true,
        _ => unreachable!("Bad Racial value in Feat Table"),
      },
      match row.get::<usize, u32>(16).expect("Wrong number of Feat Elements") {
        0 => false,
        1 => true,
        _ => unreachable!("Bad Companions/Familiar value in Feat Table"),
      },
      &row.get::<usize, String>(17).expect("Wrong number of Feat Elements"),
      &row.get::<usize, String>(18).expect("Wrong number of Feat Elements"),
      &row.get::<usize, String>(19).expect("Wrong number of Feat Elements"),
      &row.get::<usize, String>(20).expect("Wrong number of Feat Elements"),
      match row.get::<usize, u32>(21).expect("Wrong number of Feat Elements") {
        0 => false,
        _ => true,
      },
      &row.get::<usize, String>(22).expect("Wrong number of Feat Elements"),
      None,
    ));
  }
}

pub fn write_general_equipment_to_db(database: &rusqlite::Connection, items: &Vec<equipment::GeneralItem>) {
  let mut cmd_string = String::with_capacity(32768);

  cmd_string += "INSERT INTO GENERAL_EQUIPMENT (ID,CATEGORY_ID,PRICE,WEIGHT_LB,SIZE_MOD,NAME,DESCRIPTION)\nVALUES\n";

  for item in items {
    cmd_string += &format!(
      "({}, {}, {}, {}, {}, \"{}\", \"{}\"),",
      item.id, equipment::eq_category_id_to_str(&item.item_type), item.price, item.weight,
      item.size_mod, &item.name, &item.description
    );
  }

  /* Remove ending comma, replace with a semicolon */
  cmd_string.pop();
  cmd_string += ";";

  /* Execute the command */
  database.execute(&cmd_string, ()).unwrap();
}

pub fn read_general_equipment_from_db(database: &rusqlite::Connection, item_map: &mut HashMap<u32, equipment::GeneralItem>) {
  let query_string = "SELECT ID,NAME,DESCRIPTION,CATEGORY_ID,PRICE,WEIGHT_LB,SIZE_MOD FROM GENERAL_EQUIPMENT\n";

  let mut stmt = database.prepare(query_string).expect("General Equipment Table Query Failed");
  let mut rows = stmt.query([]).expect("General Equipment Table Query Failed");

  while let Some(row) = rows.next().unwrap() {
    item_map.entry(row.get::<usize, u32>(0).expect("Wrong number of General Equipment Elements")).or_insert_with(|| equipment::GeneralItem::new(
      row.get::<usize, u32>(0).expect("Wrong number of General Equipment Elements"),
      &row.get::<usize, String>(1).expect("Wrong number of General Equipment Elements"),
      &row.get::<usize, String>(2).expect("Wrong number of General Equipment Elements"),
      equipment::index_to_eq_category_id(row.get::<usize, u32>(3).expect("Wrong number of General Equipment Elements")),
      row.get::<usize, u32>(4).expect("Wrong number of General Equipment Elements"),
      row.get::<usize, f64>(5).expect("Wrong number of General Equipment Elements"),
      match row.get::<usize, u32>(6).expect("Wrong number of General Equipment Elements") {
        0 => false,
        1 => true,
        _ => unreachable!("Unexpected size_mod value in General Equipment table")
      },
    ));
  }
}

pub fn write_weapons_to_db(database: &rusqlite::Connection, items: &Vec<equipment::Weapon>) {
  let mut cmd_string = String::with_capacity(32768);
  
  cmd_string += "INSERT INTO WEAPONS (ID,CATEGORY_ID,PROFICIENCY_ID,PRICE,WEIGHT_LB,DMG_S,DMG_M,CRIT,RANGE,DAMAGE_TYPE,SPECIAL,AMMO_COUNT,NAME,DESCRIPTION)\nVALUES\n";

  for item in items {
    cmd_string += &format!(
      "({}, {}, {}, {}, {}, \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\"),",
      item.id, equipment::weapon_category_to_index(&item.weapon_type), equipment::proficiency_to_index(&item.proficiency), 
      item.price, item.weight, &item.dmg_s, &item.dmg_m, &item.crit, &item.range,
      &item.damage_type, &item.special, item.ammo_count, &item.name, &item.description
    );
  }

  /* Remove ending comma, replace with a semicolon */
  cmd_string.pop();
  cmd_string += ";";

  /* Execute the command */
  database.execute(&cmd_string, ()).unwrap();
}

pub fn read_weapons_from_db(database: &rusqlite::Connection, weapon_map: &mut HashMap<u32, equipment::Weapon>) {
  let query_string = "SELECT ID,NAME,DESCRIPTION,PRICE,WEIGHT_LB,DAMAGE_TYPE,\
    CATEGORY_ID,PROFICIENCY_ID,DMG_S,DMG_M,CRIT,RANGE,SPECIAL,AMMO_COUNT FROM WEAPONS\n";

  let mut stmt = database.prepare(query_string).expect("Weapon Table Query Failed");
  let mut rows = stmt.query([]).expect("Weapon Table Query Failed");

  while let Some(row) = rows.next().unwrap() {
    weapon_map.entry(row.get::<usize, u32>(0).expect("Wrong number of Weapon Elements")).or_insert_with(|| equipment::Weapon::new(
      row.get::<usize, u32>(0).expect("Wrong number of Weapon Elements"),
      &row.get::<usize, String>(1).expect("Wrong number of Weapon Elements"),
      &row.get::<usize, String>(2).expect("Wrong number of Weapon Elements"),
      row.get::<usize, u32>(3).expect("Wrong number of Weapon Elements"),
      row.get::<usize, f64>(4).expect("Wrong number of Weapon Elements"),
      true,
      false,
      &row.get::<usize, String>(5).expect("Wrong number of Weapon Elements"),
      equipment::index_to_weapon_category(row.get::<usize, u32>(6).expect("Wrong number of Weapon Elements")).unwrap(),
      equipment::index_to_proficiency(row.get::<usize, u32>(7).expect("Wrong number of Weapon Elements")).unwrap(),
      None,
      &row.get::<usize, String>(8).expect("Wrong number of Weapon Elements"),
      &row.get::<usize, String>(9).expect("Wrong number of Weapon Elements"),
      &row.get::<usize, String>(10).expect("Wrong number of Weapon Elements"),
      &row.get::<usize, String>(11).expect("Wrong number of Weapon Elements"),
      &row.get::<usize, String>(12).expect("Wrong number of Weapon Elements"),
      row.get::<usize, u32>(13).expect("Wrong number of Weapon Elements"),
    ));
  }
}

pub fn write_armor_to_db(database: &rusqlite::Connection, items: &Vec<equipment::Armor>) {
  let mut cmd_string = String::with_capacity(32768);
  
  cmd_string += "INSERT INTO ARMOR (ID,CATEGORY_ID,PRICE,WEIGHT_LB,ARMOR_BONUS,\
    MAX_DEX_BONUS,ARMOR_CHECK_PENALTY,SPELL_FAIL_CHANCE,SPEED_30FT,SPEED_20FT,\
    NAME,DESCRIPTION)\nVALUES\n";

  for item in items {
    cmd_string += &format!(
      "({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, \"{}\", \"{}\"),",
      item.id, equipment::armor_category_to_index(&item.armor_type),
      item.price, item.weight, item.armor_bonus,
      match item.max_dex_bonus {
        Some(x) => x.to_string(),
        None => String::from("NULL"),
      }, item.armor_check_penalty, item.spell_fail_chance,
      match item.speed_30 {
        Some(x) => x.to_string(),
        None => String::from("NULL"),
      },
      match item.speed_20 {
        Some(x) => x.to_string(),
        None => String::from("NULL"),
      }, &item.name, &item.description
    );
  }

  /* Remove ending comma, replace with a semicolon */
  cmd_string.pop();
  cmd_string += ";";

  /* Execute the command */
  database.execute(&cmd_string, ()).unwrap();
}

pub fn read_armor_from_db(database: &rusqlite::Connection, armor_map: &mut HashMap<u32, equipment::Armor>) {
  let query_string = "SELECT ID,NAME,DESCRIPTION,PRICE,WEIGHT_LB,CATEGORY_ID,ARMOR_BONUS,\
    MAX_DEX_BONUS,ARMOR_CHECK_PENALTY,SPELL_FAIL_CHANCE,SPEED_30FT,SPEED_20FT FROM ARMOR\n";

  let mut stmt = database.prepare(query_string).expect("Armor Table Query Failed");
  let mut rows = stmt.query([]).expect("Armor Table Query Failed");

  while let Some(row) = rows.next().unwrap() {
    armor_map.entry(row.get::<usize, u32>(0).expect("Wrong number of Armor Elements")).or_insert_with(|| equipment::Armor::new(
      row.get::<usize, u32>(0).expect("Wrong number of Armor Elements"),
      &row.get::<usize, String>(1).expect("Wrong number of Armor Elements"),
      &row.get::<usize, String>(2).expect("Wrong number of Armor Elements"),
      row.get::<usize, u32>(3).expect("Wrong number of Armor Elements"),
      row.get::<usize, f64>(4).expect("Wrong number of Armor Elements"),
      true,
      false,
      equipment::index_to_armor_category(row.get::<usize, u32>(5).expect("Wrong number of Armor Elements")).unwrap(),
      None,
      row.get::<usize, u32>(6).expect("Wrong number of Armor Elements"),
      row.get::<usize, Option<u32>>(7).expect("Wrong number of Armor Elements"),
      row.get::<usize, i32>(8).expect("Wrong number of Armor Elements"),
      row.get::<usize, u32>(9).expect("Wrong number of Armor Elements"),
      row.get::<usize, Option<u32>>(10).expect("Wrong number of Armor Elements"),
      row.get::<usize, Option<u32>>(11).expect("Wrong number of Armor Elements"),
    ));
  }
}