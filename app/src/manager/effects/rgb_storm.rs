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
        let mut user = profile.rgb_array();
        if user.iter().all(|&x| x == 0) {
            for i in 0..4 {
                user[i * 3] = 255;
                user[i * 3 + 1] = 255;
                user[i * 3 + 2] = 255;
            }
        }

        let strike_zone = rng.random_range(0..4);
        let mut z = [0; 12];

        z[strike_zone * 3] = user[strike_zone * 3];
        z[strike_zone * 3 + 1] = user[strike_zone * 3 + 1];
        z[strike_zone * 3 + 2] = user[strike_zone * 3 + 2];

        let _ = manager.keyboard.set_colors_to(&z);
        thread::sleep(Duration::from_millis(delay));

        // flash secondary
        if rng.random_bool(0.5) {
            let z2 = (strike_zone + 1) % 4;
            z[z2 * 3] = user[z2 * 3] / 2;
            z[z2 * 3 + 1] = user[z2 * 3 + 1] / 2;
            z[z2 * 3 + 2] = user[z2 * 3 + 2] / 2;
            let _ = manager.keyboard.set_colors_to(&z);
            thread::sleep(Duration::from_millis(delay / 2));
        }

        let _ = manager.keyboard.set_colors_to(&[0; 12]);
        thread::sleep(Duration::from_millis(rng.random_range((delay * 2)..(delay * 15))));
    }
}
