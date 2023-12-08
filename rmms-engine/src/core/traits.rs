use super::engine::EngineHandle;

pub trait AudioInputDevice {}

pub trait AudioOutputDevice {
    fn init(handle: EngineHandle) -> Option<Box<Self>> where Self: Sized;
    fn rate(&self) -> u32;
    fn reset(&mut self);
    fn write(&mut self, chunk: &[[f32; 2]]);
}



pub trait FrameModifier {
    fn clamp(self) -> Self;
    fn amplify(self, value: f32) -> Self;
    fn force_channel(self, channel: usize) -> Self;
    fn swap_channels(self) -> Self;
}

impl FrameModifier for [f32; 2] {
    fn clamp(self) -> Self {
        self.map(|s| s.clamp(-1.0, 1.0))
    }

    fn amplify(self, value: f32) -> Self {
        self.map(|s| (s * value))
    }

    fn force_channel(mut self, channel: usize) -> Self {
        let sample = self[channel];
        self.fill(sample);
        self
    }

    fn swap_channels(mut self) -> Self {
        self.reverse();
        self
    }
}
