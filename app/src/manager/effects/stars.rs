use crate::manager::{profile::Profile, Inner};
use rand::Rng;
use std::sync::atomic::Ordering;
use std::{thread, time::Duration};

pub fn play(manager: &mut Inner, profile: &Profile) {
    let delay = match profile.speed {
        1 => 400,
        2 => 300,
        3 => 200,
        4 => 100,
        _ => 200,
    } as u64;
    let mut rng = rand::rng();
    let mut current;
    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let zone = rng.random_range(0..4);
        current = [0; 12];
        current[zone * 3] = 200;
        current[zone * 3 + 1] = 240;
        current[zone * 3 + 2] = 255;
        let _ = manager.keyboard.transition_colors_to(&current, 5, delay / 5);
        thread::sleep(Duration::from_millis(delay));

        let _ = manager.keyboard.transition_colors_to(&[0; 12], 5, delay / 5);
    }
}
