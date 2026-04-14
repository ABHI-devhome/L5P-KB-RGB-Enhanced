use crate::manager::{profile::Profile, Inner};
use std::sync::atomic::Ordering;
use std::{thread, time::Duration};

pub fn play(manager: &mut Inner, profile: &Profile) {
    let delay = match profile.speed {
        1 => 3000,
        2 => 2000,
        3 => 1000,
        4 => 500,
        _ => 1000,
    };
    let u_neon = [0, 255, 255, 255, 0, 255, 255, 255, 0, 0, 255, 0];
    let u_pastel = [255, 182, 193, 173, 216, 230, 255, 253, 208, 221, 160, 221];
    let mut state = true;

    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        if state {
            let _ = manager.keyboard.set_colors_to(&u_neon);
        } else {
            let _ = manager.keyboard.set_colors_to(&u_pastel);
        }
        state = !state;
        thread::sleep(Duration::from_millis(delay));
    }
}
