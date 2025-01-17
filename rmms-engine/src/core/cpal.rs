use std::sync::Arc;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Sample;
use rb::{RbConsumer, RbProducer, RB};

use crate::core::SampleBuffer;
use std::sync::mpsc::{self, Sender};

use super::engine::{Frame, EngineHandle};
use super::traits::AudioOutputDevice;

enum Event {
    CurrentFrame(usize),
    Finished,
}

/// The audio engine must stream audio to 
/// 
pub struct CpalOutputDevice {
    _device: cpal::Device,
    _stream: cpal::Stream,
    /// sample rate of the output device.
    sample_rate: usize,
    buffer: rb::Producer<f32>,
    handle: EngineHandle,
}

impl CpalOutputDevice {
    pub fn start(handle: super::engine::EngineHandle) -> Option<Self> {
        let host = cpal::default_host();

        let _device: cpal::Device = host
            .default_output_device()?;
            // .expect("failed to find output device");

        let config = _device.default_output_config().unwrap();

        let buf_ms: usize = 64;
        let channels = config.channels() as usize;
        let sample_rate = config.sample_rate().0 as usize;
        let buffer_size = ((sample_rate * channels) as f32 * (buf_ms as f32 / 1000.0)) as usize;
        
        dbg!(buffer_size);

        let rb = rb::SpscRb::<f32>::new(buffer_size);
        let (tx, rx) = (rb.producer(), rb.consumer());

        let write_silence = |data: &mut [f32]| data.iter_mut().for_each(|f| *f = 0.0);

        let _stream: cpal::Stream = _device
            .build_output_stream(
                &config.into(),
                // todo: use read_blocking instead?
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    for frame in data.chunks_mut(channels) {

                        match rx.read(frame) {
                    // Some(_) => {},
                    // None => {},
                    Ok(written) => {
                        // if written != frame.len() {
                        //     // fill remaining buffer with silence
                        //     write_silence(&mut frame[written..]);
                        // }
                    }

                    // Write silence if buffer is empty
                    Err(_) => (),
                    //  write_silence(frame),
                    }
                }
                },
                |e| println!("{e}"),
                None,
            )
            .unwrap();

        // start the stream    
        _stream.play().unwrap();

        Some(Self {
            _device,
            _stream,
            buffer: tx,
            sample_rate,
            handle,
        })
    }

    pub fn write(&self, frame: Frame) {
        self.write_batch(&[frame]);
    }

    pub fn write_batch(&self, frames: impl AsRef<[Frame]>) {
        // write_blocking is necessaray to ensure ALL samples are written
        let _ = self.buffer.write_blocking(bytemuck::cast_slice(&frames.as_ref()));
    }

    pub fn output_rate(&self) -> usize {
        self.sample_rate
    }

}

impl AudioOutputDevice for CpalOutputDevice {
    fn init(handle: super::engine::EngineHandle) -> Option<Box<Self>> {
        Self::start(handle).map(Box::new)
    }

    fn rate(&self) -> u32 {
        self.sample_rate as u32
    }

    fn reset(&mut self) {
        if let Some(device) = Self::start(self.handle.to_owned()) {
            *self = device
        }
    }

    fn write(&mut self, chunk: &[[f32; 2]]) {
        self.write_batch(chunk)
    }
}