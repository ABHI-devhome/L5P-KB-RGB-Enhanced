use crate::manager::{profile::Profile, Inner};
use std::sync::atomic::Ordering;
use std::{thread, time::Duration};

pub fn play(manager: &mut Inner, profile: &Profile) {
    let delay = match profile.speed {
        1 => 300,
        2 => 200,
        3 => 150,
        4 => 100,
        _ => 150,
    } as u64;
    let mut pos_a = 0;

    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let mut user = profile.rgb_array();
        if user.iter().all(|&x| x == 0) {
            for i in 0..4 {
                user[i * 3] = 0;
                user[i * 3 + 1] = 255;
                user[i * 3 + 2] = 255;
            }
        } // Teal and Orange
        let c1 = [user[0], user[1], user[2]];
        let c2 = [255, 128, 0]; // Strand B is always bright orange

        let pos_b = (3 - pos_a) % 4; // inverse
        let mut z = [0; 12];
        z[pos_a * 3] = c1[0];
        z[pos_a * 3 + 1] = c1[1];
        z[pos_a * 3 + 2] = c1[2];

        if pos_a == pos_b {
            // crossover merge
            z[pos_b * 3] = 255;
            z[pos_b * 3 + 1] = 255;
            z[pos_b * 3 + 2] = 255;
        } else {
            z[pos_b * 3] = c2[0];
            z[pos_b * 3 + 1] = c2[1];
            z[pos_b * 3 + 2] = c2[2];
        }

        let _ = manager.keyboard.transition_colors_to(&z, 10, delay / 10);
        thread::sleep(Duration::from_millis(delay));
        pos_a = (pos_a + 1) % 4;
    }
}
