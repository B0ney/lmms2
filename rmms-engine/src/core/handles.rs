pub mod file;
pub mod panner;
pub mod resampler;
pub mod note;

pub trait PlayHandle: Send + 'static {
    fn is_complete(&self) -> bool;
    fn next(&mut self) -> Option<[f32; 2]>;
    fn render(&mut self, frames: &mut [[f32; 2]]) -> Option<usize> {
        let mut written: usize = 0;

        for frame in frames.iter_mut() {
            match self.next() {
                Some(f) => {
                    frame[0] += f[0]; // todo: mix or overwrite?
                    frame[1] += f[1];
                    written += 1;
                }
                None => {}
            }
        }
        Some(written)
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
        P: PlayHandle
    {
        other(self)
    }
}

impl <P: PlayHandle>PlayHandleChain for P {}




struct Handle {
    playhandle: Box<dyn PlayHandle>
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