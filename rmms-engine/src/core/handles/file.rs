use std::fs::File;
// use std::io::{BufReader, Cursor};
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use crate::core::cpal::CpalOutputDevice;
use crate::core::traits::{FrameModifier};
use crate::core::handles::PlayHandle;
use crate::core::SampleBuffer as RmmsSampleBuffer;

use symphonia::core::audio::{
    AudioBuffer, AudioBufferRef, AudioPlanes, Channels, RawSampleBuffer, SampleBuffer, Signal,
    SignalSpec,
};
use symphonia::core::codecs::{CodecRegistry, Decoder, DecoderOptions};
use symphonia::core::errors::Error;
use symphonia::core::formats::{FormatOptions, FormatReader, Packet};
use symphonia::core::io::{MediaSource, MediaSourceStream, MediaSourceStreamOptions};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::{Hint, Probe};
use symphonia::core::units::Duration;

pub struct SymphoniaDecoder {
    registry: &'static CodecRegistry,
    probe: &'static Probe,
    options: MediaSourceStreamOptions,
    /// TODO: seek_index_fill_rate has a cpu vs memory tradeoff
    format_options: FormatOptions,
}

pub struct Stream {
    reader: Box<dyn FormatReader>,
    decoder: Box<dyn Decoder>,
    // sender: Sender<[f32; 2]>,
    raw_sample: RawSampleBuffer<f32>,
}

pub struct StreamHandle(Receiver<[f32; 2]>);

// impl PlayHandle for Stream {
//     fn next(&mut self) -> Option<[f32; 2]> {
//         match self.reader.next_packet() {
//             Ok(packet) => {
//                 match self.decoder.decode(&packet) {
//                     Ok(decoded) => {
//                         // skip empty frames
//                         // if decoded.frames() == 0 {
//                         //     continue;
//                         // }
//                         self.raw_sample.copy_interleaved_ref(decoded);
//                         // self.raw_sample.
//                         // let mut smp = SampleBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec());
//                         // smp.copy_planar_ref(decoded);
//                         // todo!()
//                         // let audio_buffer =
//                         //     audio_buffer.get_or_insert(SampleContainer::new(&decoded));

//                         // audio_buffer.append(decoded);

//                         // for (in_channel, out_channel) in
//                         //     audio_buffer.frames().zip(out_buffer.iter_mut())
//                         // {
//                         //     out_channel.extend_from_slice(in_channel)
//                         // }

//                         // continue;
//                     }
//                     Err(_) => todo!(),
//                 }
//             }
//             Err(_) => None,
//         }
//     }

//     fn reset(&mut self) {
//         todo!()
//     }

//     fn jump(&mut self, tick: usize) {
//         todo!()
//     }
// }

// impl Stream {
//     pub fn new<R>(reader: R, extension: Option<&str>) -> Self
//     where
//         R: MediaSource + 'static,
//     {
//         let decoder = SymphoniaDecoder::init();
//         let source = Box::new(reader) as Box<dyn MediaSource>;
//         let source_stream = MediaSourceStream::new(source, decoder.options);
//         let mut hint = Hint::new();
//         let hint = match extension {
//             Some(extension) => hint.with_extension(extension),
//             None => &mut hint,
//         };

//         let mut format_reader = decoder
//             .probe
//             .format(
//                 &hint,
//                 source_stream,
//                 &decoder.format_options,
//                 &MetadataOptions::default(),
//             )
//             .unwrap()
//             .format;

//         let options = DecoderOptions::default();
//         let codec_params = &format_reader.default_track().expect("tracks").codec_params;

//         let mut decoder = decoder
//             .registry
//             .make(codec_params, &options)
//             .expect("failed to instantiate decoder");

//         let rate = codec_params.sample_rate.unwrap_or(44100);
//         let channels = codec_params.channels.unwrap_or_default().count();

//         Self {
//             reader: format_reader,
//             decoder,
//             // sender: todo!(),
//             raw_sample: RawSampleBuffer::<f32>::new(),
//         }
//     }
// }

// impl SymphoniaDecoder {
//     pub fn load_from_file(path: impl AsRef<Path>) -> RmmsSampleBuffer {
//         let path = path.as_ref();
//         let extension = path.extension().map(|f| f.to_str()).flatten();

//         let file = File::open(path).unwrap();
//         Self::load_from_reader(file, extension)
//     }

//     pub fn load_from_reader<R: MediaSource + 'static>(
//         reader: R,
//         extension: Option<&str>,
//     ) -> RmmsSampleBuffer {
//         let decoder = Self::init();

//         let source = Box::new(reader) as Box<dyn MediaSource>;
//         let source_stream = MediaSourceStream::new(source, decoder.options);

//         let mut hint = Hint::new();
//         let hint = match extension {
//             Some(extension) => hint.with_extension(extension),
//             None => &mut hint,
//         };

//         let mut format_reader = decoder
//             .probe
//             .format(
//                 &hint,
//                 source_stream,
//                 &decoder.format_options,
//                 &MetadataOptions::default(),
//             )
//             .unwrap()
//             .format;

//         let options = DecoderOptions::default();
//         let codec_params = &format_reader.default_track().expect("tracks").codec_params;

//         let mut decoder = decoder
//             .registry
//             .make(codec_params, &options)
//             .expect("failed to instantiate decoder");

//         let rate = codec_params.sample_rate.unwrap_or(44100);
//         let channels = codec_params.channels.unwrap_or_default().count();

//         dbg!("loading audio data to buffer...");

//         let mut audio_buffer: Option<SampleContainer> = None;
//         let mut out_buffer: Vec<Vec<f32>> = vec![Vec::new(); channels];
//         let mut errors = 0;

//         loop {
//             if errors > 100 {
//                 dbg!("too many errors!");
//                 break;
//             }

//             match format_reader.next_packet() {
//                 Ok(packet) => match decoder.decode(&packet) {
//                     Ok(decoded) => {
//                         // skip empty frames
//                         if decoded.frames() == 0 {
//                             continue;
//                         }

//                         let audio_buffer =
//                             audio_buffer.get_or_insert(SampleContainer::new(&decoded));

//                         audio_buffer.append(decoded);

//                         for (in_channel, out_channel) in
//                             audio_buffer.frames().zip(out_buffer.iter_mut())
//                         {
//                             out_channel.extend_from_slice(in_channel)
//                         }

//                         continue;
//                     }
//                     Err(_) => (),
//                 },
//                 Err(Error::IoError(io)) => match io.kind() {
//                     std::io::ErrorKind::UnexpectedEof => break,
//                     _ => (),
//                 },
//                 _ => (),
//             }

//             errors += 1;
//         }

//         let mut sample = RmmsSampleBuffer::new(out_buffer, rate);
//         // crate::core::dsp::resample(&mut sample, 44100);
//         sample
//     }

//     pub fn init() -> Self {
//         Self {
//             registry: symphonia::default::get_codecs(),
//             probe: symphonia::default::get_probe(),
//             options: MediaSourceStreamOptions::default(),
//             format_options: FormatOptions {
//                 enable_gapless: true,
//                 ..Default::default()
//             },
//         }
//     }
// }
