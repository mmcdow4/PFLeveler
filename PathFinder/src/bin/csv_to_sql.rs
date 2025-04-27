use rusqlite::Connection;
use std::{
    fs,
    env,
    process,
    str,
    // io::{self, prelude::*, BufReader, BufRead, Seek, SeekFrom},
    collections::HashMap,
    error::Error,
};
use csv;
use PathFinder::{
    ability_scores,
    character_class,
    race,
    skill,
    spell,
    feat,
    equipment,
    utilities::{self}};

fn main() {
    let cfg: CsvToSqlConfig =
        CsvToSqlConfig::build(env::args()).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments : {err}");
            process::exit(1);
    });

    if fs::exists(&cfg.output_file).unwrap() {
        fs::remove_file(&cfg.output_file).unwrap();
    }

    let db_connection = Connection::open(cfg.output_file).unwrap();

    utilities::init_sqlit_db(&db_connection).unwrap();

    // Parse Spells into database
    {
        let spell_csv_name = String::from(String::from(&cfg.input_path) + "\\spells.tsv");
        let spell_map = parse_csv_file(&spell_csv_name, b'\t')
            .expect(&format!("Unable to parse spell CSV {spell_csv_name}"));
        let line_count = spell_map.get("id").unwrap().len();
        let mut spell_list = Vec::with_capacity(line_count);

        println!("Read {line_count} spells from {spell_csv_name}");
        for spell_idx in 0..line_count {
            let id = spell_map.get("id").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let sorc_level = match spell_map.get("sor").unwrap().get(spell_idx).unwrap().parse::<u8>() {
                Ok(lvl) => Some(lvl),
                Err(_) => None,
            };
            let wiz_level = match spell_map.get("wiz").unwrap().get(spell_idx).unwrap().parse::<u8>() {
                Ok(lvl) => Some(lvl),
                Err(_) => None,
            };
            let cleric_level = match spell_map.get("cleric").unwrap().get(spell_idx).unwrap().parse::<u8>() {
                Ok(lvl) => Some(lvl),
                Err(_) => None,
            };
            let bard_level = match spell_map.get("bard").unwrap().get(spell_idx).unwrap().parse::<u8>() {
                Ok(lvl) => Some(lvl),
                Err(_) => None,
            };
            let druid_level = match spell_map.get("druid").unwrap().get(spell_idx).unwrap().parse::<u8>() {
                Ok(lvl) => Some(lvl),
                Err(_) => None,
            };
            let pal_level = match spell_map.get("paladin").unwrap().get(spell_idx).unwrap().parse::<u8>() {
                Ok(lvl) => Some(lvl),
                Err(_) => None,
            };
            let ranger_level = match spell_map.get("ranger").unwrap().get(spell_idx).unwrap().parse::<u8>() {
                Ok(lvl) => Some(lvl),
                Err(_) => None,
            };
            let sla_level = match spell_map.get("SLA_Level").unwrap().get(spell_idx).unwrap().parse::<u8>() {
                Ok(lvl) => Some(lvl),
                Err(_) => None,
            };
            let name = spell_map.get("name").unwrap().get(spell_idx).unwrap();
            let cast_time = spell_map.get("casting_time").unwrap().get(spell_idx).unwrap();
            let spell_school = spell_map.get("school").unwrap().get(spell_idx).unwrap();
            let spell_sub_school = spell_map.get("subschool").unwrap().get(spell_idx).unwrap();
            let range = spell_map.get("range").unwrap().get(spell_idx).unwrap();
            let components = spell_map.get("components").unwrap().get(spell_idx).unwrap();
            let duration = spell_map.get("duration").unwrap().get(spell_idx).unwrap();
            let saving_throw = spell_map.get("saving_throw").unwrap().get(spell_idx).unwrap();
            let spell_resistance = spell_map.get("spell_resistance").unwrap().get(spell_idx).unwrap();
            let description = spell_map.get("description").unwrap().get(spell_idx).unwrap();
            let short_description = spell_map.get("short_description").unwrap().get(spell_idx).unwrap();
            
            /* construct the element mask */
            let acid = spell_map.get("acid").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let air = spell_map.get("air").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let chaotic = spell_map.get("chaotic").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let cold = spell_map.get("cold").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let curse = spell_map.get("curse").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let darkness = spell_map.get("darkness").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let death = spell_map.get("death").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let disease = spell_map.get("disease").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let earth = spell_map.get("earth").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let electricity = spell_map.get("electricity").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let emotion = spell_map.get("emotion").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let evil = spell_map.get("evil").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let fear = spell_map.get("fear").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let fire = spell_map.get("fire").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let force = spell_map.get("force").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let good = spell_map.get("good").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let lawful = spell_map.get("lawful").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let light = spell_map.get("light").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let mind_affecting = spell_map.get("mind_affecting").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let pain = spell_map.get("pain").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let poison = spell_map.get("poison").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let shadow = spell_map.get("shadow").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let sonic = spell_map.get("sonic").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            let water = spell_map.get("water").unwrap().get(spell_idx).unwrap().parse::<u32>().unwrap();
            
            let element_mask: u32 = acid + (air << 1) + (chaotic << 2) + (cold << 3) +
                (curse << 4) + (darkness << 5) + (death << 6) + (disease << 7) + (earth << 8) +
                (electricity << 9) + (emotion << 10) + (evil << 11) + (fear << 12) +
                (fire << 13) + (force << 14) + (good << 15) + (lawful << 16) + (light << 17) +
                (mind_affecting << 18) + (pain << 19) + (poison << 20) + (shadow << 21) +
                (sonic << 22) + (water << 23);

            
            let new_spell = spell::Spell::new(id, &name, &spell_school, &spell_sub_school, description, short_description, cast_time, range, components, duration, saving_throw, spell_resistance, sorc_level, wiz_level, cleric_level, druid_level, ranger_level, bard_level, pal_level, sla_level, element_mask);
            spell_list.push(new_spell);
        }

        // And finally, write the spells
        utilities::write_spells_to_db(&db_connection, &spell_list);
    }

    // Parse Races into db
    {
        let race_csv_name = String::from(String::from(&cfg.input_path) + "\\races.tsv");
        let race_map = parse_csv_file(&race_csv_name, b'\t')
            .expect(&format!("Unable to parse race CSV {race_csv_name}"));
        let line_count = race_map.get("id").unwrap().len();
        let mut race_list = Vec::with_capacity(line_count);


        println!("Read {line_count} races from {race_csv_name}");
        for race_idx in 0..line_count {
            let mut ability_score_offsets: [i8; ability_scores::NUMBER_ABILITY_SCORES + 1] = [0; ability_scores::NUMBER_ABILITY_SCORES + 1];
            let id = race_map.get("id").unwrap().get(race_idx).unwrap().parse::<u32>().unwrap();
            let name = race_map.get("name").unwrap().get(race_idx).unwrap();
            let size = race_map.get("size").unwrap().get(race_idx).unwrap();
            let speed = race_map.get("speed").unwrap().get(race_idx).unwrap().parse::<usize>().unwrap();
            let languages: Vec<String> = race_map.get("langKnown").unwrap().get(race_idx).unwrap().split(';').map(|s| String::from(s)).collect();
            let languages_available: Vec<String> = race_map.get("langAvail").unwrap().get(race_idx).unwrap().split(';').map(|s| String::from(s)).collect();
            let bonus_feat = race_map.get("bonusFeat").unwrap().get(race_idx).unwrap().parse::<usize>().unwrap() > 0;
            let bonus_skill = race_map.get("bonusSkill").unwrap().get(race_idx).unwrap().parse::<usize>().unwrap() > 0;
            let num_favored_classes = race_map.get("numFavoredClass").unwrap().get(race_idx).unwrap().parse::<usize>().unwrap();
            ability_score_offsets[
                ability_scores::ability_score_to_index(Some(ability_scores::AbilityScore::Strength))
            ] = race_map.get("bonusStr").unwrap().get(race_idx).unwrap().parse::<i8>().unwrap();
            ability_score_offsets[
                ability_scores::ability_score_to_index(Some(ability_scores::AbilityScore::Dexterity))
            ] = race_map.get("bonusDex").unwrap().get(race_idx).unwrap().parse::<i8>().unwrap();
            ability_score_offsets[
                ability_scores::ability_score_to_index(Some(ability_scores::AbilityScore::Constitution))
            ] = race_map.get("bonusCon").unwrap().get(race_idx).unwrap().parse::<i8>().unwrap();
            ability_score_offsets[
                ability_scores::ability_score_to_index(Some(ability_scores::AbilityScore::Wisdom))
            ] = race_map.get("bonusWis").unwrap().get(race_idx).unwrap().parse::<i8>().unwrap();
            ability_score_offsets[
                ability_scores::ability_score_to_index(Some(ability_scores::AbilityScore::Intelligence))
            ] = race_map.get("bonusInt").unwrap().get(race_idx).unwrap().parse::<i8>().unwrap();
            ability_score_offsets[
                ability_scores::ability_score_to_index(Some(ability_scores::AbilityScore::Charisma))
            ] = race_map.get("bonusCha").unwrap().get(race_idx).unwrap().parse::<i8>().unwrap();
            ability_score_offsets[
                ability_scores::ability_score_to_index(None)
            ] = race_map.get("bonusFlex").unwrap().get(race_idx).unwrap().parse::<i8>().unwrap();

            let racials: Vec<race::RacialAbility> = vec![];
            let new_race = race::Race::new(id, name, size, speed, &languages, &languages_available, &racials, ability_score_offsets, bonus_feat, bonus_skill, num_favored_classes);
            
            race_list.push(new_race);
        }

        utilities::write_races_to_db(&db_connection, &race_list);
    }

    // Parse Racials into db
    {
        let racial_csv_name = String::from(String::from(&cfg.input_path) + "\\race_abilities.tsv");
        let racial_map = parse_csv_file(&racial_csv_name, b'\t')
            .expect(&format!("Unable to parse racial ability CSV {racial_csv_name}"));
        let line_count = racial_map.get("id").unwrap().len();
        let mut racial_list = Vec::with_capacity(line_count);


        println!("Read {line_count} racial abilitis from {racial_csv_name}");
        for racial_idx in 0..line_count {
            let id = racial_map.get("id").unwrap().get(racial_idx).unwrap().parse::<u32>().unwrap();
            let race_id = racial_map.get("raceId").unwrap().get(racial_idx).unwrap().parse::<u32>().unwrap();
            let name = racial_map.get("name").unwrap().get(racial_idx).unwrap();
            let description = racial_map.get("description").unwrap().get(racial_idx).unwrap();

            let new_racial = race::RacialAbility::new(id, race_id, name, description);
            racial_list.push(new_racial);
        }

        utilities::write_racials_to_db(&db_connection, &racial_list);
    }

    // Parse Skills into db
    {
        let skill_csv_name = String::from(String::from(&cfg.input_path) + "\\skills.tsv");
        let skill_map = parse_csv_file(&skill_csv_name, b'\t')
            .expect(&format!("Unable to parse spell CSV {skill_csv_name}"));
        let line_count = skill_map.get("id").unwrap().len();
        let mut skill_list = Vec::with_capacity(line_count);
        
        println!("Read {line_count} skills from {skill_csv_name}");
        for skill_idx in 0..line_count {
            let _id = skill_map.get("id").unwrap().get(skill_idx).unwrap().parse::<u32>().unwrap();
            let trained_only = match skill_map.get("trainedOnly").unwrap().get(skill_idx).unwrap().parse::<u32>().unwrap() {
                1 => true,
                _ => false,
            };
            let ac_applied_val = skill_map.get("acPenaltyApplies").unwrap().get(skill_idx).unwrap().parse::<u32>().unwrap();
            let ac_applied = match ac_applied_val {
                1 => true,
                _ => false,
            };
            let name = skill_map.get("name").unwrap().get(skill_idx).unwrap();
            let ability_str = skill_map.get("baseAbility").unwrap().get(skill_idx).unwrap();
            let base_ability = ability_scores::string_to_ability_score(ability_str).unwrap();

            let new_skill = skill::Skill::new(name, 0, false, trained_only, ac_applied, base_ability);
            skill_list.push(new_skill);
        }

        utilities::write_skills_to_db(&db_connection, &skill_list);
    }
    
    // Parse Classes into db
    {
        let class_csv_name = String::from(String::from(&cfg.input_path) + "\\classes.tsv");
        let class_map = parse_csv_file(&class_csv_name, b'\t')
            .expect(&format!("Unable to parse spell CSV {class_csv_name}"));
        let line_count = class_map.get("id").unwrap().len();
        let mut class_list = Vec::with_capacity(line_count);


        println!("Read {line_count} classes from {class_csv_name}");
        for class_idx in 0..line_count {
            let id = class_map.get("id").unwrap().get(class_idx).unwrap().parse::<u32>().unwrap();
            let name = class_map.get("name").unwrap().get(class_idx).unwrap();
            let alignments = class_map.get("Alignments").unwrap().get(class_idx).unwrap();
            let class_skill_list = class_map.get("classSkills").unwrap().get(class_idx).unwrap();
            let description = class_map.get("description").unwrap().get(class_idx).unwrap();
            let hit_die = class_map.get("hitDie").unwrap().get(class_idx).unwrap().parse::<u32>().unwrap();
            let skills_per_level = class_map.get("skillsPerLvl").unwrap().get(class_idx).unwrap().parse::<u32>().unwrap();
            let starting_wealth_d6 = class_map.get("Starting Wealth").unwrap().get(class_idx).unwrap().parse::<u32>().unwrap();
            let caster_ability = ability_scores::string_to_ability_score(class_map.get("casterAbility").unwrap().get(class_idx).unwrap());

            let features: Vec<character_class::ClassFeature> = vec![];
            let abilities: Vec<character_class::ClassAbility> = vec![];
            let choices: Vec<character_class::ClassChoice> = vec![];
            let level_ups: [character_class::ClassLevelUpInfo; character_class::MAX_CHARACTER_LEVEL] = 
                [character_class::ClassLevelUpInfo::new_blank(); character_class::MAX_CHARACTER_LEVEL];
            let new_class = character_class::CharacterClass::new(
                id,
                name,
                description,
                hit_die,
                skills_per_level,
                alignments,
                class_skill_list,
                caster_ability,
                starting_wealth_d6,
                &features,
                &abilities,
                &choices,
                &level_ups
            );
            class_list.push(new_class);
        }

        utilities::write_classes_to_db(&db_connection, &class_list);
    }

    // Parse Class Features into db
    {
        let class_feature_csv_name = String::from(String::from(&cfg.input_path) + "\\classFeatures.tsv");
        let class_feature_map = parse_csv_file(&class_feature_csv_name, b'\t')
            .expect(&format!("Unable to parse class feature CSV {class_feature_csv_name}"));
        let line_count = class_feature_map.get("id").unwrap().len();
        let mut class_feature_list = Vec::with_capacity(line_count);


        println!("Read {line_count} class features from {class_feature_csv_name}");
        for feature_idx in 0..line_count {
            let id = class_feature_map.get("id").unwrap().get(feature_idx).unwrap().parse::<u32>().unwrap();
            let class_id = class_feature_map.get("classId").unwrap().get(feature_idx).unwrap().parse::<u32>().unwrap();
            let category_id = class_feature_map.get("categoryId").unwrap().get(feature_idx).unwrap().parse::<u32>().unwrap();
            let num_choices = class_feature_map.get("numChoices").unwrap().get(feature_idx).unwrap().parse::<u32>().unwrap();
            let level_req = class_feature_map.get("lvlReq").unwrap().get(feature_idx).unwrap().parse::<u32>().unwrap();
            let optional = class_feature_map.get("optional").unwrap().get(feature_idx).unwrap().parse::<u32>().unwrap();

            let name = class_feature_map.get("name").unwrap().get(feature_idx).unwrap();
            let desc = class_feature_map.get("description").unwrap().get(feature_idx).unwrap();
            
            let new_feature = character_class::ClassFeature::new(
                id,
                class_id,
                category_id,
                num_choices,
                level_req,
                name,
                desc,
                optional == 1,
            );

            class_feature_list.push(new_feature);
        }

        utilities::write_class_features_to_db(&db_connection, &class_feature_list);
    }

    // Parse Class Choices into db
    {
        let class_choice_csv_name = String::from(String::from(&cfg.input_path) + "\\classChoices.tsv");
        let class_choice_map = parse_csv_file(&class_choice_csv_name, b'\t')
            .expect(&format!("Unable to parse class choice CSV {class_choice_csv_name}"));
        let line_count = class_choice_map.get("id").unwrap().len();
        let mut class_choice_list = Vec::with_capacity(line_count);


        println!("Read {line_count} class choices from {class_choice_csv_name}");
        for choice_idx in 0..line_count {
            let id = class_choice_map.get("id").unwrap().get(choice_idx).unwrap().parse::<u32>().unwrap();
            let class_id = class_choice_map.get("classId").unwrap().get(choice_idx).unwrap().parse::<u32>().unwrap();
            let category_id = class_choice_map.get("categoryId").unwrap().get(choice_idx).unwrap().parse::<u32>().unwrap();
            let level_req = class_choice_map.get("lvlReq").unwrap().get(choice_idx).unwrap().parse::<u32>().unwrap();
            let name = class_choice_map.get("name").unwrap().get(choice_idx).unwrap();
            let description = class_choice_map.get("description").unwrap().get(choice_idx).unwrap();
            let max_num_selections = class_choice_map.get("maxNumSelections").unwrap().get(choice_idx).unwrap().parse::<u32>().unwrap();
            let num_subsequent_choices = class_choice_map.get("numSubsequentChoices").unwrap().get(choice_idx).unwrap().parse::<u32>().unwrap();
            let subsequent_choice_category = match class_choice_map.get("subsequentChoiceCat").unwrap().get(choice_idx).unwrap().parse::<u32>() {
                Ok(val) => Some(val),
                Err(_) => None,
            };
            let prereq_id = match class_choice_map.get("prereqId").unwrap().get(choice_idx).unwrap().parse::<u32>() {
                Ok(val) => Some(val),
                Err(_) => None,
            };
            let feat_id: Option<u32> = match class_choice_map.get("featId").unwrap().get(choice_idx).unwrap().parse::<u32>() {
                Ok(val) => Some(val),
                Err(_) => None,
            };

            let new_choice = character_class::ClassChoice::new(
                id,
                class_id,
                category_id,
                level_req,
                &name,
                &description,
                max_num_selections,
                num_subsequent_choices,
                subsequent_choice_category,
                prereq_id,
                feat_id,
                "", // extra
            );

            class_choice_list.push(new_choice);
        }

        utilities::write_class_choices_to_db(&db_connection, &class_choice_list);
    }

    // Parse Class Abilities into db
    {
        let class_ability_csv_name = String::from(String::from(&cfg.input_path) + "\\classAbilities.tsv");
        let class_ability_map = parse_csv_file(&class_ability_csv_name, b'\t')
            .expect(&format!("Unable to parse class ability CSV {class_ability_csv_name}"));
        let line_count = class_ability_map.get("id").unwrap().len();
        let mut class_ability_list = Vec::with_capacity(line_count);


        println!("Read {line_count} class features from {class_ability_csv_name}");
        for ability_idx in 0..line_count {
            let id = class_ability_map.get("id").unwrap().get(ability_idx).unwrap().parse::<u32>().unwrap();
            let class_id = class_ability_map.get("classId").unwrap().get(ability_idx).unwrap().parse::<u32>().unwrap();
            let choice_prereq_id = match class_ability_map.get("choicePrereqId").unwrap().get(ability_idx).unwrap().parse::<u32>() {
                Ok(val) => Some(val),
                Err(_) => None,
            };
            let name = class_ability_map.get("name").unwrap().get(ability_idx).unwrap();
            let description = class_ability_map.get("description").unwrap().get(ability_idx).unwrap();
            let level_req = class_ability_map.get("lvlReq").unwrap().get(ability_idx).unwrap().parse::<u32>().unwrap();
            let spell_id = match class_ability_map.get("spellId").unwrap().get(ability_idx).unwrap().parse::<u32>() {
                Ok(val) => Some(val),
                Err(_) => None,
            };
            let feat_id = match class_ability_map.get("featId").unwrap().get(ability_idx).unwrap().parse::<u32>() {
                Ok(val) => Some(val),
                Err(_) => None,
            };

            let new_ability = character_class::ClassAbility::new(
                id,
                class_id,
                choice_prereq_id,
                name,
                description,
                level_req,
                spell_id,
                feat_id,
            );

            class_ability_list.push(new_ability);
        }

        utilities::write_class_abilities_to_db(&db_connection, &class_ability_list);
    }

    // Parse Class Level Up Tables into db
    {
        let class_level_csv_name = String::from(String::from(&cfg.input_path) + "\\classLvl.tsv");
        let class_level_map = parse_csv_file(&class_level_csv_name, b'\t')
            .expect(&format!("Unable to parse spell CSV {class_level_csv_name}"));
        let line_count = class_level_map.get("id").unwrap().len();
        let mut class_level_array: [[character_class::ClassLevelUpInfo; character_class::MAX_CHARACTER_LEVEL]; character_class::NUMBER_BASE_CLASSES] =
            [[character_class::ClassLevelUpInfo::new_blank(); character_class::MAX_CHARACTER_LEVEL]; character_class::NUMBER_BASE_CLASSES];

        println!("Read {line_count} class levels from {class_level_csv_name}");
        for item_idx in 0..line_count {
            let class_idx = class_level_map.get("classId").unwrap().get(item_idx).unwrap().parse::<usize>().unwrap();
            let level = class_level_map.get("lvl").unwrap().get(item_idx).unwrap().parse::<usize>().unwrap();
            let bab = class_level_map.get("bab").unwrap().get(item_idx).unwrap().parse::<u8>().unwrap();
            let base_fort_save = class_level_map.get("fortSave").unwrap().get(item_idx).unwrap().parse::<u8>().unwrap();
            let base_reflex_save = class_level_map.get("refSave").unwrap().get(item_idx).unwrap().parse::<u8>().unwrap();
            let base_will_save = class_level_map.get("willSave").unwrap().get(item_idx).unwrap().parse::<u8>().unwrap();

            let spells_per_day_0: Option<u8> = match class_level_map.get("spellsPerDay0").unwrap().get(item_idx).unwrap().parse::<u8>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let spells_per_day_1: Option<u8> = match class_level_map.get("spellsPerDay1").unwrap().get(item_idx).unwrap().parse::<u8>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let spells_per_day_2: Option<u8> = match class_level_map.get("spellsPerDay2").unwrap().get(item_idx).unwrap().parse::<u8>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let spells_per_day_3: Option<u8> = match class_level_map.get("spellsPerDay3").unwrap().get(item_idx).unwrap().parse::<u8>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let spells_per_day_4: Option<u8> = match class_level_map.get("spellsPerDay4").unwrap().get(item_idx).unwrap().parse::<u8>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let spells_per_day_5: Option<u8> = match class_level_map.get("spellsPerDay5").unwrap().get(item_idx).unwrap().parse::<u8>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let spells_per_day_6: Option<u8> = match class_level_map.get("spellsPerDay6").unwrap().get(item_idx).unwrap().parse::<u8>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let spells_per_day_7: Option<u8> = match class_level_map.get("spellsPerDay7").unwrap().get(item_idx).unwrap().parse::<u8>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let spells_per_day_8: Option<u8> = match class_level_map.get("spellsPerDay8").unwrap().get(item_idx).unwrap().parse::<u8>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let spells_per_day_9: Option<u8> = match class_level_map.get("spellsPerDay9").unwrap().get(item_idx).unwrap().parse::<u8>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            
            let spells_known_0: Option<u8> = match class_level_map.get("spellsKnown0").unwrap().get(item_idx).unwrap().parse::<u8>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let spells_known_1: Option<u8> = match class_level_map.get("spellsKnown1").unwrap().get(item_idx).unwrap().parse::<u8>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let spells_known_2: Option<u8> = match class_level_map.get("spellsKnown2").unwrap().get(item_idx).unwrap().parse::<u8>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let spells_known_3: Option<u8> = match class_level_map.get("spellsKnown3").unwrap().get(item_idx).unwrap().parse::<u8>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let spells_known_4: Option<u8> = match class_level_map.get("spellsKnown4").unwrap().get(item_idx).unwrap().parse::<u8>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let spells_known_5: Option<u8> = match class_level_map.get("spellsKnown5").unwrap().get(item_idx).unwrap().parse::<u8>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let spells_known_6: Option<u8> = match class_level_map.get("spellsKnown6").unwrap().get(item_idx).unwrap().parse::<u8>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let spells_known_7: Option<u8> = match class_level_map.get("spellsKnown7").unwrap().get(item_idx).unwrap().parse::<u8>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let spells_known_8: Option<u8> = match class_level_map.get("spellsKnown8").unwrap().get(item_idx).unwrap().parse::<u8>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let spells_known_9: Option<u8> = match class_level_map.get("spellsKnown9").unwrap().get(item_idx).unwrap().parse::<u8>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };

            class_level_array[class_idx][level-1] = character_class::ClassLevelUpInfo::new(
                bab,
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
            );
        }

        utilities::write_class_level_info_to_db(&db_connection, &class_level_array);
    }

    // Parse Feats into db
    {
        let feat_csv_name = String::from(String::from(&cfg.input_path) + "\\feats.tsv");
        let feat_map = parse_csv_file(&feat_csv_name, b'\t')
            .expect(&format!("Unable to parse feat CSV {feat_csv_name}"));
        let line_count = feat_map.get("id").unwrap().len();
        let mut feat_list = Vec::with_capacity(line_count);


        println!("Read {line_count} feats from {feat_csv_name}");
        for feat_idx in 0..line_count {
            let id = feat_map.get("id").unwrap().get(feat_idx).unwrap().parse::<u32>().unwrap();
            let name = feat_map.get("name").unwrap().get(feat_idx).unwrap();
            let category = feat_map.get("type").unwrap().get(feat_idx).unwrap();
            let description = feat_map.get("description").unwrap().get(feat_idx).unwrap();
            let prerequisites = feat_map.get("prerequisites").unwrap().get(feat_idx).unwrap();
            let prerequisite_feats = feat_map.get("prerequisite_feats").unwrap().get(feat_idx).unwrap();
            let benefit = feat_map.get("benefit").unwrap().get(feat_idx).unwrap();
            let normal = feat_map.get("normal").unwrap().get(feat_idx).unwrap();
            let special = feat_map.get("special").unwrap().get(feat_idx).unwrap();
            let source: &String = feat_map.get("source").unwrap().get(feat_idx).unwrap();
            let teamwork = match feat_map.get("teamwork").unwrap().get(feat_idx).unwrap().parse::<u32>() {
                Ok(1) => true,
                Ok(0) => false,
                _ => panic!("Found unexpected teamwork value on line {feat_idx}"),
            };
            let critical = match feat_map.get("critical").unwrap().get(feat_idx).unwrap().parse::<u32>() {
                Ok(1) => true,
                Ok(0) => false,
                _ => panic!("Found unexpected critical value on line {feat_idx}"),
            };
            let grit = match feat_map.get("grit").unwrap().get(feat_idx).unwrap().parse::<u32>() {
                Ok(1) => true,
                Ok(0) => false,
                _ => panic!("Found unexpected grit value on line {feat_idx}"),
            };
            let style = match feat_map.get("style").unwrap().get(feat_idx).unwrap().parse::<u32>() {
                Ok(1) => true,
                Ok(0) => false,
                _ => panic!("Found unexpected style value on line {feat_idx}"),
            };
            let performance = match feat_map.get("performance").unwrap().get(feat_idx).unwrap().parse::<u32>() {
                Ok(1) => true,
                Ok(0) => false,
                _ => panic!("Found unexpected performance value on line {feat_idx}"),
            };
            let racial = match feat_map.get("racial").unwrap().get(feat_idx).unwrap().parse::<u32>() {
                Ok(1) => true,
                Ok(0) => false,
                _ => panic!("Found unexpected racial value on line {feat_idx}"),
            };
            let companion_familiar = match feat_map.get("companion_familiar").unwrap().get(feat_idx).unwrap().parse::<u32>() {
                Ok(1) => true,
                Ok(0) => false,
                _ => panic!("Found unexpected companion_familiar value on line {feat_idx}"),
            };
            let race_name: &String = feat_map.get("race_name").unwrap().get(feat_idx).unwrap();
            let note: &String = feat_map.get("note").unwrap().get(feat_idx).unwrap();
            let goal: &String = feat_map.get("goal").unwrap().get(feat_idx).unwrap();
            let completion_benefit: &String = feat_map.get("completion_benefit").unwrap().get(feat_idx).unwrap();

            let multiple: bool = match feat_map.get("multiple").unwrap().get(feat_idx).unwrap().parse::<u32>() {
                Ok(1) => true,
                Ok(0) => false,
                _ => panic!("Found unexpected multiple value on line {feat_idx}"),
            };
            let suggested_traits: &String = feat_map.get("suggested_traits").unwrap().get(feat_idx).unwrap();
            
            let choice:Option<feat::FeatChoiceType>  = match feat_map.get("choice").unwrap().get(feat_idx).unwrap().parse::<u32>() {
                Ok(val) => feat::index_to_feat_choice_type(val),
                Err(_) => None,
            };

            let new_feat = feat::Feat::new(
                id,
                &name,
                &category,
                &description,
                &prerequisites,
                &prerequisite_feats,
                &benefit,
                &normal,
                &special,
                &source,
                teamwork,
                critical,
                grit,
                style,
                performance,
                racial,
                companion_familiar,
                &race_name,
                &note,
                &goal,
                &completion_benefit,
                multiple,
                &suggested_traits,
                choice
            );

            feat_list.push(new_feat);
        }

        utilities::write_feats_to_db(&db_connection, &feat_list);
    }

    // Parse General Equipment into db
    {
        let equipment_csv_name = String::from(String::from(&cfg.input_path) + "\\general_equipment.tsv");
        let equipment_map = parse_csv_file(&equipment_csv_name, b'\t')
            .expect(&format!("Unable to parse equipment CSV {equipment_csv_name}"));
        let line_count = equipment_map.get("id").unwrap().len();
        let mut equipment_list = Vec::with_capacity(line_count);


        println!("Read {line_count} equipments from {equipment_csv_name}");
        for equip_idx in 0..line_count {
            let id = equipment_map.get("id").unwrap().get(equip_idx).unwrap().parse::<u32>().unwrap();
            let name: &String = equipment_map.get("name").unwrap().get(equip_idx).unwrap();
            let description: &String = equipment_map.get("description").unwrap().get(equip_idx).unwrap();
            let item_type = equipment::index_to_eq_category_id(equipment_map.get("category").unwrap().get(equip_idx).unwrap().parse::<u32>().unwrap());
            let price = equipment::string_to_currency(equipment_map.get("price").unwrap().get(equip_idx).unwrap());
            let weight = equipment_map.get("category").unwrap().get(equip_idx).unwrap().parse::<f64>().unwrap();
            let size_mod = match equipment_map.get("size_mod").unwrap().get(equip_idx).unwrap().parse::<u32>().unwrap() {
                0 => false,
                1 => true,
                _ => unreachable!("Unexpected size_mod string for line {equip_idx} of general equipment"),
            };
            let new_equipment = equipment::GeneralItem::new(
                id,
                name,
                description,
                item_type,
                price,
                weight,
                size_mod,
            );

            equipment_list.push(new_equipment);
        }

        utilities::write_general_equipment_to_db(&db_connection, &equipment_list);
    }

    // Parse Weapons into db
    {
        let weapon_csv_name = String::from(String::from(&cfg.input_path) + "\\weapons.tsv");
        let weapon_map = parse_csv_file(&weapon_csv_name, b'\t')
            .expect(&format!("Unable to parse weapon CSV {weapon_csv_name}"));
        let line_count = weapon_map.get("id").unwrap().len();
        let mut weapon_list = Vec::with_capacity(line_count);


        println!("Read {line_count} weapons from {weapon_csv_name}");
        for weapon_idx in 0..line_count {
            let id = weapon_map.get("id").unwrap().get(weapon_idx).unwrap().parse::<u32>().unwrap();
            let name = weapon_map.get("name").unwrap().get(weapon_idx).unwrap();
            let description = weapon_map.get("description").unwrap().get(weapon_idx).unwrap();
            let price = equipment::string_to_currency(weapon_map.get("price").unwrap().get(weapon_idx).unwrap());
            let weight = weapon_map.get("weight").unwrap().get(weapon_idx).unwrap().parse::<f64>().unwrap();
            let damage_type = weapon_map.get("type").unwrap().get(weapon_idx).unwrap();
            let weapon_type = match weapon_map.get("category").unwrap().get(weapon_idx).unwrap().parse::<u32>().unwrap() {
                0 => equipment::WeaponCategoryMarker::LightMeleeWeapon,
                1 => equipment::WeaponCategoryMarker::OneHandedMelee,
                2 => equipment::WeaponCategoryMarker::TwoHandedMelee,
                3 => equipment::WeaponCategoryMarker::Ranged,
                4 => equipment::WeaponCategoryMarker::Ammunition,
                _ => unreachable!("Unexpected weapon category value at row {weapon_idx}"),
            };
            let proficiency = match weapon_map.get("weapon_proficiency").unwrap().get(weapon_idx).unwrap().parse::<u32>().unwrap() {
                0 => equipment::WeaponProficiencyMarker::MartialWeapon,
                1 => equipment::WeaponProficiencyMarker::SimpleWeapon,
                2 => equipment::WeaponProficiencyMarker::ExoticWeapon,
                _ => unreachable!("Unexpected weapon proficiency value at row {weapon_idx}"),
            };
            let dmg_s = weapon_map.get("dmg_s").unwrap().get(weapon_idx).unwrap();
            let dmg_m = weapon_map.get("dmg_m").unwrap().get(weapon_idx).unwrap();
            let crit = weapon_map.get("crit").unwrap().get(weapon_idx).unwrap();
            let range = weapon_map.get("range").unwrap().get(weapon_idx).unwrap();
            let special = weapon_map.get("special").unwrap().get(weapon_idx).unwrap();
            let ammo_count = weapon_map.get("ammo_count").unwrap().get(weapon_idx).unwrap().parse::<u32>().unwrap();

            let new_weapon = equipment::Weapon::new(
                id,
                name,
                description,
                price,
                weight,
                true,
                false,
                damage_type,
                weapon_type,
                proficiency,
                None,
                dmg_s,
                dmg_m,
                crit,
                range,
                special,
                ammo_count
            );

            weapon_list.push(new_weapon);
        }

        utilities::write_weapons_to_db(&db_connection, &weapon_list);
    }

    // Parse Armor into db
    {
        let armor_csv_name = String::from(String::from(&cfg.input_path) + "\\armor.tsv");
        let armor_map = parse_csv_file(&armor_csv_name, b'\t')
            .expect(&format!("Unable to parse armor CSV {armor_csv_name}"));
        let line_count = armor_map.get("id").unwrap().len();
        let mut armor_list = Vec::with_capacity(line_count);


        println!("Read {line_count} armor from {armor_csv_name}");
        for armor_idx in 0..line_count {
            let id = armor_map.get("id").unwrap().get(armor_idx).unwrap().parse::<u32>().unwrap();
            let name = armor_map.get("name").unwrap().get(armor_idx).unwrap();
            let description = armor_map.get("description").unwrap().get(armor_idx).unwrap();
            let price = equipment::string_to_currency(armor_map.get("price").unwrap().get(armor_idx).unwrap());
            let weight = armor_map.get("weight").unwrap().get(armor_idx).unwrap().parse::<f64>().unwrap();
            let armor_type = match armor_map.get("category").unwrap().get(armor_idx).unwrap().parse::<u32>().unwrap() {
                0 => equipment::ArmorCategoryMarker::LightArmor,
                1 => equipment::ArmorCategoryMarker::MediumArmor,
                2 => equipment::ArmorCategoryMarker::HeavyArmor,
                3 => equipment::ArmorCategoryMarker::Shield,
                4 => equipment::ArmorCategoryMarker::Extra,
                _ => unreachable!("Unexpected armor category value at row {armor_idx}"),
            };
            let armor_bonus = armor_map.get("armor_bonus").unwrap().get(armor_idx).unwrap().parse::<u32>().unwrap();
            let armor_check_penalty = armor_map.get("armor_bonus").unwrap().get(armor_idx).unwrap().parse::<i32>().unwrap();
            let spell_fail_chance = armor_map.get("spell_fail_chance").unwrap().get(armor_idx).unwrap().parse::<u32>().unwrap();
            let max_dex_bonus = match armor_map.get("max_dex_bonus").unwrap().get(armor_idx).unwrap().parse::<u32>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let speed_30 = match armor_map.get("speed_30ft").unwrap().get(armor_idx).unwrap().parse::<u32>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let speed_20 = match armor_map.get("speed_20ft").unwrap().get(armor_idx).unwrap().parse::<u32>() {
                Ok(x) => Some(x),
                Err(_) => None,
            };

            let new_armor = equipment::Armor::new(
                id,
                name,
                description,
                price,
                weight,
                true,
                false,
                armor_type,
                None,
                armor_bonus,
                max_dex_bonus,
                armor_check_penalty,
                spell_fail_chance,
                speed_30,
                speed_20,
            );

            armor_list.push(new_armor);
        }

        utilities::write_armor_to_db(&db_connection, &armor_list);
    }

}

pub struct CsvToSqlConfig {
    pub input_path: String,
    pub output_file: String,
}

impl CsvToSqlConfig {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<CsvToSqlConfig, &'static str> {
        args.next(); //Skip the exe name

        let input_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an input csv path"),
        };

        let output_file = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an output file path"),
        };

        Ok(CsvToSqlConfig { input_path, output_file })
    }
}

fn parse_csv_file(filename: &str, delimiter: u8) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    //Open file
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(true)
        .from_path(filename)?;

    // Initialize columns map
    let headers = reader
        .headers()
        .expect(&format!("File {filename} did not have a header line!"))
        .clone();
    let mut columns: HashMap<String, Vec<String>> = headers
        .iter()
        .map(|header| (header.to_string(), Vec::new()))
        .collect();

    // Read records and populate columns
    for record in reader.records() {
        let record = record?;
        for (header, value) in headers.iter().zip(record.iter()) {
            columns
                .get_mut(header)
                .expect(&format!("column for {header} should already exist"))
                .push(value.to_string());
        }
    }

    Ok(columns)
}