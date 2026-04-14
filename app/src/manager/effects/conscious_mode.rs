use crate::manager::{profile::Profile, Inner};
use rand::Rng;
use std::mem;
use std::sync::atomic::Ordering;
use std::{thread, time::Duration};
use winapi::um::sysinfoapi::GetTickCount;
use winapi::um::winuser::{GetLastInputInfo, LASTINPUTINFO};

fn get_idle_time_ms() -> u32 {
    unsafe {
        let mut lii: LASTINPUTINFO = mem::zeroed();
        lii.cbSize = mem::size_of::<LASTINPUTINFO>() as u32;
        if GetLastInputInfo(&mut lii) != 0 {
            GetTickCount() - lii.dwTime
        } else {
            0
        }
    }
}

pub fn play(manager: &mut Inner, profile: &Profile) {
    let delay = match profile.speed {
        1 => 100,
        2 => 50,
        3 => 30,
        4 => 15,
        _ => 30,
    } as u64;
    let mut rng = rand::rng();
    let mut breath_val: f32 = 0.0;

    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let mut user = profile.rgb_array();
        if user.iter().all(|&x| x == 0) {
            for i in 0..4 {
                user[i * 3] = 0;
                user[i * 3 + 1] = 255;
                user[i * 3 + 2] = 200;
            }
        }

        let idle_ms = get_idle_time_ms();

        if idle_ms < 5000 {
            // Typing active: Energetic flashes
            let mut z = [0; 12];
            for i in 0..4 {
                if rng.random_bool(0.5) {
                    z[i * 3] = user[i * 3];
                    z[i * 3 + 1] = user[i * 3 + 1];
                    z[i * 3 + 2] = user[i * 3 + 2];
                }
            }
            let _ = manager.keyboard.set_colors_to(&z);
            thread::sleep(Duration::from_millis(delay * 3));
        } else if idle_ms < 20000 {
            // Idle 5s-20s: Calm breathing
            breath_val += 0.05;
            let scale = (breath_val.sin() + 1.0) / 2.0; // 0.0 to 1.0

            let mut z = [0; 12];
            for i in 0..12 {
                z[i] = (user[i] as f32 * scale) as u8;
            }
            let _ = manager.keyboard.set_colors_to(&z);
            thread::sleep(Duration::from_millis(50));
        } else {
            // Long idle: Slow dim glow
            let mut z = [0; 12];
            for i in 0..12 {
                z[i] = (user[i] as f32 * 0.1) as u8;
            }
            let _ = manager.keyboard.set_colors_to(&z);
            thread::sleep(Duration::from_millis(500));
        }
    }
}
