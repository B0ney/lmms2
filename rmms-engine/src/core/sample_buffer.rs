// const CHANNELS
use std::{
    collections::HashMap,
    fmt::Debug,
    path::PathBuf,
    sync::{Arc, Weak},
};

use parking_lot::RwLock;

use super::SampleCache;

pub struct Mixer {}

#[derive(Default, Clone)]
pub struct SampleBuffer {
    /// Raw sample data
    pub samples: Vec<Vec<f32>>,
    /// The file the sample came from
    pub path: Option<PathBuf>,
    /// The original sample rate
    pub sample_rate_original: u32,
    pub sample_rate_current: u32,
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
        FramesIter::new(&self.samples)
    }

    pub fn frame(&self, idx: usize) -> Option<Box<[f32]>> {
        let mut sample_frame = vec![0f32; self.channels()].into_boxed_slice();

        for channel in 0..self.channels() {
            sample_frame[channel] = *self.audio()[channel].get(idx)?;
        }

        Some(sample_frame)
    }

    /// what if there's more than 2 channels?
    pub fn to_stereo(mut self) -> Self {
        if self.channels() == 0 {
            return Self {
                samples: vec![Vec::new(); 2],
                ..self
            };
        }

        if self.channels() > 1 {
            return self;
        }

        let dupe = self.samples[0].clone();
        self.samples.push(dupe);

        self
    }

    pub fn make_mut<'a>(self: &'a mut Arc<Self>) -> &'a mut Self {
        Arc::make_mut(self)
    }

    pub fn reverse(&mut self) {}
}

impl Debug for SampleBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SampleBuffer")
            .field("channels", &self.channels())
            .field("frames", &self.frames())
            .field(
                "duration (secs)",
                &(self.frames() as f32 / self.sample_rate_original as f32),
            )
            .field("sample_rate_original", &self.sample_rate_original)
            .field("sample_rate_current", &self.sample_rate_current)
            .field("path", &self.path)
            .finish()
    }
}

struct FramesIter<'a> {
    samples: &'a [Vec<f32>],
    channels: usize,
    frame: usize,
    buffer: Box<[f32]>,
    reversed: bool,
    
}

impl<'a> FramesIter<'a> {
    pub fn new(samples: &'a [Vec<f32>]) -> Self {
        assert!(!samples.is_empty(), "must have at least 1 channel");
        Self {
            samples,
            channels: samples.len(),
            frame: 0,
            buffer: vec![0f32; samples.len()].into_boxed_slice(),
            reversed: false
        }
    }

    pub fn reverse(mut self) -> Self {
        self.reversed = !self.reversed;

        self.frame = match self.reversed {
            true => self.samples[0].len() - 1,
            false => 0,
        };
        
        self
    }

    pub fn override_channels(self, new_channels: usize) -> Self {
        Self {
            channels: std::cmp::min(new_channels, self.channels),
            ..self
        }
    }
}

impl<'a> Iterator for FramesIter<'a> {
    type Item = Box<[f32]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.frame == 0 && self.reversed {
            return None;
        }

        if self.frame >= self.samples[0].len() {
            return None;
        }

        let sample_frame = &mut self.buffer;

        for channel in 0..self.channels {
            sample_frame[channel] = self.samples[channel][self.frame]
        }

        match self.reversed {
            true => self.frame -= 1,
            false => self.frame += 1,
        }

        Some(sample_frame.clone())
    }
}

#[derive(Clone)]
pub enum AudioFrame {
    Mono([f32; 1]),
    Stereo([f32; 2]),
    Multi(Box<[f32]>),
}

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

//     pub fn from_slice(buf: &mut Box<[f32]>) -> Self {
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
//             _ => Self::Multi(buf.clone()),
//         }
//     }
// }
#[test]
fn a() {
    let mut sample = SampleBuffer::default();
    let cache = SampleCache::default();
}