use std::fs::File;
// use std::io::{BufReader, Cursor};
use std::path::Path;
use std::sync::Arc;

use crate::core::cpal::CpalOutputDevice;
use crate::core::traits::FrameModifier;
use crate::core::SampleBuffer as RmmsSampleBuffer;

use symphonia::core::audio::{
    AudioBuffer, AudioBufferRef, AudioPlanes, Channels, RawSampleBuffer, SampleBuffer, Signal,
    SignalSpec,
};
use symphonia::core::codecs::{CodecRegistry, Decoder, DecoderOptions};
use symphonia::core::errors::Error;
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::{MediaSource, MediaSourceStream, MediaSourceStreamOptions};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::{Hint, Probe};
use symphonia::core::units::Duration;

/// Supported formats:
///
/// WAV
/// AIFF
/// FLAC
/// OGG
/// MP3
pub struct SymphoniaDecoder {
    registry: &'static CodecRegistry,
    probe: &'static Probe,
    options: MediaSourceStreamOptions,
    /// TODO: seek_index_fill_rate has a cpu vs memory tradeoff
    format_options: FormatOptions,
}

impl SymphoniaDecoder {
    pub fn load_from_file(path: impl AsRef<Path>) -> RmmsSampleBuffer {
        let path = path.as_ref();
        let extension = path.extension().map(|f| f.to_str()).flatten();

        let file = File::open(path).unwrap();
        Self::load_from_reader(file, extension)
    }

    pub fn load_from_reader<R: MediaSource + 'static>(
        reader: R,
        extension: Option<&str>,
    ) -> RmmsSampleBuffer {
        let decoder = Self::init();

        let source = Box::new(reader) as Box<dyn MediaSource>;
        let source_stream = MediaSourceStream::new(source, decoder.options);

        let mut hint = Hint::new();
        let hint = match extension {
            Some(extension) => hint.with_extension(extension),
            None => &mut hint,
        };

        let mut format_reader = decoder
            .probe
            .format(
                &hint,
                source_stream,
                &decoder.format_options,
                &MetadataOptions::default(),
            )
            .unwrap()
            .format;

        let options = DecoderOptions::default();
        let codec_params = &format_reader.default_track().expect("tracks").codec_params;

        let mut decoder = decoder
            .registry
            .make(codec_params, &options)
            .expect("failed to instantiate decoder");

        let rate = codec_params.sample_rate.unwrap_or(44100);
        let channels = codec_params.channels.unwrap_or_default().count();

        dbg!("loading audio data to buffer...");

        let mut audio_buffer: Option<SampleContainer> = None;
        let mut out_buffer: Vec<Vec<f32>> = vec![Vec::new(); channels];
        let mut errors = 0;

        loop {
            if errors > 100 {
                dbg!("too many errors!");
                break;
            }

            match format_reader.next_packet() {
                Ok(packet) => match decoder.decode(&packet) {
                    Ok(decoded) => {
                        // skip empty frames
                        if decoded.frames() == 0 {
                            continue;
                        }

                        let audio_buffer =
                            audio_buffer.get_or_insert(SampleContainer::new(&decoded));

                        audio_buffer.append(decoded);

                        for (in_channel, out_channel) in
                            audio_buffer.frames().zip(out_buffer.iter_mut())
                        {
                            out_channel.extend_from_slice(in_channel)
                        }

                        continue;
                    }
                    Err(_) => (),
                },
                Err(Error::IoError(io)) => match io.kind() {
                    std::io::ErrorKind::UnexpectedEof => break,
                    _ => (),
                },
                _ => (),
            }

            errors += 1;
        }
        
        let mut sample = RmmsSampleBuffer::new(out_buffer, rate);
        crate::core::dsp::resample(&mut sample, 44100);
        sample
    }

    pub fn init() -> Self {
        Self {
            registry: symphonia::default::get_codecs(),
            probe: symphonia::default::get_probe(),
            options: MediaSourceStreamOptions::default(),
            format_options: FormatOptions {
                enable_gapless: true,
                ..Default::default()
            },
        }
    }
}

