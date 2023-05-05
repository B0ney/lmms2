// const CHANNELS

pub mod cpal;
pub mod decoder;
pub mod engine;
pub mod playback;
pub mod dsp;
pub mod adsr;
pub mod note;
pub mod midi;

use std::{
    collections::HashMap,
    fmt::Debug,
    path::PathBuf,
    sync::{Arc, Weak},
};

use parking_lot::RwLock;

// type Frame

#[derive(Default)]
pub struct SampleCache {
    cache: RwLock<HashMap<String, Weak<SampleBuffer>>>,
}

impl SampleCache {
    pub fn get(&self, id: &String) -> Option<Arc<SampleBuffer>> {
        self.cache.read().get(id)?.upgrade()
    }

    #[must_use = "Cache will be immediately invalidated as this is the only owning reference."]
    pub fn add(&self, id: String, sample: SampleBuffer) -> Arc<SampleBuffer> {
        let sample = Arc::new(sample);

        self.cache.write().insert(id, Arc::downgrade(&sample));

        sample
    }
}

pub struct Mixer {}

#[derive(Default, Clone)]
pub struct SampleBuffer {
    pub samples: Vec<Vec<f32>>,
    path: Option<PathBuf>,
    sample_rate_original: u32,
    sample_rate_current: u32,
}

impl SampleBuffer {
    pub fn new(buffer: Vec<Vec<f32>>, rate: u32) -> Self {
        Self {
            samples: buffer,
            sample_rate_original: rate,
            ..Default::default()
        }
    }
    pub fn audio(&self) -> &[Vec<f32>] {
        &self.samples
    }

    pub fn audio_mut(&mut self) {
        // &self
    }

    pub fn channels(&self) -> usize {
        self.samples.len()
    }

    /// assume channel data is the same length
    pub fn frames(&self) -> usize {
        let Some(buffer) = self.samples.get(0) else {
            return 0;
        };
        buffer.len()
    }

    pub fn iter_frames(&self) -> impl Iterator<Item = Box<[f32]>> + '_ {
        (0..self.frames()).map(|f| {
            let mut sample_frame = Vec::with_capacity(self.channels());

            for channel in self.audio() {
                sample_frame.push(channel[f])
            }

            sample_frame.into_boxed_slice()
        })
    }

    pub fn frame(&self, idx: usize) -> Option<Box<[f32]>> {
        let mut sample_frame = Vec::with_capacity(self.channels());

        for channel in self.audio() {
            sample_frame.push(*channel.get(idx)?)
        }

        Some(sample_frame.into_boxed_slice())
    }

    /// what if there's more than 2 channels?
    pub fn to_stereo(mut self) -> Self {
        if self.channels() == 0 {
            return Self {
                samples: vec![Vec::new(); 2],
                ..self
            };
        }

        if self.channels() > 1{
            return self;
        }

        let dupe = self.samples[0].clone();
        self.samples.push(dupe);

        self

    }

    pub fn reverse(&mut self) {

    }
}

impl Debug for SampleBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SampleBuffer")
            .field("channels", &self.channels())
            .field("frames", &self.frames())
            .field("duration (secs)", &(self.frames() as f32 / self.sample_rate_original as f32))
            .field("sample_rate_original", &self.sample_rate_original)
            .field("sample_rate_current", &self.sample_rate_current)
            .field("path", &self.path)
            .finish()
    }
}

pub struct Sample {
    buffer: Arc<SampleBuffer>,
}

// #[derive(Clone)]
// pub enum AudioFrame {
//     Mono([f32; 1]),
//     Stereo([f32; 2]),
//     Multi(Box<[f32]>),
// }




// impl Default for AudioFrame {
//     fn default() -> Self {
//         AudioFrame::Stereo([0.0; 2])
//     }
// }

// impl AsRef<[f32]> for AudioFrame {
//     fn as_ref(&self) -> &[f32] {
//         match self {
//             Self::Mono(array) => array,
//             Self::Stereo(array) => array,
//             Self::Multi(heap) => heap,
//         }
//     }
// }

// impl AudioFrame {
//     pub fn as_mut(&mut self) -> &mut [f32] {
//         match self {
//             Self::Mono(array) => array,
//             Self::Stereo(array) => array,
//             Self::Multi(heap) => heap,
//         }
//     }

//     pub fn new(channels: usize) -> Self {
//         // assert!(channels <)
//         match channels {
//             1 => Self::Mono([0.0; 1]),
//             2 => Self::Stereo([0.0; 2]),
//             multi => Self::Multi(vec![0.0f32; multi].into_boxed_slice()),
//         }
//     }

//     pub fn from_slice(buf: &[f32]) -> Self {
//         assert!(!buf.is_empty(), "slice cannot be empty");

//         match buf.len() {
//             1 => {
//                 let mut dest = [0.0_f32];
//                 dest.copy_from_slice(&buf[..1]);
//                 Self::Mono(dest)
//             }
//             2 => {
//                 let mut dest = [0.0_f32; 2];
//                 dest.copy_from_slice(&buf[..2]);
//                 Self::Stereo(dest)
//             }
//             _ => Self::Multi(buf.to_vec().into_boxed_slice()),
//         }
//     }
// }


#[test]
fn a() {
    let mut sample = SampleBuffer::default();
    let cache = SampleCache::default();
}
