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
    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let mut c = [0; 12];
        for i in 0..4 {
            c[i * 3] = rng.random_range(0..=255);
            c[i * 3 + 1] = rng.random_range(0..=255);
            c[i * 3 + 2] = rng.random_range(0..=255);
        }
        let _ = manager.keyboard.transition_colors_to(&c, 5, delay / 5);
        thread::sleep(Duration::from_millis(delay));
    }
}