struct SampleContainer {
    channels: u16,
    sample_buffer: SampleBuffer<f32>,
}

impl SampleContainer {
    fn new(audio: &AudioBufferRef) -> Self {
        let spec = *audio.spec();

        let channels = spec.channels.count() as u16;
        let duration = audio.capacity() as u64;

        let sample_buffer = SampleBuffer::<f32>::new(duration, spec);

        Self {
            channels,
            sample_buffer,
        }
    }

    fn frames(&self) -> impl Iterator<Item = &[f32]> {
        let chunk_size = self.sample_buffer.samples().len() / self.channels as usize;
        self.sample_buffer.samples().chunks_exact(chunk_size)
    }

    fn append(&mut self, decoded: AudioBufferRef) {
        self.sample_buffer.copy_planar_ref(decoded)
    }
}

fn output_to_wav(audio_data: &RmmsSampleBuffer, file: impl AsRef<Path>) {
    let spec = hound::WavSpec {
        channels: audio_data.channels() as u16,
        sample_rate: audio_data.sample_rate_original,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create(file, spec).unwrap();

    for sample_frame in audio_data.iter_frames() {
        for sample in sample_frame.as_ref() {
            writer.write_sample(*sample).unwrap()
        }
    }
}

/// cargo test --release --package rmms-engine --lib -- core::decoder::tests --nocapture
// #[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::core::{engine::AudioEngine, event::Event, sample::Sample, SampleCache};

    use super::SymphoniaDecoder;

    #[test]
    fn h() {
        let cache = SampleCache::new();
        let kick = cache.add(
            "kick",
            SymphoniaDecoder::load_from_file("../audio/kick.wav"),
        );
        let snare = cache.add(
            "snare",
            SymphoniaDecoder::load_from_file("../audio/snare.wav"),
        );
        let crash = cache.add(
            "crash",
            SymphoniaDecoder::load_from_file("../audio/crash_5.wav"),
        );
        let jungle = cache.add(
            "jungle",
            SymphoniaDecoder::load_from_file("../audio/jungle.wav"),
        );

        let kick = Sample::new(kick);
        let snare = Sample::new(snare);
        let crash = Sample::new(crash);
        let jungle = Sample::new(jungle);

        let handle = AudioEngine::new();

        let send = |sample: Sample, msg: &str| {
            handle.send(Event::play_handle(sample));
            println!("{msg}"); // Don't use this in time critical events like this
        };

        let play_kick = || send(kick.clone(), "kick");
        let play_snare = || send(snare.clone(), "   snare");
        let play_crash = || send(crash.clone(), "       crash");
        let play_jungle = || send(jungle.clone(), "jungle");

        let sleep_ms = |time: u64| std::thread::sleep(Duration::from_millis(time));
        let clear_handles = || handle.send(Event::Clear);

        sleep_ms(500);

        println!("drums");
        for _ in 0..5 {
            for _ in 0..4 {
                play_kick();
                sleep_ms(256);

                play_snare();
                sleep_ms(256);

                play_kick();
                sleep_ms(128);
                play_kick();
                sleep_ms(128);

                play_snare();
                sleep_ms(256);
            }
            play_crash()
        }

        println!("playing jungle beat");
        sleep_ms(500);

        play_jungle();
        sleep_ms(250);

        for _ in 0..2 {
            clear_handles();
            play_jungle();
            sleep_ms(128);
        }

        for _ in 0..2 {
            clear_handles();
            play_jungle();
            sleep_ms(256);
        }

        clear_handles();
        play_jungle();
        sleep_ms(3500);

        clear_handles();
        play_jungle();
        sleep_ms(256);

        for _ in 0..3 {
            clear_handles();
            play_jungle();
            sleep_ms(5580);
        }
    }
}
