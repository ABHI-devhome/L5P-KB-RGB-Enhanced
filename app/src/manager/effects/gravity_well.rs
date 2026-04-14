use crate::manager::{profile::Profile, Inner};
use std::sync::atomic::Ordering;
use std::{thread, time::Duration};

pub fn play(manager: &mut Inner, profile: &Profile) {
    let delay = match profile.speed {
        1 => 300,
        2 => 200,
        3 => 150,
        4 => 100,
        _ => 150,
    } as u64;
    let variants = [
        [0, 0, 0, 100, 100, 100, 255, 255, 255, 100, 100, 100],
        [100, 100, 100, 0, 0, 0, 100, 100, 100, 255, 255, 255],
        [255, 255, 255, 100, 100, 100, 0, 0, 0, 100, 100, 100],
        [100, 100, 100, 255, 255, 255, 100, 100, 100, 0, 0, 0],
    ];
    let mut idx = 0;
    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let _ = manager.keyboard.transition_colors_to(&variants[idx], 10, delay / 8);
        thread::sleep(Duration::from_millis(delay * 4));
        idx = (idx + 1) % variants.len();
    }
}
