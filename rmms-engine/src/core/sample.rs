use std::sync::Arc;

use super::{engine::Frame, SampleBuffer, SampleCache, traits::PlayHandle};

#[derive(Clone)]
pub struct Sample {
    /// Arc can provide ``clone-on-write`` with ``make_mut``.
    ///
    /// Do note that the modified sample needs to be cached
    buffer: Arc<SampleBuffer>,

    is_cached: bool,
    is_reversed: bool,
    start: usize,
    end: usize,
    current_frame: usize,
}

impl Sample {
    pub fn new(buffer: Arc<SampleBuffer>) -> Self {
        let end = buffer.frames();
        Self {
            buffer,
            is_cached: false,
            is_reversed: true,
            start: 0,
            end,
            current_frame: 0,
        }
    }

    fn len(&self) -> usize {
        self.end - self.start
    }

    fn play(&mut self, out_buffer: &mut [Frame], frames: usize) {
        let offset = self.start + self.current_frame;
        let frames_left = frames.saturating_sub(self.current_frame);

        let end = std::cmp::min(self.buffer.frames(), offset + frames_left);

        if offset >= self.buffer.frames() {
            return;
        }

        // let end =
        // let total_frames = std::cmp::min()
        let mut frames_written = 0;

        let range = match self.is_reversed {
            true => end..offset,
            false => offset..end,
        };

        for i in range {
            out_buffer[..1][0] = self.buffer.frame(i).map(|f| [f[0], f[1]]).unwrap();
            frames_written += 1;
        }

        self.current_frame += frames_written;
        // let Some(frame) = self.buffer.frame(self.current) else {
        //     return
        // }
    }

    pub fn sample_mut(&mut self) -> &mut SampleBuffer {
        self.is_cached = false;
        self.buffer.make_mut()
    }

    pub fn from_sample_cache(cache: &SampleCache, key: &String) -> Option<Self> {
        Some(Self {
            buffer: cache.get(key)?,
            ..todo!()
        })
    }
}

impl PlayHandle for Sample {
    fn next(&mut self) -> Option<[f32; 2]> {
        let offset = self.start + self.current_frame;
        // let frames_left = frames.saturating_sub(self.current_frame);

        // let end = std::cmp::min(self.buffer.frames(), offset + frames_left);

        if offset >= self.buffer.frames() {
            return None;
        };

        let result = self.buffer.frame(offset).map(|f| [f[0], f[1]]);
        self.current_frame += 1;

        result

    }

    fn reset(&mut self) {
        todo!()
    }

    fn jump(&mut self, tick: usize) {
        todo!()
    }
}