use crate::enums::{Direction, Effects, Message};

use crossbeam_channel::{Receiver, Sender};
use effects::{
    battery, black_hole, bouncing, christmas, color_mutation, conscious_mode, dimension_shift, disco, dna, echo, entropy, fade, fire, forest, fractal_pulse, glitch, gravity_drop, gravity_well,
    heartbeat, heartbeat_sync, infection, lightning, magnetic, matrix, memory_trail, mic_sound, mirror_collapse, neural_pulse, ocean, ping_pong, police, predator, quantum_teleport, radar, randomizer,
    reverse_wave, rgb_storm, ripple, scanner, schrodinger, solar_flare, sound, stars, strobe, sunset, swipe, temperature, time_clock, time_rewind, twister, warning,
};
use error_stack::{Result, ResultExt};
use legion_rgb_driver::{BaseEffects, Keyboard, SPEED_RANGE};
use profile::Profile;
use rand::{rng, rngs::ThreadRng};
use single_instance::SingleInstance;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};
use std::{sync::Arc, thread::JoinHandle};
use thiserror::Error;

use self::custom_effect::{CustomEffect, EffectType, PlaybackMode};

pub mod custom_effect;
mod effects;
pub mod profile;

pub use effects::show_effect_ui;

#[derive(Debug, Error, PartialEq)]
#[error("Could not create keyboard manager")]
pub enum ManagerCreationError {
    #[error("There was an error getting a valid keyboard")]
    AcquireKeyboard,
    #[error("An instance of the program is already running")]
    InstanceAlreadyRunning,
}

/// Manager wrapper
pub struct EffectManager {
    pub tx: Sender<Message>,
    inner_handle: Option<JoinHandle<()>>,
    stop_signals: StopSignals,
}

/// Controls the keyboard lighting logic
struct Inner {
    keyboard: Keyboard,
    rx: Receiver<Message>,
    stop_signals: StopSignals,
    last_profile: Profile,
    // Can't drop this else it stops "reserving" whatever underlying implementation identifier it uses
    #[allow(dead_code)]
    single_instance: SingleInstance,
}

#[derive(Clone, Copy)]
pub enum OperationMode {
    Cli,
    Gui,
}

impl EffectManager {
    pub fn new(operation_mode: OperationMode) -> Result<Self, ManagerCreationError> {
        let stop_signals = StopSignals {
            manager_stop_signal: Arc::new(AtomicBool::new(false)),
            keyboard_stop_signal: Arc::new(AtomicBool::new(false)),
        };

        // Use the crate's name as the identifier, should be unique enough
        let single_instance = SingleInstance::new(env!("CARGO_PKG_NAME")).unwrap();

        if !single_instance.is_single() {
            return Err(ManagerCreationError::InstanceAlreadyRunning.into());
        }

        let keyboard = legion_rgb_driver::get_keyboard(stop_signals.keyboard_stop_signal.clone())
            .change_context(ManagerCreationError::AcquireKeyboard)
            .attach_printable("Ensure that you have a supported model and that the application has access to it.")
            .attach_printable("On Linux, you may need to configure additional permissions")
            .attach_printable("https://github.com/4JX/L5P-Keyboard-RGB#usage")?;

        let (tx, rx) = crossbeam_channel::unbounded::<Message>();

        let mut inner = Inner {
            keyboard,
            rx,
            stop_signals: stop_signals.clone(),
            last_profile: Profile::default(),
            single_instance,
        };

        macro_rules! effect_thread_loop {
            ($e: expr) => {
                thread::spawn(move || loop {
                    match $e {
                        Some(message) => match message {
                            Message::Profile { profile } => {
                                inner.set_profile(profile);
                            }
                            Message::CustomEffect { effect } => {
                                inner.custom_effect(&effect);
                            }
                            Message::Exit => break,
                        },
                        None => {
                            thread::sleep(Duration::from_millis(20));
                        }
                    }
                })
            };
        }

        let inner_handle = match operation_mode {
            OperationMode::Cli => effect_thread_loop!(inner.rx.try_recv().ok()),
            OperationMode::Gui => effect_thread_loop!(inner.rx.try_iter().last()),
        };

        let manager = Self {
            tx,
            inner_handle: Some(inner_handle),
            stop_signals,
        };

        Ok(manager)
    }

