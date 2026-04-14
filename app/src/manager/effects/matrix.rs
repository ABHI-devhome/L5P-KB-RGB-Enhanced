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
    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let zone = rng.random_range(0..4);
        let mut curr = [0; 12];
        curr[zone * 3] = 0;
        curr[zone * 3 + 1] = 255;
        curr[zone * 3 + 2] = 0;
        let _ = manager.keyboard.set_colors_to(&curr);
        thread::sleep(Duration::from_millis(delay));

        curr[zone * 3] = 0;
        curr[zone * 3 + 1] = 50;
        curr[zone * 3 + 2] = 0;
        let _ = manager.keyboard.set_colors_to(&curr);
        thread::sleep(Duration::from_millis(delay));

        let _ = manager.keyboard.set_colors_to(&[0; 12]);
        thread::sleep(Duration::from_millis(delay / 2));
    }
}
