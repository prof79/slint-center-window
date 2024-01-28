use std::env::set_var;

fn main() {

    // Slint backend renderer
    set_var("SLINT_BACKEND", "winit-skia");

    slint_build::compile("ui/appwindow.slint").unwrap();
}
