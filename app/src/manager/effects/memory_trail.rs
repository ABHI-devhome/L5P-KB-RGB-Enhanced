use crate::manager::{profile::Profile, Inner};
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
    let mut pos = 0;
    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let mut user = profile.rgb_array();
        if user.iter().all(|&x| x == 0) {
            for i in 0..4 {
                user[i * 3] = 255;
                user[i * 3 + 1] = 0;
                user[i * 3 + 2] = 0;
            }
        }

        let mut z = [0; 12];
        // Active
        z[pos * 3] = user[pos * 3];
        z[pos * 3 + 1] = user[pos * 3 + 1];
        z[pos * 3 + 2] = user[pos * 3 + 2];
        // Trail 1
        let t1 = (pos + 3) % 4; // Previous pos
        z[t1 * 3] = user[t1 * 3] / 3;
        z[t1 * 3 + 1] = user[t1 * 3 + 1] / 3;
        z[t1 * 3 + 2] = user[t1 * 3 + 2] / 3;
        // Trail 2
        let t2 = (pos + 2) % 4;
        z[t2 * 3] = user[t2 * 3] / 8;
        z[t2 * 3 + 1] = user[t2 * 3 + 1] / 8;
        z[t2 * 3 + 2] = user[t2 * 3 + 2] / 8;

        let _ = manager.keyboard.transition_colors_to(&z, 5, delay / 5);
        pos = (pos + 1) % 4;
        thread::sleep(Duration::from_millis(delay));
    }
}
