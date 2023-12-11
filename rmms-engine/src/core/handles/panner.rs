use std::f32::consts::PI;

use crate::core::handles::PlayHandle;

pub struct Panner<P: PlayHandle> {
    handle: P,
    panning: f32,
    ratio: (f32, f32),
}

impl <P: PlayHandle>Panner<P> {
    pub fn new(handle: P, mut panning: f32) -> Self {
        panning = panning.clamp(-1.0, 1.0);
        Self {
            handle,
            panning,
            ratio: calculate_ratio(panning),
        }
    }

    pub fn update_panning(&mut self, panning: f32) {
        let (l, r) = calculate_ratio(self.panning);
        self.panning = panning;
        self.ratio = (l, r);
    }
}

impl <P: PlayHandle>PlayHandle for Panner <P> {
    fn next(&mut self) -> Option<[f32; 2]> {
        let mut frame = self.handle.next()?;

        let (l, r) = self.ratio;

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
