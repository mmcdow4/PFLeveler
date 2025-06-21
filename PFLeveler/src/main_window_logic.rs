use crate::ui::{self, MainWindow};
use slint::ComponentHandle;
use PathFinder::pf_character;
use std::{cell::RefCell, rc::Rc};
use crate::{summary_page_logic, race_page_logic, skill_page_logic, ability_score_page_logic};

pub fn setup_callbacks(main_window: &MainWindow, current_character: &Rc<RefCell<Option<pf_character::PFCharacter>>>) {
    

    let weak_window = main_window.as_weak();
    let char_clone = Rc::clone(&current_character);
    main_window.on_handle_file_new_character(move || {
        if let Some(main_window) = weak_window.upgrade() {
            let new_character = pf_character::PFCharacter::new();
            summary_page_logic::reset_summary_page(
                &new_character,
                &main_window
            );
            race_page_logic::reset_race_page(
                &new_character,
                &main_window
            );
            skill_page_logic::reset_skill_page(
                &new_character,
                &main_window
            );
            char_clone.replace(Some(new_character));
        }
    });

    main_window.on_handle_file_exit(move || {
        slint::quit_event_loop().expect("Failed to exit gracefully!");
    });

    let weak_window = main_window.as_weak();
    let char_clone = current_character.clone();
    main_window.on_summary_lock_button_clicked(move || {
        if let Some(main_window) = weak_window.upgrade() {
            let _ = summary_page_logic::handle_summary_lock_button(
                char_clone.borrow_mut(),
                &main_window
            );
        }
    });

    let weak_window = main_window.as_weak();
    main_window.on_race_selected(move |race_name| {
        if let Some(main_window) = weak_window.upgrade() {
            race_page_logic::handle_race_selected(
                &race_name.to_string(),
                &main_window
            );
        }
    });

    let weak_window = main_window.as_weak();
    let char_clone = current_character.clone();
    main_window.on_race_lock_button_clicked(move || {
        if let Some(main_window) = weak_window.upgrade() {
            let _ = race_page_logic::handle_race_lock_button(
                char_clone.borrow_mut(),
                &main_window
            );
            // match char_clone.borrow() {
            //     Some(curr_char) => {
            //         skill_page_logic::update_skill_page(
            //             &curr_char,
            //             &main_window
            //         );
            //     },
            //     None => {
            //         ui::launch_error_dialog(&String::from("Clicked race lock without a character?!"));
            //     }
            // }
        }
    });

    let weak_window = main_window.as_weak();
    let char_clone = current_character.clone();
    main_window.on_skill_lock_button_clicked(move || {
        if let Some(main_window) = weak_window.upgrade() {
            let _ = skill_page_logic::handle_skill_lock_button(
                char_clone.borrow_mut(),
                &main_window
            );
        }
    });

    let weak_window = main_window.as_weak();
    main_window.on_ability_score_select_clicked(move |index| {
        if let Some(main_window) = weak_window.upgrade() {
            ability_score_page_logic::handle_ability_score_select_button(
                &main_window,
                index
            );
        }
    });

    let weak_window = main_window.as_weak();
    main_window.on_ability_score_dropdown_selected(move |ability_score_index, index| {
        if let Some(main_window) = weak_window.upgrade() {
            ability_score_page_logic::handle_ability_score_select_dropdown(
                &main_window,
                ability_score_index,
                index
            );
        }
    });

    let weak_window = main_window.as_weak();
    main_window.on_ability_score_roll_value(move |index| {
        if let Some(main_window) = weak_window.upgrade() {
            println!("TESTPOINT1: index {index}");
            ability_score_page_logic::handle_ability_score_roll_value(
                &main_window,
                index);
        } else {
            println!("TESTPOINT2: index {index}");
        }
    });

    let weak_window = main_window.as_weak();
    let char_clone = current_character.clone();
    main_window.on_ability_score_lock_button_clicked(move || {
        if let Some(main_window) = weak_window.upgrade() {
            match ability_score_page_logic::verify_lockable(&main_window) {
                Ok(Some(s)) => {
                    if s.is_empty() {
                        let _ = ability_score_page_logic::lock_ability_scores(char_clone.borrow_mut(), &main_window).expect("Failed to lock ability scores");
                    } else {
                        let temp_clone = char_clone.clone();
                        ui::launch_warning_dialog(&s, move |input_value| {
                        match input_value {
                            ui::ButtonType::CANCEL => {},
                            ui::ButtonType::OK => {
                                let _ = ability_score_page_logic::lock_ability_scores(temp_clone.borrow_mut(), &main_window).expect("Failed to lock ability scores");
                            }
                            _ => unreachable!("Unexpected button type"),
                         }}).expect("Failed to launch warning window");
                    }
                },
                Err(err) => panic!("{}", err),
                Ok(None) => {},
            }
            // let _ = ability_score_page_logic::handle_ability_score_lock_clicked(
            //     char_clone.borrow_mut(),
            //     &main_window
            // );
        }
    });
}