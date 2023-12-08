use rubato::Resampler as SyncResampler;

use super::PlayHandle;

/// Generic resampler for playhandles
pub struct Resampler {
    resampler: rubato::SincFixedIn<f32>,
    handle: Box<dyn PlayHandle>,
    input: Vec<Vec<f32>>,
    output: Vec<Vec<f32>>,
    frame: usize,
}

impl Resampler {
    pub fn new(handle: impl PlayHandle, source_rate: u32, target_rate: u32) -> Self {
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
            ).unwrap(),
            handle: Box::new(handle),
            input: Vec::new(),
            output: Vec::new(),
            frame: 0,
        }
    }
}

impl PlayHandle for Resampler {
    fn is_complete(&self) -> bool {
        self.handle.is_complete()
    }

    fn next(&mut self) -> Option<[f32; 2]> {
        // read frames from playhandle into buffer

        // self.handle.render(frames);


        self.resampler.process_into_buffer(
            &mut self.input, 
            &mut self.output, 
            None
        );
        todo!()
    }
}
