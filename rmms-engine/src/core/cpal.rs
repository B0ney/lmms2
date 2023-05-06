use std::sync::Arc;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Sample;
use rb::{RbConsumer, RbProducer, RB};

use crate::core::SampleBuffer;
use std::sync::mpsc::{self, Sender};

use super::engine::SampleFrame;

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
}

impl CpalOutputDevice {
    pub fn init() -> Self {
        let host = cpal::default_host();

        let _device: cpal::Device = host
            .default_output_device()
            .expect("failed to find output device");

        let config = _device.default_output_config().unwrap();

        let buf_ms: usize = 128;
        let channels = 2;
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
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| match rx.read(data) {
                    // Some(_) => {},
                    // None => {},
                    Ok(written) => {
                        if written != data.len() {
                            // fill remaining buffer with silence
                            write_silence(&mut data[written..]);
                        }
                    }

                    // Write silence if buffer is empty
                    Err(_) => write_silence(data),
                },
                |e| println!("{e}"),
                None,
            )
            .unwrap();

        // start the stream    
        _stream.play().unwrap();

        Self {
            _device,
            _stream,
            buffer: tx,
            sample_rate,
        }
    }

    pub fn write(&self, frame: SampleFrame) {
        self.write_batch(&[frame]);
    }

    pub fn write_batch(&self, frames: impl AsRef<[SampleFrame]>) {
        // write_blocking is necessaray to ensure ALL samples are written
        let _ = self.buffer.write_blocking(bytemuck::cast_slice(&frames.as_ref()));
    }

    pub fn output_rate(&self) -> usize {
        self.sample_rate
    }

}
