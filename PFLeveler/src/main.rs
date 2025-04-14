mod ui;
mod summary_page_logic;
mod race_page_logic;
mod skill_page_logic;

use ui::MainWindow;
use slint::ComponentHandle;
use PathFinder::{pf_table, pf_character};
use std::rc::Rc;
use std::cell::RefCell;

fn main() -> Result<(), slint::PlatformError> {
    let filename = String::from("E:\\dev\\PFLeveler\\cfg\\PathFinder.db");
    let _table = pf_table::init_pf_table(&filename);
    let current_character: Rc<RefCell<Option<pf_character::PFCharacter>>> =
        Rc::new(RefCell::new(None));
    // Now you can access the table anywhere with:
    // pf_table::PF_TABLE.get().unwrap()

    let main_window = MainWindow::new()?;
    let weak_window = main_window.as_weak();
    let char_clone = Rc::clone(&current_character);
    main_window.on_handle_file_new_character(move || {
        if let Some(ui) = weak_window.upgrade() {
            let new_character = pf_character::PFCharacter::new();
            summary_page_logic::reset_summary_page(&new_character, &ui);
            race_page_logic::reset_race_page(&new_character, &ui);
            skill_page_logic::reset_skill_page(&new_character, &ui);
            char_clone.replace(Some(new_character));
        }
    });

    main_window.on_handle_file_exit(move || {
        slint::quit_event_loop().expect("Failed to exit gracefully!");
    });

    let weak_window = main_window.as_weak();
    let char_clone = current_character.clone();
    main_window.on_summary_lock_button_clicked(move || {
        if let Some(ui) = weak_window.upgrade() {
            let _ = summary_page_logic::handle_summary_lock_button(char_clone.borrow_mut(), &ui);
        }
    });

    let weak_window = main_window.as_weak();
    main_window.on_race_selected(move |race_name| {
        if let Some(ui) = weak_window.upgrade() {
            race_page_logic::handle_race_selected(&race_name.to_string(), &ui);
        }
    });

    let weak_window = main_window.as_weak();
    let char_clone = current_character.clone();
    main_window.on_race_lock_button_clicked(move || {
        if let Some(ui) = weak_window.upgrade() {
            let _ = race_page_logic::handle_race_lock_button(char_clone.borrow_mut(), &ui);
        }
    });

    let weak_window = main_window.as_weak();
    let char_clone = current_character.clone();
    main_window.on_skill_lock_button_clicked(move || {
        if let Some(ui) = weak_window.upgrade() {
            let _ = skill_page_logic::handle_skill_lock_button(char_clone.borrow_mut(), &ui);
        }
    });

    main_window.run()
}

