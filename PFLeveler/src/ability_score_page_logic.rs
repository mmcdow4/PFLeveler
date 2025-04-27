use crate::ui::{
    self, launch_error_dialog, AbilityScoreMode, MainWindow
};
use slint::{ComponentHandle, Model, ModelRc, VecModel, SharedString};
use PathFinder::ability_scores;
use rand::prelude::*;

fn roll_nd6(n: usize) -> Vec<u32> {
    let mut output_vec = Vec::new();

    let mut rng = SmallRng::from_os_rng();

    for _ in 0..n {
        output_vec.push(rng.random_range(1..=6));
    }
    output_vec.sort();

    output_vec
}

pub fn handle_ability_score_select_button(main_window: &MainWindow, index: i32) {
    // Zero out ability score base values
    let ability_score_data = main_window.get_race__as__ability_score_data();
    for as_index in 0..ability_scores::NUMBER_ABILITY_SCORES {
        let mut data = ability_score_data.row_data(as_index).unwrap();
        data.base_value = 0;
        data.modifier = ability_scores::value_to_modifier(data.racial_bonus);
        ability_score_data.set_row_data(as_index, data);
    }
    main_window.set_race__as__ability_score_data(ability_score_data);
    main_window.set_race__as__dropdown_assignments(ModelRc::new(VecModel::from(vec![-1, -1, -1, -1, -1, -1])));
    
    match index {
        0 => {
            /* Standard - roll 4d6 and drop the lowest die, repeat 5 times */
            let mut score_vec = Vec::new();
            for _ in 0..6 {
                let rolls = roll_nd6(4);
                score_vec.push(rolls[1] + rolls[2] + rolls[3]);
            }
            score_vec.sort();
            let score_strings: Vec<SharedString> = score_vec.iter().map(|&x| SharedString::from(x.to_string())).collect();
            main_window.set_race__as__dropdown_values(ModelRc::new(VecModel::from(score_strings)));
            main_window.set_race__as__text_box_string(
                SharedString::from(
                    format!(
                        "Assign your rolled values: [{}, {}, {}, {}, {}, {}].\nYou \
                        can also switch methods or re-roll your values by clicking \
                        the select button again.",
                        score_vec[0],
                        score_vec[1],
                        score_vec[2],
                        score_vec[3],
                        score_vec[4],
                        score_vec[5]
            )));
            main_window.set_race__as__mode(AbilityScoreMode::Dropdown);
        },
        1 => {
            /* Classic - roll 3d6, repeat 5 times */
            let mut score_vec = Vec::new();
            for _ in 0..6 {
                let rolls = roll_nd6(3);
                score_vec.push(rolls[0] + rolls[1] + rolls[2]);
            }
            score_vec.sort();
            let score_strings: Vec<SharedString> = score_vec.iter().map(|&x| SharedString::from(x.to_string())).collect();
            main_window.set_race__as__dropdown_values(ModelRc::new(VecModel::from(score_strings)));
            main_window.set_race__as__text_box_string(
                SharedString::from(
                    format!(
                        "Assign your rolled values: [{}, {}, {}, {}, {}, {}].\nYou \
                        can also switch methods or re-roll your values by clicking \
                        the select button again.",
                        score_vec[0],
                        score_vec[1],
                        score_vec[2],
                        score_vec[3],
                        score_vec[4],
                        score_vec[5]
            )));
            main_window.set_race__as__mode(AbilityScoreMode::Dropdown);
        },
        2 => {
            /* Heroic - roll 2d6 and add 6, repeat 5 times */
            let mut score_vec = Vec::new();
            for _ in 0..6 {
                let rolls = roll_nd6(2);
                score_vec.push(rolls[0] + rolls[1] + 6);
            }
            score_vec.sort();
            let score_strings: Vec<SharedString> = score_vec.iter().map(|&x| SharedString::from(x.to_string())).collect();
            main_window.set_race__as__dropdown_values(ModelRc::new(VecModel::from(score_strings)));
            main_window.set_race__as__text_box_string(
                SharedString::from(
                    format!(
                        "Assign your rolled values: [{}, {}, {}, {}, {}, {}].\nYou \
                        can also switch methods or re-roll your values by clicking \
                        the select button again.",
                        score_vec[0],
                        score_vec[1],
                        score_vec[2],
                        score_vec[3],
                        score_vec[4],
                        score_vec[5]
            )));
            main_window.set_race__as__mode(AbilityScoreMode::Dropdown);
        },
        3 => {
            /* Dice Pool */
            main_window.set_race__as__dice_remaining(24);
            main_window.set_race__as__mode(AbilityScoreMode::DicePool);
            main_window.set_race__as__text_box_string("You have 24 dice remaining\nYou \
                can also switch methods or start over by clicking the select \
                button again.".into());
        },
        4 => {
            /* Purchase */
            let prompt = String::from("Enter your total point budget:\n10 points is low power\n\
                    15 points is standard\n20 points is high power\n25 points is very high power");
            let weak_window = main_window.as_weak();
            ui::launch_input_dialog(
                &prompt,  move |input_value| {
                    if let Some(main_window) = weak_window.upgrade() {
                        setup_purchase_mode(&main_window, input_value);
                    }
                }
            ).expect("Failed to launch point budget dialog");
        },
        5 => {
            /* Direct Input */
            main_window.set_race__as__mode(AbilityScoreMode::Input);
            main_window.set_race__as__text_box_string("Generate your values however you choose \
                and type them in directly.\nYou can also switch methods by clicking \
                the select button again.".into());
        },
        _ => {
            unreachable!("How did you click ability score select with value {index}?!");
        }
    }
}

