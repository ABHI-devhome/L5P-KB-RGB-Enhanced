use crate::manager::{profile::Profile, Inner};
use rand::Rng;
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
    let mut rng = rand::rng();
    let mut magnet;

    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        magnet = rng.random_range(0..4);

        for step in 1..=4 {
            let mut curr = [0; 12];
            for i in 0..4 {
                let dist = (magnet as i32 - i as i32).abs();
                let intensity = if dist == 0 {
                    255
                } else if dist <= step {
                    255 / (dist as u8 + 1)
                } else {
                    0
                };
                curr[i * 3] = intensity;
                curr[i * 3 + 1] = 0;
                curr[i * 3 + 2] = 255 - intensity;
            }
            let _ = manager.keyboard.transition_colors_to(&curr, 5, delay / 5);
            thread::sleep(Duration::from_millis(delay));
            if manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
                return;
            }
        }
        thread::sleep(Duration::from_millis(delay * 4));
    }
}
