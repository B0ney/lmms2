use super::traits::PlayHandle;

pub enum Event {
    RequestAudioDeviceReset,
    PushPlayHandle(Box<dyn PlayHandle>),
    Clear,
}

impl Event {
    pub fn play_handle(play: impl PlayHandle + 'static) -> Self {
        Self::PushPlayHandle(Box::new(play))
    }
}