use crate::manager::{profile::Profile, Inner};
use std::sync::atomic::Ordering;
use std::{thread, time::Duration};

pub fn play(manager: &mut Inner, _profile: &Profile) {
    let base = 50;
    let mut t1 = 0;
    let mut t2 = 0;
    let mut t3 = 0;
    let mut t4 = 0;
    let mut z = [0; 12];

    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        t1 += 1; // ms fast
        if t1 % 2 == 0 {
            z[0..3].copy_from_slice(&[255, 0, 0]);
        } else {
            z[0..3].copy_from_slice(&[0, 0, 0]);
        }

        if t1 % 10 == 0 {
            t2 += 1;
        } // seconds
        if t2 % 2 == 0 {
            z[3..6].copy_from_slice(&[0, 255, 0]);
        } else {
            z[3..6].copy_from_slice(&[0, 0, 0]);
        }

        if t1 % 50 == 0 {
            t3 += 1;
        } // minutes
        if t3 % 2 == 0 {
            z[6..9].copy_from_slice(&[0, 0, 255]);
        } else {
            z[6..9].copy_from_slice(&[0, 0, 0]);
        }

        if t1 % 200 == 0 {
            t4 += 1;
        } // hours
        if t4 % 2 == 0 {
            z[9..12].copy_from_slice(&[255, 255, 0]);
        } else {
            z[9..12].copy_from_slice(&[0, 0, 0]);
        }

        let _ = manager.keyboard.set_colors_to(&z);
        thread::sleep(Duration::from_millis(base));
    }
}
