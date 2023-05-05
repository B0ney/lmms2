#[derive(Debug, Default, Clone, Copy)]
pub enum ADSRState {
    #[default]
    Attack,
    Decay,
    Sustain,
    Release,
}

impl ADSRState {
    pub fn next(&self) -> Self {
        match self {
            Self::Attack => Self::Decay,
            Self::Decay => Self::Sustain,
            Self::Sustain => Self::Release,
            // todo: what to do at this stage?
            Self::Release => Self::Release,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ADSR {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    
    state: ADSRState,
}

impl From<(f32, f32, f32, f32)> for ADSR {
    fn from((attack, decay, sustain, release): (f32, f32, f32, f32)) -> Self {
        Self {
            attack,
            decay,
            sustain,
            release,
            ..Default::default()
        }
    }
}
