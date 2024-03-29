mod winit_helper;
use winit_helper::center_window;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // See: https://slint.dev/releases/1.3.2/docs/rust/slint/docs/generated_code/struct.SampleComponent
    let ui = AppWindow::new()?;

    // Window must be shown first so sizes get calculated properly
    // Don't know if there is a better way, slight redraw artifacting on move
    ui.show()?;

    center_window(ui.window());

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.run()
}
