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
                user[i * 3 + 1] = 0;
                user[i * 3 + 2] = 255;
            }
        }

        let mut z = [0; 12];
        for _ in 0..rng.random_range(1..=3) {
            let zone = rng.random_range(0..4);
            z[zone * 3] = user[zone * 3];
            z[zone * 3 + 1] = user[zone * 3 + 1];
            z[zone * 3 + 2] = user[zone * 3 + 2];
        }

        let _ = manager.keyboard.set_colors_to(&z);
        thread::sleep(Duration::from_millis(delay));
        let _ = manager.keyboard.set_colors_to(&[0; 12]);
        thread::sleep(Duration::from_millis(delay));
    }
}
