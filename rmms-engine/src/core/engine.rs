use std::{collections::HashSet, sync::Arc};

use super::SampleCache;

const DEFAULT_RATE: usize = 44100;
const DEFAULT_CHANNELS: usize = 2;

/// The sample frame used by the engine (produced by the mixer)
pub type SampleFrame = [f32; DEFAULT_CHANNELS];

/// Ring buffer size in frames. 
/// The latency (in ms) can be calculated with the following equation:
/// 
/// (BUFFER_SIZE / SAMPLE_RATE) * 1000
const DEFAULT_BUFFER_SIZE: usize = 2048; 

pub enum Event {

}

struct MidiOutputDevice {

}

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
pub trait AudioInputDevice {}

pub trait AudioOutputDevice {}

struct Dummy;

impl AudioInputDevice for Dummy {}
impl AudioOutputDevice for Dummy {}


pub struct EngineConfiguration {

}
pub struct Mixer;

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
pub struct Engine {
    config: EngineConfiguration,

    midi_in: HashSet<Arc<MidiInputDevice>>,
    midi_out: HashSet<Arc<MidiOutputDevice>>,

    /// The output device is where audio data is streamed to.
    /// Can be changed at runtime
    output_device: Box<dyn AudioOutputDevice>,
    
    /// Input device to recieve audio streams 
    input_device: Arc<dyn AudioInputDevice>,

    /// Frames written to the output device must have a matching sample rate
    /// The resampler can help
    resampler: Option<Resampler>,

    
    
    rb_consumber: rb::Consumer<Event>,

    sample_cache: SampleCache,
    mixer: Mixer,
}

struct Resampler;