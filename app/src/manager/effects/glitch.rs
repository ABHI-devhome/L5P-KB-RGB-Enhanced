use crate::manager::{profile::Profile, Inner};
use rand::Rng;
use std::sync::atomic::Ordering;
use std::{thread, time::Duration};

pub fn play(manager: &mut Inner, profile: &Profile) {
    let delay = match profile.speed {
        1 => 50,
        2 => 40,
        3 => 30,
        4 => 10,
        _ => 30,
    };
    let mut rng = rand::rng();

    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let mut user = profile.rgb_array();
        if user.iter().all(|&x| x == 0) {
            for i in 0..4 {
                user[i * 3] = 0;
                user[i * 3 + 1] = 255;
                user[i * 3 + 2] = 255;
            }
        }

        let mut z = [0; 12];
        for i in 0..12 {
            z[i] = user[i];
        }

        if rng.random_bool(0.15) {
            let glitch_zone = rng.random_range(0..4);
            if rng.random_bool(0.5) {
                z[glitch_zone * 3] = 0;
                z[glitch_zone * 3 + 1] = 0;
                z[glitch_zone * 3 + 2] = 0;
            } else {
                z[glitch_zone * 3] = 255;
                z[glitch_zone * 3 + 1] = 255;
                z[glitch_zone * 3 + 2] = 255;
            }
        }

        let _ = manager.keyboard.set_colors_to(&z);
        thread::sleep(Duration::from_millis(delay));
    }
}