fn setup_purchase_mode(main_window: &MainWindow, input_value: SharedString) {
    let mut budget = 0;
    let mut err_str = String::new();
    match input_value.parse::<i32>() {
        Ok(x) => { 
            budget = x;
            if x < 0 {
                err_str = String::from("Input a positive integer");
            }
        },
        Err(_) => { err_str = String::from("Input an integer value"); }
    }

    let ability_score_data = main_window.get_race__as__ability_score_data();
    for as_index in 0..ability_scores::NUMBER_ABILITY_SCORES {
        let mut data = ability_score_data.row_data(as_index).unwrap();
        data.base_value = 10;
        data.modifier = ability_scores::value_to_modifier(10 + data.racial_bonus);
        ability_score_data.set_row_data(as_index, data);
    }
    main_window.set_race__as__ability_score_data(ability_score_data);

    if !err_str.is_empty() {
        launch_error_dialog(&err_str).expect("Failed to launch error window");
    } else {
        main_window.set_race__as__points_remaining(budget);
        main_window.set_race__as__mode(AbilityScoreMode::Purchase);
        // main_window.set_race__as__text_box_string(format!("You have {} points \
        //     remaining\nYou can also switch methods or start over by clicking the \
        //     select button again.", budget).into());
        main_window.set_race__as__points_remaining(budget);
    }
}

pub fn handle_ability_score_select_dropdown(main_window: &MainWindow, ability_score_index: i32, index: i32) {
    let dropdown_values = main_window.get_race__as__dropdown_values();
    let dropdown_assignments = main_window.get_race__as__dropdown_assignments();
    let ability_score_data = main_window.get_race__as__ability_score_data();

    /* Update the ability score base value to the chosen value */
    let prev_index = dropdown_assignments.iter().position(|x| x == ability_score_index);
    
    let mut data = ability_score_data.row_data(ability_score_index as usize).unwrap();
    data.base_value = dropdown_values.row_data(index as usize).unwrap().parse::<i32>().unwrap();
    data.modifier = ability_scores::value_to_modifier(data.base_value + data.racial_bonus);
    ability_score_data.set_row_data(ability_score_index as usize, data);

    /* If this was assigned to a different ability score previously, swap the value in */
    let prev_assignment = dropdown_assignments.row_data(index as usize).unwrap();
    if prev_assignment != -1 {
        let mut prev_value = 0;
        if let Some(x) = prev_index {
            /* If the ability score we are setting here was previously unassigned, we just wa */
            prev_value = dropdown_values.row_data(x as usize).unwrap().parse::<i32>().unwrap();
            dropdown_assignments.set_row_data(x , prev_assignment);
        }
        let mut data = ability_score_data.row_data(prev_assignment as usize).unwrap();
        data.base_value = prev_value;
        data.modifier = ability_scores::value_to_modifier(data.base_value + data.racial_bonus);
        ability_score_data.set_row_data(prev_assignment as usize, data);
    }

    /* Update the assignment for this value */
    dropdown_assignments.set_row_data(index as usize, ability_score_index);
}

pub fn handle_ability_score_roll_value(
    main_window: &MainWindow,
    ability_score_index: i32) {

    /* Determine range for allowed number of dice */
    let ability_score_data = main_window.get_race__as__ability_score_data();
    let dice_remaining = main_window.get_race__as__dice_remaining();
    let mut unassigned_ability_scores = 0;
    for idx in 0..ability_score_data.row_count() {
        if ability_score_data.row_data(idx).unwrap().base_value == 0 {
            unassigned_ability_scores += 1;
        }
    }
    let min_dice = 3;
    let max_dice = (dice_remaining - (unassigned_ability_scores - 1) * 3) as usize;
    
    /* launch window to request number of dice */
    let weak_window = main_window.as_weak();
    ui::launch_input_dialog(
        &format!("How many dice do you want to roll? minimum 3, maximum {}", max_dice),
        move |input_value| {
            if let Some(main_window) = weak_window.upgrade() {
                roll_dice_and_save_value(
                    &main_window,
                    input_value,
                    ability_score_index as usize,
                    min_dice,
                    max_dice);
            }
        }
    ).expect("Failed to launch input window");
}

fn roll_dice_and_save_value(
    main_window: &MainWindow,
    n_string: SharedString,
    ability_score_index: usize,
    min_n: usize,
    max_n: usize
) {
    let mut n = 0;
    let mut err_str = String::new();
    match n_string.parse::<i32>() {
        Ok(x) => { 
            n = x as usize;
            if n < min_n || n > max_n {
                err_str = format!(
                    "You must choose a number of dice between {} and {}",
                    min_n,
                    max_n
                );
            }
        },
        Err(_) => { err_str = String::from("Input an integer value"); }
    }

    if !err_str.is_empty() {
        ui::launch_error_dialog(&err_str).expect("Failed to launch error window");
    } else {
        /* Roll the dice and sum the three highest dice */
        let values = roll_nd6(n);
        let value = values[n-1] + values[n-2] + values[n-3];

        let ability_score_data = main_window.get_race__as__ability_score_data();
        let mut data = ability_score_data.row_data(ability_score_index).unwrap();
        data.base_value = value as i32;
        data.modifier = ability_scores::value_to_modifier(data.base_value + data.racial_bonus);
        ability_score_data.set_row_data(ability_score_index, data);
        main_window.set_race__as__ability_score_data(ability_score_data);
        let dice_remaining = main_window.get_race__as__dice_remaining() - (n as i32);
        main_window.set_race__as__dice_remaining(dice_remaining);
        main_window.set_race__as__text_box_string(format!("You have {} dice remaining\n\
            You can also switch methods or start over by clicking the select \
            button again.", dice_remaining).into());
    }
}