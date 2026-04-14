use crate::manager::{profile::Profile, Inner};
use std::sync::atomic::Ordering;
use std::{thread, time::Duration};

pub fn play(manager: &mut Inner, profile: &Profile) {
    let delay = match profile.speed {
        1 => 1500,
        2 => 1000,
        3 => 800,
        4 => 400,
        _ => 800,
    } as u64;
    let variants = [
        [0, 255, 0, 34, 139, 34, 0, 255, 127, 0, 128, 0],
        [34, 139, 34, 0, 255, 127, 0, 128, 0, 0, 255, 0],
        [0, 255, 127, 0, 128, 0, 0, 255, 0, 34, 139, 34],
    ];
    let mut idx = 0;
    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let _ = manager.keyboard.transition_colors_to(&variants[idx], 10, delay / 10);
        thread::sleep(Duration::from_millis(delay));
        idx = (idx + 1) % variants.len();
    }
}
