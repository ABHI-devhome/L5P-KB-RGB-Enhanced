use crate::manager::{profile::Profile, Inner};
use std::sync::atomic::Ordering;
use std::{thread, time::Duration};

pub fn play(manager: &mut Inner, profile: &Profile) {
    let delay = match profile.speed {
        1 => 120,
        2 => 90,
        3 => 60,
        4 => 30,
        _ => 60,
    };
    let frames = [
        [0, 3], // outer
        [1, 2], // inner
    ];
    let mut idx: usize = 0;
    let mut dir: i32 = 1;
    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let mut user = profile.rgb_array();
        if user.iter().all(|&x| x == 0) {
            for i in 0..4 {
                user[i * 3] = if i % 2 == 0 { 0 } else { 255 };
                user[i * 3 + 1] = if i % 2 == 0 { 255 } else { 0 };
                user[i * 3 + 2] = 255;
            }
        }
        let mut z = [0; 12];
        let f = frames[idx as usize];
        z[f[0] * 3] = user[f[0] * 3];
        z[f[0] * 3 + 1] = user[f[0] * 3 + 1];
        z[f[0] * 3 + 2] = user[f[0] * 3 + 2];
        z[f[1] * 3] = user[f[1] * 3];
        z[f[1] * 3 + 1] = user[f[1] * 3 + 1];
        z[f[1] * 3 + 2] = user[f[1] * 3 + 2];
        let _ = manager.keyboard.set_colors_to(&z);
        if idx == 1 {
            dir = -1;
        } else if idx == 0 {
            dir = 1;
        }
        idx = (idx as i32 + dir) as usize;
        thread::sleep(Duration::from_millis(delay));
    }
}
