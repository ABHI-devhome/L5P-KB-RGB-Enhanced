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

    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let mut user = profile.rgb_array();
        if user.iter().all(|&x| x == 0) {
            for i in 0..4 {
                user[i * 3] = 255;
                user[i * 3 + 1] = 255;
                user[i * 3 + 2] = 255;
            }
        }

        for stage in 0..4 {
            let mut z = [0; 12];
            // stage 0 = Z1 bright
            // stage 1 = Z1 off, Z2 faint
            z[stage * 3] = user[stage * 3] / (stage as u8 + 1).max(1);
            z[stage * 3 + 1] = user[stage * 3 + 1] / (stage as u8 + 1).max(1);
            z[stage * 3 + 2] = user[stage * 3 + 2] / (stage as u8 + 1).max(1);

            let _ = manager.keyboard.transition_colors_to(&z, 5, delay / 5);
            thread::sleep(Duration::from_millis(delay));
        }
        thread::sleep(Duration::from_millis(delay * 3));
    }
}
