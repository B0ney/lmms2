use crate::core::playback::AudioOutputDevice;

pub enum Event {

}

struct MidiOutputDevice {

}

struct MidiInputDevice;



pub struct Engine {
    midi_in:MidiInputDevice,
    midi_out: MidiOutputDevice,
    output_stream: Box<dyn AudioOutputDevice>,
    
    rb_consumber: rb::Consumer<Event>
}