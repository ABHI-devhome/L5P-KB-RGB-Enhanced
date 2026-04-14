use std::sync::atomic::Ordering;
use std::{thread, time::Duration};

use crate::manager::{profile::Profile, Inner};

pub fn play(manager: &mut Inner, profile: &Profile) {
    let mut step: u64 = 0;

    let delay = match profile.speed {
        1 => 400,
        2 => 300,
        3 => 200,
        4 => 100,
        _ => 200,
    };

    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        if step % 2 == 0 {
            // Zone 1 & 2 Red, Zone 3 & 4 Blue
            let _ = manager.keyboard.set_colors_to(&[
                255, 0, 0, // Zone 1
                255, 0, 0, // Zone 2
                0, 0, 255, // Zone 3
                0, 0, 255, // Zone 4
            ]);
        } else {
            // Zone 1 & 2 Blue, Zone 3 & 4 Red
            let _ = manager.keyboard.set_colors_to(&[0, 0, 255, 0, 0, 255, 255, 0, 0, 255, 0, 0]);
        }

        step = step.wrapping_add(1);
        thread::sleep(Duration::from_millis(delay));
    }
}
