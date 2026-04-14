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
    };
    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let mut user = profile.rgb_array();
        if user.iter().all(|&x| x == 0) {
            for i in 0..4 {
                user[i * 3] = 255;
                user[i * 3 + 1] = 200;
                user[i * 3 + 2] = 0;
            }
        }
        let mut yellow = [0; 12];
        let mut black = [0; 12];
        for i in 0..4 {
            if i % 2 == 0 {
                yellow[i * 3] = user[i * 3];
                yellow[i * 3 + 1] = user[i * 3 + 1];
                yellow[i * 3 + 2] = user[i * 3 + 2];
            } else {
                black[i * 3] = user[i * 3];
                black[i * 3 + 1] = user[i * 3 + 1];
                black[i * 3 + 2] = user[i * 3 + 2];
            }
        }
        let _ = manager.keyboard.set_colors_to(&yellow);
        thread::sleep(Duration::from_millis(delay));
        let _ = manager.keyboard.set_colors_to(&black);
        thread::sleep(Duration::from_millis(delay));
    }
}
