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
    let mut z = [128i32; 12];

    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        for i in 0..12 {
            let shift = rng.random_range(-20..=20);
            z[i] = (z[i] + shift).clamp(0, 255);
        }
        let out = [
            z[0] as u8,
            z[1] as u8,
            z[2] as u8,
            z[3] as u8,
            z[4] as u8,
            z[5] as u8,
            z[6] as u8,
            z[7] as u8,
            z[8] as u8,
            z[9] as u8,
            z[10] as u8,
            z[11] as u8,
        ];

        let _ = manager.keyboard.transition_colors_to(&out, 5, delay / 5);
        thread::sleep(Duration::from_millis(delay));
    }
}
