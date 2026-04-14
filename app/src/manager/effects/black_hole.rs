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
    let seq = vec![
        [100, 100, 100, 0, 0, 0, 0, 0, 0, 100, 100, 100],
        [0, 0, 0, 150, 150, 150, 150, 150, 150, 0, 0, 0],
        [0, 0, 0, 255, 255, 255, 255, 255, 255, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let mut i = 0;
    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let _ = manager.keyboard.transition_colors_to(&seq[i], 10, delay / 10);
        thread::sleep(Duration::from_millis(delay * 2));
        i = (i + 1) % seq.len();
        if i == 0 {
            thread::sleep(Duration::from_millis(delay * 5));
        }
    }
}
