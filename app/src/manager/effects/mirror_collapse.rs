use crate::manager::{profile::Profile, Inner};
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

    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let mut user = profile.rgb_array();
        if user.iter().all(|&x| x == 0) {
            for i in 0..4 {
                user[i * 3] = 0;
                user[i * 3 + 1] = 255;
                user[i * 3 + 2] = 255;
            }
        }

        // Z1 & Z4
        let mut z = [0; 12];
        z[0] = user[0];
        z[1] = user[1];
        z[2] = user[2];
        z[9] = user[9];
        z[10] = user[10];
        z[11] = user[11];
        let _ = manager.keyboard.transition_colors_to(&z, 10, delay / 10);
        thread::sleep(Duration::from_millis(delay * 2));

        // Z2 & Z3
        z = [0; 12];
        z[3] = user[3];
        z[4] = user[4];
        z[5] = user[5];
        z[6] = user[6];
        z[7] = user[7];
        z[8] = user[8];
        let _ = manager.keyboard.transition_colors_to(&z, 10, delay / 10);
        thread::sleep(Duration::from_millis(delay * 2));

        // Collapse (all on, flash to white, then black)
        let _ = manager.keyboard.transition_colors_to(&[255; 12], 5, delay / 5);
        thread::sleep(Duration::from_millis(delay));

        let _ = manager.keyboard.transition_colors_to(&[0; 12], 10, delay / 5);
        thread::sleep(Duration::from_millis(delay * 3));
    }
}
