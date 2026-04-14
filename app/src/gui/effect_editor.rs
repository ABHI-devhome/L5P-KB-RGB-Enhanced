use eframe::egui::{Color32, ComboBox, CornerRadius, DragValue, Frame, Layout, RichText, ScrollArea, Ui};
use strum::IntoEnumIterator;

use crate::gui::{LoadedEffect, State};
use crate::manager::custom_effect::{EffectStep, EffectType, PlaybackMode};

pub fn show_effect_editor(ui: &mut Ui, loaded_effect: &mut LoadedEffect, changed: &mut bool) {
    ui.vertical(|ui| {
        // --- PRESETS & HELPERS ---
        ui.horizontal(|ui| {
            if ui.button("🗑 Clear").on_hover_text("Start from scratch").clicked() {
                loaded_effect.effect.effect_steps.clear();
                *changed = true;
            }

            if ui.button("🌊 Wave").on_hover_text("Auto-generate a 4-zone wave effect").clicked() {
                generate_wave_preset(&mut loaded_effect.effect.effect_steps);
                *changed = true;
            }

            if ui.button("🌈 Rainbow").on_hover_text("Auto-generate a color-shifting rainbow").clicked() {
                generate_rainbow_preset(&mut loaded_effect.effect.effect_steps);
                *changed = true;
            }

            if ui.button("🚨 Police").on_hover_text("Auto-generate Red/Blue flash").clicked() {
                generate_police_preset(&mut loaded_effect.effect.effect_steps);
                *changed = true;
            }

            ui.separator();

            ui.label("Loop:");
            ui.checkbox(&mut loaded_effect.effect.should_loop, "On");

            ui.add_space(10.0);

            ui.label("Mode:");
            ComboBox::from_id_salt("playback_mode")
                .selected_text(format!("{}", loaded_effect.effect.playback_mode))
                .show_ui(ui, |ui| {
                    for mode in PlaybackMode::iter() {
                        if ui.selectable_value(&mut loaded_effect.effect.playback_mode, mode.clone(), format!("{}", mode)).clicked() {
                            *changed = true;
                        }
                    }
                });
        });

        ui.separator();

        // --- GLOBAL SETTINGS ---
        ui.horizontal(|ui| {
            ui.label("Name:");
            let mut name = loaded_effect.effect.name.clone().unwrap_or_default();
            if ui.text_edit_singleline(&mut name).changed() {
                loaded_effect.effect.name = Some(name);
                *changed = true;
            }

            ui.add_space(20.0);

            let is_playing = loaded_effect.is_playing();
            if ui.button(if is_playing { "⏹ Stop Test" } else { "▶ Play Test" }).clicked() {
                if is_playing {
                    loaded_effect.state = State::None;
                } else {
                    loaded_effect.state = State::Queued;
                }
                *changed = true;
            }
        });

        ui.add_space(10.0);

        if ui.button("➕ Add New Step").clicked() {
            loaded_effect.effect.effect_steps.push(EffectStep {
                rgb_array: [0; 12],
                step_type: EffectType::Set,
                brightness: 255,
                steps: 20,
                delay_between_steps: 20,
                sleep: 500,
            });
            *changed = true;
        }

        ui.add_space(5.0);

        // --- SEQUENCE LIST ---
        Frame {
            corner_radius: CornerRadius::same(6),
            fill: Color32::from_gray(20),
            ..Frame::default()
        }
        .show(ui, |ui| {
            ui.set_height(ui.available_height());
            ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                if loaded_effect.effect.effect_steps.is_empty() {
                    ui.centered_and_justified(|ui| ui.label(RichText::new("Sequence is empty.\nClick a preset or 'Add New Step' to begin!").weak()));
                    return;
                }

                let mut to_remove = None;
                let mut to_move_up = None;
                let mut to_move_down = None;

                let steps_len = loaded_effect.effect.effect_steps.len();
                for (i, step) in loaded_effect.effect.effect_steps.iter_mut().enumerate() {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(format!("Step {}", i + 1)).strong().color(Color32::LIGHT_BLUE));

                            ui.with_layout(Layout::right_to_left(eframe::egui::Align::Center), |ui| {
                                if ui.button("❌").clicked() {
                                    to_remove = Some(i);
                                }
                                if i < steps_len - 1 && ui.button("▼").clicked() {
                                    to_move_down = Some(i);
                                }
                                if i > 0 && ui.button("▲").clicked() {
                                    to_move_up = Some(i);
                                }
                            });
                        });

                        ui.separator();

                        ui.horizontal(|ui| {
                            ui.label("Zones Colors:");
                            for z in 0..4 {
                                ui.vertical(|ui| {
                                    ui.label(RichText::new(format!("Z{}", z + 1)).small().weak());
                                    let mut rgb = [step.rgb_array[z * 3], step.rgb_array[z * 3 + 1], step.rgb_array[z * 3 + 2]];
                                    if ui.color_edit_button_srgb(&mut rgb).changed() {
                                        step.rgb_array[z * 3] = rgb[0];
                                        step.rgb_array[z * 3 + 1] = rgb[1];
                                        step.rgb_array[z * 3 + 2] = rgb[2];
                                        *changed = true;
                                    }
                                });
                            }
                        });

                        ui.add_space(5.0);

                        ui.horizontal(|ui| {
                            ui.label("Stay on these colors for (ms):");
                            if ui.add(DragValue::new(&mut step.sleep).suffix(" ms")).changed() {
                                *changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            let mut is_fade = matches!(step.step_type, EffectType::Transition);
                            if ui.checkbox(&mut is_fade, "Fade in smoothly from previous color?").changed() {
                                step.step_type = if is_fade { EffectType::Transition } else { EffectType::Set };
                                *changed = true;
                            }

                            if is_fade {
                                ui.label("Fade speed:");
                                if ui.add(DragValue::new(&mut step.delay_between_steps).suffix(" ms speed")).changed() {
                                    *changed = true;
                                }
                            }
                        });
                    });
                    ui.add_space(5.0);
                }

                if let Some(i) = to_remove {
                    loaded_effect.effect.effect_steps.remove(i);
                    *changed = true;
                }
                if let Some(i) = to_move_up {
                    loaded_effect.effect.effect_steps.swap(i, i - 1);
                    *changed = true;
                }
                if let Some(i) = to_move_down {
                    loaded_effect.effect.effect_steps.swap(i, i + 1);
                    *changed = true;
                }
            });
        });
    });
}

