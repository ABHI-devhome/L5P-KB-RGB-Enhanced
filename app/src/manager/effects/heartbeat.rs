use crate::manager::{profile::Profile, Inner};
use std::sync::atomic::Ordering;
use std::{thread, time::Duration};

pub fn play(manager: &mut Inner, profile: &Profile) {
    let mut c = profile.rgb_array();
    if c.iter().all(|&x| x == 0) {
        for i in 0..4 {
            c[i * 3] = 255;
            c[i * 3 + 1] = 20;
            c[i * 3 + 2] = 147;
        } // Deep Pink default
    }
    let d1 = 150;
    let d2 = 800;
    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let _ = manager.keyboard.set_colors_to(&c);
        thread::sleep(Duration::from_millis(d1));
        let _ = manager.keyboard.set_colors_to(&[0; 12]);
        thread::sleep(Duration::from_millis(d1));

        if manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
            break;
        }

        let _ = manager.keyboard.set_colors_to(&c);
        thread::sleep(Duration::from_millis(d1));
        let _ = manager.keyboard.set_colors_to(&[0; 12]);
        thread::sleep(Duration::from_millis(d2));
    }
}
