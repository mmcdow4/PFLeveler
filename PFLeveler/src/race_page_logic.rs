use crate::ui::{
    self,
    MainWindow,
    AbilityScoreInfo,
    AbilityScoreMode
};
use slint::{
    Model,
    ModelRc,
    VecModel,
    SharedString,
    StandardListViewItem
};
use std::cell::RefMut;
use PathFinder::{
    pf_table,
    pf_character::PFCharacter,
    race,
    ability_scores
};

pub fn reset_race_page(
    current_character: &PFCharacter,
    main_window: &MainWindow)
{
    let mut race_names: Vec<SharedString> = pf_table::get_pf_table()
        .get_races()
        .values()
        .map(|r| SharedString::from(r.name.clone()))
        .collect::<Vec<_>>();

    race_names.sort();

    /* Set default values */
    main_window.set_race__race_names(ModelRc::new(VecModel::from(race_names)));
    main_window.set_race__character_race("".into());
    main_window.set_race__size("".into());
    main_window.set_race__speed("".into());
    main_window.set_race__ability_score_offsets("".into());
    main_window.set_race__languages_known("".into());
    main_window.set_race__languages_available("".into());
    main_window.set_race__racial_ability_names(ModelRc::new(VecModel::from(vec![])));
    main_window.set_race__racial_ability_descriptions(ModelRc::new(VecModel::from(vec![])));
    main_window.set_race__selected_racial_description("".into());
    
    /* Handle ability score section as well */
    main_window.set_race__as__ability_score_locked(false);
    main_window.set_race__as__mode(AbilityScoreMode::None);
    main_window.set_race__as__flex_racial_bonus(false);

    let mut ability_score_info: Vec<AbilityScoreInfo> = Vec::new();
    for as_index in 0..ability_scores::NUMBER_ABILITY_SCORES {
        let ability_score = ability_scores::index_to_ability_score(Some(as_index)).unwrap();
        ability_score_info.push( AbilityScoreInfo {
            name: SharedString::from(ability_scores::ability_score_to_string(Some(ability_score))),
            base_value: current_character.ability_scores[&ability_score],
            racial_bonus: 0,
            modifier: current_character.get_ability_mod(&ability_score),
        });
    }
    main_window.set_race__as__ability_score_data(ModelRc::new(VecModel::from(ability_score_info)));

    main_window.set_race__as__armor_val(0);
    main_window.set_race__as__shield_val(0);
    main_window.set_race__as__size_combat_mod(0);
    main_window.set_race__as__deflection_mod(0);
    main_window.set_race__as__natural_armor(0);
    main_window.set_race__as__misc_ac_mod(0);
    main_window.set_race__as__misc_initiative_mod(0);
    
    main_window.set_race__as__saving_throw_bases(ModelRc::new(VecModel::from(vec![
        current_character.get_base_fort_save() as i32,
        current_character.get_base_reflex_save() as i32,
        current_character.get_base_will_save() as i32,
    ])));

    main_window.set_race__as__saving_throw_magic_mods(ModelRc::new(VecModel::from(vec![0, 0, 0])));
    main_window.set_race__as__saving_throw_misc_mods(ModelRc::new(VecModel::from(vec![0, 0, 0])));
    main_window.set_race__as__saving_throw_temp_mods(ModelRc::new(VecModel::from(vec![0, 0, 0])));

    main_window.set_race__as__base_attack_bonus(current_character.get_base_attack_bonus() as i32);
    if let Some(character_race) = &current_character.race {
        main_window.set_race__locked(true);
        main_window.set_race__as__ability_score_locked(true);
        main_window.set_race__character_race(character_race.name.clone().into());
        update_race_page(character_race, main_window);
    } else {
        main_window.set_race__locked(false);
    }

}

pub fn handle_race_selected(race_name: &String, main_window: &MainWindow) {
    for race in pf_table::get_pf_table().get_races().values() {
        if race.name == *race_name {
            update_race_page(race, main_window);
            main_window.set_race__selected_racial_index(-1);
            break;
        }
    }
}

