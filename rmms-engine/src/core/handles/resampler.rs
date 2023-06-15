use std::f32::consts::PI;

use crate::core::{traits::PlayHandle, SampleBuffer};

pub struct Resampler<H: PlayHandle>
{
    handle: H,
    buffer: Vec<[f32; 2]>
}

#[derive(Clone)]
pub struct Panner<H: PlayHandle> {
    handle: H,
    panning: f32,
}

impl <H: PlayHandle>Panner<H> {
    pub fn new(handle: H, mut panning: f32) -> Self {
        panning = panning.clamp(-1.0, 1.0);
        Self { handle, panning }
    }
}

impl <H: PlayHandle>PlayHandle for Panner<H> {
    fn next(&mut self) -> Option<[f32; 2]> {
        let mut frame = self.handle.next()?;

        let (l,r) = calculate_ratio(self.panning);

        frame[0] *= l;
        frame[1] *= r;

        Some(frame)
    }

    fn reset(&mut self) {
        self.handle.reset()
    }

    fn jump(&mut self, tick: usize) {
        self.handle.jump(tick)
    }
}

fn calculate_ratio(panning: f32) -> (f32, f32) {
    let phi = (panning + 1.0) * 0.25 * PI;
    let left = phi.cos();
    let right = phi.sin();

    (left, right)
}

