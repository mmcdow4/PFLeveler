slint::include_modules!(); // this pulls in everything from ui/*.slint

pub fn launch_error_dialog(message: &String) -> Result<(), slint::PlatformError> {
    slint::slint! {
        import { StandardButton, VerticalBox } from "std-widgets.slint";
        export ErrorDialog := Dialog {
        property <string> error_text: "Error";
        VerticalBox {
            alignment: start;
            Text {
                text: error_text;
            }
            StandardButton { 
                kind: ok;
                clicked => { root.close-button(); }
            }
        }

        callback close-button;
    }};
    let error_dialog = ErrorDialog::new()?;
    error_dialog.set_error_text(message.into());

    let weak_dialog = error_dialog.as_weak();
    error_dialog.on_close_button(move || {
        if let Some(dialog) = weak_dialog.upgrade() {
            dialog.hide().expect("Failed to hide error dialog!");
        }
    });
    error_dialog.run()
}