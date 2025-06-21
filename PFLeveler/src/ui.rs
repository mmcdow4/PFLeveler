slint::include_modules!(); // this pulls in everything from ui/*.slint

use slint::{ComponentHandle, SharedString};

pub enum ButtonType {
    BUTTON1,
    BUTTON2,
    OK,
    CANCEL
}

pub fn launch_error_dialog(message: &String) -> Result<(), slint::PlatformError> {
    let error_dialog = ErrorDialog::new()?;
    let weak_dialog = error_dialog.as_weak();
    
    error_dialog.on_close_button(move || {
        if let Some(dialog) = weak_dialog.upgrade() {
            dialog.hide().expect("Failed to hide error dialog!");
        }
    });

    let weak_dialog = error_dialog.as_weak();
    if let Some(dialog) = weak_dialog.upgrade() {
        dialog.set_error_text(message.into());
        dialog.show()?;
    } else {
        return Err(slint::PlatformError::Other(format!("Failed to upgrade error dialog to show message '{}'", message)));
    }
    Ok(())
}

pub fn launch_warning_dialog<F: 'static + FnMut(ButtonType)>(message: &String, mut callback: F) -> Result<(), slint::PlatformError> {
    let warning_dialog = WarningDialog::new()?;
    let weak_dialog = warning_dialog.as_weak();
    
    warning_dialog.on_close_button(move |button| {
        if let Some(dialog) = weak_dialog.upgrade() {
            dialog.hide().expect("Failed to hide warning dialog!");
            match button {
                0 => callback(ButtonType::OK),
                1 => callback(ButtonType::CANCEL),
                _ => unreachable!("Unexpected button press {button}"),
            }
        }
    });

    let weak_dialog = warning_dialog.as_weak();
    if let Some(dialog) = weak_dialog.upgrade() {
        dialog.set_warning_text(message.into());
        dialog.show()?;
    } else {
        return Err(slint::PlatformError::Other(format!("Failed to upgrade warning dialog to show message '{}'", message)));
    }
    Ok(())
}

pub fn launch_input_dialog<F: 'static + FnMut(SharedString)>(message: &String, mut callback: F) -> Result<(), slint::PlatformError> {
    let input_dialog = InputDialog::new()?;
    let weak_dialog = input_dialog.as_weak();
    
    input_dialog.on_close_button(move |input_value| {
        if let Some(dialog) = weak_dialog.upgrade() {
            dialog.hide().expect("Failed to hide input dialog!");
            callback(input_value);
        }
    });

    let weak_dialog = input_dialog.as_weak();
    if let Some(dialog) = weak_dialog.upgrade() {
        dialog.set_prompt_text(message.into());
        dialog.show()?;
    } else {
        return Err(slint::PlatformError::Other(format!("Failed to upgrade input dialog to show message '{}'", message)));
    }
    Ok(())
}