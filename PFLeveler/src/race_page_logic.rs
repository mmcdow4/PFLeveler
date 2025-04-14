use crate::ui::{self, MainWindow};
use slint::{ModelRc, VecModel, SharedString, StandardListViewItem};
use std::cell::RefMut;
use PathFinder::{pf_table, pf_character::PFCharacter, race, ability_scores};

pub fn reset_race_page(current_character: &PFCharacter, ui: &MainWindow) {
    let race_names: Vec<SharedString> = pf_table::get_pf_table()
        .get_races()
        .values()
        .map(|r| SharedString::from(r.name.clone()))
        .collect();

    /* Set default values */
    ui.set_race__race_names(ModelRc::new(VecModel::from(race_names)));
    ui.set_race__character_race("".into());
    ui.set_race__size("".into());
    ui.set_race__speed("".into());
    ui.set_race__ability_score_offsets("".into());
    ui.set_race__languages_known("".into());
    ui.set_race__languages_available("".into());
    ui.set_race__racial_ability_names(ModelRc::new(VecModel::from(vec![])));
    ui.set_race__racial_ability_descriptions(ModelRc::new(VecModel::from(vec![])));
    ui.set_race__selected_racial_description("".into());
    
    if let Some(character_race) = &current_character.race {
        ui.set_race__locked(true);
        ui.set_race__character_race(character_race.name.clone().into());
        update_race_page(character_race, ui);
    } else {
        ui.set_race__locked(false);
    }
}

pub fn handle_race_selected(race_name: &String, ui: &MainWindow) {
    for race in pf_table::get_pf_table().get_races().values() {
        if race.name == *race_name {
            update_race_page(race, ui);
            break;
        }
    }
}

pub fn handle_race_lock_button(
    mut current_character: RefMut<'_, Option<PFCharacter>>,
    ui: &MainWindow) -> Result<(), slint::PlatformError> {

    let mut err_str = String::new();
    match &mut *current_character {
        Some(curr_char) => {
            if curr_char.race.is_none() {
                let race_name = ui.get_race__selected_race().to_string();
                for race in pf_table::get_pf_table().get_races().values() {
                    if race.name == race_name {
                        curr_char.race = Some(race.clone());
                        curr_char.languages_known = race.languages.clone();
                        curr_char.languages_available = race.languages_available.clone();
                        ui.set_race__character_race(race_name.clone().into());
                        ui.set_race__locked(true);
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
        ui::launch_error_dialog(&err_str)
    } else {
        Ok(())
    }
}

fn update_race_page(character_race: &race::Race, ui: &MainWindow) {
    
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

    ui.set_race__size(race_size.into());
    ui.set_race__speed(race_speed.into());
    ui.set_race__ability_score_offsets(offsets_str.into());
    ui.set_race__languages_known(languages_known.into());
    ui.set_race__languages_available(languages_available.into());
    ui.set_race__racial_ability_names(ModelRc::new(VecModel::from(racial_names)));
    ui.set_race__racial_ability_descriptions(ModelRc::new(VecModel::from(racial_descriptions)));
    ui.set_race__selected_racial_index(-1);
    ui.set_race__selected_racial_description("".into());
}