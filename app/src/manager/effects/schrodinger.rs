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
        let mut user = profile.rgb_array();
        if user.iter().all(|&x| x == 0) {
            for i in 0..4 {
                user[i * 3] = 0;
                user[i * 3 + 1] = 255;
                user[i * 3 + 2] = 255; // Cyan
            }
        }
        let collapse_c = [255, 0, 0]; // Collapses to Red

        let mut z = [0; 12];
        for i in 0..4 {
            if rng.random_bool(0.5) {
                z[i * 3] = user[i * 3];
                z[i * 3 + 1] = user[i * 3 + 1];
                z[i * 3 + 2] = user[i * 3 + 2];
            } else {
                z[i * 3] = collapse_c[0];
                z[i * 3 + 1] = collapse_c[1];
                z[i * 3 + 2] = collapse_c[2];
            }
        }
        let _ = manager.keyboard.transition_colors_to(&z, 5, delay / 5);
        thread::sleep(Duration::from_millis(delay));
    }
}
