use crate::manager::{profile::Profile, Inner};
use rand::Rng;
use std::sync::atomic::Ordering;
use std::{thread, time::Duration};

pub fn play(manager: &mut Inner, profile: &Profile) {
    let delay = match profile.speed {
        1 => 200,
        2 => 150,
        3 => 100,
        4 => 50,
        _ => 100,
    };
    let mut rng = rand::rng();
    let mut entropy_level = 0.0;

    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let mut user = profile.rgb_array();
        if user.iter().all(|&x| x == 0) {
            for i in 0..4 {
                user[i * 3] = 0;
                user[i * 3 + 1] = 255;
                user[i * 3 + 2] = 0;
            }
        }

        let mut z = [0; 12];
        for i in 0..12 {
            let shift = rng.random_range(-255..=255) as f32 * entropy_level;
            z[i] = (user[i] as f32 + shift).clamp(0.0, 255.0) as u8;
        }

        let _ = manager.keyboard.set_colors_to(&z);
        thread::sleep(Duration::from_millis(delay));

        entropy_level += 0.01;
        if entropy_level > 1.0 {
            entropy_level = 0.0;
        } // reset
    }
}
