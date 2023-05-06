use std::sync::Arc;

use crate::core::adsr::ADSR;
use crate::core::sample::Sample;
use crate::core::{dsp, };
use crate::core::engine::AudioInputDevice;
use crate::core::{SampleBuffer, SampleCache};

/// I think "SuperAudioFileProcessor" is a cooler name.
///
/// Or what about "SuperAudioSampler"? SAS sounds nice on the tongue.
///
/// "AudioSuperSampler"...
#[derive(Default)]
pub struct AudioFileProcessor2 {
    pub sample: Option<Sample>,
    /// can record audio.
    pub input_device: Option<Arc<dyn AudioInputDevice>>,

    pub loop_point: Point,
    pub loop_type: LoopType,
    pub playback_point: Point,

    pub panning: f32,
    pub pitch: f32,
    pub volume: f32,
}

impl AudioFileProcessor2 {
    /// in practice, reversing a sample won't require making a copy of the sample.
    ///
    fn test(&mut self) {
        let Some(sample) = self.sample_mut() else {
            return;
        };

        dsp::reverse(sample)
    }

    fn sample_mut(&mut self) -> Option<&mut SampleBuffer> {
        match &mut self.sample {
            Some(sample) => Some(sample.sample_mut()),
            None => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub start: f32,
    pub end: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum LoopType {
    #[default]
    Off,
    Forward,
    Backward,
    PingPong,
}

#[derive(Debug, Clone, Copy)]
enum Interpolation {
    None,
    Linear,
}