pub fn handle_race_lock_button(
    mut current_character: RefMut<'_, Option<PFCharacter>>,
    main_window: &MainWindow,
) -> Result<(), slint::PlatformError>
{

    let mut err_str = String::new();
    match &mut *current_character {
        Some(curr_char) => {
            if curr_char.race.is_none() {
                let race_name = main_window.get_race__selected_race().to_string();
                for race in pf_table::get_pf_table().get_races().values() {
                    if race.name == race_name {
                        curr_char.race = Some(race.clone());
                        curr_char.languages_known = race.languages.clone();
                        curr_char.languages_available = race.languages_available.clone();
                        main_window.set_race__character_race(race_name.clone().into());
                        main_window.set_race__locked(true);
                        main_window.set_summary__race(race_name.clone().into());
                        main_window.set_summary__size(race.size.text.clone().into());
                        main_window.set_summary__languages(curr_char.get_languages_known().into());
                        main_window.set_summary__speed(race.speed.to_string().into());
                    }
                }
                if curr_char.race.is_none() {
                    err_str = String::from("You must select a race first!");
                }
            } else {
                err_str = String::from("Hit lock race when character already has a race!");
            }
        },
        None =>  { err_str = String::from("Hit lock race without an existing character!"); }
    }

    if !err_str.is_empty() {
        // ui::launch_error_dialog(&err_str)
        ui::launch_error_dialog(&err_str)
    } else {
        Ok(())
    }
}

fn update_race_page(character_race: &race::Race, main_window: &MainWindow) {
    
    let race_size = character_race.size.text.clone();
    let race_speed = character_race.speed.to_string();

    let mut languages_known = String::new();
    for lang in &character_race.languages {
        if !languages_known.is_empty() {
            languages_known += ", ";
        }
        languages_known += &lang.clone();
    }

    let mut languages_available = String::new();
    for lang in &character_race.languages_available {
        if !languages_available.is_empty() {
            languages_available += ", ";
        }
        languages_available += &lang.clone();
    }

    let mut offsets_str = String::new();
    for ability_idx in 0..=ability_scores::NUMBER_ABILITY_SCORES {
        if character_race.ability_score_offsets[ability_idx] != 0 {
            let mut ability_score_str =
                ability_scores::index_to_ability_score_string(Some(ability_idx));
            if ability_score_str.is_empty() {
                ability_score_str = String::from("Any One Ability Score");
            }
            offsets_str += &format!("\n    {:+} to {}",
            character_race.ability_score_offsets[ability_idx],
                ability_score_str
            );
        }
    }

    let racial_names: Vec<StandardListViewItem> = character_race.
        racials.
        iter()
        .map(|x| StandardListViewItem::from(SharedString::from(x.name.clone())))
        .collect();
    let racial_descriptions: Vec<SharedString> = character_race.
        racials
        .iter()
        .map(|x| SharedString::from(x.description.clone()))
        .collect();

    let ability_score_data = main_window.get_race__as__ability_score_data();
    for as_index in 0..ability_scores::NUMBER_ABILITY_SCORES {
        let mut data = ability_score_data.row_data(as_index).unwrap();
        data.racial_bonus = character_race.ability_score_offsets[as_index] as i32;
        data.modifier = ability_scores::value_to_modifier(data.base_value + data.racial_bonus);
        ability_score_data.set_row_data(as_index, data);
    }

    main_window.set_race__as__ability_score_data(ability_score_data);
    main_window.set_race__size(race_size.into());
    main_window.set_race__speed(race_speed.into());
    main_window.set_race__ability_score_offsets(offsets_str.into());
    main_window.set_race__languages_known(languages_known.into());
    main_window.set_race__languages_available(languages_available.into());
    main_window.set_race__racial_ability_names(ModelRc::new(VecModel::from(racial_names)));
    main_window.set_race__racial_ability_descriptions(ModelRc::new(VecModel::from(racial_descriptions)));
    main_window.set_race__selected_racial_index(-1);
    main_window.set_race__selected_racial_description("".into());

    main_window.set_race__as__size_combat_mod(character_race.size.combat_mod as i32);
}