    pub fn set_profile(&mut self, profile: Profile) {
        self.stop_signals.store_true();
        self.tx.try_send(Message::Profile { profile }).unwrap();
    }

    pub fn custom_effect(&self, effect: CustomEffect) {
        self.stop_signals.store_true();
        self.tx.send(Message::CustomEffect { effect }).unwrap();
    }

    pub fn shutdown(mut self) {
        self.stop_signals.store_true();
        self.tx.send(Message::Exit).unwrap();
        if let Some(handle) = self.inner_handle.take() {
            handle.join().unwrap();
        };
    }
}

impl Inner {
    fn set_profile(&mut self, mut profile: Profile) {
        self.last_profile = profile.clone();
        self.stop_signals.store_false();
        let mut rng = rng();

        if profile.effect.is_built_in() {
            let clamped_speed = self.clamp_speed(profile.speed);
            self.keyboard.set_speed(clamped_speed).unwrap();
        } else {
            // All custom effects rely on rapidly switching a static color
            self.keyboard.set_effect(BaseEffects::Static).unwrap();
        }

        self.keyboard.set_brightness(profile.brightness as u8 + 1).unwrap();

        self.apply_effect(&mut profile, &mut rng);
        self.stop_signals.store_false();
    }

    fn clamp_speed(&self, speed: u8) -> u8 {
        speed.clamp(SPEED_RANGE.min().unwrap(), SPEED_RANGE.max().unwrap())
    }

    fn apply_effect(&mut self, profile: &mut Profile, rng: &mut ThreadRng) {
        match profile.effect {
            Effects::Static => {
                self.keyboard.set_colors_to(&profile.rgb_array()).unwrap();
                self.keyboard.set_effect(BaseEffects::Static).unwrap();
            }
            Effects::Breath => {
                self.keyboard.set_colors_to(&profile.rgb_array()).unwrap();
                self.keyboard.set_effect(BaseEffects::Breath).unwrap();
            }
            Effects::Smooth => {
                self.keyboard.set_effect(BaseEffects::Smooth).unwrap();
            }
            Effects::Wave => {
                let effect = match profile.direction {
                    Direction::Left => BaseEffects::LeftWave,
                    Direction::Right => BaseEffects::RightWave,
                };
                self.keyboard.set_effect(effect).unwrap();
            }
            Effects::Lightning => lightning::play(self, profile, rng),

            Effects::SmoothWave { mode, clean_with_black } => {
                profile.rgb_zones = profile::arr_to_zones([255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 0, 255]);
                swipe::play(self, profile, mode, clean_with_black);
            }
            Effects::Swipe { mode, clean_with_black } => swipe::play(self, profile, mode, clean_with_black),
            Effects::Disco => disco::play(self, profile, rng),
            Effects::Christmas => christmas::play(self, rng),
            Effects::Fade => fade::play(self, profile),
            Effects::Temperature => temperature::play(self),
            Effects::Ripple => ripple::play(self, profile),
            Effects::Sound => sound::play(self, profile),
            Effects::Police => police::play(self, profile),
            Effects::Radar => radar::play(self, profile),
            Effects::Heartbeat => heartbeat::play(self, profile),
            Effects::Fire => fire::play(self, profile),
            Effects::Strobe => strobe::play(self, profile),
            Effects::Stars => stars::play(self, profile),
            Effects::Warning => warning::play(self, profile),
            Effects::Ocean => ocean::play(self, profile),
            Effects::Forest => forest::play(self, profile),
            Effects::Bouncing => bouncing::play(self, profile),
            Effects::Randomizer => randomizer::play(self, profile),
            Effects::Sunset => sunset::play(self, profile),
            Effects::Matrix => matrix::play(self, profile),
            Effects::Battery => battery::play(self, profile),
            Effects::PingPong => ping_pong::play(self, profile),
            Effects::Scanner => scanner::play(self, profile),
            Effects::Twister => twister::play(self, profile),
            Effects::Schrodinger => schrodinger::play(self, profile),
            Effects::MemoryTrail => memory_trail::play(self, profile),
            Effects::Infection => infection::play(self, profile),
            Effects::GravityWell => gravity_well::play(self, profile),
            Effects::TimeRewind => time_rewind::play(self, profile),
            Effects::Echo => echo::play(self, profile),
            Effects::Dna => dna::play(self, profile),
            Effects::Predator => predator::play(self, profile),
            Effects::DimensionShift => dimension_shift::play(self, profile),
            Effects::ConsciousMode => conscious_mode::play(self, profile),
            Effects::GravityDrop => gravity_drop::play(self, profile),
            Effects::BlackHole => black_hole::play(self, profile),
            Effects::TimeClock => time_clock::play(self, profile),
            Effects::NeuralPulse => neural_pulse::play(self, profile),
            Effects::ColorMutation => color_mutation::play(self, profile),
            Effects::Glitch => glitch::play(self, profile),
            Effects::Magnetic => magnetic::play(self, profile),
            Effects::SolarFlare => solar_flare::play(self, profile),
            Effects::FractalPulse => fractal_pulse::play(self, profile),
            Effects::Entropy => entropy::play(self, profile),
            Effects::ReverseWave => reverse_wave::play(self, profile),
            Effects::RgbStorm => rgb_storm::play(self, profile),
            Effects::HeartbeatSync => heartbeat_sync::play(self, profile),
            Effects::MirrorCollapse => mirror_collapse::play(self, profile),
            Effects::QuantumTeleport => quantum_teleport::play(self, profile),
            Effects::MicSound => mic_sound::play(self, profile),
        }
    }

