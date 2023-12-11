use rubato::Resampler as SyncResampler;

use super::PlayHandle;

/// Generic resampler for playhandles
pub struct Resampler<P: PlayHandle> {
    handle: P,
    resampler: rubato::SincFixedIn<f32>,
    input: Vec<Vec<f32>>,
    output: Vec<Vec<f32>>,
    frame: usize,
}

impl<P: PlayHandle> Resampler<P> {
    pub fn new(handle: P, source_rate: u32, target_rate: u32) -> Self {
        let ratio = source_rate as f64 / target_rate as f64;

        Self {
            resampler: rubato::SincFixedIn::<f32>::new(
                ratio,
                ratio * 5.0,
                rubato::InterpolationParameters {
                    sinc_len: 256,
                    f_cutoff: 0.95,
                    oversampling_factor: 128,
                    interpolation: rubato::InterpolationType::Linear,
                    window: rubato::WindowFunction::Blackman,
                },
                256,
                2,
            )
            .unwrap(),
            handle,
            input: Vec::new(),
            output: Vec::new(),
            frame: 0,
        }
    }
}

impl<P: PlayHandle> PlayHandle for Resampler<P> {
    fn is_complete(&self) -> bool {
        // TODO
        self.handle.is_complete()
    }

    fn next(&mut self) -> Option<[f32; 2]> {
        if self.is_complete() {
            return None;
        } 

        let mut frame = [[0.0, 0.0]];
        self.render(&mut frame);
        Some(frame[0])
    }

    fn render(&mut self, frames: &mut [[f32; 2]]) -> usize {
        self.resampler.process_into_buffer(
            &mut self.input,
            &mut self.output,
            None
        );

        todo!()
    }
}
