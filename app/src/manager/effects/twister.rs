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
    let mut parity = true;

    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let mut user = profile.rgb_array();
        if user.iter().all(|&x| x == 0) {
            user[0..3].copy_from_slice(&[0, 255, 0]);
            user[3..6].copy_from_slice(&[255, 0, 255]);
            user[6..9].copy_from_slice(&[0, 255, 0]);
            user[9..12].copy_from_slice(&[255, 0, 255]);
        }
        let c1 = [user[0], user[1], user[2]];
        let c2 = [user[3], user[4], user[5]];
        let mut z = [0; 12];
        if parity {
            z[0..3].copy_from_slice(&c1);
            z[3..6].copy_from_slice(&c2);
            z[6..9].copy_from_slice(&c1);
            z[9..12].copy_from_slice(&c2);
        } else {
            z[0..3].copy_from_slice(&c2);
            z[3..6].copy_from_slice(&c1);
            z[6..9].copy_from_slice(&c2);
            z[9..12].copy_from_slice(&c1);
        }

        let _ = manager.keyboard.transition_colors_to(&z, 10, delay / 10);
        thread::sleep(Duration::from_millis(delay));
        parity = !parity;
    }
}
