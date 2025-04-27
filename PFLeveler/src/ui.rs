slint::include_modules!(); // this pulls in everything from ui/*.slint

use slint::{ComponentHandle, SharedString};

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

pub fn launch_input_dialog<F: 'static + Fn(SharedString)>(message: &String, callback: F) -> Result<(), slint::PlatformError> {
    let input_dialog = InputDialog::new()?;
    let weak_dialog = input_dialog.as_weak();
    
    input_dialog.on_close_button(move |input_value| {
        if let Some(dialog) = weak_dialog.upgrade() {
            dialog.hide().expect("Failed to hide error dialog!");
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