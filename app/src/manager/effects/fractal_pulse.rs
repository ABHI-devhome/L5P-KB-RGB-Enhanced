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
    };

    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let mut user = profile.rgb_array();
        if user.iter().all(|&x| x == 0) {
            for i in 0..4 {
                user[i * 3] = 128;
                user[i * 3 + 1] = 0;
                user[i * 3 + 2] = 255;
            }
        }

        for macro_intensity in [255, 150, 50, 0].iter() {
            for micro_intensity in [255, 0].iter() {
                let mut z = [0; 12];
                for i in 0..12 {
                    let base = (user[i] as f32 * (*macro_intensity as f32 / 255.0)) as u8;
                    let micro = (base as f32 * (*micro_intensity as f32 / 255.0)) as u8;
                    z[i] = micro;
                }
                let _ = manager.keyboard.set_colors_to(&z);
                thread::sleep(Duration::from_millis(delay / 4));
                if manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
                    return;
                }
            }
            thread::sleep(Duration::from_millis(delay));
        }
    }
}
