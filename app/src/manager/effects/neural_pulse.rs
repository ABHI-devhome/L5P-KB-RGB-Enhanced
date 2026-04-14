use crate::manager::{profile::Profile, Inner};
use rand::Rng;
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
    let mut rng = rand::rng();

    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let mut z = [0; 12];
        let zone = rng.random_range(0..4);
        z[zone * 3] = 0;
        z[zone * 3 + 1] = 255;
        z[zone * 3 + 2] = 255; // Cyan sparks
        if rng.random_bool(0.3) {
            let zone2 = rng.random_range(0..4);
            z[zone2 * 3] = 255;
            z[zone2 * 3 + 1] = 0;
            z[zone2 * 3 + 2] = 255; // Magenta sparks
        }

        let _ = manager.keyboard.set_colors_to(&z);
        thread::sleep(Duration::from_millis(delay));
        let _ = manager.keyboard.set_colors_to(&[0; 12]);
        thread::sleep(Duration::from_millis(rng.random_range(delay..(delay * 10))));
        // erratic delays
    }
}
