use std::sync::atomic::Ordering;
use std::{thread, time::Duration};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::manager::{profile::Profile, Inner};

pub fn play(manager: &mut Inner, profile: &Profile) {
    let host = cpal::default_host();
    let audio_device = match host.default_output_device() {
        Some(device) => device,
        None => {
            eprintln!("No default audio output device found.");
            while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(500));
            }
            return;
        }
    };

    let config = match audio_device.default_output_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to get default output config: {}", e);
            while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(500));
            }
            return;
        }
    };

    let (sender, receiver): (Sender<Vec<f32>>, Receiver<Vec<f32>>) = unbounded();

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => audio_device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &_| {
                let _ = sender.send(data.to_vec());
            },
            |err| eprintln!("An error occurred on the audio stream: {}", err),
            None,
        ),
        _ => {
            eprintln!("Unsupported sample format. Only F32 is currently supported by this effect.");
            while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(500));
            }
            return;
        }
    };

    if let Ok(s) = stream {
        s.play().unwrap();

        while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
            if let Ok(samples) = receiver.recv_timeout(Duration::from_millis(100)) {
                let mut nsamples = 0_i64;
                let mut sum_l = 0_i64;
                let mut sum_r = 0_i64;

                for (n, sample) in samples.iter().enumerate() {
                    nsamples += 1;
                    let i64sample = (*sample * 32768.0) as i64; // roughly to 16-bit range for matching formula

                    if n & 1 == 0 {
                        sum_l += i64sample * i64sample;
                    } else {
                        sum_r += i64sample * i64sample;
                    }
                }

                if nsamples > 0 {
                    let per_channel_samples = (nsamples / 2).max(1);

                    let rms_l = (((sum_l / per_channel_samples) as f64).sqrt()) * 255.0 / 16384.0;
                    let rms_r = (((sum_r / per_channel_samples) as f64).sqrt()) * 255.0 / 16384.0;

                    // Boost the signal slightly so it's more visible on soft music
                    let average_rms = (((rms_l + rms_r) / 2.0) * 1.5).clamp(0.0, 255.0) as u8;

                    let mut custom_colors = profile.rgb_array();
                    let fade = average_rms as f32 / 255.0;
                    for i in 0..12 {
                        custom_colors[i] = (custom_colors[i] as f32 * fade) as u8;
                    }
                    let _ = manager.keyboard.set_colors_to(&custom_colors);
                }
            } else {
                // Dim to nothing if no audio is caught in timeout
                let _ = manager.keyboard.set_colors_to(&[0; 12]);
            }
        }
    } else {
        eprintln!("Failed to build audio input stream.");
        while !manager.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(500));
        }
    }
}
