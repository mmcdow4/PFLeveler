mod ui;
mod main_window_logic;
mod summary_page_logic;
mod race_page_logic;
mod skill_page_logic;
mod ability_score_page_logic;

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

    let main_window = MainWindow::new()?;

    main_window_logic::setup_callbacks(&main_window, &current_character);

    main_window.run()
}

