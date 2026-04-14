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
    let mut _arr = profile.rgb_array();
    if _arr.iter().all(|&x| x == 0) {
        _arr = [255, 0, 0, 255, 0, 0, 255, 0, 0, 255, 0, 0];
    }
    let c = [_arr[0], _arr[1], _arr[2]];
    let mut pos = 0;
    let mut dir = 1;
    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let mut z = [0; 12];
        z[pos * 3] = c[0];
        z[pos * 3 + 1] = c[1];
        z[pos * 3 + 2] = c[2];
        let _ = manager.keyboard.set_colors_to(&z);
        pos = (pos as i32 + dir) as usize;
        if pos == 3 {
            dir = -1;
        } else if pos == 0 {
            dir = 1;
        }
        thread::sleep(Duration::from_millis(delay));
    }
}
