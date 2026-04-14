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
    let mut dir: i32 = 1;
    let mut fill: i32 = 0; // 0 to 4

    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let mut user = profile.rgb_array();
        if user.iter().all(|&x| x == 0) {
            for i in 0..4 {
                user[i * 3] = 0;
                user[i * 3 + 1] = 255;
                user[i * 3 + 2] = 200;
            }
        }
        let mut z = [0; 12];
        for i in 0..4 {
            if i < fill {
                z[(i as usize) * 3] = user[(i as usize) * 3];
                z[(i as usize) * 3 + 1] = user[(i as usize) * 3 + 1];
                z[(i as usize) * 3 + 2] = user[(i as usize) * 3 + 2];
            }
        }
        let _ = manager.keyboard.transition_colors_to(&z, 5, delay / 5);

        fill += dir;
        if fill == 5 {
            dir = -1;
            fill = 4;
        } else if fill == -1 {
            dir = 1;
            fill = 0;
        }

        thread::sleep(Duration::from_millis(delay));
    }
}
