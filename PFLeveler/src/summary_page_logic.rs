use crate::ui::{self, InputField, MainWindow};
use slint::{
    ComponentHandle,
    Model,
    ModelRc,
    VecModel,
    SharedString,
    StandardListViewItem
};
use std::cell::RefMut;
use PathFinder::{
    error,
    pf_table,
    ability_scores,
    pf_character::PFCharacter
};

pub fn reset_summary_page(
    current_character: &PFCharacter,
    main_window: &MainWindow)
{
    let weak_window = main_window.as_weak();
    if let Some(main_window) = weak_window.upgrade() {
        main_window.set_summary__character_alignment_text(current_character.alignment.clone().into());
        main_window.set_summary__alignment_text(current_character.alignment.clone().into());

        // Populate race data
        if current_character.race.is_none() {
            main_window.set_summary__race("".into());
            main_window.set_summary__size("".into());
            main_window.set_summary__speed("".into());
        } else {
            let char_race = current_character.race.clone().unwrap();
            main_window.set_summary__race(char_race.name.clone().into());
            main_window.set_summary__size(char_race.size.text.clone().into());
            main_window.set_summary__speed(char_race.speed.to_string().into());
        }

        // Populate language string
        let language_string = current_character.get_languages_known();
        main_window.set_summary__languages(language_string.into());
        
        // Fill in class level table
        let mut class_level_text: Vec<StandardListViewItem> = Vec::new();
        for (id, level) in current_character.class_levels.iter() {
            class_level_text.push(
                StandardListViewItem::from(
                    SharedString::from(
                        pf_table::get_pf_table().get_class(*id).expect(&format!("No class with ID {id}")).name.clone() + 
                        " : level " +
                        &level.to_string()
                ))
            );
        }
        main_window.set_summary__class_levels_text(ModelRc::new(VecModel::from(class_level_text)));

        // Fill in ability score table
        let mut ability_score_text: Vec<StandardListViewItem> = Vec::new();
        for (id, value) in current_character.ability_scores.iter() {
            ability_score_text.push(
                StandardListViewItem::from(
                    SharedString::from(
                        ability_scores::ability_score_to_string(Some(*id)) +
                        " = " +
                        &value.to_string()
                ))
            );
        }
        main_window.set_summary__ability_score_text(ModelRc::new(VecModel::from(ability_score_text)));

        // Fill in skills table
        let mut skills_text: Vec<StandardListViewItem> = Vec::new();
        for index in 0..current_character.skills.len() {
            let value = match current_character.get_effective_skill_rank(index) {
                Err(error::PathFinderError::InvalidArgument(err_str)) => {
                    ui::launch_error_dialog(&err_str).expect("Failed to launch error window");
                    0
                },
                Ok(val) => val,
            };
            skills_text.push(
                StandardListViewItem::from(
                    SharedString::from(
                        current_character.skills[index].name.clone() +
                        " : " +
                        &value.to_string()
                ))
            );
        }
        main_window.set_summary__skills_text(ModelRc::new(VecModel::from(skills_text)));

        // Fill in the feats table
        let mut feats_text: Vec<StandardListViewItem> = Vec::new();
        for value in &current_character.feats {
            feats_text.push(
                StandardListViewItem::from(SharedString::from(value.full_name()))
            );
        }
        main_window.set_summary__feats_text(ModelRc::new(VecModel::from(feats_text)));

        // Fill in the abilities table
        let /* mut */ abilities_text: Vec<StandardListViewItem> = Vec::new();
        main_window.set_summary__abilities_text(ModelRc::new(VecModel::from(abilities_text)));

        // reset spell class choice and clear spell slots and spells tables 
        main_window.set_summary__spells_class_idx(-1);
        main_window.set_summary__spell_slots_text(ModelRc::new(VecModel::from(VecModel::from(vec![]))));
        main_window.set_summary__spells_text(ModelRc::new(VecModel::from(vec![])));

        // Set biographical information
        let fields = vec![
            InputField { 
                label: SharedString::from("Character Name"),
                value: SharedString::from(current_character.name.clone()),
                input: SharedString::from("") },
            InputField {
                label: SharedString::from("Player Name"),
                value: SharedString::from(current_character.player_name.clone()),
                input: SharedString::from("") },
            InputField {
                label: SharedString::from("Height"),
                value: SharedString::from(current_character.height.clone()),
                input: SharedString::from("") },
            InputField {
                label: SharedString::from("Weight"),
                value: SharedString::from(current_character.weight.clone()),
                input: SharedString::from("") },
            InputField {
                label: SharedString::from("Hair"),
                value: SharedString::from(current_character.hair.clone()),
                input: SharedString::from("") },
            InputField {
                label: SharedString::from("Eyes"),
                value: SharedString::from(current_character.eyes.clone()),
                input: SharedString::from("") },
            InputField {
                label: SharedString::from("Deity"),
                value: SharedString::from(current_character.deity.clone()),
                input: SharedString::from("") },
            InputField {
                label: SharedString::from("Homeland"),
                value: SharedString::from(current_character.homeland.clone()),
                input: SharedString::from("") },
            InputField {
                label: SharedString::from("Gender"),
                value: SharedString::from(current_character.gender.clone()),
                input: SharedString::from("") },
            InputField {
                label: SharedString::from("Age"),
                value: SharedString::from(current_character.age.clone()),
                input: SharedString::from("") }
        ];
        main_window.set_summary__input_fields(ModelRc::new(VecModel::from(fields)));

        main_window.set_summary__locked(!current_character.name.is_empty());
    }
}

pub fn handle_summary_lock_button(
    mut current_character: RefMut<'_, Option<PFCharacter>>,
    main_window: &MainWindow
) -> Result<(), slint::PlatformError> {

    let mut fields: Vec<InputField> = main_window.get_summary__input_fields().iter().collect();
    let alignment_string = main_window.get_summary__alignment_text();

    let mut err_str = String::new();
    match &mut *current_character {
        Some(curr_char) => {
            for item in &mut fields {
                // println!("Inspecting '{}', value '{}', input '{}'", item.label, item.value, item.input);
                if item.input.is_empty() {
                    err_str = format!("You must input a value for {}", item.label);
                    break;
                }
                item.value = item.input.clone();
                item.input = SharedString::from("");
            }

            if alignment_string.is_empty() {
                err_str = String::from("You must select an alignment");
            }

            if err_str.is_empty() {
                /* All fields populated - update the struct values */
                curr_char.name = fields[0].value.to_string();
                curr_char.player_name = fields[1].value.to_string();
                curr_char.height = fields[2].value.to_string();
                curr_char.weight = fields[3].value.to_string();
                curr_char.hair = fields[4].value.to_string();
                curr_char.eyes = fields[5].value.to_string();
                curr_char.deity = fields[6].value.to_string();
                curr_char.homeland = fields[7].value.to_string();
                curr_char.gender = fields[8].value.to_string();
                curr_char.age = fields[9].value.to_string();
                curr_char.alignment = alignment_string.clone().to_string();
                /* Update the UI as well = */
                main_window.set_summary__input_fields(ModelRc::new(VecModel::from(fields)));
                main_window.set_summary__character_alignment_text(alignment_string);
                main_window.set_summary__locked(true);
            }
        },
        None => { err_str = String::from("You have not created a character yet"); },
    }

    if !err_str.is_empty() {
        // ui::launch_error_dialog(&err_str)?;
        ui::launch_error_dialog(&err_str)?;
    }

    Ok(())

}