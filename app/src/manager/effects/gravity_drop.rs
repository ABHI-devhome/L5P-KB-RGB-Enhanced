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
    let mut pos: i32 = 0;
    let frames = vec![
        [255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let _ = manager.keyboard.set_colors_to(&frames[pos as usize]);
        pos += 1;
        if pos >= frames.len() as i32 {
            pos = 0;
        }

        let sleep_var = if pos == 0 { delay * 4 } else { delay.saturating_sub((pos as u64) * 10) };
        thread::sleep(Duration::from_millis(sleep_var.max(10)));
    }
}
