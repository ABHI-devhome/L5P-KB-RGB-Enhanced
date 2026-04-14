use crate::manager::{profile::Profile, Inner};
use std::sync::atomic::Ordering;
use std::{thread, time::Duration};

pub fn play(manager: &mut Inner, profile: &Profile) {
    let delay = match profile.speed {
        1 => 80,
        2 => 60,
        3 => 40,
        4 => 20,
        _ => 40,
    } as u64;

    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let mut user = profile.rgb_array();
        if user.iter().all(|&x| x == 0) {
            for i in 0..4 {
                user[i * 3] = 255;
                user[i * 3 + 1] = 0;
                user[i * 3 + 2] = 0;
            }
        }

        // P wave (Z1)
        let mut z = [0; 12];
        z[0] = user[0] / 2;
        z[1] = user[1] / 2;
        z[2] = user[2] / 2;
        let _ = manager.keyboard.transition_colors_to(&z, 3, delay / 3);
        thread::sleep(Duration::from_millis(delay * 2));

        // QRS complex (Z2 -> Z3 spike)
        z = [0; 12];
        z[3] = user[3];
        z[4] = user[4];
        z[5] = user[5];
        let _ = manager.keyboard.set_colors_to(&z);
        thread::sleep(Duration::from_millis(delay / 2));

        z = [0; 12];
        z[6] = user[6];
        z[7] = user[7];
        z[8] = user[8];
        let _ = manager.keyboard.set_colors_to(&z);
        thread::sleep(Duration::from_millis(delay / 2));

        // T wave (Z4)
        z = [0; 12];
        z[9] = user[9] / 2;
        z[10] = user[10] / 2;
        z[11] = user[11] / 2;
        let _ = manager.keyboard.transition_colors_to(&z, 5, delay / 2);
        thread::sleep(Duration::from_millis(delay * 3));

        let _ = manager.keyboard.transition_colors_to(&[0; 12], 5, delay);
        thread::sleep(Duration::from_millis(delay * 10));
    }
}
