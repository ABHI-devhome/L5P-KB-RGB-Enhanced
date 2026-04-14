use crate::manager::{profile::Profile, Inner};
use rand::Rng;
use std::sync::atomic::Ordering;
use std::{thread, time::Duration};

pub fn play(manager: &mut Inner, profile: &Profile) {
    let delay = match profile.speed {
        1 => 100,
        2 => 70,
        3 => 40,
        4 => 15,
        _ => 40,
    } as u64;
    let mut rng = rand::rng();

    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let center = rng.random_range(0..4);
        let mut z;

        for intensity in (50..=255).step_by(10) {
            z = [0; 12];
            z[center * 3] = intensity;
            z[center * 3 + 1] = intensity / 2;
            z[center * 3 + 2] = 0;
            let _ = manager.keyboard.set_colors_to(&z);
            thread::sleep(Duration::from_millis(delay));
            if manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
                return;
            }
        }

        let _ = manager.keyboard.transition_colors_to(&[255; 12], 5, delay / 5);
        thread::sleep(Duration::from_millis(delay));
        let _ = manager.keyboard.transition_colors_to(&[0; 12], 10, delay / 2);
        thread::sleep(Duration::from_millis(delay * 10));
    }
}
