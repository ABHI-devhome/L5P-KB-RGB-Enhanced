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
    };

    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let mut user = profile.rgb_array();
        if user.iter().all(|&x| x == 0) {
            for i in 0..4 {
                user[i * 3] = 0;
                user[i * 3 + 1] = 255;
                user[i * 3 + 2] = 0;
            }
        }

        let mut mem = Vec::new();
        for pos in 0..4 {
            let mut z = [0; 12];
            z[pos * 3] = user[pos * 3];
            z[pos * 3 + 1] = user[pos * 3 + 1];
            z[pos * 3 + 2] = user[pos * 3 + 2];
            mem.push(z);
            let _ = manager.keyboard.set_colors_to(&z);
            thread::sleep(Duration::from_millis(delay));
            if manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
                return;
            }
        }

        thread::sleep(Duration::from_millis(delay * 3));

        for z in mem.iter().rev() {
            let _ = manager.keyboard.set_colors_to(z);
            thread::sleep(Duration::from_millis(delay));
            if manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
                return;
            }
        }
    }
}
