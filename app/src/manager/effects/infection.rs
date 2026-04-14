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
    let mut fill = 0;
    let mut virus = [0, 255, 0];

    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let mut user = profile.rgb_array();
        if user.iter().all(|&x| x == 0) {
            for i in 0..4 {
                user[i * 3] = 0;
                user[i * 3 + 1] = 0;
                user[i * 3 + 2] = 255;
            }
        }

        if fill > 4 {
            fill = 0;
            virus = [user[1], user[2], user[0]];
        }

        let mut z = [0; 12];
        for i in 0..4 {
            if i < fill {
                z[i * 3] = virus[0];
                z[i * 3 + 1] = virus[1];
                z[i * 3 + 2] = virus[2];
            } else {
                z[i * 3] = user[i * 3];
                z[i * 3 + 1] = user[i * 3 + 1];
                z[i * 3 + 2] = user[i * 3 + 2];
            }
        }

        let _ = manager.keyboard.transition_colors_to(&z, 10, delay / 10);
        fill += 1;
        thread::sleep(Duration::from_millis(delay * 2));
    }
}
