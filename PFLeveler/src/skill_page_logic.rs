use crate::ui::{self, MainWindow, SkillInfo};
use slint::{Model, ModelRc, VecModel};
use std::cell::RefMut;
use PathFinder::{pf_character::PFCharacter, ability_scores};

pub fn reset_skill_page(current_character: &PFCharacter, main_window: &MainWindow) {
    update_skill_page(current_character, main_window);
    main_window.set_skill__num_points_remaining(0);
    main_window.set_skill__locked(true);
}

pub fn update_skill_page(current_character: &PFCharacter, main_window: &MainWindow) {
    let skills_vec: Vec<SkillInfo> = current_character
        .skills
        .iter()
        .map(|s| SkillInfo {
            is_class_skill: s.is_favored,
            name: s.name.clone().into(),
            ability: ability_scores::ability_score_to_abbrev(Some(s.base_ability)).into(),
            ability_mod: current_character.get_ability_mod(&s.base_ability),
            ranks: s.rank,
            temp_ranks: 0,
            misc: match s.rank > 0 && s.is_favored {
                true => 3,
                false => 0,
            },
        })
        .collect();

    main_window.set_skill__skill_data(ModelRc::new(VecModel::from(skills_vec)));
}

pub fn handle_skill_lock_button(
    mut current_character: RefMut<'_, Option<PFCharacter>>,
    main_window: &MainWindow,
) -> Result<(), slint::PlatformError>
{

    let skills_vec = main_window.get_skill__skill_data();
    let num_skills = skills_vec.row_count();

    match &mut *current_character {
        Some(curr_char) => {
            if curr_char.skills.len() != num_skills {
                return ui::launch_error_dialog(
                    &String::from(
                        "Skills in character do not match skills in the GUI"
                ));
            }
            for skill_index in 0..num_skills {
                curr_char.skills[skill_index].rank += skills_vec.row_data(skill_index).unwrap().temp_ranks;
                skills_vec.row_data(skill_index).unwrap().temp_ranks = 0;
            }
        },
        None =>  {
            return ui::launch_error_dialog(
                &String::from("Hit lock skills without an existing character!"
            ));
        },
    }

    main_window.set_skill__skill_data(skills_vec);
    Ok(())
}