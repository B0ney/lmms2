use crate::core::midi::note::Note as MIDINote;

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum Key {
    A,
    B,
    #[default]
    C,

}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Note {
    key: Key,
    /// is an f32 for easy conversion
    /// 0.0..=1.0
    velocity: f32,
    
    octave: u8,
    
    position: u32,
    
    /// -1.0..=1.0
    panning: f32,

    /// in what units?
    length: u32,
    // pitch:
}