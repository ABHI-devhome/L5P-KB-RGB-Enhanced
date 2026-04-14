use crate::manager::{profile::Profile, Inner};
use std::sync::atomic::Ordering;
use std::{thread, time::Duration};

pub fn play(manager: &mut Inner, profile: &Profile) {
    let delay = match profile.speed {
        1 => 100,
        2 => 70,
        3 => 40,
        4 => 15,
        _ => 40,
    };
    let c = [255; 12]; // Full White
    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let _ = manager.keyboard.set_colors_to(&c);
        thread::sleep(Duration::from_millis(delay));
        let _ = manager.keyboard.set_colors_to(&[0; 12]);
        thread::sleep(Duration::from_millis(delay));
    }
}
