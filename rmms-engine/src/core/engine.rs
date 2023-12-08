use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicUsize, Ordering, AtomicU32},
        mpsc, Arc,
    },
};

use crate::core::cpal::CpalOutputDevice;

use super::{
    event::Event,
    traits::{AudioInputDevice, AudioOutputDevice, FrameModifier},
    SampleCache,
};
use super::handles::PlayHandle;

const DEFAULT_RATE: usize = 44100;
const DEFAULT_CHANNELS: usize = 2;

/// The sample frame used by the engine (produced by the mixer)
pub type Frame = [f32; DEFAULT_CHANNELS];

/// Ring buffer size in frames.
/// The latency (in ms) can be calculated with the following equation:
///
/// (BUFFER_SIZE / SAMPLE_RATE) * 1000
const DEFAULT_BUFFER_SIZE: usize = 2048;

struct MidiOutputDevice {}

struct MidiInputDevice;

/// When recording audio, a ring buffer is needed to "store" the input audio.
/// The audio data from the ring buffer is asynchronously streamed to a file.
/// Since the file is a raw PCM, it can be easily recovered. (It will take up more space though, need feedback)
///
///
/// If I am recording 16-bit mono audio at a sample rate of 44100 Hz,
/// the minimun disk write speed (KB/s) can be calculated with the following equation:
///
/// ( SAMPLE_RATE * BYTES_PER_SAMPLE * CHANNELS ) / 1000     (or 1024 if we want KiB/s)
///
/// (44100 * 2 * 1) / 1000 = 88.2 KB/s
///
/// It's not that much! :P
///
/// But when picking a ring buffer size,
/// add extra breathing room because io-bound operations can be unpredictable.
/// Say an extra 64 ms of audio?
///
/// 64 ms = (64 / 1000) * 44100 = 2822.4 -> 2823 extra frames
///
/// A "frame" is a unit of representing a group of N channel samples.
///
/// If I am recording 16 bit 2 channel audio, a sample frame would be ``[i16, i16]``
///
///

struct Dummy;

impl AudioInputDevice for Dummy {}
impl AudioOutputDevice for Dummy {
    fn init(handle: EngineHandle) -> Option<Box<Self>> {
        Some(Box::new(Self))
    }

    fn rate(&self) -> u32 {
        DEFAULT_RATE as u32
    }

    fn reset(&mut self) {}

    fn write(&mut self, chunk: &[[f32; 2]]) {}
}

pub struct EngineConfiguration {}
pub struct Mixer;

#[derive(Clone)]
pub struct EngineHandle {
    tx: mpsc::Sender<Event>,
    engine_rate: Arc<AtomicU32>,
}

impl EngineHandle {
    pub fn send(&self, event: Event) {
        self.tx.send(event);
    }
    pub fn output_device_sample_rate(&self) -> u32 {
        self.engine_rate.load(Ordering::Relaxed)
    }
}

/*
TODO: inlcude Time somewhere

[Midi Input (maybe)]
        |                    |-> [Midi Output]
        \                    |
    Instruments---->|------->|
                    |
    Audio Streams-->|
                    |---> SampleFrames -> Mixer -> RingBuffer -> [Output Device]

                            TODO: where do filters/effects fit?
*/

pub struct AudioEngine {
    // config: EngineConfiguration,

    // midi_in: HashSet<Arc<MidiInputDevice>>,
    // midi_out: HashSet<Arc<MidiOutputDevice>>,
    /// The output device is where audio data is streamed to.
    /// Can be changed at runtime
    output_device: Box<dyn AudioOutputDevice>,

    /// Input device to recieve audio streams
    // input_device: Arc<dyn AudioInputDevice>,

    /// Frames written to the output device must have a matching sample rate
    /// The resampler can help
    // resampler: Option<Resampler>,

    // rb_consumber: rb::Consumer<Event>,
    pub sample_cache: SampleCache,
    mixer: Mixer,

    handles: Vec<Box<dyn PlayHandle>>,
}

impl AudioEngine {
    fn init(handle: EngineHandle) -> Self {
        let output_device: Box<dyn AudioOutputDevice> = match CpalOutputDevice::init(handle.clone())
        {
            Some(device) => device,
            None => Dummy::init(handle.clone()).expect("Dummy should be infallible"),
        };

        handle
            .engine_rate
            .store(output_device.rate(), Ordering::Relaxed);

        Self {
            output_device,
            sample_cache: SampleCache::default(),
            mixer: Mixer,
            handles: Vec::with_capacity(128),
        }
    }

    pub fn new() -> EngineHandle {
        let (tx, rx) = std::sync::mpsc::channel::<Event>();
        let engine_rate = Arc::new(AtomicU32::new(44100));
        let handle = EngineHandle {
            tx,
            engine_rate: engine_rate.clone(),
        };

        let audio_handle = handle.clone();
        std::thread::spawn(move || {
            let mut engine = AudioEngine::init(audio_handle);
            loop {
                for event in rx.try_iter().take(32) {
                    engine.handle_event(event);
                }
                engine.tick();
            }
        });

        handle
    }

    pub fn tick(&mut self) {
        let mut frame: Frame = [0.0, 0.0];
        let mut index: usize = 0;

        while index < self.handles.len() {
            let handle = &mut self.handles[index];

            let Some(next_frame) = handle.next() else {
                self.handles.swap_remove(index);
                continue;
            };

            frame[0] += next_frame[0];
            frame[1] += next_frame[1];

            index += 1;
        }

        self.output_device.write(&[frame.amplify(0.75).clamp()]);
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::RequestAudioDeviceReset => self.output_device.reset(),
            Event::PushPlayHandle(play_handle) => self.handles.push(play_handle),
            // Event::PlayEvent(state) => self.state = state,
            Event::Clear => self.handles.clear(),
        }
    }
}
struct Resampler;
