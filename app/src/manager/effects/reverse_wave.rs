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
    } as u64;
    let mut rng = rand::rng();
    let mut pos: i32 = 4;
    let mut dir: i32 = -1; // starts right to left

    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let mut user = profile.rgb_array();
        if user.iter().all(|&x| x == 0) {
            for i in 0..4 {
                user[i * 3] = 0;
                user[i * 3 + 1] = 200;
                user[i * 3 + 2] = 255;
            }
        }

        let mut z = [0; 12];
        if pos >= 0 && pos < 4 {
            let u = pos as usize;
            z[u * 3] = user[u * 3];
            z[u * 3 + 1] = user[u * 3 + 1];
            z[u * 3 + 2] = user[u * 3 + 2];
        }

        let _ = manager.keyboard.transition_colors_to(&z, 5, delay / 5);
        thread::sleep(Duration::from_millis(delay));

        pos += dir;

        if rng.random_bool(0.1) {
            dir *= -1;
        } // random flip

        if pos > 4 {
            pos = 4;
            dir = -1;
        }
        if pos < -1 {
            pos = -1;
            dir = 1;
        }
    }
}
