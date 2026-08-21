use bevy::prelude::*;

#[derive(Debug, Component, Clone, Deref, DerefMut)]
struct FireDelay(pub Timer);

impl FireDelay {
    // #[allow(dead_code)]
    // pub fn from_seconds(seconds: f32) -> Self {
    //     Self::new(Timer::from_seconds(seconds, TimerMode::Repeating))
    // }
    //
    // #[allow(dead_code)]
    // pub fn from_duration(duration: Duration) -> Self {
    //     Self::new(Timer::new(duration, TimerMode::Repeating))
    // }
}
