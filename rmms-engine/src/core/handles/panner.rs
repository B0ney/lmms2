use std::f32::consts::PI;

use crate::core::{handles::PlayHandle, SampleBuffer};

pub struct Panner {
    handle: Box<dyn PlayHandle>,
    panning: f32,
    // ratio: (f32, f32),
}

impl Panner {
    pub fn new(handle: impl PlayHandle, mut panning: f32) -> Self {
        panning = panning.clamp(-1.0, 1.0);
        Self {
            handle: Box::new(handle),
            panning,
        }
    }
}

impl PlayHandle for Panner {
    fn next(&mut self) -> Option<[f32; 2]> {
        let mut frame = self.handle.next()?;

        let (l, r) = calculate_ratio(self.panning);

        frame[0] *= l;
        frame[1] *= r;

        Some(frame)
    }

    fn is_complete(&self) -> bool {
        self.handle.is_complete()
    }
}

fn calculate_ratio(panning: f32) -> (f32, f32) {
    let phi = (panning + 1.0) * 0.25 * PI;
    let left = phi.cos();
    let right = phi.sin();

    (left, right)
}