    fn custom_effect(&mut self, custom_effect: &CustomEffect) {
        self.stop_signals.store_false();
        let _ = self.keyboard.set_effect(BaseEffects::Static);
        let mut rng = rand::rng();

        if custom_effect.effect_steps.is_empty() {
            return;
        }

        let mut index = 0;
        let mut forward = true;
        let mut steps_played = 0;

        loop {
            let step = match custom_effect.playback_mode {
                PlaybackMode::Random => {
                    use rand::Rng;
                    let r = rng.random_range(0..custom_effect.effect_steps.len());
                    &custom_effect.effect_steps[r]
                }
                _ => &custom_effect.effect_steps[index],
            };

            // --- PLAY STEP ---
            let brightness = (step.brightness % 4) + 1;
            let _ = self.keyboard.set_brightness(brightness);
            match step.step_type {
                EffectType::Set => {
                    let _ = self.keyboard.set_colors_to(&step.rgb_array);
                }
                _ => {
                    let _ = self.keyboard.transition_colors_to(&step.rgb_array, step.steps, step.delay_between_steps);
                }
            }

            let mut remaining_ms = step.sleep;
            while remaining_ms > 0 {
                if self.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
                    return;
                }
                let wait = remaining_ms.min(10);
                thread::sleep(Duration::from_millis(wait));
                remaining_ms -= wait;
            }

            // --- CALCULATE NEXT INDEX ---
            steps_played += 1;
            match custom_effect.playback_mode {
                PlaybackMode::Repeat => {
                    index = (index + 1) % custom_effect.effect_steps.len();
                    if index == 0 && !custom_effect.should_loop {
                        break;
                    }
                }
                PlaybackMode::PingPong => {
                    let len = custom_effect.effect_steps.len();
                    if len <= 1 {
                        if !custom_effect.should_loop {
                            break;
                        }
                    } else if forward {
                        if index + 1 < len {
                            index += 1;
                        } else {
                            forward = false;
                            index -= 1;
                        }
                    } else {
                        if index > 0 {
                            index -= 1;
                        } else {
                            if !custom_effect.should_loop {
                                break;
                            }
                            forward = true;
                            index += 1;
                        }
                    }
                }
                PlaybackMode::Random => {
                    if steps_played >= custom_effect.effect_steps.len() && !custom_effect.should_loop {
                        break;
                    }
                }
            }

            if self.stop_signals.manager_stop_signal.load(Ordering::SeqCst) {
                break;
            }
        }
    }
}

impl Drop for EffectManager {
    fn drop(&mut self) {
        let _ = self.tx.send(Message::Exit);
    }
}

#[derive(Clone)]
pub struct StopSignals {
    pub manager_stop_signal: Arc<AtomicBool>,
    pub keyboard_stop_signal: Arc<AtomicBool>,
}

impl StopSignals {
    pub fn store_true(&self) {
        self.keyboard_stop_signal.store(true, Ordering::SeqCst);
        self.manager_stop_signal.store(true, Ordering::SeqCst);
    }
    pub fn store_false(&self) {
        self.keyboard_stop_signal.store(false, Ordering::SeqCst);
        self.manager_stop_signal.store(false, Ordering::SeqCst);
    }
}
