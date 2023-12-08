pub mod file;
pub mod note;
pub mod panner;
pub mod resampler;
mod metronome;

pub trait PlayHandle: Send + 'static {
    /// Indicate that this playhandle will no longer produce frames.
    fn is_complete(&self) -> bool;

    /// Produce the next frame.
    /// 
    /// Returns None if the playhandle has terminated.
    fn next(&mut self) -> Option<[f32; 2]>;
    
    /// Write frames into a buffer.
    /// 
    /// Returns how many frames were written.
    fn render(&mut self, frames: &mut [[f32; 2]]) -> usize {
        let mut written: usize = 0;

        for frame in frames.iter_mut() {
            let Some([l, r]) = self.next() else { 
                break 
            };

            let [out_l, out_r] = frame;

            *out_l += l; // todo: mix or overwrite?
            *out_r += r;

            written += 1;
        }
        written
    }
}

impl PlayHandle for Box<dyn PlayHandle> {
    fn next(&mut self) -> Option<[f32; 2]> {
        (**self).next()
    }

    fn is_complete(&self) -> bool {
        (**self).is_complete()
    }
}


pub trait PlayHandleChain: Sized {
    fn chain<F, P>(self, other: F) -> P
    where
        F: FnOnce(Self) -> P,
        P: PlayHandle,
    {
        other(self)
    }
}

impl<P: PlayHandle> PlayHandleChain for P {}

struct Handle {
    playhandle: Box<dyn PlayHandle>,
}

impl PlayHandle for Handle {
    fn is_complete(&self) -> bool {
        self.playhandle.is_complete()
    }

    fn next(&mut self) -> Option<[f32; 2]> {
        self.playhandle.next()
    }
}

impl Handle {
    // pub fn map(self, f impl Fn(Self) -> B)
}

// impl From
