use crate::core::sample::Sample;
use super::PlayHandle;

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