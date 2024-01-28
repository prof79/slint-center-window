// winit_helper.rs

use winit::dpi::PhysicalPosition;
use winit::monitor::MonitorHandle;
use winit::window::Window;
use i_slint_backend_winit::WinitWindowAccessor;

/// Centers the window on the primary monitor.
/// Only works with `winit` backends.
pub fn center_window(window: &slint::Window) {

    if window.has_winit_window() {

        window.with_winit_window(|window| {
            
            match window.primary_monitor() {
                Some(monitor) => set_centered(&window, monitor),
                None => (),
            };

            None as Option<()>
        });
    }
}

/// Sets the window position to be in the center of the given monitor.
/// Will do nothing if the window is in fullscreen.
/// 
/// ## Platform-specific
/// 
/// This only works on Windows and X11.
/// It's not tested on macOS.
/// 
/// This has no effect on Android, Wayland or iOS.
fn set_centered(window: &Window, monitor: MonitorHandle) {
    let window_size = window.outer_size();

    let monitor_size = monitor.size();
    let monitor_position = monitor.position();

    let mut monitor_window_position = PhysicalPosition { x: 0, y: 0 };

    monitor_window_position.x =
        (monitor_position.x as f32 + (monitor_size.width as f32 * 0.5) - (window_size.width as f32 * 0.5)) as i32;

    monitor_window_position.y =
        (monitor_position.y as f32 + (monitor_size.height as f32 * 0.5) - (window_size.height as f32 * 0.5)) as i32;

    window.set_outer_position(monitor_window_position);
}
