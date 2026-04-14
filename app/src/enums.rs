use crate::manager::{custom_effect::CustomEffect, profile::Profile};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString, IntoStaticStr};

#[derive(Clone, Copy, EnumString, Serialize, Deserialize, Display, EnumIter, Debug, IntoStaticStr, Default)]
pub enum Effects {
    #[default]
    Static,
    Breath,
    Smooth,
    Wave,
    Lightning,

    SmoothWave {
        mode: SwipeMode,
        clean_with_black: bool,
    },
    Swipe {
        mode: SwipeMode,
        clean_with_black: bool,
    },
    Disco,
    Christmas,
    Fade,
    Temperature,
    Ripple,
    Sound,
    Police,
    Radar,
    Heartbeat,
    Fire,
    Strobe,
    Stars,
    Warning,
    Ocean,
    Forest,
    Bouncing,
    Randomizer,
    Sunset,
    Matrix,
    Battery,
    PingPong,
    Scanner,
    Twister,
    Schrodinger,
    MemoryTrail,
    Infection,
    GravityWell,
    TimeRewind,
    Echo,
    Dna,
    Predator,
    DimensionShift,
    ConsciousMode,
    GravityDrop,
    BlackHole,
    TimeClock,
    NeuralPulse,
    ColorMutation,
    Glitch,
    Magnetic,
    SolarFlare,
    FractalPulse,
    Entropy,
    ReverseWave,
    RgbStorm,
    HeartbeatSync,
    MirrorCollapse,
    QuantumTeleport,
    MicSound,
}

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, EnumIter, EnumString, PartialEq)]
pub enum SwipeMode {
    #[default]
    Change,
    Fill,
}

impl PartialEq for Effects {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

#[allow(dead_code)]
impl Effects {
    pub fn takes_color_array(self) -> bool {
        matches!(
            self,
            Self::Static | Self::Breath | Self::Lightning | Self::Swipe { .. } | Self::Fade | Self::Ripple | Self::Radar | Self::Heartbeat | Self::Strobe | Self::Bouncing | Self::Sound
        )
    }

    pub fn takes_direction(self) -> bool {
        matches!(self, Self::Wave | Self::SmoothWave { .. } | Self::Swipe { .. })
    }

    pub fn takes_speed(self) -> bool {
        matches!(
            self,
            Self::Breath
                | Self::Smooth
                | Self::Wave
                | Self::Lightning
                | Self::SmoothWave { .. }
                | Self::Swipe { .. }
                | Self::Disco
                | Self::Fade
                | Self::Ripple
                | Self::Police
                | Self::Radar
                | Self::Heartbeat
                | Self::Fire
                | Self::Strobe
                | Self::Stars
                | Self::Warning
                | Self::Ocean
                | Self::Forest
                | Self::Bouncing
                | Self::Randomizer
                | Self::Sunset
                | Self::Matrix
                | Self::Battery
                | Self::PingPong
                | Self::Scanner
                | Self::Twister
                | Self::Schrodinger
                | Self::MemoryTrail
                | Self::Infection
                | Self::GravityWell
                | Self::TimeRewind
                | Self::Echo
                | Self::Dna
                | Self::Predator
                | Self::DimensionShift
                | Self::ConsciousMode
                | Self::GravityDrop
                | Self::BlackHole
                | Self::TimeClock
                | Self::NeuralPulse
                | Self::ColorMutation
                | Self::Glitch
                | Self::Magnetic
                | Self::SolarFlare
                | Self::FractalPulse
                | Self::Entropy
                | Self::ReverseWave
                | Self::RgbStorm
                | Self::HeartbeatSync
                | Self::MirrorCollapse
                | Self::QuantumTeleport
                | Self::MicSound
        )
    }

    pub fn is_built_in(self) -> bool {
        matches!(self, Self::Static | Self::Breath | Self::Smooth | Self::Wave)
    }
}

#[derive(Clone, Copy, EnumString, Serialize, Deserialize, Debug, EnumIter, IntoStaticStr, PartialEq, Eq, Default)]
pub enum Direction {
    #[default]
    Left,
    Right,
}

#[derive(PartialEq, Eq, EnumIter, IntoStaticStr, Clone, Copy, Default, Serialize, Deserialize, Debug, Display, EnumString)]
pub enum Brightness {
    #[default]
    Low,
    High,
}

#[derive(Debug)]
pub enum Message {
    CustomEffect { effect: CustomEffect },
    Profile { profile: Profile },
    Exit,
}
