use crate::manager::{profile::Profile, Inner};
use std::sync::atomic::Ordering;
use std::{thread, time::Duration};

pub fn play(manager: &mut Inner, profile: &Profile) {
    let delay = match profile.speed {
        1 => 300,
        2 => 200,
        3 => 100,
        4 => 50,
        _ => 100,
    };
    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let mut z = [0; 12];
        let _ = manager.keyboard.set_colors_to(&z);
        thread::sleep(Duration::from_millis(delay));

        for i in 0..4 {
            z[i * 3] = 0;
            z[i * 3 + 1] = 255;
            z[i * 3 + 2] = 0;
            let _ = manager.keyboard.set_colors_to(&z);
            thread::sleep(Duration::from_millis(delay));
            if manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
                return;
            }
        }
        thread::sleep(Duration::from_millis(delay * 3));
    }
}
