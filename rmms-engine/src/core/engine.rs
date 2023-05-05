use std::{collections::HashSet, sync::Arc};

use super::SampleCache;

const DEFAULT_RATE: usize = 44100;
const DEFAULT_CHANNELS: usize = 2;

/// The sample frame used by the engine (produced by the mixer)
type SampleFrame = [f32; DEFAULT_CHANNELS];

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


trait AudioInputDevice {}
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
    
    rb_consumber: rb::Consumer<Event>,

    sample_cache: SampleCache,
    mixer: Mixer,
}