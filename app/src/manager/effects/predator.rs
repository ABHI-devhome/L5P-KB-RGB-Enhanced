use crate::manager::{profile::Profile, Inner};
use rand::Rng;
use std::sync::atomic::Ordering;
use std::{thread, time::Duration};

pub fn play(manager: &mut Inner, profile: &Profile) {
    let delay = match profile.speed {
        1 => 200,
        2 => 150,
        3 => 100,
        4 => 50,
        _ => 100,
    };
    let mut rng = rand::rng();
    let mut hunter = 0;
    let mut prey = 3;

    while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
        let mut z = [0; 12];
        z[prey * 3] = 0;
        z[prey * 3 + 1] = 255;
        z[prey * 3 + 2] = 0; // Prey Green
        z[hunter * 3] = 255;
        z[hunter * 3 + 1] = 0;
        z[hunter * 3 + 2] = 0; // Hunter Red

        if hunter == prey {
            z[hunter * 3] = 255;
            z[hunter * 3 + 1] = 255;
            z[hunter * 3 + 2] = 255; // flash white death
        }

        let _ = manager.keyboard.set_colors_to(&z);
        thread::sleep(Duration::from_millis(delay));

        if hunter == prey {
            prey = rng.random_range(0..4);
            while hunter == prey {
                prey = rng.random_range(0..4);
            } // new prey location
        } else {
            // hunter moves towards prey
            if hunter < prey {
                hunter += 1;
            } else {
                hunter -= 1;
            }

            // prey occasionally panics and jumps
            if rng.random_bool(0.3) {
                if prey < 3 && prey > hunter {
                    prey += 1;
                } else if prey > 0 && prey < hunter {
                    prey -= 1;
                }
            }
        }
    }
}