fn generate_wave_preset(steps: &mut Vec<EffectStep>) {
    steps.clear();
    for z in 0..4 {
        let mut rgb_array = [0u8; 12];
        rgb_array[z * 3] = 255;
        steps.push(EffectStep {
            rgb_array,
            step_type: EffectType::Transition,
            brightness: 255,
            steps: 15,
            delay_between_steps: 10,
            sleep: 50,
        });
    }
}

fn generate_rainbow_preset(steps: &mut Vec<EffectStep>) {
    steps.clear();
    let colors = [[255, 0, 0], [255, 127, 0], [255, 255, 0], [0, 255, 0], [0, 0, 255], [75, 0, 130], [148, 0, 211]];
    for color in colors {
        let mut rgb_array = [0u8; 12];
        for z in 0..4 {
            rgb_array[z * 3] = color[0];
            rgb_array[z * 3 + 1] = color[1];
            rgb_array[z * 3 + 2] = color[2];
        }
        steps.push(EffectStep {
            rgb_array,
            step_type: EffectType::Transition,
            brightness: 255,
            steps: 30,
            delay_between_steps: 20,
            sleep: 200,
        });
    }
}

fn generate_police_preset(steps: &mut Vec<EffectStep>) {
    steps.clear();
    // Step 1: Zones 1&2 Red, 3&4 Blue
    steps.push(EffectStep {
        rgb_array: [255, 0, 0, 255, 0, 0, 0, 0, 255, 0, 0, 255],
        step_type: EffectType::Set,
        brightness: 255,
        steps: 1,
        delay_between_steps: 1,
        sleep: 150,
    });
    // Step 2: Zones 1&2 Blue, 3&4 Red
    steps.push(EffectStep {
        rgb_array: [0, 0, 255, 0, 0, 255, 255, 0, 0, 255, 0, 0],
        step_type: EffectType::Set,
        brightness: 255,
        steps: 1,
        delay_between_steps: 1,
        sleep: 150,
    });
}
