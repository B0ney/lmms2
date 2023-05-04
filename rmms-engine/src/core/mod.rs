// const CHANNELS

pub mod cpal;
pub mod decoder;
pub mod engine;
pub mod playback;

use std::{
    collections::HashMap,
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

#[derive(Default)]
pub struct SampleBuffer {
    samples: Vec<Vec<f32>>,
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

    pub fn audio_mut(&mut self) {}

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
}

pub struct Sample {
    buffer: Arc<SampleBuffer>,
}

#[test]
fn a() {
    let mut sample = SampleBuffer::default();
    let cache = SampleCache::default();
}
