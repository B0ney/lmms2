use std::f32::consts::E;
use std::sync::Arc;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Sample;

use crate::core::SampleBuffer;
use std::sync::mpsc::{self, Sender};

enum Event {
    CurrentFrame(usize),
    Finished
}

pub fn test(sample: SampleBuffer) {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("failed to find output device");

    println!("Output device: {}", device.name().unwrap());

    let config = device.default_output_config().unwrap();
    println!("Default output config: {:?}", config);

    let config = device.default_output_config().unwrap();
    println!("Default output config: {:?}", config);

    run(&device, &config.into(), Arc::new(sample));
}

fn run(device: &cpal::Device, config: &cpal::StreamConfig, sample: Arc<SampleBuffer>) {
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // let frame = Arc::new(sample);
    let mut frame: f32 = 0.0;

    let mut next_value = move || {
        let samples = sample.frame(frame.round() as usize);
        frame += 1.0;
        samples
    };

    let (tx, rx) = mpsc::channel::<Event>();

    let stream = device
        .build_output_stream(
            config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                write_data(data, channels, &mut next_value, tx.clone() )
            },
            |_| {},
            None,
        )
        .unwrap();


    stream.play().unwrap();
    
    loop {
        match rx.recv() {
            Ok(event) => match event {
                Event::CurrentFrame(frame) => {
                    // dbg!("Decoded frame: {}", frame);
                },
                Event::Finished => {
                    println!("Done!");
                    break;
                },
            }
            Err(_) => break,
        }
    }
}

fn write_data(
    output: &mut [f32],
    channels: usize,
    next_sample: &mut dyn FnMut() -> Option<Box<[f32]>>,
    sender: Sender<Event>,
) {
    for channel_out in output.chunks_mut(channels) {
        let value = next_sample().unwrap_or_default();

        for (frame, sample) in channel_out.iter_mut().enumerate() {
            let Some(idx)  = value.get(frame) else {
                let _  = sender.send(Event::Finished);
                return;
            };

            sender.send(Event::CurrentFrame(frame));

            *sample = *idx;
        }
    }
}
