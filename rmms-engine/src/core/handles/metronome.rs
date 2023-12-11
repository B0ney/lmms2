use super::PlayHandle;
use crate::core::sample::Sample;

use std::time::{Duration, Instant};

/// Metronome playhandle.
///
/// Intended to live as long as the engine
#[derive(Clone)]
pub struct Metronome {
    is_active: bool,
    timesig: (u8, u8),
    tick: Sample,
    tock: Sample,
    timer: Time,
}

impl Metronome {
    pub fn new(numerator: u8, denominator: u8) -> Self {
        assert_ne!(numerator, 0, "Time signature numerator cannot be zero");
        assert_ne!(denominator, 0, "Time signature denominator cannot be zero");

        todo!()
    }

    pub fn custom_sound(&mut self, tick: Sample, tock: Sample) {
        self.tick = tick;
        self.tock = tock;
    }
}

impl PlayHandle for Metronome {
    fn is_complete(&self) -> bool {
        false
    }

    fn next(&mut self) -> Option<[f32; 2]> {
        if !self.is_active {
            return Some([0.0, 0.0]);
        }

        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Time {
    start: Instant,
    duration: Duration,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            start: Instant::now(),
            duration: Default::default(),
        }
    }
}

impl Time {
    pub fn start(&mut self) {
        self.start = Instant::now();
    }

    pub fn stop(&mut self) {
        self.duration = self.start.elapsed();
    }

    pub fn elapsed(&self) -> u128 {
        self.duration.as_micros()
    }

    pub fn init() -> Self {
        Self::default()
    }
}
