use default_ui::{show_brightness, show_direction, show_effect_settings};
use eframe::egui::{self, ComboBox};
use strum::IntoEnumIterator;

use crate::{
    enums::{Effects, SwipeMode},
    manager::profile::Profile,
};

pub mod battery;
pub mod black_hole;
pub mod bouncing;
pub mod christmas;
pub mod color_mutation;
pub mod conscious_mode;
pub mod default_ui;
pub mod dimension_shift;
pub mod disco;
pub mod dna;
pub mod echo;
pub mod entropy;
pub mod fade;
pub mod fire;
pub mod forest;
pub mod fractal_pulse;
pub mod glitch;
pub mod gravity_drop;
pub mod gravity_well;
pub mod heartbeat;
pub mod heartbeat_sync;
pub mod infection;
pub mod lightning;
pub mod magnetic;
pub mod matrix;
pub mod memory_trail;
pub mod mic_sound;
pub mod mirror_collapse;
pub mod neural_pulse;
pub mod ocean;
pub mod ping_pong;
pub mod police;
pub mod predator;
pub mod quantum_teleport;
pub mod radar;
pub mod randomizer;
pub mod reverse_wave;
pub mod rgb_storm;
pub mod ripple;
pub mod scanner;
pub mod schrodinger;
pub mod solar_flare;
pub mod sound;
pub mod stars;
pub mod strobe;
pub mod sunset;
pub mod swipe;
pub mod temperature;
pub mod time_clock;
pub mod time_rewind;
pub mod twister;
pub mod warning;
pub mod zones;

pub fn show_effect_ui(ui: &mut egui::Ui, profile: &mut Profile, update_lights: &mut bool, theme: &crate::gui::style::Theme) {
    let mut effect = profile.effect;

    match &mut effect {
        Effects::SmoothWave { mode, clean_with_black } | Effects::Swipe { mode, clean_with_black } => {
            ui.scope(|ui| {
                ui.style_mut().spacing.item_spacing = theme.spacing.default;

                show_brightness(ui, profile, update_lights);
                show_direction(ui, profile, update_lights);
                show_effect_settings(ui, profile, update_lights);
                ComboBox::from_label("Swipe mode").width(30.0).selected_text(format!("{:?}", mode)).show_ui(ui, |ui| {
                    for swipe_mode in SwipeMode::iter() {
                        *update_lights |= ui.selectable_value(mode, swipe_mode, format!("{:?}", swipe_mode)).changed();
                    }
                });
                *update_lights |= ui.add_enabled(matches!(mode, SwipeMode::Fill), egui::Checkbox::new(clean_with_black, "Clean with black")).changed();
            });
        }

        _ => {
            default_ui::show(ui, profile, update_lights, &theme.spacing);
        }
    }

    profile.effect = effect;
}
